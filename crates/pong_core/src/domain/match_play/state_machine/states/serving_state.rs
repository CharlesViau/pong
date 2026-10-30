use super::{
    super::{ActiveState, Effect, Event, State, Transition},
    PausedState, PlayingState,
};
use crate::domain::{Phase, rules};

#[derive(Debug)]
pub(super) struct ServingState {
    remaining_ticks: u16,
}

impl ServingState {
    pub(super) fn new() -> Self {
        Self {
            remaining_ticks: rules::SERVE_TICKS,
        }
    }

    fn advance(mut self: Box<Self>) -> Transition {
        if self.remaining_ticks == 1 {
            return Transition::change(PlayingState, Effect::ServeBall);
        }
        self.remaining_ticks -= 1;
        Transition::stay(self)
    }
}

impl ActiveState for ServingState {}

impl State for ServingState {
    fn phase(&self) -> Phase {
        Phase::Serving
    }

    fn can_advance(&self) -> bool {
        true
    }

    fn serve_ticks_remaining(&self) -> u16 {
        self.remaining_ticks
    }

    fn on_event(self: Box<Self>, event: Event) -> Transition {
        match event {
            Event::Primary | Event::Pause => {
                Transition::change(PausedState::new(self), Effect::None)
            }
            Event::Restart => Transition::restart(),
            Event::Tick => self.advance(),
            _ => Transition::stay(self),
        }
    }
}
