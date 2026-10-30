use pong_core::{Action, BallMotion, Mode, Phase, Side, rules};

use crate::support::{
    BEYOND_GOAL_MARGIN, GOAL_CENTER_LIMIT, INACTIVE_TICKS, MatchFixture, PhysicsCommand,
};

#[test]
fn a_ball_still_overlapping_the_court_does_not_score() {
    for conceding_side in [Side::Left, Side::Right] {
        for distance_from_center in [
            rules::COURT_WIDTH / 2.0,
            GOAL_CENTER_LIMIT - BEYOND_GOAL_MARGIN,
        ] {
            // Given a ball that has not fully departed the court.
            let mut fixture = MatchFixture::playing(Mode::Solo);
            fixture.physics.ball.position.x = conceding_side.sign() * distance_from_center;

            // When the match observes the completed physics step.
            fixture.finish_tick();

            // Then neither player scores and the ball is not reset.
            assert_eq!(fixture.game.score(Side::Left), 0);
            assert_eq!(fixture.game.score(Side::Right), 0);
            assert_eq!(fixture.game.phase(), Phase::Playing);
            fixture.physics.assert_commands(&[]);
        }
    }
}

#[test]
fn a_fully_departed_ball_awards_one_point_and_resets_only_the_ball_once() {
    for scorer in [Side::Left, Side::Right] {
        for distance_from_center in [GOAL_CENTER_LIMIT, GOAL_CENTER_LIMIT + BEYOND_GOAL_MARGIN] {
            // Given a ball whose trailing edge reaches or passes the opponent's goal line.
            let mut fixture = MatchFixture::playing(Mode::Solo);
            fixture.physics.ball.position.x = scorer.opponent().sign() * distance_from_center;

            // When the scoring step completes.
            fixture.finish_tick();

            // Then the point starts a full countdown and resets the ball once.
            assert_eq!(fixture.game.score(scorer), 1);
            assert_eq!(fixture.game.score(scorer.opponent()), 0);
            assert_eq!(fixture.game.phase(), Phase::Serving);
            assert_eq!(fixture.game.serve_ticks_remaining(), rules::SERVE_TICKS);
            assert_eq!(fixture.physics.ball, BallMotion::default());
            fixture
                .physics
                .assert_commands(&[PhysicsCommand::ResetBall]);

            // When another observation arrives, then the point is not repeated.
            fixture.finish_tick();
            assert_eq!(fixture.game.score(scorer), 1);
            fixture
                .physics
                .assert_commands(&[PhysicsCommand::ResetBall]);
        }
    }
}

#[test]
fn the_next_serve_targets_the_conceding_player_and_alternates_vertical_direction() {
    // Given the initial upward serve.
    let mut fixture = MatchFixture::playing(Mode::Solo);
    assert!(fixture.physics.ball.velocity.y > 0.0);

    // When the right player scores and the next countdown completes.
    fixture.score_point(Side::Right);
    fixture.advance(rules::SERVE_TICKS);

    // Then the serve goes left toward the conceding player, this time downward.
    assert!(fixture.physics.ball.velocity.x < 0.0);
    assert!(fixture.physics.ball.velocity.y < 0.0);
}

#[test]
fn winning_freezes_the_result_despite_further_goals_ticks_or_pause() {
    // Given a finished match.
    let mut fixture = MatchFixture::playing(Mode::Solo);
    fixture.finish_match(Side::Left);
    fixture.physics.clear_commands();

    // When more goal observations and a pause request arrive after the win.
    fixture.physics.ball.position.x = GOAL_CENTER_LIMIT + BEYOND_GOAL_MARGIN;
    fixture.act(Action::Pause);
    fixture.advance(INACTIVE_TICKS);

    // Then the winner and winning score remain fixed with no physics commands.
    assert_eq!(fixture.game.phase(), Phase::Finished(Side::Left));
    assert_eq!(fixture.game.score(Side::Left), rules::WINNING_SCORE);
    fixture.physics.assert_commands(&[]);
}

#[test]
fn primary_after_a_win_clears_scores_and_starts_a_rematch_in_the_same_mode() {
    // Given a completed local match.
    let mut fixture = MatchFixture::playing(Mode::Local);
    fixture.finish_match(Side::Right);
    fixture.physics.clear_commands();

    // When a rematch is requested.
    fixture.act(Action::Primary);

    // Then the court resets once and a fresh countdown starts in local mode.
    assert_eq!(fixture.game.mode(), Mode::Local);
    assert_eq!(fixture.game.phase(), Phase::Serving);
    assert_eq!(fixture.game.serve_ticks_remaining(), rules::SERVE_TICKS);
    assert_eq!(fixture.game.score(Side::Left), 0);
    assert_eq!(fixture.game.score(Side::Right), 0);
    assert_eq!(fixture.physics.ball, BallMotion::default());
    fixture
        .physics
        .assert_commands(&[PhysicsCommand::ResetCourt]);

    // When that countdown finishes, then the opening serve points right again.
    fixture.advance(rules::SERVE_TICKS);
    assert!(fixture.physics.ball.velocity.x > 0.0);
}
