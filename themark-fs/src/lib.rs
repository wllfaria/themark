use std::fs::DirEntry;
use std::path::Path;

use themark_parser::{parse, Token};

pub fn load_markdown<P: AsRef<Path>>(path: P) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let contents = read_file(path)?;
    Ok(parse(&contents))
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(path.as_ref())?)
}

pub fn read_curr_dir<T: From<DirEntry>>() -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir()?;
    read_dir(cwd)
}

fn readable_byte_size(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, units[unit_index])
}

pub fn read_file_size(entry: DirEntry) -> String {
    let Ok(metadata) = entry.metadata() else {
        return "unknown".into();
    };

    let size = metadata.len();
    readable_byte_size(size)
}

pub fn read_dir<P: AsRef<Path>, T: From<DirEntry>>(
    path: P,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let entries = std::fs::read_dir(path.as_ref());
    let files = entries?
        .flatten()
        .filter(|e| is_markdown(e.path()))
        .map(Into::into)
        .collect::<Vec<_>>();
    Ok(files)
}

fn is_markdown<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().extension().is_some_and(|ext| ext == "md")
}
