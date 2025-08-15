use std::path::Path;

use crate::errors::AppResult;
use std::fs;

use indicatif::{ProgressBar, ProgressStyle};

pub fn read_frames_to_bytes<P: AsRef<Path>>(frames_dir: P) -> AppResult<Vec<u8>> {
    let mut entries: Vec<_> = fs::read_dir(&frames_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "png").unwrap_or(false))
        .collect();

    entries.sort_by_key(|e| {
        e.path()
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.split('_').last())
            .and_then(|n| n.parse::<u32>().ok())
            .unwrap_or(0)
    });

    let mut data = Vec::new();
    let bar = ProgressBar::new(entries.len() as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
        .progress_chars("##-"));

    for entry in entries {
        let img = image::open(entry.path())?.to_luma8();
        data.extend(img.into_raw());
        bar.inc(1);
    }

    bar.finish();
    Ok(data)
}