use nalgebra::{Matrix3, Rotation3, Translation3, Vector3};

pub fn to_translation_3(translation_data: &[f64]) -> Translation3<f64> {
    return Translation3::from(Vector3::from_row_slice(translation_data));
}

pub fn to_rotation_3(rotation_data: &[f64]) -> Rotation3<f64> {
    return Rotation3::from_matrix(&Matrix3::from_row_slice(rotation_data));
}
