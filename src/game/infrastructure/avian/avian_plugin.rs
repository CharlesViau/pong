use avian2d::{physics_transform::PhysicsTransformConfig, prelude::*};
use bevy::prelude::*;
use pong_core::Phase;

use super::{body_interpolation::BodyInterpolation, paddle_contacts::PaddleContacts};
use crate::game::infrastructure::bevy::MatchResource;

pub(in crate::game) struct AvianPlugin;

impl Plugin for AvianPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec2::ZERO))
            .insert_resource(PhysicsTransformConfig {
                propagate_before_physics: false,
                transform_to_position: false,
                position_to_transform: false,
                transform_to_collider_scale: false,
            })
            .add_plugins(PhysicsPlugins::new(FixedUpdate).with_length_unit(100.0))
            .configure_sets(
                FixedUpdate,
                PhysicsSystems::StepSimulation.run_if(Self::is_active),
            )
            .add_systems(
                PhysicsSchedule,
                PaddleContacts::respond
                    .after(PhysicsStepSystems::NarrowPhase)
                    .before(PhysicsStepSystems::Solver),
            )
            .add_systems(Update, BodyInterpolation::sync_positions);
    }
}

impl AvianPlugin {
    fn is_active(game: Res<MatchResource>) -> bool {
        matches!(game.0.phase(), Phase::Serving | Phase::Playing)
    }
}
