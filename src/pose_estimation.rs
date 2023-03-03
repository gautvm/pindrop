use nalgebra::{Rotation, Translation};

pub struct AprilTagPoseEstimation {
    pub id: usize,
    pub error: f64,
    pub translation: Translation<f64, 3>,
    pub rotation: Rotation<f64, 3>,
}
