use crate::application::PongPhysics;
use crate::domain::{Effect, Match};

/// Carries out the domain's decisions through the application-owned physics port.
pub(super) struct MatchEffects;

impl MatchEffects {
    pub(super) fn apply(effect: Effect, game: &Match, physics: &mut impl PongPhysics) {
        match effect {
            Effect::None => {}
            Effect::ResetMatch => physics.reset_court(),
            Effect::ResetBall => physics.reset_ball(),
            Effect::ServeBall => physics.set_ball_velocity(game.serve_velocity()),
        }
    }
}
