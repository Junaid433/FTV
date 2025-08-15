#!/bin/bash

FFMPEG_VERSION="n7.0.1"
INSTALL_DIR="/usr/local/bin"

if command -v ffmpeg &>/dev/null;
then
    echo "FFmpeg is already installed."
    exit 0
fi

echo "FFmpeg not found. Installing..."

FFMPEG_TARBALL="ffmpeg-${FFMPEG_VERSION}-amd64-static.tar.xz"
FFMPEG_URL="https://johnvansickle.com/ffmpeg/releases/${FFMPEG_TARBALL}"

echo "Downloading FFmpeg from ${FFMPEG_URL}..."
if ! wget -q --show-progress ${FFMPEG_URL}; then
    echo "Error: Failed to download FFmpeg."
    exit 1
fi

echo "Extracting FFmpeg..."
if ! tar -xf ${FFMPEG_TARBALL}; then
    echo "Error: Failed to extract FFmpeg."
    rm ${FFMPEG_TARBALL}
    exit 1
fi

FFMPEG_DIR=$(tar -tf ${FFMPEG_TARBALL} | head -n 1 | cut -d'/' -f1)

echo "Moving FFmpeg binaries to ${INSTALL_DIR}..."
if ! sudo mv ${FFMPEG_DIR}/ffmpeg ${INSTALL_DIR}/ffmpeg; then
    echo "Error: Failed to move ffmpeg binary."
    rm -rf ${FFMPEG_DIR}
    rm ${FFMPEG_TARBALL}
    exit 1
fi

if ! sudo mv ${FFMPEG_DIR}/ffprobe ${INSTALL_DIR}/ffprobe; then
    echo "Error: Failed to move ffprobe binary."
    rm -rf ${FFMPEG_DIR}
    rm ${FFMPEG_TARBALL}
    exit 1
fi

if ! sudo chmod +x ${INSTALL_DIR}/ffmpeg ${INSTALL_DIR}/ffprobe; then
    echo "Error: Failed to make binaries executable."
    rm -rf ${FFMPEG_DIR}
    rm ${FFMPEG_TARBALL}
    exit 1
fi

rm -rf ${FFMPEG_DIR}
rm ${FFMPEG_TARBALL}

echo "FFmpeg installed successfully to ${INSTALL_DIR}"

if command -v ffmpeg &>/dev/null;
then
    echo "FFmpeg version:"
    ffmpeg -version | head -n 1
else
    echo "FFmpeg installation verification failed. Please check your PATH."
    echo "You might need to add ${INSTALL_DIR} to your PATH environment variable."
    echo "Example: echo 'export PATH=\"$PATH:${INSTALL_DIR}\" >> ~/.bashrc && source ~/.bashrc"
fi
