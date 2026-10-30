use pong_core::{Mode, PaddleHit, Side, Velocity, rules};

use crate::support::{FLOAT_TOLERANCE, MatchFixture, PhysicsCommand, assert_close};

const CENTER_HIT: f32 = 0.0;
const INCOMING_HORIZONTAL_SPEED: f32 = 400.0;
// Independent rule examples: a 400-unit/s hit gains 7%, and the maximum angle is 60 degrees.
const EXPECTED_ACCELERATED_SPEED: f32 = 428.0;
const EXPECTED_MAX_ANGLE_DEGREES: f32 = 60.0;
const EXPECTED_MIN_ANGLE_DEGREES: f32 = 12.0;

#[test]
fn a_paddle_hit_turns_the_ball_away_and_increases_speed_once() {
    for side in [Side::Left, Side::Right] {
        // Given a centered hit during an active rally.
        let mut fixture = MatchFixture::playing(Mode::Solo);

        // When the paddle contact is reported.
        fixture.hit_paddle(PaddleHit {
            side,
            offset: CENTER_HIT,
            incoming_speed: INCOMING_HORIZONTAL_SPEED,
        });

        // Then one velocity command sends it away with the expected speed increase.
        let outgoing = fixture.physics.ball.velocity;
        assert!(outgoing.x * side.sign() < 0.0, "{side:?}: {outgoing:?}");
        assert_close(outgoing.speed(), EXPECTED_ACCELERATED_SPEED);
        fixture
            .physics
            .assert_commands(&[PhysicsCommand::SetBallVelocity(outgoing)]);
    }
}

#[test]
fn extreme_contact_offsets_clamp_the_angle_and_the_ball_speed() {
    const OFFSET_BEYOND_PADDLE: f32 = 1_000.0;
    for side in [Side::Left, Side::Right] {
        for offset in [-OFFSET_BEYOND_PADDLE, OFFSET_BEYOND_PADDLE] {
            // Given a ball already at its speed cap and an extreme contact offset.
            let mut fixture = MatchFixture::playing(Mode::Solo);

            // When the contact is reported.
            fixture.hit_paddle(PaddleHit {
                side,
                offset,
                incoming_speed: rules::MAX_BALL_SPEED,
            });

            // Then angle and speed stay capped, with the offset controlling vertical direction.
            let outgoing = fixture.physics.ball.velocity;
            let angle = outgoing.y.abs().atan2(outgoing.x.abs()).to_degrees();
            assert_close(outgoing.speed(), rules::MAX_BALL_SPEED);
            assert_close(angle, EXPECTED_MAX_ANGLE_DEGREES);
            assert!(outgoing.x * side.sign() < 0.0, "{side:?}: {outgoing:?}");
            assert_eq!(outgoing.y.is_sign_negative(), offset.is_sign_negative());
        }
    }
}

#[test]
fn center_and_near_center_hits_cannot_create_flat_rallies() {
    const INCOMING_VERTICAL_SPEED: f32 = 100.0;
    const NEAR_CENTER_OFFSET: f32 = 4.0;
    // sqrt(400^2 + 100^2), increased by 7%, rounded to f32 precision.
    const EXPECTED_DIAGONAL_REBOUND_SPEED: f32 = 441.1723;
    for side in [Side::Left, Side::Right] {
        // Explicit cases specify direction; the oracle does not repeat the production branch.
        for (incoming_y, offset, expected_upward) in [
            (-INCOMING_VERTICAL_SPEED, -NEAR_CENTER_OFFSET, false),
            (-INCOMING_VERTICAL_SPEED, CENTER_HIT, false),
            (-INCOMING_VERTICAL_SPEED, NEAR_CENTER_OFFSET, true),
            (0.0, -NEAR_CENTER_OFFSET, false),
            (0.0, CENTER_HIT, true),
            (0.0, NEAR_CENTER_OFFSET, true),
            (INCOMING_VERTICAL_SPEED, -NEAR_CENTER_OFFSET, false),
            (INCOMING_VERTICAL_SPEED, CENTER_HIT, true),
            (INCOMING_VERTICAL_SPEED, NEAR_CENTER_OFFSET, true),
        ] {
            // Given the incoming direction and near-center contact from this case.
            let mut fixture = MatchFixture::playing(Mode::Solo);
            fixture.physics.ball.velocity = Velocity {
                x: side.sign() * INCOMING_HORIZONTAL_SPEED,
                y: incoming_y,
            };
            let incoming_speed = fixture.physics.ball.velocity.speed();

            // When the paddle is hit.
            fixture.hit_paddle(PaddleHit {
                side,
                offset,
                incoming_speed,
            });

            // Then the rebound has a playable angle and the expected vertical direction.
            let outgoing = fixture.physics.ball.velocity;
            let angle = outgoing.y.abs().atan2(outgoing.x.abs()).to_degrees();
            let scenario = format!(
                "side={side:?}, incoming_y={incoming_y}, offset={offset}, outgoing={outgoing:?}"
            );
            assert!(
                angle >= EXPECTED_MIN_ANGLE_DEGREES - FLOAT_TOLERANCE,
                "{scenario}"
            );
            assert!(outgoing.x * side.sign() < 0.0, "{scenario}");
            let expected_speed = if incoming_y == 0.0 {
                EXPECTED_ACCELERATED_SPEED
            } else {
                EXPECTED_DIAGONAL_REBOUND_SPEED
            };
            assert_close(outgoing.speed(), expected_speed);
            assert_eq!(outgoing.y > 0.0, expected_upward, "{scenario}");
        }
    }
}
