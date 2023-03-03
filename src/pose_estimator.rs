use apriltag::Pose;

pub struct PoseEstimator {
    poses: Vec<Pose> 
}

impl PoseEstimator {
    pub fn new() -> Self {
        Self { poses: vec![] }
    }

    pub fn add_pose(&mut self, pose: Pose) {
        self.poses.push(pose);
    }

}