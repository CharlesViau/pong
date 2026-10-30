use std::fmt::Debug;

use super::{Event, Transition};
use crate::domain::Phase;

pub(super) trait State: Debug + Send + Sync {
    fn phase(&self) -> Phase;

    fn can_advance(&self) -> bool {
        false
    }

    fn serve_ticks_remaining(&self) -> u16 {
        0
    }

    fn on_event(self: Box<Self>, event: Event) -> Transition;
}
