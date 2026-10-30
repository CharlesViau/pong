use std::ops::RangeInclusive;

pub(super) struct VisiblePiece {
    pub(super) name: &'static str,
    pub(super) rgb: [RangeInclusive<f32>; 3],
    pub(super) minimum_pixels: usize,
}
