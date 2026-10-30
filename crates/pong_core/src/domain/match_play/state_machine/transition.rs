use super::{Effect, State, states::ReadyState};

pub(super) struct Transition {
    pub(super) state: Box<dyn State>,
    pub(super) effect: Effect,
}

impl Transition {
    pub(super) fn stay(state: Box<dyn State>) -> Self {
        Self {
            state,
            effect: Effect::None,
        }
    }

    pub(super) fn change(state: impl State + 'static, effect: Effect) -> Self {
        Self {
            state: Box::new(state),
            effect,
        }
    }

    pub(super) fn restart() -> Self {
        Self::change(ReadyState, Effect::ResetMatch)
    }
}
