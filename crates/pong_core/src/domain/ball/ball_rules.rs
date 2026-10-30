use crate::domain::{PaddleHit, Side, Velocity, rules};

pub(crate) struct BallRules;

impl BallRules {
    pub(crate) fn serve_velocity(toward: Side, upward: bool) -> Velocity {
        Velocity {
            x: toward.sign() * rules::SERVE_SPEED * 0.94_f32.sqrt(),
            y: if upward { 1.0 } else { -1.0 } * rules::SERVE_SPEED * 0.06_f32.sqrt(),
        }
    }

    pub(crate) fn bounce_velocity(hit: PaddleHit, incoming_vertical_speed: f32) -> Velocity {
        let angle = Self::bounce_angle(hit.offset, incoming_vertical_speed);
        let speed = (hit.incoming_speed * 1.07).min(rules::MAX_BALL_SPEED);
        Velocity {
            x: hit.side.opponent().sign() * speed * angle.cos(),
            y: speed * angle.sin(),
        }
    }

    fn bounce_angle(offset: f32, incoming_vertical_speed: f32) -> f32 {
        let normalized_offset = (offset / (rules::PADDLE_HEIGHT / 2.0)).clamp(-1.0, 1.0);
        let magnitude = (normalized_offset.abs() * rules::MAX_BOUNCE_ANGLE_DEGREES)
            .max(rules::MIN_BOUNCE_ANGLE_DEGREES);
        let downward = offset < 0.0 || (offset == 0.0 && incoming_vertical_speed < 0.0);
        if downward {
            -magnitude.to_radians()
        } else {
            magnitude.to_radians()
        }
    }

    pub(crate) fn scorer(ball_x: f32) -> Option<Side> {
        if ball_x >= rules::GOAL_X {
            return Some(Side::Left);
        }
        if ball_x <= -rules::GOAL_X {
            return Some(Side::Right);
        }
        None
    }
}
