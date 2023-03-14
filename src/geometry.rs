use nalgebra::{Vector3};

pub fn to_nalgebra(data: &[f64]) -> Vector3<f64> {
    return Vector3::from_row_slice(data);
}
