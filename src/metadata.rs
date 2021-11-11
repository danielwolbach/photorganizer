use crate::exif::ExifInterface;
use exif::{DateTime, Reader};
use std::{fs::File, io::BufReader};

#[derive(Debug)]
pub struct Metadata {
    pub date_time_original: Option<DateTime>,
}

impl Metadata {
    pub fn from_file(file: &File) -> Option<Self> {
        let mut buf_reader = BufReader::new(file);
        let exif_reader = Reader::new();

        match exif_reader.read_from_container(&mut buf_reader) {
            Ok(exif) => Some(Self {
                date_time_original: exif.date_time_original(),
            }),
            Err(_) => None,
        }
    }
}
