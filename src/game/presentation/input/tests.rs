use bevy::prelude::*;
use pong_core::{Direction, Mode};

use super::KeyboardControls;

#[test]
fn arrow_controls_use_the_supplied_mode_without_a_game_world() {
    // Given held arrow input and the mode supplied by the caller.
    let mut keys = ButtonInput::default();
    keys.press(KeyCode::ArrowUp);
    let window_focused = true;

    for (mode, expected_left) in [(Mode::Solo, Direction::Up), (Mode::Local, Direction::Still)] {
        let mut controls = KeyboardControls::default();

        // When presentation translates the input without any infrastructure resources.
        let exit = controls.capture_keyboard(&keys, window_focused, mode);

        // Then solo assigns arrows to the left player, while local reserves them for the right.
        assert_eq!(controls.movement.left, expected_left);
        assert_eq!(controls.movement.right, Direction::Up);
        assert!(controls.actions.is_empty());
        assert!(exit.is_none());
    }
}
