use apriltag::{Image, TagParams};
use apriltag_image::ImageExt;
use nokhwa::{
    pixel_format::LumaFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};
use pindrop::{
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
            let requested_format =
                RequestedFormat::new::<LumaFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
            let mut camera = Camera::new(
                CameraIndex::Index(config.cam_settings.index),
                requested_format,
            )
            .unwrap();

            if rpi4 {
                camera.open_stream().unwrap_or_else(|err| {
                    eprintln!("Failed to open stream: {}", err);
                    std::process::exit(1);
                });

                loop {
                    let start = Instant::now();
                    let frame = camera.frame().unwrap();
                    let decoded = frame.decode_image::<LumaFormat>().unwrap();

                    let duration = start.elapsed();
                    println!("Time since last iteration: {:?}", duration);
                }
            }
        }
    }
}
