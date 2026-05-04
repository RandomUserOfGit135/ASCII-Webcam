use opencv::prelude::*;
use opencv::videoio;
use std::io::Write;
use std::process::{Command, Stdio};

const W: i32 = 320;
const H: i32 = 180;
const CHARS: &[u8] = b" .:-=+*#%@";

fn lum(b: u8, g: u8, r: u8) -> usize {
    let v = (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as usize;
    v * (CHARS.len() - 1) / 255
}

fn main() -> opencv::Result<()> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();

    let mut ffmpeg = Command::new("ffmpeg")
        .args([
            "-f", "rawvideo",
            "-pix_fmt", "rgb24",
            "-s", "320x180",
            "-i", "-",
            "-f", "v4l2",
            "/dev/video10"
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("ffmpeg failed");

    let mut stdin = ffmpeg.stdin.take().unwrap();

    loop {
        cam.read(&mut frame)?;
        if frame.empty()? { continue; }

        let mut buf = vec![0u8; (W * H * 3) as usize];

        for y in 0..H {
            for x in 0..W {
                let p = frame.at_2d::<opencv::core::Vec3b>(y, x)?;
                let c = CHARS[lum(p[0], p[1], p[2])];

                let i = ((y * W + x) * 3) as usize;
                buf[i] = c;
                buf[i+1] = c;
                buf[i+2] = c;
            }
        }

        stdin.write_all(&buf).ok();
    }
}
