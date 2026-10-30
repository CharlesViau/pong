use crate::application::PongPhysics;
use crate::domain::{BallRules, Match, PaddleHit, Phase};

pub(in crate::application::match_play) struct BounceBall;

impl BounceBall {
    pub(in crate::application::match_play) fn execute(
        game: &Match,
        hit: PaddleHit,
        physics: &mut impl PongPhysics,
    ) {
        if game.phase() != Phase::Playing {
            return;
        }
        let incoming_vertical_speed = physics.ball().velocity.y;
        let outgoing_velocity = BallRules::bounce_velocity(hit, incoming_vertical_speed);
        physics.set_ball_velocity(outgoing_velocity);
    }
}
