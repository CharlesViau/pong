//! Opt-in desktop evidence; regular tests do not need a GPU.
mod visible_piece;

use std::time::SystemTime;
use visible_piece::VisiblePiece;

use bevy::{
    app::AppExit,
    image::{CompressedImageFormats, ImageType},
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
    winit::WinitSettings,
};
use pong_core::Action;

use crate::game::presentation::KeyboardControls;

// Leave rendering warm-up and screenshot readback time between scripted actions.
const READY_CAPTURE_FRAME: u32 = 45;
const START_FRAME: u32 = 65;
const PLAYING_CAPTURE_FRAME: u32 = 150;
const PAUSE_FRAME: u32 = 170;
const PAUSED_CAPTURE_FRAME: u32 = 190;
const EXIT_FRAME: u32 = 220;
const READY_CAPTURE: &str = "target/pong-ready.png";
const PLAYING_CAPTURE: &str = "target/pong-playing.png";
const PAUSED_CAPTURE: &str = "target/pong-paused.png";
const MIN_SCREENSHOT_BYTES: u64 = 1_000;
const COURT_TOP_PERCENT: u32 = 16;
const COURT_BOTTOM_PERCENT: u32 = 84;
const HORIZONTAL_MARGIN_FRACTION: u32 = 15;

// Broad sRGB masks tolerate antialiasing; thresholds reject isolated stray pixels.
const VISIBLE_PIECES: [VisiblePiece; 3] = [
    VisiblePiece {
        name: "left paddle",
        rgb: [0.0..=0.5, 0.7..=1.0, 0.6..=1.0],
        minimum_pixels: 200,
    },
    VisiblePiece {
        name: "right paddle",
        rgb: [0.8..=1.0, 0.4..=0.8, 0.0..=0.6],
        minimum_pixels: 200,
    },
    VisiblePiece {
        name: "ball",
        rgb: [0.8..=1.0, 0.8..=1.0, 0.8..=1.0],
        minimum_pixels: 100,
    },
];

/// cargo test -p pong render_smoke -- --ignored --nocapture
#[test]
#[ignore = "requires a desktop, GPU, and a writable target directory"]
fn render_smoke() {
    // Given the actual desktop app and a fresh capture time.
    let started = SystemTime::now();
    let mut app = crate::desktop_app::DesktopApp::build();
    app.insert_resource(WinitSettings::continuous())
        .add_systems(Update, smoke_steps);

    // When the scripted start/pause sequence runs and captures three screens.
    let result = app.run();

    // Then the app exits cleanly, all captures are fresh, and game pieces are visible.
    assert_eq!(result, AppExit::Success);
    for path in [READY_CAPTURE, PLAYING_CAPTURE, PAUSED_CAPTURE] {
        let metadata = std::fs::metadata(path).unwrap();
        assert!(
            metadata.len() > MIN_SCREENSHOT_BYTES,
            "empty capture: {path}"
        );
        assert!(
            metadata.modified().unwrap() >= started,
            "stale capture: {path}"
        );
    }
    assert_visible_game_pieces();
}

fn smoke_steps(
    mut frame: Local<u32>,
    mut commands: Commands,
    mut controls: ResMut<KeyboardControls>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    match *frame {
        READY_CAPTURE_FRAME => capture(&mut commands, READY_CAPTURE),
        START_FRAME => controls.actions.push_back(Action::Primary),
        PLAYING_CAPTURE_FRAME => capture(&mut commands, PLAYING_CAPTURE),
        PAUSE_FRAME => controls.actions.push_back(Action::Pause),
        PAUSED_CAPTURE_FRAME => capture(&mut commands, PAUSED_CAPTURE),
        EXIT_FRAME => {
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}

fn capture(commands: &mut Commands, path: &'static str) {
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path));
}

fn assert_visible_game_pieces() {
    let bytes = std::fs::read(PLAYING_CAPTURE).unwrap();
    let image = Image::from_buffer(
        &bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        true,
        default(),
        default(),
    )
    .unwrap();
    let mut pixel_counts = [0; VISIBLE_PIECES.len()];
    // Inspect court interior only: scoreboard and footer contain similar colors.
    let top = image.height() * COURT_TOP_PERCENT / 100;
    let bottom = image.height() * COURT_BOTTOM_PERCENT / 100;
    let margin = image.width() / HORIZONTAL_MARGIN_FRACTION;
    for y in top..bottom {
        for x in margin..image.width() - margin {
            let color = image.get_color_at(x, y).unwrap().to_srgba();
            for (piece, count) in VISIBLE_PIECES.iter().zip(&mut pixel_counts) {
                if piece
                    .rgb
                    .iter()
                    .zip([color.red, color.green, color.blue])
                    .all(|(range, channel)| range.contains(&channel))
                {
                    *count += 1;
                }
            }
        }
    }
    for (piece, count) in VISIBLE_PIECES.iter().zip(pixel_counts) {
        assert!(
            count > piece.minimum_pixels,
            "{} is not visible: {count} pixels, expected more than {}",
            piece.name,
            piece.minimum_pixels
        );
    }
}
