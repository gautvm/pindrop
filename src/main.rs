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

            pose_estimations
                .into_iter()
                .map(|pose| AprilTagPoseEstimation {
                    id: detection.id(),
                    error: pose.error,
                    pose: pose.pose,
                })
                .collect()
        })
        .collect();

    pose_estimations
        .into_iter()
        .enumerate()
        .for_each(|estimations| {
            estimations
                .1
                .into_iter()
                .enumerate()
                .for_each(|(index, estimation)| {
                    println!(
                        "pose_estimation {}: {{ id: {}, error: {}, pose: {:#?} }}",
                        index, estimation.id, estimation.error, estimation.pose
                    )
                })
        });
}
