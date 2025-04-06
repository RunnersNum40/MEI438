use crate::motion::types::{FlowVector, MotionEstimate, Point2D};
use std::f32::consts::PI;

pub fn estimate_motion(flows: &[FlowVector], pivot: Point2D) -> MotionEstimate {
    let mut sum_dx = 0.0;
    let mut sum_dy = 0.0;
    let mut sum_dtheta = 0.0;

    for f in flows {
        sum_dx += f.to.x - f.from.x;
        sum_dy += f.to.y - f.from.y;

        let angle_before = (f.from.y - pivot.y).atan2(f.from.x - pivot.x);
        let angle_after = (f.to.y - pivot.y).atan2(f.to.x - pivot.x);
        let mut dtheta = angle_after - angle_before;

        if dtheta > PI {
            dtheta -= 2.0 * PI;
        } else if dtheta < -PI {
            dtheta += 2.0 * PI;
        }
        sum_dtheta += dtheta;
    }

    let n = flows.len().max(1) as f32;
    MotionEstimate {
        dx: sum_dx / n,
        dy: sum_dy / n,
        dtheta: sum_dtheta / n,
    }
}
