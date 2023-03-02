use apriltag::{Detection, DetectorBuilder, TagParams, Image, Family};
/**
.

# Panics

Panics if .
*/
fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/apriltag_board.pnm");
    let image = Image::from_pnm_file(path).unwrap();

    let mut detector = DetectorBuilder::new()
        .add_family_bits(Family::tag_16h5(), 1)
        .build()
        .expect("Valid builder");

    let detections: Vec<Detection> = detector.detect(&image);   

    // detections.into_iter().enumerate().for_each(|(index, det)| {
    //     println!("  - detection {index}: {det:#?}");
    // });

    let tag_params = TagParams { //! idk these numbers
        cx : 0.0,
        cy : 0.0,
        fx : 220.0,
        fy : 220.0,
        tagsize : 16.0,
    };
    
    detections.into_iter().enumerate().for_each(|(index, det)| {
        let fin_pose = det.estimate_tag_pose(&tag_params);
        println!(" - detection {index}: {det:#?}");
        println!(" - pose {index}: {fin_pose:#?}");
    });
}
