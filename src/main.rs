pub mod detect;

use clap::Parser;
use detect::{app::AppInfo, os::OsInfo, *};
use serde::{Deserialize, Serialize};

const CREAMYCOURIER_API_URL: &str = "https://creamycourier.pizzapill.com/ioctl/packagesubmissions";

/// CLI/ENV options.
#[derive(Parser)]
#[clap(version)]
struct Config {
    #[clap(short, long, help = "Submit a list of your installed packages")]
    submit: bool,

    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long, env = "CREAMYCOURIER_API_URL", default_value = CREAMYCOURIER_API_URL, help = "Customize API endpoint URL")]
    api_url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    let hw_id = hw::id();
    let pacman_packages = pacman::packages();
    let os_info = os::info();
    let os_distro = os::distro();
    let package_manager = String::from("pacman");
    let app_info = app::info();

    if config.verbose {
        let packages: Vec<String> = pacman_packages.iter().map(|package| package.to_string()).collect();
        let packages_out = packages.join(", ");
        println!("{}", packages_out);

        if let Ok(info) = os_info.as_ref() {
            println!("{:?}", info);
        }

        if let Ok(distro) = os_distro.as_ref() {
            println!("Distribution: {}", distro);
        }

        if let Ok(id) = hw_id.as_ref() {
            println!("Hardware ID: {}", id);
        }

        println!("{:?}", app_info);

        println!("Package Manager: {}", package_manager);
    }

    if config.submit {
        let payload = Payload {
            hw_id: hw_id.ok(),
            distro: os_distro.ok(),
            package_manager,
            os: os_info.ok(),
            package_list: pacman_packages,
        };

        match submit(&payload, &app_info, &config.api_url) {
            Ok(()) => {
                if config.verbose {
                    println!("Packages submitted. Thank you!");
                }
            }
            Err(e) => {
                return Err(format!("Submit failed: {}", e).into());
            }
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Payload {
    hw_id: Option<String>,
    distro: Option<String>,
    package_manager: String,
    #[serde(flatten)]
    os: Option<OsInfo>,
    package_list: Vec<String>,
}

/// Submit the packagelist & metadata
fn submit(payload: &Payload, app_info: &AppInfo, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();


    let response = client.post(url)
    .header("API-Version", "0.1")
    .header("AppName", app_info.detect_app.to_string())
    .header("AppVersion", app_info.detect_version.to_string())
    .json(payload).send()?;

    match response.status() {
        reqwest::StatusCode::CREATED | reqwest::StatusCode::OK => Ok(()),
        reqwest::StatusCode::UNAUTHORIZED => Err("unauthorized".into()),
        _ => {
            let error_message = response.text()?;
            Err(error_message.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use serde_json;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn test_payload_json() {
        let hw_id = "testhwid";
        let distro = "testdistro";
        let package_manager = "testpm";
        let os = "testos";
        let kernel = "testkernel";
        let arch = "testarch";
        let package_list = vec![String::from("testpackage1"), String::from("testpackage2")];

        let payload = Payload {
            hw_id: Some(hw_id.to_string()),
            distro: Some(distro.to_string()),
            package_manager: package_manager.to_string(),
            os: Some(OsInfo {
                os: os.to_string(),
                kernel: kernel.to_string(),
                arch: arch.to_string(),
            }),
            package_list: package_list.clone()
        };

        // Serialize/Deserialize JSON
        let payload_json = serde_json::to_string(&payload).unwrap();

        let deserialized_payload: HashMap<String, Value> =
            serde_json::from_str(&payload_json).unwrap();

        assert_matches!(deserialized_payload.get("hwId"), Some(Value::String(ref val)) if val == &hw_id);
        assert_matches!(deserialized_payload.get("distro"), Some(Value::String(ref val)) if val == &distro);
        assert_matches!(deserialized_payload.get("packageManager"), Some(Value::String(ref val)) if val == &package_manager);
        assert_matches!(deserialized_payload.get("os"), Some(Value::String(ref val)) if val == &os);
        assert_matches!(deserialized_payload.get("kernel"), Some(Value::String(ref val)) if val == &kernel);
        assert_matches!(deserialized_payload.get("arch"), Some(Value::String(ref val)) if val == &arch);
    }
}
