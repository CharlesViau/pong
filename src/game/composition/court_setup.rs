use bevy::prelude::*;
use pong_core::Side;

use super::{ball_factory::BallFactory, paddle_factory::PaddleFactory};
use crate::game::{
    infrastructure::avian::CourtWalls,
    infrastructure::bevy::CourtEntities,
    presentation::{CourtScene, Hud, SceneAssets},
};

pub(in crate::game) struct CourtSetup;

impl CourtSetup {
    pub(in crate::game) fn spawn(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let assets = SceneAssets::new(&mut meshes, &mut materials);
        CourtScene::spawn(&mut commands, &assets);
        Self::spawn_game_pieces(&mut commands, &assets);
        Hud::spawn(&mut commands, &assets);
        CourtWalls::spawn(&mut commands);
    }

    fn spawn_game_pieces(commands: &mut Commands, assets: &SceneAssets) {
        let paddles =
            [Side::Left, Side::Right].map(|side| PaddleFactory::spawn(commands, assets, side));
        let ball = BallFactory::spawn(commands, assets);
        commands.insert_resource(CourtEntities { paddles, ball });
    }
}
