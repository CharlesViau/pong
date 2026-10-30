use avian2d::prelude::*;
use bevy::prelude::*;
use pong_core::rules;

pub(in crate::game) struct CourtWalls;

impl CourtWalls {
    pub(in crate::game) fn spawn(commands: &mut Commands) {
        let thickness = 20.0;
        for sign in [-1.0, 1.0] {
            commands.spawn((
                Name::new("Court wall collider"),
                RigidBody::Static,
                Transform::default(),
                Position(Vec2::new(
                    0.0,
                    sign * (rules::COURT_HEIGHT + thickness) / 2.0,
                )),
                Collider::rectangle(rules::COURT_WIDTH + 100.0, thickness),
                Friction::ZERO,
                Restitution::PERFECTLY_ELASTIC,
            ));
        }
    }
}
