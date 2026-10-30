use pong_core::{Controls, Direction, Mode, Side, rules};

use crate::support::{MatchFixture, PADDLE_CENTER_LIMIT, assert_close};

const OPPOSITE_INPUTS: Controls = Controls {
    left: Direction::Up,
    right: Direction::Down,
};

#[test]
fn local_players_request_independent_velocities() {
    // Given two local players at the court center.
    let mut fixture = MatchFixture::playing(Mode::Local);

    // When they hold opposite directions.
    fixture.prepare_tick(OPPOSITE_INPUTS);

    // Then each paddle gets its own requested velocity.
    assert_eq!(
        fixture.physics.paddle_velocity(Side::Left),
        rules::PLAYER_SPEED
    );
    assert_eq!(
        fixture.physics.paddle_velocity(Side::Right),
        -rules::PLAYER_SPEED
    );
}

#[test]
fn movement_is_limited_to_the_remaining_distance_to_each_wall() {
    const DISTANCE_TO_WALL: f32 = 1.0;
    // Given paddles closer to the walls than a full-speed step can travel.
    let mut fixture = MatchFixture::playing(Mode::Local);
    fixture.physics.paddle_y = [
        PADDLE_CENTER_LIMIT - DISTANCE_TO_WALL,
        -PADDLE_CENTER_LIMIT + DISTANCE_TO_WALL,
    ];

    // When both players move toward their nearest wall.
    fixture.prepare_tick(OPPOSITE_INPUTS);

    // Then the requested displacement stops exactly at the boundary.
    assert_close(
        fixture.physics.paddle_velocity(Side::Left) * rules::TICK_SECONDS,
        DISTANCE_TO_WALL,
    );
    assert_close(
        fixture.physics.paddle_velocity(Side::Right) * rules::TICK_SECONDS,
        -DISTANCE_TO_WALL,
    );
}

#[test]
fn paddles_at_the_wall_cannot_move_further_outward() {
    // Given paddles already at opposite walls.
    let mut fixture = MatchFixture::playing(Mode::Local);
    fixture.physics.paddle_y = [PADDLE_CENTER_LIMIT, -PADDLE_CENTER_LIMIT];

    // When both players try to move beyond the court.
    fixture.prepare_tick(OPPOSITE_INPUTS);

    // Then both requested velocities are zero.
    assert_eq!(fixture.physics.paddle_velocity(Side::Left), 0.0);
    assert_eq!(fixture.physics.paddle_velocity(Side::Right), 0.0);
}

#[test]
fn the_computer_tracks_an_approaching_ball_and_ignores_right_player_input() {
    const BALL_ABOVE_PADDLE_Y: f32 = 100.0;
    // Given an approaching ball above the computer's paddle.
    let mut fixture = MatchFixture::playing(Mode::Solo);
    fixture.physics.ball.position.y = BALL_ABOVE_PADDLE_Y;

    // When the human requests the opposite direction for the right paddle.
    fixture.prepare_tick(Controls {
        right: Direction::Down,
        ..Default::default()
    });

    // Then the computer tracks upward at its own speed.
    assert_eq!(
        fixture.physics.paddle_velocity(Side::Right),
        rules::COMPUTER_SPEED
    );
}

#[test]
fn the_computer_recenters_a_receding_ball_but_stops_inside_its_dead_zone() {
    const RECEDING_HORIZONTAL_SPEED: f32 = -100.0;
    const DISPLACED_PADDLE_Y: f32 = 100.0;
    const WITHIN_DEAD_ZONE_Y: f32 = 5.0;
    for (paddle_y, expected_speed) in [
        (DISPLACED_PADDLE_Y, -rules::COMPUTER_SPEED),
        (WITHIN_DEAD_ZONE_Y, 0.0),
    ] {
        // Given a receding ball and a computer paddle at the scenario position.
        let mut fixture = MatchFixture::playing(Mode::Solo);
        fixture.physics.ball.velocity.x = RECEDING_HORIZONTAL_SPEED;
        fixture.physics.paddle_y[1] = paddle_y;

        // When the computer chooses its next movement.
        fixture.prepare_tick(Controls::default());

        // Then it recenters only while outside the dead zone.
        assert_eq!(
            fixture.physics.paddle_velocity(Side::Right),
            expected_speed,
            "paddle y={paddle_y}"
        );
    }
}
