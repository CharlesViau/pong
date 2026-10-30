use avian2d::prelude::Position;
use bevy::prelude::*;

use super::PreviousPosition;

pub(in crate::game) struct BodyInterpolation;

impl BodyInterpolation {
    pub(in crate::game) fn sync_positions(
        time: Res<Time<Fixed>>,
        mut bodies: Query<(&Position, &PreviousPosition, &mut Transform)>,
    ) {
        for (position, previous, mut transform) in &mut bodies {
            let rendered = previous.0.lerp(position.0, time.overstep_fraction());
            transform.translation.x = rendered.x;
            transform.translation.y = rendered.y;
        }
    }
}
