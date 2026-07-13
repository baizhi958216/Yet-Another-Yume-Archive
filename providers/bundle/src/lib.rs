//! GENERATED DEFAULT — scripts/gen-bundle.mjs may overwrite this file.
//! The checked-in version registers only the built-in direct provider.
//!
//! This crate is the single static registration point: desktop hosts use it
//! for the built-in provider, mobile hosts additionally get site providers
//! linked in here by the (gitignored) bundle config.

use yaya_provider_host::HostedProvider;

/// Providers compiled into the host binary.
pub fn providers() -> Vec<HostedProvider> {
    let mut values = Vec::new();
    match provider_direct::DirectProvider::new() {
        Ok(provider) => values.push(HostedProvider::new(provider)),
        Err(error) => eprintln!("bundle: direct provider failed to initialize: {error}"),
    }
    values
}
