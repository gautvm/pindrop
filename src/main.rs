use apriltag::{Image, TagParams};
use apriltag_image::ImageExt;
use nokhwa::pixel_format::LumaFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;
use pindrop::{parser, pose};

fn main() {
    let config = parser::parse("pindrop.config.json").unwrap();
    println!("{:#?}", config.pose_estimations.distortion_coefficients);

    let index = CameraIndex::Index(0);
    let requested =
        RequestedFormat::new::<LumaFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    let mut camera = Camera::new(index, requested).unwrap();
    camera.open_stream();

    let frame = camera.frame().unwrap();
    let decoded = frame.decode_image::<LumaFormat>().unwrap();

    let image = Image::from_image_buffer(&decoded);
    let tag_params = TagParams {
        cx: 0.0,
        cy: 0.0,
        fx: 220.0,
        fy: 220.0,
        tagsize: 16.0,
    };

    let pose_estimations: Vec<Vec<pose::PindropPoseEstimation>> = pose::estimate(image, tag_params);
    pose_estimations
        .into_iter()
        .enumerate()
        .for_each(|estimations| {
            if let Some(best_pose) = estimations.1.first() {
                println!(
                    "Best pose for detection {}: {{ id: {}, error: {}, pose: {:#?} }}",
                    estimations.0, best_pose.id, best_pose.error, best_pose.pose,
                );
            } else {
                println!("No valid pose estimation for detection {}", estimations.0)
            }
        });
    // let args = Cli::from_args();

    // match args.command {
    //     Command::Deploy { rpi4, config } => {
    //         println!("Deploying Pindrop with config file {:?}", config);
    //         if rpi4 {
    //             let path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/apriltag_board.pnm");
    //             let image = match Image::from_pnm_file(path) {
    //                 Ok(image) => image,
    //                 Err(e) => panic!("Error loading image: {}", e),
    //             };
    //             let tag_params = TagParams {
    //                 cx: 0.0,
    //                 cy: 0.0,
    //                 fx: 220.0,
    //                 fy: 220.0,
    //                 tagsize: 16.0,
    //             };

    //             let pose_estimations: Vec<Vec<pose::PindropPoseEstimation>> =
    //                 pose::estimate(image, tag_params);
    //             pose_estimations
    //                 .into_iter()
    //                 .enumerate()
    //                 .for_each(|estimations| {
    //                     if let Some(best_pose) = estimations.1.first() {
    //                         println!(
    //                             "Best pose for detection {}: {{ id: {}, error: {}, translation: {:#?}, rotation: {:#?} }}",
    //                             estimations.0, best_pose.id, best_pose.error, best_pose.translation, best_pose.rotation
    //                         );
    //                     } else {
    //                         println!("No valid pose estimation for detection {}", estimations.0)
    //                     }
    //                 });
    //         }
    //     }
    // }
}
