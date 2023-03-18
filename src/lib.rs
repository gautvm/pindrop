use nalgebra::Isometry3;

// Type alias for pose 3d
pub type Pose3<T> = Isometry3<T>;
pub mod cli;
pub mod field;
pub mod geometry;
pub mod nt;
pub mod parser;
pub mod pose;
pub mod pose_estimator;
