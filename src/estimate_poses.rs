use crate::pose_estimation::AprilTagPoseEstimation;
use apriltag::{Detection, DetectorBuilder, Family, Image, PoseEstimation, TagParams};
use nalgebra::{Matrix3, Rotation3, Translation3, Vector3};

pub fn estimate_poses(image: Image, tag_params: TagParams) -> Vec<Vec<AprilTagPoseEstimation>> {
    let mut detector = DetectorBuilder::new()
        .add_family_bits(Family::tag_16h5(), 1)
        .build()
        .unwrap_or_else(|e| panic!("Error building detector: {}", e));

    let detections: Vec<Detection> = detector.detect(&image);

    let pose_estimations: Vec<Vec<AprilTagPoseEstimation>> = detections
        .into_iter()
        .map(|detection| {
            let pose_estimations: Vec<PoseEstimation> =
                detection.estimate_tag_pose_orthogonal_iteration(&tag_params, 40);

            let mut pose_estimations: Vec<AprilTagPoseEstimation> = pose_estimations
                .into_iter()
                .map(|pose| AprilTagPoseEstimation {
                    id: detection.id(),
                    error: pose.error,
                    translation: Translation3::from(Vector3::from_row_slice(
                        pose.pose.translation().data(),
                    )),
                    rotation: Rotation3::from_matrix(&Matrix3::from_row_slice(
                        pose.pose.rotation().data(),
                    )),
                })
                .collect();

            // Sort estimations based on error
            pose_estimations
                .sort_by(|pose_a, pose_b| pose_a.error.partial_cmp(&pose_b.error).unwrap());

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
}
