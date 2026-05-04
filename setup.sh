#!/usr/bin/env bash

set -e

echo "ASCII Webcam setup"

# Rust install
if ! command -v cargo >/dev/null 2>&1; then
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source "$HOME/.cargo/env"
fi

install() {
    if command -v apt >/dev/null 2>&1; then
        sudo apt install -y "$1"
    elif command -v dnf >/dev/null 2>&1; then
        sudo dnf install -y "$1"
    elif command -v pacman >/dev/null 2>&1; then
        sudo pacman -S --noconfirm "$1"
    elif command -v zypper >/dev/null 2>&1; then
        sudo zypper install -y "$1"
    fi
}

read -p "Install system deps? (ffmpeg, v4l2loopback) [y/n]: " a
if [[ "$a" == "y" ]]; then
    install ffmpeg
    install v4l2loopback-dkms
fi

read -p "Load virtual webcam? [y/n]: " b
if [[ "$b" == "y" ]]; then
    sudo modprobe v4l2loopback devices=1 video_nr=10 card_label="ASCII Cam" exclusive_caps=1
fi

echo "Done. Run: cargo run --release"
