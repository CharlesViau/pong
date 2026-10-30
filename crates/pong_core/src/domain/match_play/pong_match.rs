use super::{
    Effect, Mode, Phase,
    state_machine::{Event, StateMachine},
};
use crate::domain::{Action, BallRules, Side, Velocity, rules};

/// Owns match progress and rules without knowing how physics is performed.
#[derive(Debug)]
pub struct Match {
    mode: Mode,
    state_machine: StateMachine,
    scores: [u8; 2],
    serve_toward: Side,
}

impl Default for Match {
    fn default() -> Self {
        Self {
            mode: Mode::Solo,
            state_machine: StateMachine::default(),
            scores: [0, 0],
            serve_toward: Side::Right,
        }
    }
}

impl Match {
    pub fn phase(&self) -> Phase {
        self.state_machine.phase()
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn score(&self, side: Side) -> u8 {
        self.scores[side.index()]
    }

    pub fn serve_ticks_remaining(&self) -> u16 {
        self.state_machine.serve_ticks_remaining()
    }

    pub(crate) fn can_advance(&self) -> bool {
        self.state_machine.can_advance()
    }

    pub(crate) fn handle_action(&mut self, action: Action) -> Effect {
        let event = match action {
            Action::Primary => Event::Primary,
            Action::Restart => Event::Restart,
            Action::Pause => Event::Pause,
            Action::SwitchMode => {
                self.mode = match self.mode {
                    Mode::Solo => Mode::Local,
                    Mode::Local => Mode::Solo,
                };
                Event::Restart
            }
        };
        self.apply_event(event)
    }

    pub(crate) fn advance(&mut self, ball_x: Option<f32>) -> Effect {
        if self.phase() == Phase::Playing
            && let Some(scorer) = ball_x.and_then(BallRules::scorer)
        {
            return self.award_point(scorer);
        }
        self.apply_event(Event::Tick)
    }

    pub(crate) fn serve_velocity(&self) -> Velocity {
        let upward = (self.scores[0] + self.scores[1]).is_multiple_of(2);
        BallRules::serve_velocity(self.serve_toward, upward)
    }

    fn apply_event(&mut self, event: Event) -> Effect {
        let effect = self.state_machine.transition(event);
        if matches!(effect, Effect::ResetMatch) {
            self.scores = [0, 0];
            self.serve_toward = Side::Right;
        }
        effect
    }

    fn award_point(&mut self, scorer: Side) -> Effect {
        self.scores[scorer.index()] += 1;
        self.serve_toward = scorer.opponent();
        let winner = (self.score(scorer) == rules::WINNING_SCORE).then_some(scorer);
        self.apply_event(Event::RallyEnded { winner })
    }
}
