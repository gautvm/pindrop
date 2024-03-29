use pindrop::{capture::Capture, cli::Command, parser, pose};
use std::time::{Duration, Instant};
use structopt::StructOpt;

fn main() {
    let args = cli::Cli::from_args();
    let config = parser::parse(config_path.to_str().unwrap()).unwrap();

    match args.command {
        Command::Deploy { rpi4, config_path } => {
            let device_index = config.cam_settings.index.to_string();
            let format = Format::new(
                config.cam_settings.resolution[0],
                config.cam_settings.resolution[1],
                v4l::FourCC::new(b"MJPG"),
            );
            let frame_count = 1000;
            let buffer_count = 4;

            if rpi4 {
                let mut image_capture =
                    Capture::new(device_index, format, frame_count, buffer_count)?;
                let images = image_capture.capture_images()?;
                image_capture.print_statistics();
            }
        }
    }
}
