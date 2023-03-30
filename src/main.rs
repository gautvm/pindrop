use apriltag::{Image, TagParams};
use apriltag_image::ImageExt;
use nokhwa::{
    pixel_format::LumaFormat, utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};
use pindrop::{parser, pose, PindropPoseEstimation};

use std::time::{Duration, Instant};

fn main() {
    let config = parser::parse("pindrop.config.json").unwrap();
    let index = CameraIndex::Index(0);
    let requested = RequestedFormat::new::<LumaFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    let mut camera = Camera::new(index, requested).unwrap();
    camera.open_stream().unwrap_or_else(|err| {
        println!("Failed to open stream: {}", err);
        std::process::exit(1);
    });

    loop {
        let start = Instant::now();
        let tag_params = TagParams {
            tagsize: config.tag_params.tag_size,
            fx: config.tag_params.fx,
            fy: config.tag_params.fy,
            cx: config.tag_params.cx,
            cy: config.tag_params.cy,
        };
        let frame = camera.frame().unwrap();
        let decoded = frame.decode_image::<LumaFormat>().unwrap();

        let image = Image::from_image_buffer(&decoded);
        let pose_estimations = pose::estimate(image, tag_params);
        for (idx, estimations) in pose_estimations.into_iter().enumerate() {
            if let Some(best_pose) = estimations.first() {
                println!("Best pose for detection {}: {{ id: {}, error: {} }}",
                         idx, best_pose.id, best_pose.error);
            } else {
                eprintln!("No valid pose estimation for detection {}", idx)
            }
        }

        let duration = start.elapsed();
        println!("Time since last iteration {:?}", duration);
    }
}
