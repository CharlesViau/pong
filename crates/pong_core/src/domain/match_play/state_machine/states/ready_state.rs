use super::{
    super::{Effect, Event, State, Transition},
    ServingState,
};
use crate::domain::Phase;

#[derive(Debug)]
pub(in crate::domain::match_play::state_machine) struct ReadyState;

impl State for ReadyState {
    fn phase(&self) -> Phase {
        Phase::Ready
    }

    fn on_event(self: Box<Self>, event: Event) -> Transition {
        match event {
            Event::Primary => Transition::change(ServingState::new(), Effect::None),
            Event::Restart => Transition::restart(),
            _ => Transition::stay(self),
        }
    }
}
