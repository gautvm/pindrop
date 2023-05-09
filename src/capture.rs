use std::io;
use std::time::Instant;
use apriltag::Image;
use apriltag_image::ImageExt;
use apriltag_image::image::ImageBuffer;
use v4l::io::traits::CaptureStream;
use v4l::video::{Capture, Output};
use v4l::{prelude::*, Format};

pub struct Capture {
    stream: MmapStream,
    frame_count: usize,
    buffer_count: usize,
    start_time: Instant,
    megabytes_per_second: f64,
}

impl Capture {
    pub fn new(
        device_path: &str,
        format: Format,
        count: usize,
        buffer_count: usize,
    ) -> io::Result<Self> {
        let dev = Device::with_path(device_path)?;
        let params = dev.params()?;
        println!("Active format:\n{}", format);
        println!("Active parameters:\n{}", params);

        let stream = MmapStream::with_buffers(&dev, Type::VideoCapture, buffer_count)?;
        stream.next()?;

        Ok(Self {
            stream,
            frame_count,
            buffer_count,
            start_time: Instant::now(),
            megabytes_per_second: 0.0,
        })
    }

    pub fn capture_images(&mut self) -> io::Result<Vec<Image<Luma<u8>>>> {
        let mut images = Vec::with_capacity(self.frame_count);
        for i in 0..self.buffer_count {
            let t0 = Instant::now();
            let (buf, _) = self.stream.next()?;
            let duration_us = t0.elapsed().as_micros();
    
            let cur = buf.len() as f64 / 1_048_576.0 * 1_000_000.0 / duration_us as f64;
            if i == 0 {
                self.megabytes_per_second = cur;
            } else {
                let prev = self.megabytes_per_second * (i as f64 / (i + 1) as f64);
                let now = cur * (1.0 / (i + 1) as f64);
                self.megabytes_per_second = prev + now;
            }
    
            let image_buffer = ImageBuffer::<Luma<u8>, _>::from_raw(format.width, format.height, buf)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid image data"))?;
            let image = Image::from_image_buffer(&image_buf);
            images.push(image);
        }
    
        Ok(images)
    }

    pub fn print_statistics(&self) {
        println!();
        println!(
            "FPS: {}",
            self.buffer_count as f64 / self.start_time.elapsed().as_secs_f64()
        );
        println!("MB/s: {}", self.megabytes_per_second);
    }
}
