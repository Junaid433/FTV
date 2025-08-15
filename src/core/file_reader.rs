use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use crate::errors::AppResult;

pub fn read_file_with_padding<P: AsRef<Path>>(input_file: P, frame_size: usize) -> AppResult<Vec<u8>> {
    let mut file = File::open(&input_file)?;
    let file_size = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(0))?;

    let mut buffer = vec![0u8; file_size as usize];
    file.read_exact(&mut buffer)?;

    let remainder = buffer.len() % frame_size;
    if remainder != 0 {
        let padding = frame_size - remainder;
        buffer.extend(vec![0u8; padding]);
    }

    Ok(buffer)
}
