#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    #[default]
    Still,
}

impl Direction {
    pub(crate) fn sign(self) -> f32 {
        match self {
            Self::Up => 1.0,
            Self::Down => -1.0,
            Self::Still => 0.0,
        }
    }
}
