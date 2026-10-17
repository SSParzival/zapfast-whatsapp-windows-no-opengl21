//! Self-update from GitHub releases, through fastframe-update.
//!
//! The crate checks for a newer release, refuses package-managed copies,
//! downloads and verifies the update against the publisher signature, and
//! hands it to a helper that installs it and rolls back if it does not start.
//! ZapFast keeps its names, its key, its proxy and its interface.

pub use fastframe_update::{
    CHECK_INTERVAL, DownloadState, Installation, Kind, Prepared, Release, Source, Unsupported,
    Updater,
};
use fastframe_update::{MacConfig, ReqwestTransport, UpdateConfig};

/// Official Windows updates would replace this compatibility renderer.
pub const SELF_UPDATE_ENABLED: bool = !cfg!(windows);

/// ZapFast's releases and the names its installations have had.
pub const CONFIG: UpdateConfig = UpdateConfig {
    // Cask and bundle names from before the rename. This also accepts
    // fastsapp-* marker files and `fastsapp <version>` answers, which no
    // release produces.
    legacy_names: &["fastsapp"],
    macos: MacConfig {
        bundle_ids: &["me.paolino.fastsapp"],
        executable_names: &[],
        legacy_bundle_names: &["FastsApp.app"],
    },
    publisher_key: Some(include_str!("../assets/update-public-key.hex")),
    // The next release key, backed up outside GitHub. Releases stay signed
    // with the current key until installs trust this one too.
    additional_publisher_keys: &[include_str!("../assets/update-public-key-next.hex")],
    ..UpdateConfig::new(
        "crmne/zapfast",
        "ZapFast",
        "zapfast",
        env!("CARGO_PKG_VERSION"),
    )
};

/// An updater on the proxy-aware reqwest client.
pub fn updater() -> anyhow::Result<Updater> {
    anyhow::ensure!(
        SELF_UPDATE_ENABLED,
        "This Direct3D compatibility build cannot install official OpenGL updates"
    );
    let mut builder = reqwest::blocking::Client::builder();
    if let Some(proxy) = crate::proxy::reqwest_proxy() {
        builder = builder.proxy(proxy);
    }
    Ok(Updater::new(CONFIG, ReqwestTransport::new(builder)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_config_is_valid() {
        CONFIG.validate().unwrap();
        assert_eq!(CONFIG.current_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(CONFIG.slug, "zapfast");
    }

    #[test]
    fn official_updates_are_available_only_on_supported_builds() {
        if SELF_UPDATE_ENABLED {
            assert!(updater().unwrap().source().is_github());
        } else {
            assert!(updater().is_err());
        }
    }
}
