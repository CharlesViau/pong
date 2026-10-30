use crate::domain::{Position, Velocity};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BallMotion {
    pub position: Position,
    pub velocity: Velocity,
}
