//! File-name derivation for direct downloads: Content-Disposition, URL path,
//! extension inference and collision-free target paths.

use std::path::{Path, PathBuf};

use percent_encoding::percent_decode_str;
use url::Url;

pub(crate) fn file_name_from_url(url: &Url) -> Option<String> {
    let segment = url.path_segments()?.rfind(|value| !value.is_empty())?;
    let value = percent_decode_str(segment)
        .decode_utf8_lossy()
        .trim()
        .to_string();
    (!value.is_empty()).then_some(value)
}

pub(crate) fn file_name_from_disposition(value: &str) -> Option<String> {
    for part in value.split(';').map(str::trim) {
        let Some((name, raw_value)) = part.split_once('=') else {
            continue;
        };
        let raw_value = raw_value.trim().trim_matches('"');
        if name.eq_ignore_ascii_case("filename*") {
            let encoded = raw_value
                .split_once("''")
                .map(|(_, value)| value)
                .unwrap_or(raw_value);
            return Some(percent_decode_str(encoded).decode_utf8_lossy().into_owned());
        }
        if name.eq_ignore_ascii_case("filename") && !raw_value.is_empty() {
            return Some(raw_value.to_string());
        }
    }
    None
}

pub(crate) fn ensure_extension(file_name: String, mime_type: &str) -> String {
    if Path::new(&file_name).extension().is_some() {
        return file_name;
    }
    match mime_guess::get_mime_extensions_str(mime_type).and_then(|values| values.first()) {
        Some(extension) => format!("{file_name}.{extension}"),
        None => file_name,
    }
}

pub(crate) fn default_file_name(mime_type: &str) -> String {
    ensure_extension("download".into(), mime_type)
}

/// `name.ext` → `name (1).ext` → … until the path is free.
pub(crate) fn unique_file_path(directory: &Path, file_name: &str) -> PathBuf {
    let candidate = directory.join(file_name);
    if !candidate.exists() {
        return candidate;
    }
    let path = Path::new(file_name);
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("download");
    let extension = path.extension().and_then(|value| value.to_str());
    for index in 1.. {
        let name = match extension {
            Some(extension) => format!("{stem} ({index}).{extension}"),
            None => format!("{stem} ({index})"),
        };
        let candidate = directory.join(name);
        if !candidate.exists() {
            return candidate;
        }
    }
    unreachable!()
}
