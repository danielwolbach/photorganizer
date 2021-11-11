use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Scanner {
    pub entries: Vec<PathBuf>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            entries: vec![]
        }
    }

    pub fn from_directory(path: &Path) -> Self {
        let mut scanner = Self::new();

        for entry in WalkDir::new(path) {
            match entry {
                Ok(entry) => {
                    if entry.path().is_file() && Scanner::supported(&entry.path()) {
                        scanner.entries.push(entry.into_path());
                    }
                },
                Err(_) => (),
            }
        }

        scanner
    }

    fn supported(path: &Path) -> bool {
        match path.extension() {
            Some(extension) => {
                match extension.to_str() {
                    Some("jpg") => true,
                    Some("JPG") => true,
                    Some("png") => true,
                    Some("PNG") => true,
                    _ => false,
                }
            },
            None => false
        }
    }
}
