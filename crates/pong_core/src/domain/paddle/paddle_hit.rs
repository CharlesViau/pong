use crate::domain::Side;

#[derive(Debug, Clone, Copy)]
pub struct PaddleHit {
    pub side: Side,
    pub offset: f32,
    pub incoming_speed: f32,
}
