use super::super::match_effects::MatchEffects;
use crate::application::PongPhysics;
use crate::domain::{Action, Match};

pub(in crate::application::match_play) struct HandleMatchAction;

impl HandleMatchAction {
    pub(in crate::application::match_play) fn execute(
        game: &mut Match,
        action: Action,
        physics: &mut impl PongPhysics,
    ) {
        let effect = game.handle_action(action);
        MatchEffects::apply(effect, game, physics);
    }
}
