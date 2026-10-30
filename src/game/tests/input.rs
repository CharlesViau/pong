use bevy::{app::AppExit, prelude::*};
use pong_core::{Phase, Side};

use super::support::*;
use crate::game::presentation::KeyboardControls;

#[test]
fn escape_exits_only_while_the_window_is_focused() {
    // Given a fresh game with either focused or unfocused input.
    for focused in [true, false] {
        let mut app = fixed_app();
        set_focus(&mut app, focused);

        // When Escape is pressed through the bound input system.
        press(&mut app, KeyCode::Escape);

        // Then composition forwards the presentation request only for the focused window.
        assert_eq!(app.should_exit().is_some(), focused);
        if focused {
            assert_eq!(app.should_exit(), Some(AppExit::Success));
        }
    }
}

#[test]
fn a_short_space_press_survives_frames_without_a_fixed_tick() {
    // Given a ready game between fixed ticks.
    let mut app = fixed_app();

    // When Space is pressed and released across input frames before simulation runs.
    press(&mut app, KeyCode::Space);
    release(&mut app, KeyCode::Space);
    capture_input(&mut app);

    // Then the action waits for a fixed tick and is consumed exactly once.
    assert_eq!(game(&app).phase(), Phase::Ready);
    step(&mut app, 1);
    assert_eq!(game(&app).phase(), Phase::Serving);
    step(&mut app, 1);
    assert_eq!(game(&app).phase(), Phase::Serving);
    assert!(
        app.world()
            .resource::<KeyboardControls>()
            .actions
            .is_empty()
    );
}

#[test]
fn keyboard_movement_reaches_physics_and_the_render_transform() {
    const TICKS_FOR_INTERPOLATED_MOVEMENT: usize = 2;
    // Given a ready game with the left paddle centered.
    let mut app = fixed_app();

    // When play starts and W is held for enough ticks to render movement.
    press(&mut app, KeyCode::Space);
    press(&mut app, KeyCode::KeyW);
    step(&mut app, TICKS_FOR_INTERPOLATED_MOVEMENT);
    app.world_mut().run_schedule(Update);

    // Then both the physical position and rendered position move upward.
    assert!(paddle_y(&app, Side::Left) > 0.0);
    let rendered = app
        .world()
        .get::<Transform>(paddle(&app, Side::Left))
        .unwrap();
    assert!(rendered.translation.y > 0.0);
}

#[test]
fn opposite_movement_keys_cancel() {
    // Given a game with W and S held together.
    let mut app = fixed_app();
    press(&mut app, KeyCode::Space);
    press(&mut app, KeyCode::KeyW);
    press(&mut app, KeyCode::KeyS);

    // When one simulation tick runs.
    step(&mut app, 1);

    // Then the left paddle remains centered.
    assert_eq!(paddle_y(&app, Side::Left), 0.0);
}

#[test]
fn losing_focus_pauses_and_regaining_focus_does_not_resume() {
    // Given a focused, active game.
    let mut app = fixed_app();
    press(&mut app, KeyCode::Space);
    step(&mut app, 1);

    // When the window loses focus.
    set_focus(&mut app, false);
    step(&mut app, 1);

    // Then the game pauses.
    assert_eq!(game(&app).phase(), Phase::Paused);

    // When focus returns, then an explicit resume is still required.
    set_focus(&mut app, true);
    step(&mut app, 1);
    assert_eq!(game(&app).phase(), Phase::Paused);
}

#[test]
fn restart_resets_the_rendered_paddle_as_well_as_physics() {
    const MOVEMENT_TICKS: usize = 20;
    // Given a paddle moved away from the center.
    let mut app = fixed_app();
    press(&mut app, KeyCode::Space);
    press(&mut app, KeyCode::KeyW);
    step(&mut app, MOVEMENT_TICKS);
    assert!(paddle_y(&app, Side::Left) > 0.0);

    // When R restarts the game and presentation updates.
    press(&mut app, KeyCode::KeyR);
    step(&mut app, 1);
    app.world_mut().run_schedule(Update);

    // Then physical and rendered positions both reset to the center.
    assert_eq!(paddle_y(&app, Side::Left), 0.0);
    let rendered = app
        .world()
        .get::<Transform>(paddle(&app, Side::Left))
        .unwrap();
    assert_eq!(rendered.translation.y, 0.0);
}

#[test]
fn switching_mode_updates_the_instructions() {
    const LOCAL_MODE_LABEL: &str = "TWO PLAYERS";
    // Given a ready solo game.
    let mut app = fixed_app();

    // When Tab switches mode and presentation updates.
    press(&mut app, KeyCode::Tab);
    step(&mut app, 1);
    app.world_mut().run_schedule(Update);

    // Then the instructions identify local play.
    let mut texts = app.world_mut().query::<&Text2d>();
    assert!(
        texts
            .iter(app.world())
            .any(|text| text.0.contains(LOCAL_MODE_LABEL))
    );
}
