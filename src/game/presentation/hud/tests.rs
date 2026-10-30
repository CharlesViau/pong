use bevy::{ecs::system::SystemState, prelude::*};
use pong_core::Match;

use super::{Hud, HudText, Overlay};

#[test]
fn the_hud_displays_a_core_match_without_infrastructure_resources() {
    const READY_TITLE: &str = "READY?";
    const INITIAL_SCORE_TEXT: &str = "0";

    // Given an empty view and a ready core match, with no physics or match resource in the world.
    let game = Match::default();
    let mut world = World::new();
    let title = world.spawn((HudText::OverlayTitle, Text2d::new(""))).id();
    let score = world.spawn((HudText::LeftScore, Text2d::new(""))).id();
    let overlay = world.spawn((Overlay, Visibility::Hidden)).id();
    let mut view = SystemState::<Hud>::new(&mut world);

    // When the caller supplies the core match to the presenter.
    view.get_mut(&mut world)
        .expect("the HUD only requires its view components")
        .update(&game);

    // Then the ready screen and initial score are visible.
    assert_eq!(world.get::<Text2d>(title).unwrap().0, READY_TITLE);
    assert_eq!(world.get::<Text2d>(score).unwrap().0, INITIAL_SCORE_TEXT);
    assert_eq!(world.get::<Visibility>(overlay), Some(&Visibility::Visible));
}
