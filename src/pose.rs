use crate::geometry;
use apriltag::{Detection, DetectorBuilder, Family, Image, PoseEstimation, TagParams};
use nalgebra::Isometry3;

#[derive(Debug)]
pub struct PindropPoseEstimation {
    pub id: usize,
    pub error: f64,
    pub pose: Isometry3<f64>,
}

pub fn estimate(image: Image, tag_params: TagParams) -> Vec<Vec<PindropPoseEstimation>> {
    let mut detector = DetectorBuilder::new()
        .add_family_bits(Family::tag_16h5(), 1)
        .build()
        .unwrap_or_else(|e| panic!("Error building detector: {}", e));

    let detections: Vec<Detection> = detector.detect(&image);

    let pose_estimations: Vec<Vec<PindropPoseEstimation>> = detections
        .into_iter()
        .map(|detection| {
            let pose_estimations: Vec<PoseEstimation> =
                detection.estimate_tag_pose_orthogonal_iteration(&tag_params, 40);

            let mut pose_estimations: Vec<PindropPoseEstimation> = pose_estimations
                .into_iter()
                .map(|raw_pose| PindropPoseEstimation {
                    id: detection.id(),
                    error: raw_pose.error,
                    pose: Isometry3::new(
                        geometry::to_nalgebra(raw_pose.pose.translation().data()),
                        geometry::to_nalgebra(raw_pose.pose.rotation().data()),
                    ),
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
