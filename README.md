# ASCII Webcam

Real-time webcam to ASCII converter for Linux that exposes the result as a virtual webcam device.

It captures a live camera feed, converts each frame into ASCII characters based on brightness, and outputs it as a virtual webcam using v4l2loopback.

---

## Features

- Real-time ASCII video processing
- 320x180 optimized resolution for performance
- CPU-only pipeline (no GPU required)
- Virtual webcam output compatible with most apps
- Works with OBS, Discord, browsers, and other video apps

---

## How it works

Input:
- Physical webcam device (usually /dev/video0)

Processing:
- Captures raw frames from the camera
- Converts pixel brightness into ASCII characters
- Uses a fixed character set for density mapping
- Optimized low-level CPU loop

Output:
- Virtual webcam device via v4l2loopback
- Appears as /dev/video10

---

## Requirements

Linux system with:

- Rust toolchain (installed automatically via setup script if missing)
- v4l2loopback kernel module
- ffmpeg (optional depending on setup mode)

Supported distributions:
- Debian / Ubuntu
- Fedora
- Arch Linux
- openSUSE
- Slackware (manual dependency installation may be required)

---

## Installation

Clone the repository:

git clone https://github.com/youruser/ASCII-Webcam
cd ASCII-Webcam
chmod +x setup.sh
./setup.sh

Build and run:

cargo run --release

Or using Makefile:

make setup
make run

---

## Virtual webcam

After running setup, a virtual webcam device is created:

/dev/video10

Select this device in any application that supports webcams.

If the device is not created, manually load the kernel module:

sudo modprobe v4l2loopback devices=1 video_nr=10 card_label="ASCII Cam" exclusive_caps=1

---

## ASCII mapping

The brightness-to-character mapping uses:

 .:-=+*#%@ 

Dark to light intensity mapping is applied per pixel.

---

## Performance

- Fixed resolution: 320x180
- Fully CPU-based processing
- No per-frame allocations in optimized mode
- Designed for real-time use on low and mid-range hardware

Performance depends on:
- CPU speed
- camera driver efficiency
- system video pipeline overhead

---

## Troubleshooting

Camera not detected:
ls /dev/video*

Virtual camera missing:
sudo modprobe -r v4l2loopback
sudo modprobe v4l2loopback devices=1 video_nr=10

Permission issues:
sudo usermod -aG video $USER

---

## Limitations

- Not GPU accelerated
- Requires working V4L2 camera stack
- Performance depends heavily on kernel video pipeline
- ASCII output is grayscale-based (color optional future extension)

---

## Project goals

This project focuses on:

- Low-level video processing in Rust
- Efficient CPU-only real-time pipelines
- Linux virtual camera integration
- Minimal dependency design

---
