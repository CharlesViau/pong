use super::HudText;
use bevy::prelude::*;
use pong_core::{Match, Mode, Phase, Side};

#[derive(PartialEq)]
pub(in crate::game) struct HudSnapshot {
    mode: Mode,
    phase: Phase,
    scores: [u8; 2],
}

impl HudSnapshot {
    pub(super) fn from_match(game: &Match) -> Self {
        Self {
            mode: game.mode(),
            phase: game.phase(),
            scores: [game.score(Side::Left), game.score(Side::Right)],
        }
    }

    pub(super) fn overlay_visibility(&self) -> Visibility {
        match self.phase {
            Phase::Ready | Phase::Paused | Phase::Finished(_) => Visibility::Visible,
            Phase::Serving | Phase::Playing => Visibility::Hidden,
        }
    }

    pub(super) fn text_for(&self, label: &HudText) -> String {
        let solo = self.mode == Mode::Solo;
        match label {
            HudText::LeftScore => self.scores[0].to_string(),
            HudText::RightScore => self.scores[1].to_string(),
            HudText::LeftLabel => if solo { "YOU" } else { "LEFT PLAYER" }.into(),
            HudText::RightLabel => if solo { "COMPUTER" } else { "RIGHT PLAYER" }.into(),
            HudText::Mode => self.movement_instructions().into(),
            HudText::Status => self.status_message().into(),
            HudText::OverlayTitle => self.overlay_message().0.into(),
            HudText::OverlayHint => self.overlay_message().1.into(),
        }
    }

    fn movement_instructions(&self) -> &'static str {
        match self.mode {
            Mode::Solo => "SOLO   /   Move with W S or arrow keys",
            Mode::Local => "TWO PLAYERS   /   Left: W S     Right: arrow keys",
        }
    }

    fn status_message(&self) -> &'static str {
        match self.phase {
            Phase::Ready => "Choose your mode with TAB",
            Phase::Serving => "GET READY",
            Phase::Playing => "Hit near the paddle's edges to change the angle",
            Phase::Paused => "Match paused",
            Phase::Finished(_) => "MATCH COMPLETE",
        }
    }

    fn overlay_message(&self) -> (&'static str, &'static str) {
        match (self.phase, self.mode) {
            (Phase::Ready, _) => ("READY?", "Press SPACE to serve"),
            (Phase::Paused, _) => ("PAUSED", "Press SPACE to resume"),
            (Phase::Finished(Side::Left), Mode::Solo) => ("YOU WIN", "Press SPACE to play again"),
            (Phase::Finished(Side::Right), Mode::Solo) => {
                ("COMPUTER WINS", "Press SPACE for a rematch")
            }
            (Phase::Finished(Side::Left), Mode::Local) => {
                ("LEFT PLAYER WINS", "Press SPACE for a rematch")
            }
            (Phase::Finished(Side::Right), Mode::Local) => {
                ("RIGHT PLAYER WINS", "Press SPACE for a rematch")
            }
            (Phase::Serving | Phase::Playing, _) => ("", ""),
        }
    }
}
