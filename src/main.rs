use apriltag::{Image, TagParams};
use apriltag_image::ImageExt;
use pindrop::{
    capture::Capture,
    cli::{self, Command},
    parser, pose, PindropPoseEstimation,
};
use std::time::{Duration, Instant};
use structopt::StructOpt;

fn main() {
    let args = cli::Cli::from_args();

    match args.command {
        Command::Deploy { rpi4, config_path } => {
            let config = parser::parse(config_path.to_str().unwrap()).unwrap();
            let path = "/dev/video" + config.cam_settings.index.to_string();
            let format = Format::new(
                config.cam_settings.resolution[0],
                config.cam_settings.resolution[1],
                v4l::FourCC::new(b"MJPG"),
            );
            let frame_count = 1000;
            let buffer_count = 4;

            if rpi4 {
                let mut image_capture = Capture::new(path, format, frame_count, buffer_count)?;
                let images = image_capture.capture_images()?;
                image_capture.print_statistics();
            }
        }
    }
}
