use bevy::prelude::*;
use pong_core::Match;

#[derive(Resource, Default)]
pub(in crate::game) struct MatchResource(pub(in crate::game) Match);
