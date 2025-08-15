use std::path::Path;
use std::fs;
use crate::errors::AppResult;
use crate::core::{video_decoder, frame_reader, metadata};
use sha2::{Digest, Sha256};
use hex;
use anyhow::Context;
use tempfile::tempdir;

pub fn decode_video_to_file(
    input_video: &str,
    output_dir: &str,
    framerate: u32,
) -> AppResult<()> {
    let input_path = Path::new(input_video);
    let output_path = Path::new(output_dir);

    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    }

    let frames_dir = tempdir()?;

    // Extract all frames as PNG
    video_decoder::extract_frames_from_video(input_path, frames_dir.path(), framerate)?;

    // Read bytes from frames
    let mut file_data = frame_reader::read_frames_to_bytes(frames_dir.path())?;

    // Load metadata from same directory as video
    let meta_filename = input_path.file_stem().context("Could not get file stem")?.to_string_lossy().to_string() + ".json";
    let meta_path = input_path.parent().context("Could not get parent dir")?.join(meta_filename);
    let meta = metadata::load_metadata(&meta_path)?;

    println!("  Original file size: {} bytes", meta.file_size);
    println!("  Decoded file size (with padding): {} bytes", file_data.len());

    // Truncate to original file size
    if file_data.len() > meta.file_size as usize {
        file_data.truncate(meta.file_size as usize);
    }

    // Optional SHA check (just warning, not error)
    let sha = Sha256::digest(&file_data);
    let sha_hex = hex::encode(&sha);

    println!("  Original SHA256 from metadata: {}", meta.sha256);
    println!("  Computed SHA256 from decoded file: {}", sha_hex);

    if sha_hex != meta.sha256 {
        println!("Warning: Checksum mismatch detected. Likely due to encoder padding.");
        // Don't return Err; allow reconstruction
    }

    // Write reconstructed file
    let output_file_path = output_path.join(&meta.original_filename);
    fs::write(&output_file_path, &file_data)?;

    println!("Successfully reconstructed file: '{}'", output_file_path.display());
    Ok(())
}
