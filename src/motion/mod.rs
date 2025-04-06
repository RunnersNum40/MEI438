mod keypoints;
mod optical_flow;
mod pose;
mod preprocessing;
pub mod types;

use crate::camera::Frame;
use keypoints::detect_keypoints;
use optical_flow::lk_optical_flow;
use pose::estimate_motion;
use preprocessing::preprocess_image;
use types::{MotionEstimate, Point2D};

pub fn process_frame_pair(prev: Frame, curr: Frame) -> MotionEstimate {
    let height = curr.height;
    let width = curr.width;

    let prev = preprocess_image(prev.data, width, height);
    let curr = preprocess_image(curr.data, width, height);

    let keypoints = detect_keypoints(&prev, width, height);
    let flows = lk_optical_flow(&prev, &curr, width, height, &keypoints);

    let pivot = Point2D {
        x: width as f32 / 2.0,
        y: height as f32 / 2.0,
    };

    estimate_motion(&flows, pivot)
}
