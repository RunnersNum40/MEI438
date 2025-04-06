use crate::motion::types::Point2D;

const THRESHOLD: u8 = 50;
const CONTIGUOUS: usize = 12;

pub fn detect_keypoints(data: &[u8], width: usize, height: usize) -> Vec<Point2D> {
    let mut points = Vec::new();
    const OFFSETS: [(isize, isize); 16] = [
        (0, -3),
        (1, -3),
        (2, -2),
        (3, -1),
        (3, 0),
        (3, 1),
        (2, 2),
        (1, 3),
        (0, 3),
        (-1, 3),
        (-2, 2),
        (-3, 1),
        (-3, 0),
        (-3, -1),
        (-2, -2),
        (-1, -3),
    ];

    let grid_step = 2;
    for y in (3..(height - 3)).step_by(grid_step) {
        for x in (3..(width - 3)).step_by(grid_step) {
            let center_idx = y * width + x;
            let center_val = data[center_idx];
            let high_threshold = center_val.saturating_add(THRESHOLD);
            let low_threshold = center_val.saturating_sub(THRESHOLD);

            let mut count_bright = 0;
            let mut count_dark = 0;
            let mut is_corner = false;

            for i in 0..32 {
                let (dx, dy) = OFFSETS[i % 16];
                let idx = ((y as isize + dy) as usize) * width + ((x as isize + dx) as usize);
                let pixel = data[idx];
                if pixel > high_threshold {
                    count_bright += 1;
                    count_dark = 0;
                } else if pixel < low_threshold {
                    count_dark += 1;
                    count_bright = 0;
                } else {
                    count_bright = 0;
                    count_dark = 0;
                }
                if count_bright >= CONTIGUOUS || count_dark >= CONTIGUOUS {
                    is_corner = true;
                    break;
                }
            }

            if is_corner {
                points.push(Point2D {
                    x: x as f32,
                    y: y as f32,
                });
            }
        }
    }

    points
}

