use crate::motion::types::{FlowVector, MotionEstimate};

pub fn estimate_motion(flows: &[FlowVector]) -> MotionEstimate {
    let mut dx = 0.0;
    let mut dy = 0.0;

    for f in flows {
        dx += f.to.x - f.from.x;
        dy += f.to.y - f.from.y;
    }

    let n = flows.len().max(1) as f32;
    MotionEstimate {
        dx: dx / n,
        dy: dy / n,
        valid_points: flows.len(),
    }
}
