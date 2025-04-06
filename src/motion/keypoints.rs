use crate::motion::types::Point2D;

pub fn detect_keypoints(data: &[u8], width: usize, height: usize, spacing: usize) -> Vec<Point2D> {
    let mut points = Vec::new();
    for y in (spacing..(height - spacing)).step_by(spacing) {
        for x in (spacing..(width - spacing)).step_by(spacing) {
            points.push(Point2D {
                x: x as f32,
                y: y as f32,
            });
        }
    }
    points
}
