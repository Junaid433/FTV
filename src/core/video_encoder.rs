use std::path::Path;
use crate::errors::AppResult;
use crate::utils::cmd_runner::run_command;
use anyhow::Context;

pub fn encode_frames_to_video<P: AsRef<Path>>(
    frames_dir: P,
    output_video: P,
    framerate: u32,
) -> AppResult<()> {
    let frames_pattern = frames_dir.as_ref().join("frame_%05d.png");
    let frames_pattern_str = frames_pattern.to_str().context("Frames path is not valid UTF-8")?;

    let args = [
        "-y",                             
        "-framerate", &framerate.to_string(),
        "-i", frames_pattern_str,
        "-c:v", "ffv1",                    
        "-level", "3",             
        output_video.as_ref().to_str().context("Output path is not valid UTF-8")?,
    ];

    run_command("ffmpeg", &args)?;

    Ok(())
}
