use apriltag::{Detection, DetectorBuilder, Family, Image, MatdRef, TagParams, Pose, families::ApriltagFamily};

// Delete before releasing, meant for debugging / printing
#[derive(Debug)]
struct AprilTagDetection {
    id: usize,
    confidence: f32,
    error: f32,
    pose: Option<Pose>
}

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/apriltag_board.pnm");
    let image = Image::from_pnm_file(path).unwrap();

    let mut detector = DetectorBuilder::new()
        .add_family_bits(Family::tag_16h5(), 1)
        .build()
        .expect("Valid builder");

    let detections: Vec<Detection> = detector.detect(&image);

    let tag_params = TagParams {
        cx: 0.0,
        cy: 0.0,
        fx: 220.0,
        fy: 220.0,
        tagsize: 16.0,
    };
    

    detections
        .into_iter()
        .enumerate()
        .for_each(|(index, detection)| {
            let estimated_pose = detection.estimate_tag_pose(&tag_params);
            let apriltag_detection = AprilTagDetection { id: detection.id(), confidence: 10.3, error: 10.3, pose: estimated_pose };

            println!("apriltag_detection {index} {:#?}", apriltag_detection.id);
        });
}
