use apriltag::Pose;

pub struct AprilTagPoseEstimation {
    pub id: usize,
    pub error: f64,
    pub pose: Pose,
}