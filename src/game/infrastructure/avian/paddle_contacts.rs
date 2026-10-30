use avian2d::prelude::CollisionStart;
use bevy::prelude::*;
use pong_core::{ForPlayingPong, PaddleHit, PlayMatch, PongPhysics, Side};

use super::AvianCourt;
use crate::game::infrastructure::bevy::{CourtEntities, MatchResource};

pub(super) struct PaddleContacts;

impl PaddleContacts {
    pub(super) fn respond(
        mut game: ResMut<MatchResource>,
        mut court: AvianCourt,
        entities: Res<CourtEntities>,
        mut collisions: MessageReader<CollisionStart>,
    ) {
        let velocity = court.ball().velocity;
        let incoming = Vec2::new(velocity.x, velocity.y);
        for contact in collisions.read() {
            if let Some(hit) = Self::translate(contact, &entities, &court, incoming) {
                PlayMatch::new(&mut game.0).hit_paddle(hit, &mut court);
            }
        }
    }

    fn translate(
        contact: &CollisionStart,
        entities: &CourtEntities,
        physics: &impl PongPhysics,
        incoming: Vec2,
    ) -> Option<PaddleHit> {
        let side = Self::contacted_paddle(contact, entities)?;
        if incoming.x * side.sign() <= 0.0 {
            return None;
        }
        Some(PaddleHit {
            side,
            offset: physics.ball().position.y - physics.paddle_y(side),
            incoming_speed: incoming.length(),
        })
    }

    fn contacted_paddle(contact: &CollisionStart, entities: &CourtEntities) -> Option<Side> {
        let other = if contact.collider1 == entities.ball {
            contact.collider2
        } else if contact.collider2 == entities.ball {
            contact.collider1
        } else {
            return None;
        };
        [Side::Left, Side::Right]
            .into_iter()
            .find(|&side| entities.paddle(side) == other)
    }
}
