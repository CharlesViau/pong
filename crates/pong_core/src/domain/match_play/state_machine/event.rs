use crate::domain::Side;

pub(crate) enum Event {
    Primary,
    Pause,
    Restart,
    Tick,
    RallyEnded { winner: Option<Side> },
}
