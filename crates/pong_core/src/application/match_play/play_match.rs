use super::{
    ForPlayingPong,
    use_cases::{AdvanceMatch, BounceBall, HandleMatchAction, MovePaddles},
};
use crate::application::PongPhysics;
use crate::domain::{Action, Controls, Match, PaddleHit};

/// Application entry point; borrows the domain model instead of owning its rules.
pub struct PlayMatch<'a> {
    game: &'a mut Match,
}

impl<'a> PlayMatch<'a> {
    pub fn new(game: &'a mut Match) -> Self {
        Self { game }
    }
}

impl ForPlayingPong for PlayMatch<'_> {
    fn act(&mut self, action: Action, physics: &mut impl PongPhysics) {
        HandleMatchAction::execute(self.game, action, physics);
    }

    fn prepare_tick(&mut self, controls: Controls, physics: &mut impl PongPhysics) {
        MovePaddles::execute(self.game, controls, physics);
    }

    fn hit_paddle(&mut self, hit: PaddleHit, physics: &mut impl PongPhysics) {
        BounceBall::execute(self.game, hit, physics);
    }

    fn finish_tick(&mut self, physics: &mut impl PongPhysics) {
        AdvanceMatch::execute(self.game, physics);
    }
}
