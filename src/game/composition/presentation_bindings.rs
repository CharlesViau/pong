use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::game::{
    infrastructure::bevy::MatchResource,
    presentation::{Hud, KeyboardControls},
};

/// Binds ECS storage to presentation's core-value inputs.
pub(super) struct PresentationBindings;

impl PresentationBindings {
    pub(super) fn capture_keyboard(
        keys: Res<ButtonInput<KeyCode>>,
        window: Single<&Window, With<PrimaryWindow>>,
        game: Res<MatchResource>,
        mut controls: ResMut<KeyboardControls>,
        mut exit: MessageWriter<AppExit>,
    ) {
        if let Some(request) = controls.capture_keyboard(&keys, window.focused, game.0.mode()) {
            exit.write(request);
        }
    }

    pub(super) fn update_hud(game: Res<MatchResource>, mut hud: Hud) {
        hud.update(&game.0);
    }
}
