use std::path::{Path, PathBuf};
use image::{GrayImage};
use crate::errors::AppResult;
use std::fs;

use indicatif::{ProgressBar, ProgressStyle};

pub fn write_frames_from_bytes(
    data: &[u8],
    output_dir: &Path,
    width: u32,
    height: u32,
) -> AppResult<Vec<PathBuf>> {
    let total_pixels = (width * height) as usize;

    if data.len() % total_pixels != 0 {
        return Err(crate::errors::errors::FTVError::InvalidData(
            "Data length is not a multiple of frame size".to_string(),
        ));
    }

    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::create_dir_all(output_dir)?;

    let num_frames = data.len() / total_pixels;
    let mut frame_paths = Vec::with_capacity(num_frames);

    let bar = ProgressBar::new(num_frames as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
        .progress_chars("##-"));

    for i in 0..num_frames {
        let start = i * total_pixels;
        let end = start + total_pixels;
        let frame_slice = &data[start..end];

        let img: GrayImage = GrayImage::from_raw(width, height, frame_slice.to_vec())
            .ok_or_else(|| crate::errors::errors::FTVError::InvalidData(format!("Failed to create image for frame {}", i)))?;

        let frame_path = output_dir.join(format!("frame_{:05}.png", i));
        img.save(&frame_path)?;

        frame_paths.push(frame_path);
        bar.inc(1);
    }

    bar.finish();
    Ok(frame_paths)
}