use crate::motion::types::{FlowVector, Point2D};

pub fn lk_optical_flow(
    prev: &[u8],
    curr: &[u8],
    width: usize,
    height: usize,
    points: &[Point2D],
) -> Vec<FlowVector> {
    let window_radius = 1;
    let mut tracked = Vec::new();

    for &p in points {
        let x = p.x as isize;
        let y = p.y as isize;

        if x <= window_radius
            || y <= window_radius
            || x >= (width as isize - window_radius)
            || y >= (height as isize - window_radius)
        {
            continue;
        }

        let mut sum_gx2 = 0.0;
        let mut sum_gy2 = 0.0;
        let mut sum_gxgy = 0.0;
        let mut sum_gxit = 0.0;
        let mut sum_gyit = 0.0;

        for j in -window_radius..=window_radius {
            for i in -window_radius..=window_radius {
                let idx = ((y + j) * width as isize + (x + i)) as usize;

                let idx_left = ((y + j) * width as isize + (x + i - 1)) as usize;
                let idx_right = ((y + j) * width as isize + (x + i + 1)) as usize;
                let idx_top = (((y + j - 1) * width as isize) + (x + i)) as usize;
                let idx_bottom = (((y + j + 1) * width as isize) + (x + i)) as usize;
                let gx = (prev[idx_right] as f32 - prev[idx_left] as f32) / 2.0;
                let gy = (prev[idx_bottom] as f32 - prev[idx_top] as f32) / 2.0;

                let it = curr[idx] as f32 - prev[idx] as f32;

                sum_gx2 += gx * gx;
                sum_gy2 += gy * gy;
                sum_gxgy += gx * gy;
                sum_gxit += gx * it;
                sum_gyit += gy * it;
            }
        }

        let det = sum_gx2 * sum_gy2 - sum_gxgy * sum_gxgy;
        if det.abs() < 1e-6 {
            continue;
        }

        let u = (-sum_gy2 * sum_gxit + sum_gxgy * sum_gyit) / det;
        let v = (sum_gxgy * sum_gxit - sum_gx2 * sum_gyit) / det;

        tracked.push(FlowVector {
            from: p,
            to: Point2D {
                x: p.x + u,
                y: p.y + v,
            },
        });
    }

    tracked
}

