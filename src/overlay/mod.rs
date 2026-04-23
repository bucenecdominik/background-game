//! Overlay/window behavior module.

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowLevel, WindowPlugin};

const OVERLAY_WIDTH: f32 = 480.0;
const OVERLAY_HEIGHT: f32 = 320.0;

pub fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Background Overlay".into(),
            resolution: (OVERLAY_WIDTH, OVERLAY_HEIGHT).into(),
            transparent: true,
            decorations: false,
            window_level: WindowLevel::AlwaysOnTop,
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }
}

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::NONE))
            .add_systems(Startup, apply_platform_overlay_tweaks);
    }
}

#[cfg(target_os = "windows")]
fn apply_platform_overlay_tweaks() {
    // Keep Windows-specific behavior isolated in this module.
    // Bevy-native settings currently provide the required overlay behavior.
    info!("Windows overlay tweaks: none required yet.");
}

#[cfg(not(target_os = "windows"))]
fn apply_platform_overlay_tweaks() {}
