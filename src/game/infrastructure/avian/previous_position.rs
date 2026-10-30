use avian2d::prelude::Position;
use bevy::prelude::*;

/// Position at the start of a fixed tick; teleports reset it with the current position.
#[derive(Component, Default)]
pub(in crate::game) struct PreviousPosition(pub(in crate::game) Vec2);

impl PreviousPosition {
    pub(in crate::game) fn capture(mut bodies: Query<(&Position, &mut PreviousPosition)>) {
        for (position, mut previous) in &mut bodies {
            previous.0 = position.0;
        }
    }
}
