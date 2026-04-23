//! Overlay/window behavior module.

use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowLevel, WindowPlugin, WindowPosition};
use bevy_winit::WinitWindows;

pub fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Background Overlay".into(),
            resolution: (1280.0, 720.0).into(),
            position: WindowPosition::At(IVec2::ZERO),
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
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(
                Startup,
                (
                    fit_overlay_to_primary_monitor,
                    apply_platform_overlay_tweaks,
                )
                    .chain(),
            )
            .add_systems(Update, apply_platform_overlay_tweaks);
    }
}

fn fit_overlay_to_primary_monitor(
    winit_windows: NonSend<WinitWindows>,
    mut primary_window: Query<(Entity, &mut Window), With<PrimaryWindow>>,
) {
    let Ok((entity, mut window)) = primary_window.get_single_mut() else {
        return;
    };

    let Some(winit_window) = winit_windows.get_window(entity) else {
        return;
    };

    let Some(monitor) = winit_window.primary_monitor() else {
        warn!("Could not determine primary monitor for overlay sizing.");
        return;
    };

    let monitor_position = monitor.position();
    let monitor_size = monitor.size();

    winit_window.set_outer_position(monitor_position);
    let _ = winit_window.request_inner_size(monitor_size);

    window
        .position
        .set(IVec2::new(monitor_position.x, monitor_position.y));
    window
        .resolution
        .set_physical_resolution(monitor_size.width, monitor_size.height);
}

#[cfg(target_os = "windows")]
fn apply_platform_overlay_tweaks(
    winit_windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetWindowLongPtrW, SetLayeredWindowAttributes, SetWindowLongPtrW, GWL_EXSTYLE,
        LWA_COLORKEY, WS_EX_LAYERED,
    };

    let Ok(entity) = primary_window.get_single() else {
        return;
    };

    let Some(winit_window) = winit_windows.get_window(entity) else {
        return;
    };

    let Ok(window_handle) = winit_window.window_handle() else {
        warn!("Could not get raw window handle for transparent overlay tweaks.");
        return;
    };

    let RawWindowHandle::Win32(handle) = window_handle.as_raw() else {
        warn!("Transparent overlay tweaks are only available for Win32 windows.");
        return;
    };

    let hwnd = handle.hwnd.get();

    unsafe {
        let extended_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, extended_style | WS_EX_LAYERED as isize);

        // Some Windows/GPU combinations expose only Opaque wgpu surfaces. A black color key
        // keeps the clear background transparent while preserving the non-black UI panel.
        if SetLayeredWindowAttributes(hwnd, 0, 255, LWA_COLORKEY) == 0 {
            warn!("Failed to apply Win32 layered-window color key.");
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn apply_platform_overlay_tweaks() {}
