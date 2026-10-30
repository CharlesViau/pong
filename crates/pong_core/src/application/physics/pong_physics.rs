use crate::domain::{BallMotion, Side, Velocity};

pub trait PongPhysics {
    fn ball(&self) -> BallMotion;
    fn paddle_y(&self, side: Side) -> f32;
    fn drive_paddle(&mut self, side: Side, speed: f32);
    fn set_ball_velocity(&mut self, velocity: Velocity);
    fn reset_ball(&mut self);
    fn reset_court(&mut self);
}
