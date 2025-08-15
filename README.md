# FileToVideo (FTV) ![Crates.io](https://img.shields.io/crates/v/ftv) ![Build](https://github.com/Junaid433/ftv/actions/workflows/rust.yml/badge.svg)

Convert any file to a video and back!

## Features

- **Encode:** Convert any file into a video.
- **Decode:** Reconstruct the original file from an `ftv` video.
- **Verify:** Check integrity via SHA256.

## Installation

Requires [Rust](https://www.rust-lang.org/tools/install) and [FFmpeg](https://ffmpeg.org/download.html).

```bash
cargo build --release
````

Executable: `target/release/ftv`

## Usage

### Encode

```bash
ftv encode -i <INPUT_FILE> -o <OUTPUT_DIR> --width <WIDTH> --height <HEIGHT> --framerate <FRAMERATE>
```

### Decode

```bash
ftv decode -i <INPUT_VIDEO> -o <OUTPUT_DIR> --framerate <FRAMERATE>
```

### Verify

```bash
ftv verify --video <ENCODED_VIDEO> --original <ORIGINAL_FILE>
```
