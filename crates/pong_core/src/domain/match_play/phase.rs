use crate::domain::Side;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Ready,
    Serving,
    Playing,
    Paused,
    Finished(Side),
}
