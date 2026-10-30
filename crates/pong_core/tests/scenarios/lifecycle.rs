use pong_core::{Action, BallMotion, Mode, PaddleHit, Phase, Position, Side, Velocity, rules};

use crate::support::{INACTIVE_TICKS, MatchFixture, PhysicsCommand, assert_close};

#[test]
fn restarting_from_every_phase_resets_once_and_forgets_the_previous_state() {
    // Given a match in each lifecycle phase, including a paused countdown.
    let mut paused = MatchFixture::serving(Mode::Local);
    paused.advance(rules::SERVE_TICKS / 2);
    paused.act(Action::Pause);
    let mut finished = MatchFixture::playing(Mode::Local);
    finished.finish_match(Side::Right);

    for mut fixture in [
        MatchFixture::default(),
        MatchFixture::serving(Mode::Local),
        MatchFixture::playing(Mode::Local),
        paused,
        finished,
    ] {
        let previous_phase = fixture.game.phase();
        let selected_mode = fixture.game.mode();
        fixture.physics.clear_commands();

        // When restarting through the application driving port.
        fixture.act(Action::Restart);

        // Then every state returns to ready with one court reset and no retained score.
        assert_eq!(
            fixture.game.phase(),
            Phase::Ready,
            "from {previous_phase:?}"
        );
        assert_eq!(fixture.game.mode(), selected_mode);
        assert_eq!(fixture.game.score(Side::Left), 0);
        assert_eq!(fixture.game.score(Side::Right), 0);
        assert_eq!(fixture.game.serve_ticks_remaining(), 0);
        fixture
            .physics
            .assert_commands(&[PhysicsCommand::ResetCourt]);

        // When starting again, then a full serve replaces any interrupted state.
        fixture.act(Action::Primary);
        assert_eq!(fixture.game.phase(), Phase::Serving);
        assert_eq!(fixture.game.serve_ticks_remaining(), rules::SERVE_TICKS);
    }
}

#[test]
fn ready_ignores_pause_and_elapsed_ticks_without_touching_physics() {
    // Given a match that has not started.
    let mut fixture = MatchFixture::default();

    // When pause is requested and enough time for two serves elapses.
    fixture.act(Action::Pause);
    fixture.advance(INACTIVE_TICKS);

    // Then it remains ready and issues no physics commands.
    assert_eq!(fixture.game.phase(), Phase::Ready);
    assert_eq!(fixture.game.score(Side::Left), 0);
    assert_eq!(fixture.physics.ball, BallMotion::default());
    fixture.physics.assert_commands(&[]);
}

#[test]
fn the_ball_stays_still_until_the_last_serve_tick() {
    // Given a fresh serve countdown.
    let mut fixture = MatchFixture::serving(Mode::Solo);

    // When all but the final countdown tick elapse.
    fixture.advance(rules::SERVE_TICKS - 1);

    // Then the ball has not launched yet.
    assert_eq!(fixture.game.phase(), Phase::Serving);
    assert_eq!(fixture.game.serve_ticks_remaining(), 1);
    assert_eq!(fixture.physics.ball.velocity, Velocity::default());
}

#[test]
fn the_last_serve_tick_launches_the_ball_once_without_moving_it() {
    // Given a serve with exactly one tick remaining.
    let mut fixture = MatchFixture::serving(Mode::Solo);
    fixture.advance(rules::SERVE_TICKS - 1);
    fixture.physics.clear_commands();

    // When the final tick completes, followed by another tick without a goal.
    fixture.finish_tick();
    fixture.finish_tick();

    // Then the core requests one launch; physical integration belongs to Avian.
    let launched_velocity = fixture.physics.ball.velocity;
    assert_eq!(fixture.game.phase(), Phase::Playing);
    assert!(launched_velocity.x > 0.0);
    assert_close(launched_velocity.speed(), rules::SERVE_SPEED);
    assert_eq!(fixture.physics.ball.position, Position::default());
    fixture
        .physics
        .assert_commands(&[PhysicsCommand::SetBallVelocity(launched_velocity)]);
}

#[test]
fn repeated_pause_keeps_the_last_countdown_tick_until_resume() {
    const REPEATED_PAUSE_REQUESTS: usize = 3;
    // Given one tick left in the countdown.
    let mut fixture = MatchFixture::serving(Mode::Solo);
    fixture.advance(rules::SERVE_TICKS - 1);
    fixture.physics.clear_commands();

    // When pause is requested repeatedly while time passes.
    for _ in 0..REPEATED_PAUSE_REQUESTS {
        fixture.act(Action::Pause);
        fixture.advance(INACTIVE_TICKS);
    }

    // Then the countdown and physics are untouched.
    assert_eq!(fixture.game.phase(), Phase::Paused);
    assert_eq!(fixture.game.serve_ticks_remaining(), 1);
    assert_eq!(fixture.physics.ball.velocity, Velocity::default());
    fixture.physics.assert_commands(&[]);

    // When resumed for the remaining tick, then the rally starts.
    fixture.act(Action::Primary);
    fixture.advance(1);
    assert_eq!(fixture.game.phase(), Phase::Playing);
}

#[test]
fn paused_play_ignores_contacts_and_preserves_motion_for_resume() {
    const OFF_CENTER_CONTACT: f32 = 20.0;
    const INCOMING_SPEED: f32 = 400.0;
    // Given a moving ball in a paused rally.
    let mut fixture = MatchFixture::playing(Mode::Solo);
    let motion_before_pause = fixture.physics.ball;
    fixture.act(Action::Primary);

    // When a contact arrives and ticks elapse while paused.
    fixture.hit_paddle(PaddleHit {
        side: Side::Right,
        offset: OFF_CENTER_CONTACT,
        incoming_speed: INCOMING_SPEED,
    });
    fixture.advance(INACTIVE_TICKS);

    // Then no physics command is issued and the same motion resumes.
    assert_eq!(fixture.game.phase(), Phase::Paused);
    assert_eq!(fixture.physics.ball, motion_before_pause);
    fixture.physics.assert_commands(&[]);
    fixture.act(Action::Primary);
    assert_eq!(fixture.game.phase(), Phase::Playing);
    assert_eq!(fixture.physics.ball, motion_before_pause);
}

#[test]
fn restart_clears_a_paused_match_but_keeps_its_mode() {
    const DISPLACED_PADDLE_Y: f32 = 100.0;
    // Given a paused local match with a point and displaced paddles.
    let mut fixture = MatchFixture::playing(Mode::Local);
    fixture.score_point(Side::Left);
    fixture.physics.paddle_y = [DISPLACED_PADDLE_Y, -DISPLACED_PADDLE_Y];
    fixture.act(Action::Pause);
    fixture.physics.clear_commands();

    // When restarted.
    fixture.act(Action::Restart);

    // Then the court resets exactly once and scores clear, keeping local mode.
    assert_eq!(fixture.game.phase(), Phase::Ready);
    assert_eq!(fixture.game.mode(), Mode::Local);
    assert_eq!(fixture.game.score(Side::Left), 0);
    assert_eq!(fixture.physics.paddle_y, [0.0; 2]);
    assert_eq!(fixture.physics.ball, BallMotion::default());
    fixture
        .physics
        .assert_commands(&[PhysicsCommand::ResetCourt]);

    // When started again, then no paused countdown is restored.
    fixture.act(Action::Primary);
    assert_eq!(fixture.game.serve_ticks_remaining(), rules::SERVE_TICKS);
}

#[test]
fn switching_mode_while_paused_starts_a_fresh_match() {
    // Given a solo match paused after scoring.
    let mut fixture = MatchFixture::playing(Mode::Solo);
    fixture.score_point(Side::Right);
    fixture.act(Action::Pause);
    fixture.physics.clear_commands();

    // When switching to local play.
    fixture.act(Action::SwitchMode);

    // Then the score and court reset once, ready for a full countdown.
    assert_eq!(fixture.game.phase(), Phase::Ready);
    assert_eq!(fixture.game.mode(), Mode::Local);
    assert_eq!(fixture.game.score(Side::Right), 0);
    fixture
        .physics
        .assert_commands(&[PhysicsCommand::ResetCourt]);
    fixture.act(Action::Primary);
    assert_eq!(fixture.game.serve_ticks_remaining(), rules::SERVE_TICKS);
}
