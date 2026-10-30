use super::{BEYOND_GOAL_MARGIN, GOAL_CENTER_LIMIT, PhysicsMock};
use pong_core::{
    Action, Controls, ForPlayingPong, Match, Mode, PaddleHit, Phase, PlayMatch, Side, rules,
};
/// Each test gets its own match and mock. Setup uses the public driving port.
#[derive(Default)]
pub struct MatchFixture {
    pub game: Match,
    pub physics: PhysicsMock,
}

impl MatchFixture {
    pub fn serving(mode: Mode) -> Self {
        let mut fixture = Self::default();
        if mode == Mode::Local {
            fixture.act(Action::SwitchMode);
        }
        fixture.act(Action::Primary);
        fixture.physics.clear_commands();
        fixture
    }

    pub fn playing(mode: Mode) -> Self {
        let mut fixture = Self::serving(mode);
        fixture.advance(rules::SERVE_TICKS);
        fixture.physics.clear_commands();
        fixture
    }

    pub fn act(&mut self, action: Action) {
        PlayMatch::new(&mut self.game).act(action, &mut self.physics);
    }

    pub fn prepare_tick(&mut self, controls: Controls) {
        PlayMatch::new(&mut self.game).prepare_tick(controls, &mut self.physics);
    }

    pub fn finish_tick(&mut self) {
        PlayMatch::new(&mut self.game).finish_tick(&mut self.physics);
    }

    pub fn hit_paddle(&mut self, hit: PaddleHit) {
        PlayMatch::new(&mut self.game).hit_paddle(hit, &mut self.physics);
    }

    pub fn advance(&mut self, ticks: u16) {
        for _ in 0..ticks {
            self.prepare_tick(Controls::default());
            self.finish_tick();
        }
    }

    pub fn score_point(&mut self, scorer: Side) {
        if self.game.phase() == Phase::Serving {
            self.advance(self.game.serve_ticks_remaining());
        }
        assert_eq!(
            self.game.phase(),
            Phase::Playing,
            "scoring fixture needs an active rally"
        );
        self.physics.ball.position.x =
            scorer.opponent().sign() * (GOAL_CENTER_LIMIT + BEYOND_GOAL_MARGIN);
        self.finish_tick();
    }

    pub fn finish_match(&mut self, winner: Side) {
        for _ in self.game.score(winner)..rules::WINNING_SCORE {
            self.score_point(winner);
        }
        assert_eq!(self.game.phase(), Phase::Finished(winner));
    }
}
