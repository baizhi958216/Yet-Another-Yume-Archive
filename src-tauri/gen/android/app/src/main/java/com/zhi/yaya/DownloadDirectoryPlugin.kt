package com.zhi.yaya

import android.app.Activity
import android.content.ContentValues
import android.content.Intent
import android.net.Uri
import android.os.Build
import android.os.Environment
import android.provider.MediaStore
import androidx.activity.result.ActivityResult
import androidx.documentfile.provider.DocumentFile
import app.tauri.Logger
import app.tauri.annotation.ActivityCallback
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import java.io.File
import java.io.FileInputStream
import java.io.InputStream
import java.io.OutputStream
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.atomic.AtomicBoolean

@InvokeArg
class ExportFileArg {
  lateinit var path: String
  lateinit var name: String
  lateinit var mimeType: String
}

@InvokeArg
class ExportFilesArgs {
  lateinit var id: String
  lateinit var files: Array<ExportFileArg>
  var subdirectory: String? = null
}

@InvokeArg
class CancelExportArgs {
  lateinit var id: String
}

@TauriPlugin
class DownloadDirectoryPlugin(private val activity: Activity) : Plugin(activity) {
  private val exportCancellations = ConcurrentHashMap<String, AtomicBoolean>()
  private val canceledExports = ConcurrentHashMap.newKeySet<String>()
  private val preferences by lazy {
    activity.getSharedPreferences("yaya_download_directory", Activity.MODE_PRIVATE)
  }

  @Command
  fun getDirectory(invoke: Invoke) {
    invoke.resolve(directoryResult())
  }

  @Command
  fun pickDirectory(invoke: Invoke) {
    val intent = Intent(Intent.ACTION_OPEN_DOCUMENT_TREE).apply {
      addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
      addFlags(Intent.FLAG_GRANT_WRITE_URI_PERMISSION)
      addFlags(Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION)
      addFlags(Intent.FLAG_GRANT_PREFIX_URI_PERMISSION)
      storedTreeUri()?.let { putExtra("android.provider.extra.INITIAL_URI", it) }
    }
    startActivityForResult(invoke, intent, "directoryPickerResult")
  }

  @ActivityCallback
  fun directoryPickerResult(invoke: Invoke, result: ActivityResult) {
    if (result.resultCode != Activity.RESULT_OK) {
      invoke.reject("未选择下载文件夹")
      return
    }
    val uri = result.data?.data
    if (uri == null) {
      invoke.reject("系统没有返回所选文件夹")
      return
    }
    try {
      val previous = storedTreeUri()
      val flags = (result.data?.flags ?: 0) and
        (Intent.FLAG_GRANT_READ_URI_PERMISSION or Intent.FLAG_GRANT_WRITE_URI_PERMISSION)
      activity.contentResolver.takePersistableUriPermission(uri, flags)
      if (previous != null && previous != uri) {
        try {
          activity.contentResolver.releasePersistableUriPermission(
            previous,
            Intent.FLAG_GRANT_READ_URI_PERMISSION or Intent.FLAG_GRANT_WRITE_URI_PERMISSION,
          )
        } catch (_: SecurityException) {
          // The previous provider may already have revoked the grant.
        }
      }
      val name = DocumentFile.fromTreeUri(activity, uri)?.name ?: "已选择的文件夹"
      preferences.edit().putString("uri", uri.toString()).putString("name", name).apply()
      invoke.resolve(directoryResult())
    } catch (error: Exception) {
      Logger.error("Failed to retain download directory permission", error)
      invoke.reject(error.message ?: "无法获得文件夹写入权限")
    }
  }

  @Command
  fun exportFiles(invoke: Invoke) {
    val args = try {
      invoke.parseArgs(ExportFilesArgs::class.java)
    } catch (error: Exception) {
      invoke.reject(error.message ?: "无效的导出参数")
      return
    }
    val canceled = AtomicBoolean(canceledExports.remove(args.id))
    exportCancellations[args.id] = canceled
    Thread {
      try {
        val exported = if (storedTreeUri() != null) {
          exportToTree(args, canceled)
        } else {
          exportToDownloads(args, canceled)
        }
        val response = JSObject()
        response.put("files", JSArray.from(exported.toTypedArray()))
        invoke.resolve(response)
      } catch (error: Exception) {
        Logger.error("Failed to export downloaded files", error)
        invoke.reject(error.message ?: "无法写入所选下载文件夹")
      } finally {
        exportCancellations.remove(args.id)
        canceledExports.remove(args.id)
      }
    }.start()
  }

  @Command
  fun cancelExport(invoke: Invoke) {
    val args = invoke.parseArgs(CancelExportArgs::class.java)
    canceledExports.add(args.id)
    exportCancellations[args.id]?.set(true)
    invoke.resolve(JSObject())
  }

  private fun directoryResult(): JSObject {
    val result = JSObject()
    val uri = storedTreeUri()
    result.put("uri", uri?.toString())
    val fallbackName = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
      "Download/YAYA（系统默认）"
    } else {
      "请先选择下载文件夹"
    }
    result.put("name", preferences.getString("name", null) ?: fallbackName)
    return result
  }

  private fun storedTreeUri(): Uri? {
    val value = preferences.getString("uri", null) ?: return null
    val uri = Uri.parse(value)
    val stillGranted = activity.contentResolver.persistedUriPermissions.any {
      it.uri == uri && it.isWritePermission
    }
    if (!stillGranted) {
      preferences.edit().remove("uri").remove("name").apply()
      return null
    }
    return uri
  }

  private fun exportToTree(args: ExportFilesArgs, canceled: AtomicBoolean): List<JSObject> {
    var target = DocumentFile.fromTreeUri(activity, storedTreeUri()!!)
      ?: error("无法打开所选下载文件夹")
    sanitizedDirectory(args.subdirectory)?.let { directoryName ->
      target = target.findFile(directoryName)?.takeIf { it.isDirectory }
        ?: target.createDirectory(directoryName)
        ?: error("无法创建目录 $directoryName")
    }
    val created = mutableListOf<Uri>()
    return try {
      args.files.map { file ->
        ensureNotCanceled(canceled)
        val name = uniqueTreeName(target, safeFileName(file.name))
        val document = target.createFile(normalizeMime(file.mimeType), name)
          ?: error("无法创建文件 $name")
        created.add(document.uri)
        FileInputStream(File(file.path)).use { input ->
          activity.contentResolver.openOutputStream(document.uri, "w").use { output ->
            requireNotNull(output) { "无法写入文件 $name" }
            copy(input, output, canceled)
          }
        }
        exportedResult(file.path, document.uri, name)
      }
    } catch (error: Exception) {
      created.forEach { activity.contentResolver.delete(it, null, null) }
      throw error
    }
  }

  private fun exportToDownloads(args: ExportFilesArgs, canceled: AtomicBoolean): List<JSObject> {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) {
      error("请先在设置中选择下载文件夹")
    }
    val relativeDirectory = buildString {
      append(Environment.DIRECTORY_DOWNLOADS)
      append("/YAYA")
      sanitizedDirectory(args.subdirectory)?.let { append("/").append(it) }
      append("/")
    }
    val created = mutableListOf<Uri>()
    return try {
      args.files.map { file ->
        ensureNotCanceled(canceled)
        val name = uniqueMediaName(relativeDirectory, safeFileName(file.name))
        val values = ContentValues().apply {
          put(MediaStore.MediaColumns.DISPLAY_NAME, name)
          put(MediaStore.MediaColumns.MIME_TYPE, normalizeMime(file.mimeType))
          put(MediaStore.MediaColumns.RELATIVE_PATH, relativeDirectory)
          put(MediaStore.MediaColumns.IS_PENDING, 1)
        }
        val uri = activity.contentResolver.insert(
          MediaStore.Downloads.EXTERNAL_CONTENT_URI,
          values,
        ) ?: error("无法在系统下载目录创建文件 $name")
        created.add(uri)
        FileInputStream(File(file.path)).use { input ->
          activity.contentResolver.openOutputStream(uri, "w").use { output ->
            requireNotNull(output) { "无法写入文件 $name" }
            copy(input, output, canceled)
          }
        }
        values.clear()
        values.put(MediaStore.MediaColumns.IS_PENDING, 0)
        activity.contentResolver.update(uri, values, null, null)
        exportedResult(file.path, uri, name)
      }
    } catch (error: Exception) {
      created.forEach { activity.contentResolver.delete(it, null, null) }
      throw error
    }
  }

  private fun uniqueTreeName(directory: DocumentFile, requested: String): String {
    if (directory.findFile(requested) == null) return requested
    val (stem, extension) = splitName(requested)
    var index = 1
    while (true) {
      val candidate = "$stem ($index)$extension"
      if (directory.findFile(candidate) == null) return candidate
      index += 1
    }
  }

  private fun uniqueMediaName(relativeDirectory: String, requested: String): String {
    val existing = mutableSetOf<String>()
    activity.contentResolver.query(
      MediaStore.Downloads.EXTERNAL_CONTENT_URI,
      arrayOf(MediaStore.MediaColumns.DISPLAY_NAME),
      "${MediaStore.MediaColumns.RELATIVE_PATH} = ?",
      arrayOf(relativeDirectory),
      null,
    )?.use { cursor ->
      val column = cursor.getColumnIndexOrThrow(MediaStore.MediaColumns.DISPLAY_NAME)
      while (cursor.moveToNext()) existing.add(cursor.getString(column))
    }
    if (requested !in existing) return requested
    val (stem, extension) = splitName(requested)
    var index = 1
    while ("$stem ($index)$extension" in existing) index += 1
    return "$stem ($index)$extension"
  }

  private fun splitName(name: String): Pair<String, String> {
    val dot = name.lastIndexOf('.')
    return if (dot > 0) name.substring(0, dot) to name.substring(dot) else name to ""
  }

  private fun safeFileName(value: String): String {
    val cleaned = value.replace(Regex("[\\\\/:*?\"<>|\\p{Cntrl}]"), " ").trim()
    return cleaned.ifEmpty { "download" }.take(180)
  }

  private fun sanitizedDirectory(value: String?): String? {
    val cleaned = value?.replace(Regex("[\\\\/:*?\"<>|\\p{Cntrl}]"), " ")
      ?.trim()?.take(80).orEmpty()
    return cleaned.ifEmpty { null }
  }

  private fun normalizeMime(value: String): String = value.ifBlank { "application/octet-stream" }

  private fun copy(input: InputStream, output: OutputStream, canceled: AtomicBoolean) {
    val buffer = ByteArray(DEFAULT_BUFFER_SIZE)
    while (true) {
      ensureNotCanceled(canceled)
      val count = input.read(buffer)
      if (count < 0) return
      output.write(buffer, 0, count)
    }
  }

  private fun ensureNotCanceled(canceled: AtomicBoolean) {
    if (canceled.get()) throw InterruptedException("已取消保存到下载文件夹")
  }

  private fun exportedResult(sourcePath: String, uri: Uri, name: String): JSObject = JSObject().apply {
    put("sourcePath", sourcePath)
    put("uri", uri.toString())
    put("name", name)
  }
}
