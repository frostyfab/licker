use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct OsInfo {
    pub os: String,
    pub kernel: String,
    pub arch: String,
    pub distro: String,
}

/// Retrieves information about the Linux operating system, kernel version, and architecture.
pub fn info() -> Result<OsInfo, Box<dyn Error>> {
    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };

    if unsafe { libc::uname(&mut uts) } != 0 {
        return Err("Failed to get Linux information".to_string().into());
    }

    let kernel_version = unsafe { std::ffi::CStr::from_ptr(uts.release.as_ptr()) }
        .to_string_lossy()
        .into_owned();

    let architecture = whoami::arch().to_string();
    let distro: String = whoami::distro();
    let os: String = whoami::platform().to_string();

    Ok(OsInfo {
        os,
        kernel: kernel_version,
        arch: architecture,
        distro,
    })
}
