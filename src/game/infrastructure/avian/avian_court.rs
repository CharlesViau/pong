use avian2d::prelude::{LinearVelocity, Position};
use bevy::{ecs::system::SystemParam, prelude::*};
use pong_core::{BallMotion, PongPhysics, Side, Velocity};

use super::PreviousPosition;
use crate::game::infrastructure::bevy::CourtEntities;

#[derive(SystemParam)]
pub(in crate::game) struct AvianCourt<'w, 's> {
    entities: Res<'w, CourtEntities>,
    bodies: Query<
        'w,
        's,
        (
            &'static mut Position,
            &'static mut LinearVelocity,
            &'static mut PreviousPosition,
        ),
    >,
}

impl AvianCourt<'_, '_> {
    fn reset_body(&mut self, entity: Entity, position: Vec2) {
        let (mut current, mut velocity, mut previous) =
            self.bodies.get_mut(entity).expect("court body must exist");
        current.0 = position;
        velocity.0 = Vec2::ZERO;
        previous.0 = position;
    }
}

impl PongPhysics for AvianCourt<'_, '_> {
    fn ball(&self) -> BallMotion {
        let (position, velocity, _) = self
            .bodies
            .get(self.entities.ball)
            .expect("ball must exist");
        BallMotion {
            position: pong_core::Position {
                x: position.x,
                y: position.y,
            },
            velocity: Velocity {
                x: velocity.x,
                y: velocity.y,
            },
        }
    }

    fn paddle_y(&self, side: Side) -> f32 {
        self.bodies
            .get(self.entities.paddle(side))
            .expect("paddle must exist")
            .0
            .y
    }

    fn drive_paddle(&mut self, side: Side, speed: f32) {
        let (_, mut velocity, _) = self
            .bodies
            .get_mut(self.entities.paddle(side))
            .expect("paddle must exist");
        velocity.0 = Vec2::new(0.0, speed);
    }

    fn set_ball_velocity(&mut self, velocity: Velocity) {
        let (_, mut current, _) = self
            .bodies
            .get_mut(self.entities.ball)
            .expect("ball must exist");
        current.0 = Vec2::new(velocity.x, velocity.y);
    }

    fn reset_ball(&mut self) {
        self.reset_body(self.entities.ball, Vec2::ZERO);
    }

    fn reset_court(&mut self) {
        self.reset_ball();
        for side in [Side::Left, Side::Right] {
            self.reset_body(
                self.entities.paddle(side),
                Vec2::new(side.sign() * pong_core::rules::PADDLE_X, 0.0),
            );
        }
    }
}
