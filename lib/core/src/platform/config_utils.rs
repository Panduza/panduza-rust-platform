use std::env::consts::OS;

static DEFAULT_DIR_UNIX: &str = "/etc/panduza";
static DEFAULT_DIR_WINDOWS: &str = r"C:\ProgramData\Panduza";

pub fn get_default_config_dir() -> Result<&'static str, std::io::Error> {
    let dir: &str;

    match OS {
        "linux" => dir = DEFAULT_DIR_UNIX,
        "windows" => dir = DEFAULT_DIR_WINDOWS,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unsupported OS",
            ))
        }
    }
    Ok(dir)
}
