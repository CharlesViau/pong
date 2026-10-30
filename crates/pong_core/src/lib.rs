//! Engine-independent Pong rules. There is no Bevy dependency in this crate.
//!
//! Hosts implement [`PongPhysics`] and drive [`ForPlayingPong`] around each
//! fixed physics step. No engine types cross that boundary.

#![forbid(unsafe_code)]

mod application;
mod domain;

pub use application::{ForPlayingPong, PlayMatch, PongPhysics};
pub use domain::{
    Action, BallMotion, Controls, Direction, Match, Mode, PaddleHit, Phase, Position, Side,
    Velocity, rules,
};
