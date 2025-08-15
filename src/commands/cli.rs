use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "FileToVideo")]
#[command(author = "Junaid Rahman")]
#[command(version = "1.0.0")]
#[command(about = "Convert any file to a video and back", long_about = None)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode {
        #[arg(value_name = "FILE", help = "Path to the input file to encode.")]
        input: String,

        #[arg(value_name = "OUTPUT_DIR", default_value = ".", help = "Directory to save the encoded video.")]
        output: String,

        #[arg(long, default_value_t = 640, help = "Width of the video in pixels.")]
        width: u32,

        #[arg(long, default_value_t = 480, help = "Height of the video in pixels.")]
        height: u32,

        #[arg(long, default_value_t = 24, help = "Framerate of the video in frames per second.")]
        framerate: u32,
    },

    Decode {
        #[arg(value_name = "VIDEO", help = "Path to the input video to decode.")]
        input: String,

        #[arg(value_name = "OUTPUT_DIR", default_value = ".", help = "Directory to save the decoded file.")]
        output: String,

        #[arg(long, default_value_t = 24, help = "Framerate used during encoding.")]
        framerate: u32,
    },

    Verify {
        #[arg(short, long)]
        video: String,

        #[arg(short, long)]
        original: String,
    }
}
