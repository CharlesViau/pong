use pong_core::{Side, Velocity};

#[derive(Debug, PartialEq)]
pub enum PhysicsCommand {
    DrivePaddle { side: Side, speed: f32 },
    SetBallVelocity(Velocity),
    ResetBall,
    ResetCourt,
}
