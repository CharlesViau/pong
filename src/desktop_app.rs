use crate::game::PongPlugin;
use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
    winit::WinitPlugin,
};

pub(crate) struct DesktopApp;

impl DesktopApp {
    pub(crate) fn build() -> App {
        let mut app = App::new();
        app.add_plugins(
            DefaultPlugins
                .set(Self::renderer_plugin())
                .set(WinitPlugin {
                    run_on_any_thread: cfg!(test),
                })
                .set(WindowPlugin {
                    primary_window: Some(Self::primary_window()),
                    ..default()
                }),
        )
        .add_plugins(PongPlugin);
        app
    }

    fn renderer_plugin() -> RenderPlugin {
        let mut graphics = WgpuSettings::default();
        // DX12 is the verified backend on this Windows machine. An explicit
        // WGPU_BACKEND environment variable still takes precedence.
        if cfg!(target_os = "windows") && std::env::var_os("WGPU_BACKEND").is_none() {
            graphics.backends = Some(Backends::DX12);
        }
        RenderPlugin {
            render_creation: RenderCreation::Automatic(Box::new(graphics)),
            synchronous_pipeline_compilation: true,
            ..default()
        }
    }

    fn primary_window() -> Window {
        Window {
            title: "PONG | Rust + Bevy".into(),
            resolution: (1100, 780).into(),
            resize_constraints: bevy::window::WindowResizeConstraints {
                min_width: 700.0,
                min_height: 500.0,
                ..default()
            },
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        }
    }
}
