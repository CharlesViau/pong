use super::{
    super::{Effect, Event, State, Transition},
    ServingState,
};
use crate::domain::{Phase, Side};

#[derive(Debug)]
pub(super) struct FinishedState {
    winner: Side,
}

impl FinishedState {
    pub(super) fn new(winner: Side) -> Self {
        Self { winner }
    }
}

impl State for FinishedState {
    fn phase(&self) -> Phase {
        Phase::Finished(self.winner)
    }

    fn on_event(self: Box<Self>, event: Event) -> Transition {
        match event {
            Event::Primary => Transition::change(ServingState::new(), Effect::ResetMatch),
            Event::Restart => Transition::restart(),
            _ => Transition::stay(self),
        }
    }
}
