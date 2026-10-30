use super::{HudSnapshot, HudText, Overlay};
use crate::game::presentation::{
    palette::{INK, LEFT, MUTED, RIGHT},
    scene_assets::SceneAssets,
};
use bevy::{ecs::system::SystemParam, prelude::*};
use pong_core::{Match, rules};

#[derive(SystemParam)]
pub(in crate::game) struct Hud<'w, 's> {
    previous: Local<'s, Option<HudSnapshot>>,
    labels: Query<'w, 's, (&'static HudText, &'static mut Text2d)>,
    overlays: Query<'w, 's, &'static mut Visibility, With<Overlay>>,
}

impl Hud<'_, '_> {
    pub(in crate::game) fn spawn(commands: &mut Commands, assets: &SceneAssets) {
        Self::spawn_scoreboard(commands);
        Self::spawn_instructions(commands);
        Self::spawn_overlay(commands, assets);
    }

    fn spawn_scoreboard(commands: &mut Commands) {
        Self::text(commands, "P O N G", 25.0, Vec3::new(0.0, 354.0, 1.0), INK);
        Self::text(
            commands,
            &format!("FIRST TO {}", rules::WINNING_SCORE),
            12.0,
            Vec3::new(0.0, 317.0, 1.0),
            MUTED,
        );
        for (x, color, score, label) in [
            (-170.0, LEFT, HudText::LeftScore, HudText::LeftLabel),
            (170.0, RIGHT, HudText::RightScore, HudText::RightLabel),
        ] {
            Self::text(commands, "0", 50.0, Vec3::new(x, 318.0, 1.0), color).insert(score);
            Self::text(commands, "", 14.0, Vec3::new(x * 2.1, 318.0, 1.0), color).insert(label);
        }
    }

    fn spawn_instructions(commands: &mut Commands) {
        Self::text(commands, "", 14.0, Vec3::new(0.0, -298.0, 1.0), INK).insert(HudText::Status);
        Self::text(commands, "", 14.0, Vec3::new(0.0, -327.0, 1.0), MUTED).insert(HudText::Mode);
        Self::text(
            commands,
            "SPACE  start / pause    R  restart    TAB  switch mode    ESC  quit",
            13.0,
            Vec3::new(0.0, -358.0, 1.0),
            MUTED,
        );
    }

    fn spawn_overlay(commands: &mut Commands, assets: &SceneAssets) {
        assets
            .rectangle(
                commands,
                &assets.overlay,
                Vec2::new(450.0, 170.0),
                Vec3::new(0.0, 0.0, 10.0),
            )
            .insert(Overlay);
        Self::text(commands, "", 38.0, Vec3::new(0.0, 25.0, 11.0), INK)
            .insert((HudText::OverlayTitle, Overlay));
        Self::text(commands, "", 17.0, Vec3::new(0.0, -30.0, 11.0), MUTED)
            .insert((HudText::OverlayHint, Overlay));
    }

    fn text<'a>(
        commands: &'a mut Commands,
        value: &str,
        size: f32,
        position: Vec3,
        color: Color,
    ) -> EntityCommands<'a> {
        commands.spawn((
            Text2d::new(value),
            TextFont::from_font_size(size),
            TextColor(color),
            TextLayout::justify(Justify::Center),
            Transform::from_translation(position),
        ))
    }

    pub(in crate::game) fn update(&mut self, game: &Match) {
        let snapshot = HudSnapshot::from_match(game);
        if self.previous.as_ref() == Some(&snapshot) {
            return;
        }
        for mut visibility in &mut self.overlays {
            *visibility = snapshot.overlay_visibility();
        }
        for (label, mut text) in &mut self.labels {
            text.0 = snapshot.text_for(label);
        }
        *self.previous = Some(snapshot);
    }
}
