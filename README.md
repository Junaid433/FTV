# FileToVideo (FTV)

Convert anything to video and revert back again!

## Features

- **Encode:** Convert any file into a video format, embedding the file's binary data into the video frames.
- **Decode:** Reconstruct the original file from an `ftv` encoded video.
- **Verify:** Check the integrity of the reconstructed file against the original using SHA256 checksums.

## Installation

To build and run `ftv`, you need to have Rust and Cargo installed. You also need `ffmpeg` installed on your system, as `ftv` relies on it for video processing.

1.  **Install Rust and Cargo:**
    If you don't have Rust and Cargo installed, you can get them from the official Rust website:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2.  **Install FFmpeg:**
    `ftv` uses `ffmpeg` for video encoding and decoding. Ensure `ffmpeg` is installed and accessible in your system's PATH.

    *   **Ubuntu/Debian:**
        ```bash
        sudo apt update
        sudo apt install ffmpeg
        ```
    *   **macOS (using Homebrew):**
        ```bash
        brew install ffmpeg
        ```
    *   **Windows:** Download from the official FFmpeg website and add it to your PATH.

3.  **Build `ftv`:**
    Navigate to the project's root directory and build the project in release mode:
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/ftv` (or `target/release/ftv.exe` on Windows).

## Usage

All commands are executed via the `ftv` executable. You can run it directly from `target/release/` or add it to your system's PATH.

### 1. Encode Command

Encodes an input file into a video format.

```bash
./target/release/ftv encode -i <INPUT_FILE> -o <OUTPUT_DIRECTORY> --width <WIDTH> --height <HEIGHT> --framerate <FRAMERATE>
```

*   `<INPUT_FILE>`: Path to the file you want to encode.
*   `<OUTPUT_DIRECTORY>`: Directory where the encoded video and metadata JSON will be saved. Defaults to the current directory.
*   `--width`: (Optional) Width of the video in pixels. Default is 640.
*   `--height`: (Optional) Height of the video in pixels. Default is 480.
*   `--framerate`: (Optional) Framerate of the video in frames per second. Default is 24.

**Example:**

```bash
./target/release/ftv encode -i my_document.pdf -o ./encoded_videos --width 1280 --height 720 --framerate 30
```

### 2. Decode Command

Decodes an `ftv` encoded video back into its original file format.

```bash
./target/release/ftv decode -i <INPUT_VIDEO> -o <OUTPUT_DIRECTORY> --framerate <FRAMERATE>
```

*   `<INPUT_VIDEO>`: Path to the `ftv` encoded video file.
*   `<OUTPUT_DIRECTORY>`: Directory where the decoded file will be saved. Defaults to the current directory.
*   `--framerate`: (Optional) Framerate used during encoding. Must match the framerate used during encoding. Default is 24.

**Example:**

```bash
./target/release/ftv decode -i ./encoded_videos/my_document-pdf.mkv -o ./decoded_files
```

### 3. Verify Command

Verifies the integrity of a decoded file by comparing its SHA256 checksum with the one stored in the video's metadata.

```bash
./target/release/ftv verify --video <ENCODED_VIDEO> --original <ORIGINAL_FILE>
```

*   `<ENCODED_VIDEO>`: Path to the `ftv` encoded video file.
*   `<ORIGINAL_FILE>`: Path to the original file that was encoded. This is used to compute its SHA256 for comparison.

**Example:**

```bash
./target/release/ftv verify --video ./encoded_videos/my_document-pdf.mkv --original my_document.pdf
```