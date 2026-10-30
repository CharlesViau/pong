use bevy::prelude::*;

#[derive(Component)]
pub(in crate::game) enum HudText {
    LeftScore,
    RightScore,
    LeftLabel,
    RightLabel,
    Mode,
    Status,
    OverlayTitle,
    OverlayHint,
}
