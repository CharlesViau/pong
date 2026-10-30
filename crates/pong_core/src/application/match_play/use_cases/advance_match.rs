use super::super::match_effects::MatchEffects;
use crate::application::PongPhysics;
use crate::domain::{Match, Phase};

pub(in crate::application::match_play) struct AdvanceMatch;

impl AdvanceMatch {
    pub(in crate::application::match_play) fn execute(
        game: &mut Match,
        physics: &mut impl PongPhysics,
    ) {
        if !game.can_advance() {
            return;
        }
        let ball_x = (game.phase() == Phase::Playing).then(|| physics.ball().position.x);
        let effect = game.advance(ball_x);
        MatchEffects::apply(effect, game, physics);
    }
}
