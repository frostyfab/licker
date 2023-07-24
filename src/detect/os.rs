use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Serialize, Deserialize, Debug)]
pub struct OsInfo {
    pub os: String,
    pub kernel: String,
    pub arch: String,
}

/// Retrieves the Linux distribution name from the `/etc/os-release` file.
pub fn distro() -> Result<String, Box<dyn Error>> {
    let file = File::open("/etc/os-release")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("ID=") {
            let distro = line.strip_prefix("ID=").unwrap();
            return Ok(distro.to_string());
        }
    }

    Err("Failed to determine Linux distribution".into())
}

/// Retrieves information about the Linux operating system, kernel version, and architecture.
pub fn info() -> Result<OsInfo, Box<dyn Error>> {
    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };

    if unsafe { libc::uname(&mut uts) } != 0 {
        return Err("Failed to get Linux information".to_string().into());
    }

    let os = unsafe { std::ffi::CStr::from_ptr(uts.sysname.as_ptr()) }
        .to_string_lossy()
        .into_owned();

    let kernel_version = unsafe { std::ffi::CStr::from_ptr(uts.release.as_ptr()) }
        .to_string_lossy()
        .into_owned();

    let architecture = unsafe { std::ffi::CStr::from_ptr(uts.machine.as_ptr()) }
        .to_string_lossy()
        .into_owned();

    Ok(OsInfo {
        os,
        kernel: kernel_version,
        arch: architecture,
    })
}
