use avian2d::prelude::{Collider, Position, Rotation, SimpleCollider};
use bevy::prelude::*;
use pong_core::{Action, Controls, Direction, Phase, Side, rules};

use super::PreviousPosition;
use crate::game::{infrastructure::bevy::CourtEntities, tests::support::*};

const PADDLE_APPROACH_X: f32 = 400.0;
const WALL_APPROACH_Y: f32 = 250.0;
const CONTACT_SPEED: f32 = 600.0;
const WALL_TANGENTIAL_SPEED: f32 = 100.0;
const OFF_CENTER_OFFSET: f32 = 36.0;
const EXPECTED_REBOUND_SPEED: f32 = 642.0;
const EXPECTED_MIN_BOUNCE_ANGLE: f32 = 12.0;
const BALL_WALL_LIMIT: f32 = rules::COURT_HEIGHT / 2.0 - rules::BALL_RADIUS;
const PADDLE_CENTER_LIMIT: f32 = (rules::COURT_HEIGHT - rules::PADDLE_HEIGHT) / 2.0;
const PADDLE_CONTACT_X: f32 = rules::PADDLE_X - rules::PADDLE_WIDTH / 2.0 - rules::BALL_RADIUS;
const CONTACT_SETTLE_TICKS: usize = 15;
const SPEED_TOLERANCE: f32 = 1.0;
const CONTACT_POSITION_TOLERANCE: f32 = 1.0;
const PADDLE_POSITION_TOLERANCE: f32 = 0.01;
const ANGLE_TOLERANCE_DEGREES: f32 = 0.1;

#[test]
fn collisions_still_work_when_the_complete_frame_schedule_runs() {
    const STARTUP_AND_SERVE_FRAMES: usize = FRAME_HZ as usize;
    const CONTACT_SETTLE_FRAMES: usize = 8;
    // Given an active local game driven by whole frames at a controlled 60 Hz.
    let mut app = frame_app();
    queue_action(&mut app, Action::SwitchMode);
    queue_action(&mut app, Action::Primary);
    advance_frames(&mut app, STARTUP_AND_SERVE_FRAMES);
    assert_eq!(game(&app).phase(), Phase::Playing);
    place_ball(
        &mut app,
        Vec2::new(PADDLE_APPROACH_X, OFF_CENTER_OFFSET),
        Vec2::new(CONTACT_SPEED, 0.0),
    );

    // When complete frames run through the paddle contact, including transform propagation.
    advance_frames(&mut app, CONTACT_SETTLE_FRAMES);

    // Then the paddle redirects the ball upward and left.
    let (position, velocity) = ball(&app);
    assert!(
        velocity.x < 0.0 && velocity.y > 0.0,
        "paddle: {position:?} {velocity:?}"
    );

    // When the same app then processes a wall contact, it also reflects inward.
    place_ball(
        &mut app,
        Vec2::new(0.0, WALL_APPROACH_Y),
        Vec2::new(WALL_TANGENTIAL_SPEED, CONTACT_SPEED),
    );
    advance_frames(&mut app, CONTACT_SETTLE_FRAMES);
    let (position, velocity) = ball(&app);
    assert!(
        position.y < BALL_WALL_LIMIT + CONTACT_POSITION_TOLERANCE && velocity.y < 0.0,
        "wall: {position:?} {velocity:?}"
    );
}

#[test]
fn collider_dimensions_match_the_court_and_game_pieces() {
    // Given the normal headless game setup.
    let mut app = fixed_app();
    let entities = app.world().resource::<CourtEntities>();
    let expected_dimensions = [
        (entities.ball, Vec2::splat(rules::BALL_RADIUS * 2.0)),
        (
            entities.paddles[0],
            Vec2::new(rules::PADDLE_WIDTH, rules::PADDLE_HEIGHT),
        ),
        (
            entities.paddles[1],
            Vec2::new(rules::PADDLE_WIDTH, rules::PADDLE_HEIGHT),
        ),
    ];

    // When Avian's initialized collider bounds and scales are inspected.
    for (entity, expected) in expected_dimensions {
        let collider = app.world().get::<Collider>(entity).unwrap();
        let bounds = collider.aabb(Vec2::ZERO, Rotation::default(), 0.0);

        // Then physical dimensions match the game pieces without render scaling.
        assert_eq!(bounds.max - bounds.min, expected, "collider for {entity:?}");
    }
    let mut colliders = app.world_mut().query::<&Collider>();
    for collider in colliders.iter(app.world()) {
        assert_eq!(collider.scale(), Vec2::ONE);
    }
}

#[test]
fn avian_reflects_both_walls_without_losing_speed() {
    for wall_direction in [-1.0, 1.0] {
        // Given a ball approaching either horizontal wall.
        let mut app = playing_app();
        let incoming = Vec2::new(WALL_TANGENTIAL_SPEED, wall_direction * CONTACT_SPEED);
        place_ball(
            &mut app,
            Vec2::new(0.0, wall_direction * WALL_APPROACH_Y),
            incoming,
        );

        // When real physics resolves the contact.
        step(&mut app, CONTACT_SETTLE_TICKS);

        // Then the ball is inside the court, moving inward at its original speed.
        let (position, velocity) = ball(&app);
        assert!(
            position.y.abs() < BALL_WALL_LIMIT + CONTACT_POSITION_TOLERANCE,
            "{position:?}"
        );
        assert!(velocity.y * wall_direction < 0.0, "{velocity:?}");
        assert_near(velocity.length(), incoming.length(), SPEED_TOLERANCE);
    }
}

#[test]
fn both_paddles_bounce_the_ball_and_apply_one_speed_increase() {
    for side in [Side::Left, Side::Right] {
        // Given a ball heading horizontally toward the center of either paddle.
        let mut app = playing_app();
        place_ball(
            &mut app,
            Vec2::new(side.sign() * PADDLE_APPROACH_X, 0.0),
            Vec2::new(side.sign() * CONTACT_SPEED, 0.0),
        );

        // When Avian reports and resolves the paddle contact.
        step(&mut app, CONTACT_SETTLE_TICKS);

        // Then the core's single speed increase and minimum angle survive the solver.
        let (position, velocity) = ball(&app);
        assert!(velocity.x * side.sign() < 0.0, "{side:?}: {velocity:?}");
        assert!(position.x.abs() < PADDLE_CONTACT_X, "{position:?}");
        assert_near(velocity.length(), EXPECTED_REBOUND_SPEED, SPEED_TOLERANCE);
        let angle = velocity.y.abs().atan2(velocity.x.abs()).to_degrees();
        assert!(
            angle >= EXPECTED_MIN_BOUNCE_ANGLE - ANGLE_TOLERANCE_DEGREES,
            "center hit flattened the rally: {velocity:?}, angle={angle}"
        );
    }
}

#[test]
fn off_center_contact_changes_angle_and_respects_the_speed_cap() {
    const EXPECTED_ANGLE_DEGREES: f32 = 45.0;
    const OFF_CENTER_ANGLE_TOLERANCE: f32 = 3.0;
    // Given a ball at its speed cap approaching the upper part of the right paddle.
    let mut app = playing_app();
    place_ball(
        &mut app,
        Vec2::new(PADDLE_APPROACH_X, OFF_CENTER_OFFSET),
        Vec2::new(rules::MAX_BALL_SPEED, 0.0),
    );

    // When the contact resolves.
    step(&mut app, CONTACT_SETTLE_TICKS);

    // Then it leaves at roughly 45 degrees without exceeding the speed cap.
    let (_, velocity) = ball(&app);
    assert!(velocity.x < 0.0 && velocity.y > 0.0, "{velocity:?}");
    assert_near(velocity.length(), rules::MAX_BALL_SPEED, SPEED_TOLERANCE);
    let angle = velocity.y.atan2(-velocity.x).to_degrees();
    assert_near(angle, EXPECTED_ANGLE_DEGREES, OFF_CENTER_ANGLE_TOLERANCE);
}

#[test]
fn moving_paddle_contact_keeps_arcade_speed_independent_of_solver_impulses() {
    const MOVING_PADDLE_CONTACT_Y: f32 = 30.0;
    // Given an upward-moving right paddle and an incoming ball.
    let mut app = playing_app();
    set_movement(
        &mut app,
        Controls {
            right: Direction::Up,
            ..default()
        },
    );
    place_ball(
        &mut app,
        Vec2::new(PADDLE_APPROACH_X, MOVING_PADDLE_CONTACT_Y),
        Vec2::new(CONTACT_SPEED, 0.0),
    );

    // When the moving paddle meets the ball.
    step(&mut app, CONTACT_SETTLE_TICKS);

    // Then motion of the paddle does not add solver speed on top of the arcade rule.
    let (_, velocity) = ball(&app);
    assert!(velocity.x < 0.0, "{velocity:?}");
    assert_near(velocity.length(), EXPECTED_REBOUND_SPEED, SPEED_TOLERANCE);
}

#[test]
fn ccd_stops_a_ball_that_could_cross_the_entire_paddle_in_one_step() {
    const TUNNELING_SPEED: f32 = 20_000.0;
    const HIGH_SPEED_START_X: f32 = 350.0;
    // Given a speed that would jump from before to beyond the paddle in one tick.
    let mut app = playing_app();
    const {
        assert!(
            HIGH_SPEED_START_X + TUNNELING_SPEED * rules::TICK_SECONDS
                > rules::PADDLE_X + rules::PADDLE_WIDTH
        );
    }
    place_ball(
        &mut app,
        Vec2::new(HIGH_SPEED_START_X, 0.0),
        Vec2::new(TUNNELING_SPEED, 0.0),
    );

    // When continuous collision detection processes the contact.
    step(&mut app, CONTACT_SETTLE_TICKS);

    // Then the ball rebounds, no goal is awarded, and normal speed is restored.
    let (position, velocity) = ball(&app);
    assert!(velocity.x < 0.0, "{position:?} {velocity:?}");
    assert!(position.x < rules::PADDLE_X);
    assert_eq!(game(&app).score(Side::Left), 0);
    assert_near(velocity.length(), rules::MAX_BALL_SPEED, SPEED_TOLERANCE);
}

#[test]
fn a_miss_scores_once_and_resets_position_velocity_and_interpolation() {
    const MISSED_PADDLE_Y: f32 = 150.0;
    const GOAL_CROSSING_TICKS: usize = 3;
    const EXTRA_SERVING_TICKS: usize = 20;
    // Given a ball beyond the paddle and moving through the right goal.
    let mut app = playing_app();
    place_ball(
        &mut app,
        Vec2::new(rules::COURT_WIDTH / 2.0, MISSED_PADDLE_Y),
        Vec2::new(CONTACT_SPEED, 0.0),
    );

    // When the ball fully crosses the goal line.
    step(&mut app, GOAL_CROSSING_TICKS);

    // Then physics and interpolation reset together and one point is awarded.
    assert_eq!(ball(&app), (Vec2::ZERO, Vec2::ZERO));
    assert_eq!(
        app.world()
            .get::<PreviousPosition>(ball_entity(&app))
            .unwrap()
            .0,
        Vec2::ZERO
    );
    assert_eq!(game(&app).score(Side::Left), 1);

    // When more serving ticks run, then that point is not repeated.
    step(&mut app, EXTRA_SERVING_TICKS);
    assert_eq!(game(&app).score(Side::Left), 1);
}

#[test]
fn pausing_freezes_immediately_preserves_velocities_and_resumes() {
    const MOVEMENT_TICKS: usize = 10;
    const PAUSED_TICKS: usize = rules::SERVE_TICKS as usize * 2;
    // Given a moving ball and left paddle.
    let mut app = playing_app();
    set_movement(
        &mut app,
        Controls {
            left: Direction::Up,
            ..default()
        },
    );
    step(&mut app, MOVEMENT_TICKS);
    let ball_before = ball(&app);
    let paddle_before = paddle_y(&app, Side::Left);

    // When paused through enough ticks for two serves.
    queue_action(&mut app, Action::Pause);
    step(&mut app, PAUSED_TICKS);

    // Then physical positions and stored velocity are frozen immediately.
    assert_eq!(ball(&app), ball_before);
    assert_eq!(paddle_y(&app, Side::Left), paddle_before);

    // When resumed, then movement continues using the preserved velocity.
    queue_action(&mut app, Action::Primary);
    step(&mut app, 1);
    assert_ne!(ball(&app).0, ball_before.0);
    assert_eq!(ball(&app).1, ball_before.1);
}

#[test]
fn paddles_stay_bounded_and_render_changes_do_not_move_physics() {
    const TICKS_TO_REACH_WALL: usize = rules::TICK_HZ as usize;
    const OFF_COURT_RENDER_Y: f32 = -5_000.0;
    // Given players moving toward opposite walls for a full simulated second.
    let mut app = playing_app();
    set_movement(
        &mut app,
        Controls {
            left: Direction::Up,
            right: Direction::Down,
        },
    );

    // When the paddles reach their boundaries.
    step(&mut app, TICKS_TO_REACH_WALL);

    // Then each stops at its allowed center position.
    assert_near(
        paddle_y(&app, Side::Left),
        PADDLE_CENTER_LIMIT,
        PADDLE_POSITION_TOLERANCE,
    );
    assert_near(
        paddle_y(&app, Side::Right),
        -PADDLE_CENTER_LIMIT,
        PADDLE_POSITION_TOLERANCE,
    );

    // When rendering is displaced, then the next tick still uses the physical boundary.
    let left = paddle(&app, Side::Left);
    app.world_mut()
        .get_mut::<Transform>(left)
        .unwrap()
        .translation
        .y = OFF_COURT_RENDER_Y;
    app.world_mut().run_schedule(PostUpdate);
    step(&mut app, 1);
    assert_near(
        paddle_y(&app, Side::Left),
        PADDLE_CENTER_LIMIT,
        PADDLE_POSITION_TOLERANCE,
    );
}

#[test]
fn a_paddle_and_wall_corner_contact_stays_in_the_court() {
    const CORNER_APPROACH_VELOCITY: Vec2 = Vec2::new(700.0, 500.0);
    // Given the right paddle at the top boundary and a ball approaching its outer corner.
    let mut app = playing_app();
    let right = paddle(&app, Side::Right);
    app.world_mut().get_mut::<Position>(right).unwrap().y = PADDLE_CENTER_LIMIT;
    place_ball(
        &mut app,
        Vec2::new(PADDLE_APPROACH_X, WALL_APPROACH_Y),
        CORNER_APPROACH_VELOCITY,
    );

    // When the paddle and wall contacts resolve together.
    step(&mut app, CONTACT_SETTLE_TICKS);

    // Then the ball stays inside the court and below its speed cap.
    let (position, velocity) = ball(&app);
    assert!(velocity.x < 0.0, "{position:?} {velocity:?}");
    assert!(
        position.y.abs() < BALL_WALL_LIMIT + CONTACT_POSITION_TOLERANCE,
        "{position:?} {velocity:?}"
    );
    assert!(
        velocity.length() <= rules::MAX_BALL_SPEED + SPEED_TOLERANCE,
        "{velocity:?}"
    );
}

#[test]
fn an_entire_match_finishes_using_real_physics_and_can_restart() {
    const MAX_SIMULATED_MATCH_SECONDS: f64 = 125.0;
    // Given local players who deliberately move away from the ball.
    let mut app = playing_app();
    let max_ticks = (MAX_SIMULATED_MATCH_SECONDS * rules::TICK_HZ) as usize;

    // When the real simulation runs until a win or a bounded simulation-time limit.
    for _ in 0..max_ticks {
        let away_from_ball = if ball(&app).0.y >= 0.0 {
            Direction::Down
        } else {
            Direction::Up
        };
        set_movement(
            &mut app,
            Controls {
                left: away_from_ball,
                right: away_from_ball,
            },
        );
        step(&mut app, 1);
        if matches!(game(&app).phase(), Phase::Finished(_)) {
            break;
        }
    }

    // Then someone wins at the target score rather than stalling indefinitely.
    let Phase::Finished(winner) = game(&app).phase() else {
        panic!(
            "match did not finish within {MAX_SIMULATED_MATCH_SECONDS}s: {:?}; ball {:?}",
            game(&app),
            ball(&app)
        );
    };
    assert_eq!(game(&app).score(winner), rules::WINNING_SCORE);

    // When restarted, then the same app returns to a clean ready state.
    queue_action(&mut app, Action::Restart);
    step(&mut app, 1);
    assert_eq!(ball(&app), (Vec2::ZERO, Vec2::ZERO));
    assert_eq!(game(&app).phase(), Phase::Ready);
}
