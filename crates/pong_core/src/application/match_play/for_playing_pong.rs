use crate::application::PongPhysics;
use crate::domain::{Action, Controls, PaddleHit};

pub trait ForPlayingPong {
    fn act(&mut self, action: Action, physics: &mut impl PongPhysics);
    fn prepare_tick(&mut self, controls: Controls, physics: &mut impl PongPhysics);
    fn hit_paddle(&mut self, hit: PaddleHit, physics: &mut impl PongPhysics);
    fn finish_tick(&mut self, physics: &mut impl PongPhysics);
}
