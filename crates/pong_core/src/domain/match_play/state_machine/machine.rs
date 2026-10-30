use super::{Effect, Event, State, states::ReadyState};
use crate::domain::Phase;

#[derive(Debug)]
pub(crate) struct StateMachine {
    state: Box<dyn State>,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self {
            state: Box::new(ReadyState),
        }
    }
}

impl StateMachine {
    pub(crate) fn phase(&self) -> Phase {
        self.state.phase()
    }

    pub(crate) fn can_advance(&self) -> bool {
        self.state.can_advance()
    }

    pub(crate) fn serve_ticks_remaining(&self) -> u16 {
        self.state.serve_ticks_remaining()
    }

    pub(crate) fn transition(&mut self, event: Event) -> Effect {
        // Moving the current box lets a pause retain it. The temporary ReadyState
        // is zero-sized; ordinary ticks return the existing box without allocating.
        let current = std::mem::replace(&mut self.state, Box::new(ReadyState));
        let transition = current.on_event(event);
        self.state = transition.state;
        transition.effect
    }
}
