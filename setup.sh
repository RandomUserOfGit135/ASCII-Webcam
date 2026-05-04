#!/usr/bin/env bash

set -e

echo "ASCII Webcam setup"

read -p "Install system dependencies? (ffmpeg, v4l2loopback) [y/n]: " ans

if [[ "$ans" != "y" ]]; then
    echo "Skipping system dependencies. You better know what you're doing."
else
    echo "Installing dependencies..."

    install_pkg() {
        if command -v apt >/dev/null 2>&1; then
            sudo apt install -y "$1"
        elif command -v dnf >/dev/null 2>&1; then
            sudo dnf install -y "$1"
        elif command -v pacman >/dev/null 2>&1; then
            sudo pacman -S --noconfirm "$1"
        elif command -v zypper >/dev/null 2>&1; then
            sudo zypper install -y "$1"
        elif command -v slackpkg >/dev/null 2>&1; then
            sudo slackpkg install "$1"
        else
            echo "Unsupported system. Install $1 manually."
        fi
    }

    install_pkg ffmpeg
    install_pkg v4l2loopback-dkms
fi

read -p "Load virtual webcam module now? [y/n]: " cam

if [[ "$cam" == "y" ]]; then
    sudo modprobe v4l2loopback devices=1 video_nr=10 card_label="ASCII Cam" exclusive_caps=1
fi

echo "Setup complete"
