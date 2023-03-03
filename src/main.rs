use apriltag::{Detection, DetectorBuilder, Family, Image, Pose, PoseEstimation, TagParams};

struct AprilTagPoseEstimation {
    id: usize,
    error: f64,
    pose: Pose,
}

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/apriltag_board.pnm");
    let image = match Image::from_pnm_file(path) {
        Ok(image) => image,
        Err(e) => panic!("Error loading image: {}", e),
    };

    let mut detector = DetectorBuilder::new()
        .add_family_bits(Family::tag_16h5(), 1)
        .build()
        .unwrap_or_else(|e| panic!("Error building detector: {}", e));

    let detections: Vec<Detection> = detector.detect(&image);

    let tag_params = TagParams {
        cx: 0.0,
        cy: 0.0,
        fx: 220.0,
        fy: 220.0,
        tagsize: 16.0,
    };

    let pose_estimations: Vec<Vec<AprilTagPoseEstimation>> = detections
        .into_iter()
        .map(|detection| {
            let pose_estimations: Vec<PoseEstimation> =
                detection.estimate_tag_pose_orthogonal_iteration(&tag_params, 1);

            let mut pose_estimations: Vec<AprilTagPoseEstimation> = pose_estimations
                .into_iter()
                .map(|pose| AprilTagPoseEstimation {
                    id: detection.id(),
                    error: pose.error,
                    pose: pose.pose,
                })
                .collect();

            // Sort estimations based on error
            pose_estimations.sort_by(|pose_a, pose_b| pose_a.error.partial_cmp(&pose_b.error).unwrap());

            if pose_estimations.len() > 1 {
                let error_difference = pose_estimations[1].error - pose_estimations[0].error;

                // Compares best pose to error difference by ambiguity threshold
                if error_difference / pose_estimations[0].error < 0.1 {
                    pose_estimations.clear();
                }
            }

            pose_estimations
        })
        .collect();

    pose_estimations
        .into_iter()
        .enumerate()
        .for_each(|estimations| {
            if let Some(best_pose) = estimations.1.first() {
                println!(
                    "Best pose for detection {}: {{ id: {}, error: {}, pose: {:#?} }}",
                    estimations.0, best_pose.id, best_pose.error, best_pose.pose
                )
            } else {
                println!("No valid pose estimation for detection {}", estimations.0)
            }
        });
}
