use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CameraSettings {
    pub brightness: f32,
    pub gain: u32,
    pub exposure: u32,
    pub orientation: u32,
    pub resolution: Vec<u32>,
    pub fps: u32,
    pub fov: u32,
}

#[derive(Debug, Deserialize)]
pub struct PoseEstimations {
    pub camera_matrix: Vec<Vec<f32>>,
    pub distortion_coefficients: Vec<f32>,
    pub rotation_vector: Vec<f32>,
    pub translation_vector: Vec<f32>,
}

#[derive(Debug, Deserialize)]
pub struct Networking {
    pub hostname: String,
    pub ip: String,
    pub port: u32,
}

#[derive(Debug, Deserialize)]
pub struct PindropConfig {
    pub cam_settings: CameraSettings,
    pub pose_estimations: PoseEstimations,
    pub networking: Networking,
}

pub fn parse(pindrop_config_path: &str) -> Result<PindropConfig, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(pindrop_config_path)?;
    let reader = std::io::BufReader::new(file);
    let config: PindropConfig = serde_json::from_reader(reader)?;

    Ok(config)
}
