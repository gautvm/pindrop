use apriltag::{Image, TagParams};
use pindrop::pose;

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/apriltag_board.pnm");
    let image = match Image::from_pnm_file(path) {
        Ok(image) => image,
        Err(e) => panic!("Error loading image: {}", e),
    };
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
                    "Best pose for detection {}: {{ id: {}, error: {}, translation: {:#?}, rotation: {:#?} }}",
                    estimations.0, best_pose.id, best_pose.error, best_pose.translation, best_pose.rotation
                );
            } else {
                println!("No valid pose estimation for detection {}", estimations.0)
            }
        });
}
