use avian2d::prelude::PhysicsSystems;
use bevy::{input::InputSystems, prelude::*};
use pong_core::rules;

use super::{CourtSetup, match_driver::MatchDriver, presentation_bindings::PresentationBindings};
use crate::game::{
    infrastructure::{
        avian::{AvianPlugin, PreviousPosition},
        bevy::MatchResource,
    },
    presentation::KeyboardControls,
};
pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MatchResource>()
            .add_plugins(AvianPlugin)
            .init_resource::<KeyboardControls>()
            .insert_resource(Time::<Fixed>::from_hz(rules::TICK_HZ))
            .insert_resource(ClearColor(Color::srgb(0.025, 0.04, 0.065)))
            .add_systems(Startup, CourtSetup::spawn)
            .add_systems(
                PreUpdate,
                PresentationBindings::capture_keyboard.after(InputSystems),
            )
            .add_systems(
                FixedUpdate,
                (
                    (MatchDriver::prepare_tick, PreviousPosition::capture)
                        .chain()
                        .in_set(PhysicsSystems::First),
                    MatchDriver::finish_tick.in_set(PhysicsSystems::Last),
                ),
            )
            .add_systems(Update, PresentationBindings::update_hud);
    }
}
