use std::path::Path;
use crate::errors::AppResult;
use crate::utils::cmd_runner::run_command;
use anyhow::Context;

pub fn extract_frames_from_video<P: AsRef<Path>>(
    input_video: P,
    output_dir: P,
    framerate: u32,
) -> AppResult<()> {
    let output_dir_ref = output_dir.as_ref();

    if output_dir_ref.exists() {
        std::fs::remove_dir_all(&output_dir_ref)?;
    }
    std::fs::create_dir_all(&output_dir_ref)?;

    let output_pattern = output_dir_ref.join("frame_%05d.png");

    let input_str = input_video.as_ref().to_str().context("Input path is not valid UTF-8")?;
    let output_str = output_pattern.to_str().context("Output path is not valid UTF-8")?;
    let fps_str = format!("fps={}", framerate);

    let args = [
        "-y",
        "-i",
        &input_str,
        "-vf",
        &fps_str,
        &output_str,
    ];

    run_command("ffmpeg", &args)?;

    Ok(())
}
