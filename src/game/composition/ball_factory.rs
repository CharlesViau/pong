use avian2d::prelude::{
    Collider, CollisionEventsEnabled, Friction, LockedAxes, Position, Restitution, RigidBody,
    SleepingDisabled, SweepMode, SweptCcd,
};
use bevy::prelude::*;
use pong_core::rules;

use crate::game::{
    infrastructure::{avian::PreviousPosition, bevy::BallBody},
    presentation::SceneAssets,
};

pub(super) struct BallFactory;

impl BallFactory {
    pub(super) fn spawn(commands: &mut Commands, assets: &SceneAssets) -> Entity {
        let ball = Self::spawn_body(commands);
        Self::spawn_visual(commands, assets, ball);
        ball
    }

    fn spawn_body(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                Name::new("Ball"),
                BallBody,
                PreviousPosition::default(),
                Transform::from_xyz(0.0, 0.0, 2.0),
                Visibility::default(),
                Self::physics_components(),
            ))
            .id()
    }

    fn physics_components() -> impl Bundle {
        (
            RigidBody::Dynamic,
            Position(Vec2::ZERO),
            Collider::rectangle(rules::BALL_RADIUS * 2.0, rules::BALL_RADIUS * 2.0),
            LockedAxes::ROTATION_LOCKED,
            SleepingDisabled,
            SweptCcd::new().with_mode(SweepMode::Linear),
            Friction::ZERO,
            Restitution::PERFECTLY_ELASTIC,
            CollisionEventsEnabled,
        )
    }

    fn spawn_visual(commands: &mut Commands, assets: &SceneAssets, ball: Entity) {
        assets
            .rectangle(
                commands,
                &assets.ball,
                Vec2::splat(rules::BALL_RADIUS * 2.0),
                Vec3::ZERO,
            )
            .insert(ChildOf(ball));
    }
}
