use avian2d::prelude::{Collider, Friction, Position, Restitution, RigidBody};
use bevy::prelude::*;
use pong_core::{Side, rules};

use crate::game::{
    infrastructure::{avian::PreviousPosition, bevy::PaddleBody},
    presentation::SceneAssets,
};

pub(super) struct PaddleFactory;

impl PaddleFactory {
    pub(super) fn spawn(commands: &mut Commands, assets: &SceneAssets, side: Side) -> Entity {
        let paddle = Self::spawn_body(commands, side);
        Self::spawn_visual(commands, assets, paddle, side);
        paddle
    }

    fn spawn_body(commands: &mut Commands, side: Side) -> Entity {
        commands
            .spawn((
                Name::new(format!("{side:?} paddle")),
                PaddleBody,
                PreviousPosition::default(),
                Transform::from_xyz(side.sign() * rules::PADDLE_X, 0.0, 1.0),
                Visibility::default(),
                Self::physics_components(side),
            ))
            .id()
    }

    fn physics_components(side: Side) -> impl Bundle {
        (
            RigidBody::Kinematic,
            Position(Vec2::new(side.sign() * rules::PADDLE_X, 0.0)),
            Collider::rectangle(rules::PADDLE_WIDTH, rules::PADDLE_HEIGHT),
            Friction::ZERO,
            Restitution::PERFECTLY_ELASTIC,
        )
    }

    fn spawn_visual(commands: &mut Commands, assets: &SceneAssets, paddle: Entity, side: Side) {
        assets
            .rectangle(
                commands,
                if side == Side::Left {
                    &assets.left
                } else {
                    &assets.right
                },
                Vec2::new(rules::PADDLE_WIDTH, rules::PADDLE_HEIGHT),
                Vec3::ZERO,
            )
            .insert(ChildOf(paddle));
    }
}
