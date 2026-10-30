use super::super::{ActiveState, Event, State, Transition};
use crate::domain::Phase;

#[derive(Debug)]
pub(super) struct PausedState {
    interrupted: Box<dyn ActiveState>,
}

impl PausedState {
    pub(super) fn new(interrupted: Box<dyn ActiveState>) -> Self {
        Self { interrupted }
    }
}

impl State for PausedState {
    fn phase(&self) -> Phase {
        Phase::Paused
    }

    fn serve_ticks_remaining(&self) -> u16 {
        self.interrupted.serve_ticks_remaining()
    }

    fn on_event(self: Box<Self>, event: Event) -> Transition {
        match event {
            Event::Primary => Transition::stay(self.interrupted),
            Event::Restart => Transition::restart(),
            // Focus loss and elapsed ticks cannot resume or nest a pause.
            _ => Transition::stay(self),
        }
    }
}
