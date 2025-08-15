use std::path::Path;
use crate::errors::AppResult;
use crate::core::{metadata, checksum, video_decoder, frame_reader};
use anyhow::Context;
use tempfile::tempdir;

pub fn verify_video_file(
    video_path: &str,
    original_file_path: &str,
) -> AppResult<()> {
    let video_path = Path::new(video_path);
    let original_path = Path::new(original_file_path);

    // Load metadata from video
    let meta_filename = video_path.file_stem().context("Could not get file stem from video path")?.to_string_lossy().to_string() + ".json";
    let meta_path = video_path.parent().context("Could not get parent dir from video path")?.join(meta_filename);
    let meta = metadata::load_metadata(&meta_path)?;

    // Decode video to a temporary directory
    let frames_dir = tempdir()?;
    video_decoder::extract_frames_from_video(video_path, frames_dir.path(), 24)?; // Assuming default framerate for verification
    let decoded_data = frame_reader::read_frames_to_bytes(frames_dir.path())?;

    // Truncate decoded data to original size
    let mut decoded_data_truncated = decoded_data;
    if decoded_data_truncated.len() > meta.file_size as usize {
        decoded_data_truncated.truncate(meta.file_size as usize);
    }

    // Calculate SHA256 of decoded data
    let decoded_sha = checksum::sha256_bytes(&decoded_data_truncated);

    // Calculate SHA256 of original file
    let original_sha = checksum::sha256_file(original_path)?;

    println!("  Original file SHA256: {}", original_sha);
    println!("  Decoded data SHA256: {}", decoded_sha);

    if decoded_sha == original_sha {
        println!("Verification successful: Checksums match for '{}'.", video_path.display());
    } else {
        println!("Verification failed: Checksums do NOT match for '{}'.", video_path.display());
        return Err(crate::errors::errors::FTVError::ChecksumMismatch { expected: original_sha, got: decoded_sha }.into());
    }

    Ok(())
}
