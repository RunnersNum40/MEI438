use crate::motion::types::{FlowVector, Point2D};

const WINDOW: usize = 3;

pub fn track_keypoints(
    prev: &[u8],
    curr: &[u8],
    width: usize,
    height: usize,
    points: &[Point2D],
) -> Vec<FlowVector> {
    let mut tracked = Vec::new();

    for &p in points {
        let x = p.x as isize;
        let y = p.y as isize;

        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut w = 0.0;

        for j in -(WINDOW as isize)..=(WINDOW as isize) {
            for i in -(WINDOW as isize)..=(WINDOW as isize) {
                let px = x + i;
                let py = y + j;

                if px < 1 || py < 1 || px >= (width as isize - 1) || py >= (height as isize - 1) {
                    continue;
                }

                let idx = (py * width as isize + px) as usize;

                let prev_val = prev[idx] as f32;
                let curr_val = curr[idx] as f32;

                let diff = curr_val - prev_val;

                dx += i as f32 * diff;
                dy += j as f32 * diff;
                w += diff.abs();
            }
        }

        if w > 0.0 {
            tracked.push(FlowVector {
                from: p,
                to: Point2D {
                    x: p.x + dx / w,
                    y: p.y + dy / w,
                },
            });
        }
    }

    tracked
}
