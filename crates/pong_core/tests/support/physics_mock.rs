use pong_core::{BallMotion, PongPhysics, Side, Velocity};

use super::PhysicsCommand;

/// A hand-written recording mock of the domain's driven port.
/// Tests supply observations and explicitly verify meaningful write interactions.
/// No integration, collision detection, or gameplay policy is simulated here.
#[derive(Default)]
pub struct PhysicsMock {
    pub ball: BallMotion,
    pub paddle_y: [f32; 2],
    paddle_velocity: [f32; 2],
    commands: Vec<PhysicsCommand>,
}

impl PhysicsMock {
    fn index(side: Side) -> usize {
        match side {
            Side::Left => 0,
            Side::Right => 1,
        }
    }

    pub fn paddle_velocity(&self, side: Side) -> f32 {
        self.paddle_velocity[Self::index(side)]
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }

    #[track_caller]
    pub fn assert_commands(&self, expected: &[PhysicsCommand]) {
        assert_eq!(self.commands, expected, "unexpected physics commands");
    }
}

impl PongPhysics for PhysicsMock {
    fn ball(&self) -> BallMotion {
        self.ball
    }

    fn paddle_y(&self, side: Side) -> f32 {
        self.paddle_y[Self::index(side)]
    }

    fn drive_paddle(&mut self, side: Side, speed: f32) {
        self.commands
            .push(PhysicsCommand::DrivePaddle { side, speed });
        self.paddle_velocity[Self::index(side)] = speed;
    }

    fn set_ball_velocity(&mut self, velocity: Velocity) {
        self.commands
            .push(PhysicsCommand::SetBallVelocity(velocity));
        self.ball.velocity = velocity;
    }

    fn reset_ball(&mut self) {
        self.commands.push(PhysicsCommand::ResetBall);
        self.ball = BallMotion::default();
    }

    fn reset_court(&mut self) {
        self.commands.push(PhysicsCommand::ResetCourt);
        self.ball = BallMotion::default();
        self.paddle_y = [0.0; 2];
        self.paddle_velocity = [0.0; 2];
        // Keep the interaction log: resetting bodies must not erase evidence.
    }
}
