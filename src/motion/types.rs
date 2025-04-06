#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct FlowVector {
    pub from: Point2D,
    pub to: Point2D,
}

#[derive(Debug)]
pub struct MotionEstimate {
    pub dx: f32,
    pub dy: f32,
    pub dtheta: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Pose2D {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

impl Pose2D {
    pub fn new() -> Self {
        Pose2D {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
        }
    }

    pub fn update(&mut self, dx: f32, dy: f32, dtheta: f32) {
        self.x += dx;
        self.y += dy;
        self.theta += dtheta;
    }
}
