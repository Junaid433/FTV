use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, BufReader};
use std::path::Path;
use crate::errors::AppResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileMetadata {
    pub original_filename: String,
    pub file_size: u64,
    pub sha256: String,
}

pub fn save_metadata<P: AsRef<Path>>(meta: &FileMetadata, output_path: P) -> AppResult<()> {
    let json = serde_json::to_string_pretty(meta)?;
    let mut file = File::create(output_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_metadata<P: AsRef<Path>>(input_path: P) -> AppResult<FileMetadata> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let meta = serde_json::from_reader(reader)?;
    Ok(meta)
}
