use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Result;
use esp_idf_hal::peripherals::Peripherals;

mod camera;
mod motion;

use camera::{CameraWrapper, Frame};
use motion::{process_frame_pair, types::Pose2D};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut cam = CameraWrapper::new(pins)?;
    let mut prev_frame = cam.capture_frame()?;

    let pose = Arc::new(Mutex::new(Pose2D::new()));
    let pose_clone = pose.clone();

    thread::spawn(move || loop {
        let curr_frame = match cam.capture_frame() {
            Ok(f) => f,
            Err(_) => continue,
        };

        let estimate = process_frame_pair(&prev_frame, &curr_frame);

        if let Ok(mut pose) = pose_clone.lock() {
            pose.update(estimate.dx, estimate.dy);
        }

        prev_frame = curr_frame;
        thread::sleep(Duration::from_millis(10));
    });

    loop {
        if let Ok(pose) = pose.lock() {
            println!("Pose: x = {:.2}, y = {:.2}", pose.x, pose.y);
        }
        thread::sleep(Duration::from_millis(500));
    }
}

