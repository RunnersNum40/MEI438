mod keypoints;
mod optical_flow;
mod pose;
pub mod types;

use crate::camera::Frame;
use keypoints::detect_keypoints;
use optical_flow::track_keypoints;
use pose::estimate_motion;
use types::MotionEstimate;

pub fn process_frame_pair(prev: &Frame, curr: &Frame) -> MotionEstimate {
    let keypoints = detect_keypoints(&prev.data, prev.width, prev.height, 10);
    let flows = track_keypoints(&prev.data, &curr.data, curr.width, curr.height, &keypoints);
    estimate_motion(&flows)
}
