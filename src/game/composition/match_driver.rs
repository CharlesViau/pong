use bevy::prelude::*;
use pong_core::{ForPlayingPong, PlayMatch};

use crate::game::{
    infrastructure::{avian::AvianCourt, bevy::MatchResource},
    presentation::KeyboardControls,
};

pub(in crate::game) struct MatchDriver;

impl MatchDriver {
    pub(in crate::game) fn prepare_tick(
        mut game: ResMut<MatchResource>,
        mut controls: ResMut<KeyboardControls>,
        mut court: AvianCourt,
    ) {
        let mut play = PlayMatch::new(&mut game.0);
        for action in controls.actions.drain(..) {
            play.act(action, &mut court);
        }
        play.prepare_tick(controls.movement, &mut court);
    }

    pub(in crate::game) fn finish_tick(mut game: ResMut<MatchResource>, mut court: AvianCourt) {
        PlayMatch::new(&mut game.0).finish_tick(&mut court);
    }
}
