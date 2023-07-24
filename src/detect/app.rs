use clap::{command, crate_version};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub detect_app: String,
    pub detect_version: String,
}

/// Retrieves the app name and version.
pub fn info() -> AppInfo {
    AppInfo {
        detect_app: command!().to_string(),
        detect_version: crate_version!().to_string(),
    }
}
