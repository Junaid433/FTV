use std::fmt;
use std::io;
use std::process::Output;
use image::ImageError;
use serde_json::Error as SerdeJsonError;
use anyhow::Error as AnyhowError;
use indicatif::style::TemplateError;

#[derive(Debug)]
pub enum FTVError {
    Io(io::Error),
    FfmpegFailed(String),
    InvalidData(String),
    MetadataError(String),
    ChecksumMismatch {
        expected: String,
        got: String
    },
    ImageError(ImageError),
    SerdeJsonError(SerdeJsonError),
    AnyhowError(AnyhowError),
    TemplateError(TemplateError),
}

impl fmt::Display for FTVError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            FTVError::Io(err) => write!(f, "IO error: {}", err),
            FTVError::FfmpegFailed(msg) => write!(f, "FFmpeg failed: {}", msg),
            FTVError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            FTVError::MetadataError(msg) => write!(f, "Metadata error: {}", msg),
            FTVError::ChecksumMismatch { expected, got } => {
                write!(f, "Checksum mismatch (expected {}, got {})", expected, got)
            },
            FTVError::ImageError(err) => write!(f, "Image error: {}", err),
            FTVError::SerdeJsonError(err) => write!(f, "Serde JSON error: {}", err),
            FTVError::AnyhowError(err) => write!(f, "Anyhow error: {}", err),
            FTVError::TemplateError(err) => write!(f, "Template error: {}", err),
        }
    }
}

impl std::error::Error for FTVError {}

impl From<io::Error> for FTVError {
    fn from(err: io::Error) -> Self {
        FTVError::Io(err)
    }
}

impl From<ImageError> for FTVError {
    fn from(err: ImageError) -> Self {
        FTVError::ImageError(err)
    }
}

impl From<SerdeJsonError> for FTVError {
    fn from(err: SerdeJsonError) -> Self {
        FTVError::SerdeJsonError(err)
    }
}

impl From<AnyhowError> for FTVError {
    fn from(err: AnyhowError) -> Self {
        FTVError::AnyhowError(err)
    }
}

impl From<TemplateError> for FTVError {
    fn from(err: TemplateError) -> Self {
        FTVError::TemplateError(err)
    }
}

impl FTVError {
    pub fn from_ffmpeg_output(output: Output) -> Self {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        FTVError::FfmpegFailed(stderr)
    }
}