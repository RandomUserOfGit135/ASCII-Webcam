# ASCII Webcam

Real-time webcam to ASCII converter for Linux.
Outputs as a virtual webcam device using v4l2loopback.

## Features

- 320x180 ASCII video
- Real-time conversion
- Virtual webcam output (/dev/video10)
- Works with OBS, Discord, browsers

## Usage

git clone https://github.com/youruser/ASCII-Webcam
cd ASCII-Webcam
./setup.sh
cargo run --release

or

make setup
make run

## Requirements

- Linux
- Rust
- ffmpeg
- v4l2loopback

## Output

Virtual device:

/dev/video10
