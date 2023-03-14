use nalgebra::{Vector3, Isometry3};

pub fn to_nalgebra(data: &[f64]) -> Vector3<f64> {
    return Vector3::from_row_slice(data);
}

pub fn transform_by(pose: &Isometry3<f64>, transform: &Isometry3<f64>) -> Isometry3<f64> {
    pose * transform
}