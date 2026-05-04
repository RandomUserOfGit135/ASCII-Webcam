use std::fs::File;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::ptr;

const W: usize = 320;
const H: usize = 180;

const CHARS: &[u8] = b" .:-=+*#%@";

#[inline(always)]
fn map(v: u8) -> u8 {
    CHARS[((v as usize * 10) >> 8)]
}

// V4L2 structs mínimos (sin libc wrappers pesados)
#[repr(C)]
struct Buffer {
    start: *mut u8,
    length: usize,
}

fn main() {
    // /dev/video0 = cámara
    let cam = File::open("/dev/video0").expect("camera");

    // /dev/video10 = virtual cam
    let mut out_cam = File::create("/dev/video10").expect("v4l2loopback");

    let mut buffer = vec![0u8; W * H * 3];

    loop {
        // ⚠️ REAL WORLD: aquí normalmente usarías mmap V4L2
        // simplificación: read directo
        let mut frame = vec![0u8; W * H * 3];
        let _ = unsafe {
            libc::read(
                cam.as_raw_fd(),
                frame.as_mut_ptr() as *mut _,
                frame.len(),
            )
        };

        unsafe {
            let src = frame.as_ptr();
            let dst = buffer.as_mut_ptr();

            let mut i = 0;
            let mut o = 0;

            while o < buffer.len() {
                // BGR típico V4L2
                let b = *src.add(i);
                let g = *src.add(i + 1);
                let r = *src.add(i + 2);

                // luminancia entera ultra barata
                let lum = (r as u16 * 3 + g as u16 * 6 + b as u16) >> 3;

                let c = map(lum as u8);

                *dst.add(o) = c;
                *dst.add(o + 1) = c;
                *dst.add(o + 2) = c;

                i += 3;
                o += 3;
            }

            let _ = out_cam.write(&buffer);
        }
    }
}
