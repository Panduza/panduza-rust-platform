use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    // Get rustc version
    let output = Command::new("rustc")
        .args(&["--version"])
        .output()
        .expect("failed to execute process");
    let rustc_version = String::from_utf8_lossy(&output.stdout);

    // Get version from Cargo.toml (assuming it's in the same directory)
    let cargo_toml = include_str!("Cargo.toml");
    let package: toml::value::Table = toml::from_str(cargo_toml).unwrap();
    let version = package["package"]["version"].as_str().unwrap();

    // Format information for writing
    let info = format!(
        "pub static RUSTC_VERSION: &str  = \"{}\";\n
pub static PLATFORM_VERSION: &str  =  \"{}\";\n",
        rustc_version.trim_end_matches("\n"),
        version
    );

    // Write information to file
    let mut file = File::create("src/sys_info.rs").expect("Failed to create sys_info.rs");
    file.write_all(info.as_bytes())
        .expect("Failed to write to sys_info.rs");

    println!("Information written to sys_info.rs");
}
