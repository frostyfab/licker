use std::io::BufRead;
use std::process::Command;

/// Retrieves a list of installed packages using the `pacman` package manager.
pub fn packages() -> Vec<String> {
    let packages = Command::new("pacman")
        .args(["-Qq"])
        .output()
        .expect("Failed to run pacman."); // TODO: timeout

    let stdout = &packages.stdout[..];

    let packages: Vec<String> = std::io::BufReader::new(stdout)
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .collect();

    packages
}
