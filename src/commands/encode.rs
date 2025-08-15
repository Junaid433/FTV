use std::path::Path;
use crate::errors::AppResult;
use crate::core::{file_reader, frame_writer, video_encoder, metadata, checksum};
use anyhow::Context;
use tempfile::tempdir;
use crate::errors::FTVError;

pub fn encode_file_to_video(
    input_file: &str,
    output_dir: &str,
    width: u32,
    height: u32,
    framerate: u32,
) -> AppResult<()> {
    let input_path = Path::new(input_file);

    if !input_path.exists() {
        return Err(FTVError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Input file not found")).into());
    }

    let output_path = Path::new(output_dir);
    let frames_dir = tempdir()?;

    let frame_size = (width * height) as usize;
    let file_data = file_reader::read_file_with_padding(input_path, frame_size)?;

    let file_sha = checksum::sha256_bytes(&file_data);

    frame_writer::write_frames_from_bytes(&file_data, frames_dir.path(), width, height)?;

    let file_stem = input_path.file_stem().context("Could not get file stem")?.to_str().context("Could not convert file stem to str")?;
    let extension = input_path.extension().context("Could not get file extension")?.to_str().context("Could not convert extension to str")?;
    let output_video = output_path.join(format!("{}-{}.mkv", file_stem, extension));
    video_encoder::encode_frames_to_video(frames_dir.path(), &output_video, framerate)?;

    let meta = metadata::FileMetadata {
        original_filename: input_path.file_name().context("Could not get file name")?.to_str().context("Could not convert file name to str")?.to_string(),
        file_size: std::fs::metadata(input_path)?.len(),
        sha256: file_sha,
    };
    let meta_path = output_path.join(format!("{}-{}.json", file_stem, extension));
    metadata::save_metadata(&meta, meta_path)?;

    println!("Successfully encoded '{}' to video: '{}'", input_file, output_video.display());
    Ok(())
}