use rscam;
use std::fs;
use std::io::Write;
use std::time::{Duration, Instant};

const FRAME_COUNT: usize = 100;

fn main() {
    let mut camera = rscam::new("/dev/video0").unwrap();

    camera
        .start(&rscam::Config {
            interval: (1, 30), // 30 fps.
            resolution: (1280, 720),
            //format: b"MJPG",
            ..Default::default()
        })
        .unwrap();

    let mut i = 0;
    let mut fps_sum = Duration::from_secs(0);
    let mut start_time = Instant::now();
    loop {
        let frame = camera.capture().unwrap();
        //let mut file = fs::File::create(&format!("frame-{}.jpg", i)).unwrap();
        //file.write_all(&frame[..]).unwrap();

        let elapsed_time = start_time.elapsed();
        fps_sum += elapsed_time;
        if i % FRAME_COUNT == 0 {
            let fps = (FRAME_COUNT as f64 / fps_sum.as_secs_f64()).round();
            println!("FPS: {}", fps);
            fps_sum = Duration::from_secs(0);
        }
        start_time = Instant::now();

        i += 1;
    }
}
