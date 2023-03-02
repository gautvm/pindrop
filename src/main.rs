use apriltag::{DetectorBuilder, Family, Image, Detection};

fn main() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/test_data/apriltag_board.pnm"
    );
    let image = Image::from_pnm_file(path).unwrap();

    let mut detector = DetectorBuilder::new()
        .add_family_bits(Family::tag_16h5(), 1)
        .build()
        .expect("Valid builder");

    let detections: Vec<Detection> = detector
        .detect(&image);

        detections.into_iter().enumerate().for_each(|(index, det)| {
            println!("  - detection {index}: {det:#?}");
        });
}
