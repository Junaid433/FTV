#[allow(dead_code)]

mod errors;
mod commands;
mod utils;
mod core;

use crate::errors::AppResult;
use crate::commands::cli::{Cli, Commands};
use clap::Parser;

fn main() -> AppResult<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode { input, output, width, height, framerate } => {
            crate::commands::encode::encode_file_to_video(
                input, output, *width, *height, *framerate
            )?;
        },
        Commands::Decode { input, output, framerate } => {
            crate::commands::decode::decode_video_to_file(
                input, output, *framerate
            )?;
        },
        Commands::Verify { video, original } => {
            crate::commands::verify::verify_video_file(video, original)?;
        },
    }

    Ok(())
}
