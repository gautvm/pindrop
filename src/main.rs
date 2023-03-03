use apriltag::{Image, TagParams};
use nalgebra::{Rotation3, Translation3, Vector3, Matrix3};
use pindrop::estimate_poses::estimate_poses;
use pindrop::pose_estimation::AprilTagPoseEstimation;

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

    let pose_estimations: Vec<Vec<AprilTagPoseEstimation>> = estimate_poses(image, tag_params);
    pose_estimations
        .into_iter()
        .enumerate()
        .for_each(|estimations| {
            if let Some(best_pose) = estimations.1.first() {
                let rotation = Rotation3::from_matrix(&Matrix3::from_row_slice(best_pose.pose.rotation().data()));
                let translation = Translation3::from(Vector3::from_row_slice(best_pose.pose.translation().data()));

                println!(
                    "Best pose for detection {}: {{ id: {}, error: {}, pose: {:#?} }}",
                    estimations.0, best_pose.id, best_pose.error, best_pose.pose
                );

                println!("Best pose nalgebra translation for {}: {:#?} ", estimations.0, translation);
                println!("Best pose nalgebra rotation for {}: {:#?} ", estimations.0, rotation);
            } else {
                println!("No valid pose estimation for detection {}", estimations.0)
            }
        });
}
