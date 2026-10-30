use std::collections::VecDeque;

use bevy::{app::AppExit, prelude::*};
use pong_core::{Action, Controls, Direction, Mode};

#[derive(Resource, Default)]
pub(in crate::game) struct KeyboardControls {
    pub movement: Controls,
    // Retain one-shot actions on render frames that have no fixed simulation tick.
    pub actions: VecDeque<Action>,
    was_focused: bool,
}

impl KeyboardControls {
    fn lose_focus(&mut self) {
        self.movement = Controls::default();
        if self.was_focused {
            self.actions.push_back(Action::Pause);
        }
        self.was_focused = false;
    }

    fn capture_actions(&mut self, keys: &ButtonInput<KeyCode>) {
        for (key, action) in [
            (KeyCode::Tab, Action::SwitchMode),
            (KeyCode::KeyR, Action::Restart),
            (KeyCode::Space, Action::Primary),
        ] {
            if keys.just_pressed(key) {
                self.actions.push_back(action);
            }
        }
    }

    pub(in crate::game) fn capture_keyboard(
        &mut self,
        keys: &ButtonInput<KeyCode>,
        focused: bool,
        mode: Mode,
    ) -> Option<AppExit> {
        if !focused {
            self.lose_focus();
            return None;
        }
        self.was_focused = true;
        self.capture_actions(keys);
        self.movement = Self::movement_from_keyboard(keys, mode);
        keys.just_pressed(KeyCode::Escape)
            .then_some(AppExit::Success)
    }

    fn movement_from_keyboard(keys: &ButtonInput<KeyCode>, mode: Mode) -> Controls {
        let solo = mode == Mode::Solo;
        Controls {
            left: Self::direction(
                keys.pressed(KeyCode::KeyW) || solo && keys.pressed(KeyCode::ArrowUp),
                keys.pressed(KeyCode::KeyS) || solo && keys.pressed(KeyCode::ArrowDown),
            ),
            right: Self::direction(
                keys.pressed(KeyCode::ArrowUp),
                keys.pressed(KeyCode::ArrowDown),
            ),
        }
    }

    fn direction(up: bool, down: bool) -> Direction {
        match (up, down) {
            (true, false) => Direction::Up,
            (false, true) => Direction::Down,
            _ => Direction::Still,
        }
    }
}
