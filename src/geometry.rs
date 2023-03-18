use crate::Pose3;
use nalgebra::Vector3;

pub fn to_vector_3(data: &[f64]) -> Vector3<f64> {
    return Vector3::from_row_slice(data);
}

pub fn transform_by(pose: &Pose3<f64>, transform: &Pose3<f64>) -> Pose3<f64> {
    pose * transform
}
