use bevy::prelude::*;
use pong_core::Side;

#[derive(Resource)]
pub(in crate::game) struct CourtEntities {
    pub(in crate::game) paddles: [Entity; 2],
    pub(in crate::game) ball: Entity,
}

impl CourtEntities {
    pub(in crate::game) fn paddle(&self, side: Side) -> Entity {
        self.paddles[match side {
            Side::Left => 0,
            Side::Right => 1,
        }]
    }
}
