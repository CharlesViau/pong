mod match_fixture;
mod physics_command;
mod physics_mock;

pub use match_fixture::MatchFixture;
pub use physics_command::PhysicsCommand;
pub use physics_mock::PhysicsMock;
use pong_core::rules;
pub const FLOAT_TOLERANCE: f32 = 0.001;
pub const PADDLE_CENTER_LIMIT: f32 = (rules::COURT_HEIGHT - rules::PADDLE_HEIGHT) / 2.0;
pub const GOAL_CENTER_LIMIT: f32 = rules::COURT_WIDTH / 2.0 + rules::BALL_RADIUS;
pub const BEYOND_GOAL_MARGIN: f32 = 1.0;
pub const INACTIVE_TICKS: u16 = rules::SERVE_TICKS * 2;

#[track_caller]
pub fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= FLOAT_TOLERANCE,
        "expected {expected}, got {actual}; tolerance {FLOAT_TOLERANCE}"
    );
}
