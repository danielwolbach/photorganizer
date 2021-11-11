pub mod exif;
pub mod metadata;
pub mod scanner;
pub mod options;

use std::{path::{Path, PathBuf}, fs::File};
use crate::{options::Options, metadata::Metadata};

pub fn generate_new_path(path: &Path, options: &Options) -> PathBuf {
    let file = File::open(&path).unwrap();

    if let Some(metadata) = Metadata::from_file(&file) {
        if let Some(date_time_original) = metadata.date_time_original {
            let new_path = format!(
                "{root}/{year}/{month}/{day}/{hour}-{minute}-{second} {name}.{extension}",
                root = options.destination,
                year = date_time_original.year,
                month = date_time_original.month,
                day = date_time_original.day,
                hour = date_time_original.hour,
                minute = date_time_original.minute,
                second = date_time_original.second,
                name = path.file_name().unwrap().to_str().unwrap(),
                extension = path.extension().unwrap().to_str().unwrap().to_lowercase()
            );

            return PathBuf::from(new_path);
        }
    }

    let new_path = format!(
        "{root}/{placeholder}/{name}.{extension}",
        root = options.destination,
        placeholder = "unknown",
        name = path.file_name().unwrap().to_str().unwrap(),
        extension = path.extension().unwrap().to_str().unwrap().to_lowercase()
    );

    PathBuf::from(new_path)
}
