use crate::application::PongPhysics;
use crate::domain::{Controls, Match, Mode, PaddleMovement, Side, rules};

pub(in crate::application::match_play) struct MovePaddles;

impl MovePaddles {
    pub(in crate::application::match_play) fn execute(
        game: &Match,
        controls: Controls,
        physics: &mut impl PongPhysics,
    ) {
        if !game.can_advance() {
            return;
        }
        let left_speed = PaddleMovement::bounded_speed(
            physics.paddle_y(Side::Left),
            controls.left,
            rules::PLAYER_SPEED,
        );
        physics.drive_paddle(Side::Left, left_speed);
        Self::move_right_paddle(game.mode(), controls, physics);
    }

    fn move_right_paddle(mode: Mode, controls: Controls, physics: &mut impl PongPhysics) {
        let (direction, speed) = match mode {
            Mode::Local => (controls.right, rules::PLAYER_SPEED),
            Mode::Solo => (
                PaddleMovement::computer_direction(physics.ball(), physics.paddle_y(Side::Right)),
                rules::COMPUTER_SPEED,
            ),
        };
        let right_speed =
            PaddleMovement::bounded_speed(physics.paddle_y(Side::Right), direction, speed);
        physics.drive_paddle(Side::Right, right_speed);
    }
}
