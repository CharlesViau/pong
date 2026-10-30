use super::palette::{INK, LEFT, LINE, RIGHT};
use bevy::prelude::*;

pub(in crate::game) struct SceneAssets {
    pub(super) quad: Handle<Mesh>,
    pub(super) court: Handle<ColorMaterial>,
    pub(super) line: Handle<ColorMaterial>,
    pub(in crate::game) left: Handle<ColorMaterial>,
    pub(in crate::game) right: Handle<ColorMaterial>,
    pub(in crate::game) ball: Handle<ColorMaterial>,
    pub(super) overlay: Handle<ColorMaterial>,
}

impl SceneAssets {
    pub(in crate::game) fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        Self {
            quad: meshes.add(Rectangle::new(1.0, 1.0)),
            court: materials.add(Color::srgb(0.04, 0.065, 0.095)),
            line: materials.add(LINE),
            left: materials.add(LEFT),
            right: materials.add(RIGHT),
            ball: materials.add(INK),
            overlay: materials.add(Color::srgb(0.025, 0.04, 0.065)),
        }
    }

    pub(in crate::game) fn rectangle<'a>(
        &self,
        commands: &'a mut Commands,
        material: &Handle<ColorMaterial>,
        size: Vec2,
        position: Vec3,
    ) -> EntityCommands<'a> {
        commands.spawn((
            Mesh2d(self.quad.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_translation(position).with_scale(size.extend(1.0)),
        ))
    }
}
