use exif::{DateTime, In, Tag, Value, Exif};

pub trait ExifInterface {
    fn date_time_original(&self) -> Option<DateTime>;
}

impl ExifInterface for Exif {
    fn date_time_original(&self) -> Option<DateTime> {
        match self.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
            Some(field) => match field.value {
                Value::Ascii(ref vec) if !vec.is_empty() => DateTime::from_ascii(&vec[0]).ok(),
                _ => None,
            },
            None => None,
        }
    }
}
