use std::{io, fmt::format};
use std::time::Instant;

use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use v4l::video::Capture;

//use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, GrayImage};
use apriltag::{Detector, Family, DetectorBuilder, Detection, Image};
use apriltag_image::{ImageExt, image::ImageBuffer};


fn main() -> io::Result<()> {
    let path = "/dev/video0";
    println!("Using device: {}\n", path);

    // Capture 4 frames by default
    let count = 4;

    // Allocate 4 buffers by default
    let buffer_count = 4;

    let dev = Device::with_path(path)?;
    let format = dev.format()?;
    //let params: v4l::video::capture::Parameters = dev.params()?;
    let mut params = dev.params()?;
    params.interval.denominator = 30;
    params.interval.numerator = 1;
    dev.set_params(&params)?;

    println!("Active format:\n{}", format);
    println!("Active parameters:\n{}", params);

    // Setup a buffer stream and grab a frame, then print its data
    let mut stream = MmapStream::with_buffers(&dev, Type::VideoCapture, buffer_count)?;

    // warmup
    stream.next()?;
    
    let mut detector = DetectorBuilder::new()
    .add_family_bits(Family::tag_16h5(), 1)
    .build()
    .unwrap_or_else(|e| panic!("Error building detector: {}", e));

    let start = Instant::now();
    let mut megabytes_ps: f64 = 0.0;
    for i in 0..count {
        let t0 = Instant::now();
        let (buf, meta) = stream.next()?;
        let duration_us = t0.elapsed().as_micros();

        let img_buff = ImageBuffer::from_raw(1280, 720, &buf[..]).unwrap();
        let img = Image::from_image_buffer(&img_buff);
        let result = img_buff.save_with_format("yeah.png",apriltag_image::image::ImageFormat::Png );
        // Detect AprilTags in the image
        
        let detections: Vec<Detection> = detector.detect(&img);
        println!("{:#?}", detections);
        detections.into_iter().enumerate().for_each(|(index, det)| {
            println!("  - detection {index}: {det:#?}");
        });


        println!("Buffer");
        println!("  sequence  : {}", meta.sequence);
        println!("  timestamp : {}", meta.timestamp);
        println!("  flags     : {}", meta.flags);
        println!("  length    : {}", buf.len());
    }

    println!();
    println!("FPS: {}", count as f64 / start.elapsed().as_secs_f64());
    println!("MB/s: {}", megabytes_ps);

    Ok(())
}
