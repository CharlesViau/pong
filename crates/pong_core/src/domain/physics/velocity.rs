#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn speed(self) -> f32 {
        self.x.hypot(self.y)
    }
}
