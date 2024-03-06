use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::env;
use std::path::{Path, PathBuf};
use log::log;

fn main() -> Result<(), std::io::Error> {
    let gz = File::create("test.tar.gz")?;
    let enc = GzEncoder::new(gz, Compression::default());

    let mut ar = tar::Builder::new(enc);

    println!("{:?}", get_asec_dir());
    if let Some(log_path) = get_asec_dir() {
        ar.append_dir_all("logs", log_path.join("log"))?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn get_asec_dir() -> Option<PathBuf> {
    if let Ok(program_data) = env::var("ProgramData") {
        let log_path = Path::new(&program_data).join("asec");

        if log_path.exists() && log_path.is_dir() {
            return Some(log_path);
        }
    }
    None
}

// TODO test me
#[cfg(target_os = "darwin")]
fn get_asec_log_dir() -> Option<PathBuf> {
    if let Some(mut home_dir) = home_dir() {
        home_dir.push("Library/Application Support").join("asec").join("log");
        return Some(home_dir);
    }
    None
}
