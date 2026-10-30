//! Fresh headless apps and deterministic clocks for adapter behavior tests.
use std::time::Duration;

use avian2d::prelude::{LinearVelocity, Position};
use bevy::{app::AppExit, prelude::*, time::TimeUpdateStrategy, window::PrimaryWindow};
use pong_core::{Action, Controls, Match, Phase, Side, rules};

use crate::game::{
    PongPlugin, infrastructure::bevy::CourtEntities, infrastructure::bevy::MatchResource,
    presentation::KeyboardControls,
};

pub const FRAME_HZ: f64 = 60.0;

fn headless_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin))
        .add_message::<AppExit>()
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<ColorMaterial>>()
        .add_plugins(PongPlugin);
    app.world_mut().spawn((
        Window {
            focused: true,
            ..default()
        },
        PrimaryWindow,
    ));
    app.finish();
    app.cleanup();
    app
}

/// For manually driven schedules only; use frame_app when testing App::update.
pub fn fixed_app() -> App {
    let mut app = headless_app();
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_secs_f32(rules::TICK_SECONDS));
    app.world_mut().run_schedule(Startup);
    app
}

/// Startup is left to App::update, so it runs exactly once.
pub fn frame_app() -> App {
    let mut app = headless_app();
    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
        1.0 / FRAME_HZ,
    )));
    app.update();
    app
}

pub fn playing_app() -> App {
    let mut app = fixed_app();
    queue_action(&mut app, Action::SwitchMode);
    queue_action(&mut app, Action::Primary);
    step(&mut app, usize::from(rules::SERVE_TICKS));
    assert_eq!(
        game(&app).phase(),
        Phase::Playing,
        "fixture must start an active local rally"
    );
    assert_eq!(
        ball(&app).0,
        Vec2::ZERO,
        "serve launches at the end of the last tick"
    );
    app
}

pub fn step(app: &mut App, ticks: usize) {
    for _ in 0..ticks {
        app.world_mut().run_schedule(FixedUpdate);
    }
}

pub fn advance_frames(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

pub fn queue_action(app: &mut App, action: Action) {
    app.world_mut()
        .resource_mut::<KeyboardControls>()
        .actions
        .push_back(action);
}

pub fn set_movement(app: &mut App, controls: Controls) {
    app.world_mut().resource_mut::<KeyboardControls>().movement = controls;
}

pub fn press(app: &mut App, key: KeyCode) {
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(key);
    capture_input(app);
}

pub fn release(app: &mut App, key: KeyCode) {
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(key);
    capture_input(app);
}

pub fn capture_input(app: &mut App) {
    app.world_mut().run_schedule(PreUpdate);
    // Clear edge flags but preserve held keys, like the next input frame.
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
}

pub fn set_focus(app: &mut App, focused: bool) {
    let mut windows = app
        .world_mut()
        .query_filtered::<&mut Window, With<PrimaryWindow>>();
    windows.single_mut(app.world_mut()).unwrap().focused = focused;
    capture_input(app);
}

pub fn game(app: &App) -> &Match {
    &app.world().resource::<MatchResource>().0
}

pub fn paddle(app: &App, side: Side) -> Entity {
    let index = match side {
        Side::Left => 0,
        Side::Right => 1,
    };
    app.world().resource::<CourtEntities>().paddles[index]
}

pub fn paddle_y(app: &App, side: Side) -> f32 {
    app.world().get::<Position>(paddle(app, side)).unwrap().y
}

pub fn ball_entity(app: &App) -> Entity {
    app.world().resource::<CourtEntities>().ball
}

pub fn ball(app: &App) -> (Vec2, Vec2) {
    let entity = app.world().entity(ball_entity(app));
    (
        entity.get::<Position>().unwrap().0,
        entity.get::<LinearVelocity>().unwrap().0,
    )
}

pub fn place_ball(app: &mut App, position: Vec2, velocity: Vec2) {
    let entity = ball_entity(app);
    app.world_mut()
        .entity_mut(entity)
        .insert((Position(position), LinearVelocity(velocity)));
}

#[track_caller]
pub fn assert_near(actual: f32, expected: f32, tolerance: f32) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}; tolerance {tolerance}"
    );
}
