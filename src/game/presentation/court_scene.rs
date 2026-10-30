use bevy::{camera::ScalingMode, prelude::*};
use pong_core::rules;

use super::scene_assets::SceneAssets;

pub(in crate::game) struct CourtScene;

impl CourtScene {
    pub(in crate::game) fn spawn(commands: &mut Commands, assets: &SceneAssets) {
        Self::spawn_camera(commands);
        Self::spawn_court(commands, assets);
    }

    fn spawn_camera(commands: &mut Commands) {
        commands.spawn((
            Camera2d,
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: 1080.0,
                    min_height: 780.0,
                },
                ..OrthographicProjection::default_2d()
            }),
        ));
    }

    fn spawn_court(commands: &mut Commands, assets: &SceneAssets) {
        assets.rectangle(
            commands,
            &assets.court,
            Vec2::new(rules::COURT_WIDTH, rules::COURT_HEIGHT),
            Vec3::new(0.0, 0.0, -3.0),
        );
        Self::spawn_boundary_lines(commands, assets);
        Self::spawn_center_line(commands, assets);
    }

    fn spawn_boundary_lines(commands: &mut Commands, assets: &SceneAssets) {
        for y in [-rules::COURT_HEIGHT / 2.0, rules::COURT_HEIGHT / 2.0] {
            assets.rectangle(
                commands,
                &assets.line,
                Vec2::new(rules::COURT_WIDTH, 2.0),
                Vec3::new(0.0, y, -1.0),
            );
        }
    }

    fn spawn_center_line(commands: &mut Commands, assets: &SceneAssets) {
        for y in (-240..=240).step_by(30) {
            assets.rectangle(
                commands,
                &assets.line,
                Vec2::new(2.0, 14.0),
                Vec3::new(0.0, y as f32, -1.0),
            );
        }
    }
}
