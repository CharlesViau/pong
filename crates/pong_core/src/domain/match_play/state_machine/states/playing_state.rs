use super::{
    super::{ActiveState, Effect, Event, State, Transition},
    FinishedState, PausedState, ServingState,
};
use crate::domain::Phase;

#[derive(Debug)]
pub(super) struct PlayingState;

impl ActiveState for PlayingState {}

impl State for PlayingState {
    fn phase(&self) -> Phase {
        Phase::Playing
    }

    fn can_advance(&self) -> bool {
        true
    }

    fn on_event(self: Box<Self>, event: Event) -> Transition {
        match event {
            Event::Primary | Event::Pause => {
                Transition::change(PausedState::new(self), Effect::None)
            }
            Event::Restart => Transition::restart(),
            Event::RallyEnded {
                winner: Some(winner),
            } => Transition::change(FinishedState::new(winner), Effect::ResetBall),
            Event::RallyEnded { winner: None } => {
                Transition::change(ServingState::new(), Effect::ResetBall)
            }
            _ => Transition::stay(self),
        }
    }
}
