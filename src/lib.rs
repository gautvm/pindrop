use nalgebra::Isometry3;

// Typealias for pose 3d
pub type Pose3<T> = Isometry3<T>;

// Custom pose estimation struct with required data
#[derive(Debug)]
pub struct PindropPoseEstimation {
    pub id: usize,
    pub error: f64,
    pub pose: Pose3<f64>,
}

// Modules
mod cli;
pub mod field;
pub mod geometry;
pub mod nt;
pub mod parser;
pub mod pose;
pub mod pose_estimator;
