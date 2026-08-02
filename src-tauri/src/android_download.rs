//! Android Storage Access Framework bridge for selecting a destination tree
//! and publishing completed artifacts into it.

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tauri::{plugin::PluginHandle, AppHandle, Manager, Runtime};
use tokio_util::sync::CancellationToken;
use yaya_provider_api::{Artifact, ProgressReporter, ProviderError};
use yaya_task_runtime::{ArtifactPublisher, TaskSnapshot};

const PLUGIN_IDENTIFIER: &str = "com.zhi.yaya";

pub struct AndroidDownload<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Clone for AndroidDownload<R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadDirectory {
    pub uri: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportFile {
    pub path: String,
    pub name: String,
    pub mime_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExportRequest {
    id: String,
    files: Vec<ExportFile>,
    subdirectory: Option<String>,
}

#[derive(Debug, Serialize)]
struct CancelRequest {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportedFile {
    pub source_path: String,
    pub uri: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct ExportResponse {
    files: Vec<ExportedFile>,
}

pub fn init<R: Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("android-download")
        .setup(|app, api| {
            let handle =
                api.register_android_plugin(PLUGIN_IDENTIFIER, "DownloadDirectoryPlugin")?;
            app.manage(AndroidDownload(handle));
            Ok(())
        })
        .build()
}

fn handle<R: Runtime>(app: &AppHandle<R>) -> AndroidDownload<R> {
    app.state::<AndroidDownload<R>>().inner().clone()
}

pub fn get_directory<R: Runtime>(app: &AppHandle<R>) -> Result<DownloadDirectory, String> {
    handle(app)
        .0
        .run_mobile_plugin("getDirectory", ())
        .map_err(|error| error.to_string())
}

pub fn pick_directory<R: Runtime>(app: &AppHandle<R>) -> Result<DownloadDirectory, String> {
    handle(app)
        .0
        .run_mobile_plugin("pickDirectory", ())
        .map_err(|error| error.to_string())
}

pub fn export_files<R: Runtime>(
    app: &AppHandle<R>,
    id: String,
    files: Vec<ExportFile>,
    subdirectory: Option<String>,
) -> Result<Vec<ExportedFile>, String> {
    handle(app)
        .0
        .run_mobile_plugin::<ExportResponse>(
            "exportFiles",
            ExportRequest {
                id,
                files,
                subdirectory,
            },
        )
        .map(|response| response.files)
        .map_err(|error| error.to_string())
}

fn cancel_export<R: Runtime>(app: &AppHandle<R>, id: String) {
    let _ = handle(app)
        .0
        .run_mobile_plugin::<serde_json::Value>("cancelExport", CancelRequest { id });
}

pub struct AndroidArtifactPublisher(pub AppHandle<tauri::Wry>);

#[async_trait]
impl ArtifactPublisher for AndroidArtifactPublisher {
    async fn publish(
        &self,
        task: &TaskSnapshot,
        artifacts: Vec<Artifact>,
        _reporter: Arc<dyn ProgressReporter>,
        cancellation: CancellationToken,
    ) -> Result<Vec<Artifact>, ProviderError> {
        let files = artifacts
            .iter()
            .map(|artifact| ExportFile {
                path: artifact.path.to_string_lossy().into_owned(),
                name: artifact.name.clone(),
                mime_type: artifact.mime_type.clone(),
            })
            .collect();
        let app = self.0.clone();
        let task_id = task.id.clone();
        let export_id = task_id.clone();
        let group = task.group.clone();
        let mut export = tauri::async_runtime::spawn_blocking(move || {
            export_files(&app, export_id, files, group)
        });

        let export_result = tokio::select! {
            result = &mut export => result
                .map_err(|error| ProviderError::internal(error.to_string()))
                .and_then(|result| result.map_err(ProviderError::internal)),
            _ = cancellation.cancelled() => {
                cancel_export(&self.0, task_id);
                let _ = export.await;
                Err(ProviderError::canceled())
            }
        };
        let exported = match export_result {
            Ok(exported) => exported,
            Err(error) => {
                for artifact in &artifacts {
                    tokio::fs::remove_file(&artifact.path).await.ok();
                }
                return Err(error);
            }
        };
        let replacements = exported
            .into_iter()
            .map(|file| {
                (
                    PathBuf::from(file.source_path),
                    (PathBuf::from(file.uri), file.name),
                )
            })
            .collect::<HashMap<_, _>>();
        let mut published = artifacts;
        for artifact in &mut published {
            if let Some((uri, name)) = replacements.get(&artifact.path) {
                let original = std::mem::replace(&mut artifact.path, uri.clone());
                artifact.name.clone_from(name);
                tokio::fs::remove_file(original).await.ok();
            }
        }
        Ok(published)
    }
}
