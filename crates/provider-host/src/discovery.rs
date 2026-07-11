//! Scans provider package directories for `provider.json` manifests and a
//! runnable executable for the current target.

use std::{collections::BTreeMap, path::Path};

use crate::{ExternalProvider, HostError, HostedProvider, ProviderManifest};

const PROVIDER_MANIFEST_SCHEMA_VERSION: u32 = 1;

pub fn scan(root: &Path) -> Result<Vec<HostedProvider>, HostError> {
    let entries = match std::fs::read_dir(root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error.into()),
    };
    let mut providers = Vec::new();
    for entry in entries {
        let directory = entry?.path();
        let manifest_path = directory.join("provider.json");
        if !manifest_path.is_file() {
            continue;
        }
        let Ok(manifest) =
            serde_json::from_slice::<ProviderManifest>(&std::fs::read(&manifest_path)?)
        else {
            continue;
        };
        if manifest.schema_version != PROVIDER_MANIFEST_SCHEMA_VERSION {
            continue;
        }
        let Some(candidates) = manifest.executables.get(target_key()) else {
            continue;
        };
        let Some(executable) = candidates
            .iter()
            .map(|value| directory.join(value))
            .find(|value| value.is_file())
        else {
            continue;
        };
        let data_dir = root.join(".data").join(&manifest.id);
        providers.push(HostedProvider::new(ExternalProvider::new(
            manifest, executable, data_dir,
        )));
    }
    Ok(providers)
}

/// Scan several roots; later roots win on id conflicts.
pub fn scan_roots<I, P>(roots: I) -> Result<Vec<HostedProvider>, HostError>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut providers = BTreeMap::new();
    for root in roots {
        for provider in scan(root.as_ref())? {
            providers.insert(provider.id(), provider);
        }
    }
    Ok(providers.into_values().collect())
}

pub const fn target_key() -> &'static str {
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        "aarch64-apple-darwin"
    } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        "x86_64-apple-darwin"
    } else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        "x86_64-pc-windows-msvc"
    } else if cfg!(all(target_os = "windows", target_arch = "aarch64")) {
        "aarch64-pc-windows-msvc"
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        "x86_64-unknown-linux-gnu"
    } else if cfg!(all(target_os = "linux", target_arch = "aarch64")) {
        "aarch64-unknown-linux-gnu"
    } else {
        "unsupported"
    }
}
