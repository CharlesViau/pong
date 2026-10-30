use crate::domain::{BallMotion, Direction, rules};

pub(crate) struct PaddleMovement;

impl PaddleMovement {
    pub(crate) fn bounded_speed(y: f32, direction: Direction, speed: f32) -> f32 {
        (direction.sign() * speed).clamp(
            (-rules::PADDLE_LIMIT - y) / rules::TICK_SECONDS,
            (rules::PADDLE_LIMIT - y) / rules::TICK_SECONDS,
        )
    }

    pub(crate) fn computer_direction(ball: BallMotion, paddle_y: f32) -> Direction {
        let target = if ball.velocity.x > 0.0 {
            ball.position.y
        } else {
            0.0
        };
        let difference = target - paddle_y;
        if difference > 12.0 {
            return Direction::Up;
        }
        if difference < -12.0 {
            return Direction::Down;
        }
        Direction::Still
    }
}
