//! UI module.

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::game::{Health, Player, WorldMapState, MAP_RADIUS};

const TOP_BAR_HEIGHT: f32 = 126.0;
const TOP_BAR_WIDTH: f32 = 456.0;
const TOP_BAR_MARGIN: f32 = 6.0;
const FPS_PANEL_MARGIN: f32 = 12.0;
const MINIMAP_MARGIN: f32 = 12.0;
const MINIMAP_SIZE: f32 = 168.0;
const MINIMAP_MARKER_SIZE: f32 = 10.0;
const PLAYER_HEALTH_PANEL_MARGIN: f32 = 16.0;
const PLAYER_HEALTH_PANEL_WIDTH: f32 = 240.0;
const PLAYER_HEALTH_BAR_WIDTH: f32 = 208.0;
const PLAYER_HEALTH_BAR_HEIGHT: f32 = 16.0;
const BAR_BACKGROUND: Color = Color::srgba(0.02, 0.035, 0.07, 0.94);
const PANEL_BACKGROUND: Color = Color::srgba(0.05, 0.075, 0.13, 0.96);
const MINIMAP_BACKGROUND: Color = Color::srgba(0.04, 0.055, 0.1, 0.9);
const MINIMAP_BORDER: Color = Color::srgba(0.96, 0.36, 0.34, 0.72);
const MINIMAP_MARKER: Color = Color::srgba(0.94, 0.98, 1.0, 0.96);
const BUTTON_BACKGROUND: Color = Color::srgba(0.09, 0.13, 0.2, 0.98);
const BUTTON_HOVER_BACKGROUND: Color = Color::srgba(0.13, 0.19, 0.29, 1.0);
const BUTTON_ACTIVE_BACKGROUND: Color = Color::srgba(0.0, 0.58, 0.86, 1.0);
const BUTTON_PRESSED_BACKGROUND: Color = Color::srgba(0.0, 0.72, 0.94, 1.0);
const ACTION_BACKGROUND: Color = Color::srgba(0.82, 0.98, 0.28, 1.0);
const ACTION_HOVER_BACKGROUND: Color = Color::srgba(0.92, 1.0, 0.42, 1.0);
const ACTION_PRESSED_BACKGROUND: Color = Color::srgba(0.66, 0.86, 0.17, 1.0);
const START_BACKGROUND: Color = Color::srgba(0.04, 0.72, 0.47, 1.0);
const START_HOVER_BACKGROUND: Color = Color::srgba(0.07, 0.86, 0.58, 1.0);
const PAUSE_BACKGROUND: Color = Color::srgba(0.95, 0.39, 0.18, 1.0);
const PAUSE_HOVER_BACKGROUND: Color = Color::srgba(1.0, 0.5, 0.25, 1.0);
const CLOSE_BACKGROUND: Color = Color::srgba(0.22, 0.08, 0.1, 1.0);
const CLOSE_HOVER_BACKGROUND: Color = Color::srgba(0.9, 0.16, 0.22, 1.0);
const CLOSE_PRESSED_BACKGROUND: Color = Color::srgba(1.0, 0.28, 0.34, 1.0);
const TEXT_PRIMARY: Color = Color::srgb(0.94, 0.98, 1.0);
const TEXT_MUTED: Color = Color::srgb(0.55, 0.64, 0.76);
const TEXT_DARK: Color = Color::srgb(0.02, 0.035, 0.07);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiGameState>()
            .add_systems(
                Startup,
                (
                    spawn_top_bar,
                    spawn_fps_counter,
                    spawn_minimap,
                    spawn_player_health_panel,
                    spawn_defeat_overlay,
                ),
            )
            .add_systems(
                Update,
                (
                    detect_player_defeat,
                    handle_keyboard_exit,
                    handle_ui_buttons,
                    refresh_ui_buttons,
                    refresh_fps_counter,
                    refresh_minimap,
                    refresh_player_health_panel,
                    refresh_defeat_overlay,
                ),
            );
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiGameState {
    pub selected_mode: GameMode,
    pub is_running: bool,
    pub is_defeated: bool,
}

impl Default for UiGameState {
    fn default() -> Self {
        Self {
            selected_mode: GameMode::Arcade,
            is_running: false,
            is_defeated: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Arcade,
    SideScrolling,
}

impl GameMode {
    fn label(self) -> &'static str {
        match self {
            Self::Arcade => "2D Arcade",
            Self::SideScrolling => "2D Side-Scrolling",
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum UiButtonAction {
    SelectMode(GameMode),
    NewGame,
    ToggleRunning,
    ExitGame,
}

#[derive(Component)]
struct StartPauseLabel;

#[derive(Component)]
struct ModeStatusLabel;

#[derive(Component)]
struct FpsLabel;

#[derive(Component)]
struct MinimapRoot;

#[derive(Component)]
struct MinimapMarker;

#[derive(Component)]
struct PlayerHealthRoot;

#[derive(Component)]
struct PlayerHealthFill;

#[derive(Component)]
struct PlayerHealthLabel;

#[derive(Component)]
struct DefeatOverlayRoot;

#[derive(Component)]
struct DefeatOverlayLabel;

type Style = Node;

#[derive(Bundle, Default)]
struct NodeBundle {
    style: Style,
    background_color: BackgroundColor,
    border_color: BorderColor,
    z_index: ZIndex,
    visibility: Visibility,
}

#[derive(Bundle, Default)]
struct ButtonBundle {
    button: Button,
    style: Style,
    background_color: BackgroundColor,
    border_color: BorderColor,
}

struct TextStyle {
    font_size: f32,
    color: Color,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            color: TEXT_PRIMARY,
        }
    }
}

#[derive(Bundle)]
struct TextBundle {
    text: Text,
    font: TextFont,
    color: TextColor,
}

impl TextBundle {
    fn from_section(text: impl Into<String>, style: TextStyle) -> Self {
        Self {
            text: Text::new(text),
            font: TextFont {
                font_size: style.font_size,
                ..default()
            },
            color: TextColor(style.color),
        }
    }
}

fn spawn_top_bar(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(TOP_BAR_HEIGHT + TOP_BAR_MARGIN),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                padding: UiRect::top(Val::Px(TOP_BAR_MARGIN)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            z_index: ZIndex(10),
            ..default()
        })
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(TOP_BAR_WIDTH),
                    height: Val::Px(TOP_BAR_HEIGHT),
                    padding: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(18.0)),
                    ..default()
                },
                background_color: BAR_BACKGROUND.into(),
                border_color: Color::srgba(0.28, 0.38, 0.52, 0.75).into(),
                ..default()
            })
            .with_children(|bar| {
                spawn_header_row(bar);
                spawn_action_row(bar);
            });
        });
}

fn spawn_fps_counter(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(FPS_PANEL_MARGIN),
                right: Val::Px(FPS_PANEL_MARGIN),
                padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            background_color: PANEL_BACKGROUND.into(),
            border_color: Color::srgba(0.28, 0.38, 0.52, 0.75).into(),
            z_index: ZIndex(11),
            ..default()
        })
        .with_children(|panel| {
            panel.spawn((
                TextBundle::from_section(
                    "FPS: --",
                    TextStyle {
                        font_size: 13.0,
                        color: TEXT_PRIMARY,
                        ..default()
                    },
                ),
                FpsLabel,
            ));
        });
}

fn spawn_minimap(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(MINIMAP_MARGIN),
                    left: Val::Px(MINIMAP_MARGIN),
                    width: Val::Px(MINIMAP_SIZE),
                    height: Val::Px(MINIMAP_SIZE),
                    padding: UiRect::ZERO,
                    border: UiRect::all(Val::Px(1.5)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::clip(),
                    border_radius: BorderRadius::all(Val::Percent(50.0)),
                    ..default()
                },
                background_color: MINIMAP_BACKGROUND.into(),
                border_color: MINIMAP_BORDER.into(),
                z_index: ZIndex(11),
                ..default()
            },
            MinimapRoot,
        ))
        .with_children(|minimap| {
            minimap.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px((MINIMAP_SIZE - MINIMAP_MARKER_SIZE) / 2.0),
                        top: Val::Px((MINIMAP_SIZE - MINIMAP_MARKER_SIZE) / 2.0),
                        width: Val::Px(MINIMAP_MARKER_SIZE),
                        height: Val::Px(MINIMAP_MARKER_SIZE),
                        border_radius: BorderRadius::all(Val::Percent(50.0)),
                        ..default()
                    },
                    background_color: MINIMAP_MARKER.into(),
                    ..default()
                },
                MinimapMarker,
            ));
        });
}

fn spawn_player_health_panel(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(PLAYER_HEALTH_PANEL_MARGIN),
                    bottom: Val::Px(PLAYER_HEALTH_PANEL_MARGIN),
                    width: Val::Px(PLAYER_HEALTH_PANEL_WIDTH),
                    padding: UiRect::all(Val::Px(12.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(16.0)),
                    ..default()
                },
                background_color: PANEL_BACKGROUND.into(),
                border_color: Color::srgba(0.28, 0.38, 0.52, 0.75).into(),
                z_index: ZIndex(11),
                ..default()
            },
            PlayerHealthRoot,
        ))
        .with_children(|panel| {
            panel.spawn(TextBundle::from_section(
                "PLAYER",
                TextStyle {
                    font_size: 12.0,
                    color: TEXT_MUTED,
                    ..default()
                },
            ));

            panel.spawn((
                TextBundle::from_section(
                    "HP 100 / 100",
                    TextStyle {
                        font_size: 15.0,
                        color: TEXT_PRIMARY,
                        ..default()
                    },
                ),
                PlayerHealthLabel,
            ));

            panel
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(PLAYER_HEALTH_BAR_WIDTH),
                        height: Val::Px(PLAYER_HEALTH_BAR_HEIGHT),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(999.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.04, 0.06, 0.1, 0.94).into(),
                    border_color: Color::srgba(0.2, 0.32, 0.44, 0.82).into(),
                    ..default()
                })
                .with_children(|bar| {
                    bar.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(PLAYER_HEALTH_BAR_WIDTH),
                                height: Val::Percent(100.0),
                                border_radius: BorderRadius::all(Val::Px(999.0)),
                                ..default()
                            },
                            background_color: Color::srgba(0.24, 0.94, 0.54, 0.96).into(),
                            ..default()
                        },
                        PlayerHealthFill,
                    ));
                });
        });
}

fn spawn_defeat_overlay(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                visibility: Visibility::Hidden,
                background_color: Color::srgba(0.01, 0.02, 0.05, 0.78).into(),
                z_index: ZIndex(30),
                ..default()
            },
            DefeatOverlayRoot,
        ))
        .with_children(|overlay| {
            overlay
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::axes(Val::Px(28.0), Val::Px(22.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(18.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.06, 0.08, 0.14, 0.98).into(),
                    border_color: Color::srgba(0.92, 0.28, 0.26, 0.88).into(),
                    ..default()
                })
                .with_children(|panel| {
                    panel.spawn(TextBundle::from_section(
                        "DEFEAT",
                        TextStyle {
                            font_size: 34.0,
                            color: Color::srgb(1.0, 0.33, 0.3),
                            ..default()
                        },
                    ));

                    panel.spawn((
                        TextBundle::from_section(
                            "Press New Game, then Start to play again.",
                            TextStyle {
                                font_size: 16.0,
                                color: TEXT_PRIMARY,
                                ..default()
                            },
                        ),
                        DefeatOverlayLabel,
                    ));
                });
        });
}

fn spawn_header_row(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                column_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|row| {
            spawn_brand_panel(row);
            spawn_status_panel(row);
            spawn_button(row, "X", UiButtonAction::ExitGame, ButtonKind::Close);
        });
}

fn spawn_action_row(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                column_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|row| {
            spawn_mode_panel(row);
            spawn_control_panel(row);
        });
}

fn spawn_brand_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(124.0),
                height: Val::Percent(100.0),
                padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::all(Val::Px(14.0)),
                ..default()
            },
            background_color: PANEL_BACKGROUND.into(),
            ..default()
        })
        .with_children(|panel| {
            panel.spawn(TextBundle::from_section(
                "BG GAME",
                TextStyle {
                    font_size: 15.0,
                    color: TEXT_PRIMARY,
                    ..default()
                },
            ));
            panel.spawn(TextBundle::from_section(
                "game control",
                TextStyle {
                    font_size: 8.0,
                    color: TEXT_MUTED,
                    ..default()
                },
            ));
        });
}

fn spawn_status_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                flex_grow: 1.0,
                padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::all(Val::Px(14.0)),
                ..default()
            },
            background_color: PANEL_BACKGROUND.into(),
            ..default()
        })
        .with_children(|panel| {
            panel.spawn((
                TextBundle::from_section(
                    "Mode: 2D Arcade",
                    TextStyle {
                        font_size: 11.0,
                        color: TEXT_MUTED,
                        ..default()
                    },
                ),
                ModeStatusLabel,
            ));
        });
}

fn spawn_mode_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                flex_grow: 1.0,
                padding: UiRect::axes(Val::Px(8.0), Val::Px(7.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::all(Val::Px(14.0)),
                ..default()
            },
            background_color: PANEL_BACKGROUND.into(),
            ..default()
        })
        .with_children(|panel| {
            panel
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        column_gap: Val::Px(6.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|buttons| {
                    for mode in [GameMode::Arcade, GameMode::SideScrolling] {
                        spawn_button(
                            buttons,
                            mode.label(),
                            UiButtonAction::SelectMode(mode),
                            ButtonKind::Mode,
                        );
                    }
                });
        });
}

fn spawn_control_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(128.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(7.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(6.0),
                border_radius: BorderRadius::all(Val::Px(14.0)),
                ..default()
            },
            background_color: PANEL_BACKGROUND.into(),
            ..default()
        })
        .with_children(|panel| {
            spawn_button(
                panel,
                "New Game",
                UiButtonAction::NewGame,
                ButtonKind::Action,
            );

            spawn_button(
                panel,
                "Start",
                UiButtonAction::ToggleRunning,
                ButtonKind::Start,
            );
        });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonKind {
    Mode,
    Action,
    Start,
    Close,
}

fn spawn_button(
    parent: &mut ChildSpawnerCommands,
    label: &'static str,
    action: UiButtonAction,
    kind: ButtonKind,
) -> Entity {
    let min_width = match kind {
        ButtonKind::Mode => Val::Px(84.0),
        ButtonKind::Action | ButtonKind::Start => Val::Percent(100.0),
        ButtonKind::Close => Val::Px(36.0),
    };

    let height = match kind {
        ButtonKind::Close => Val::Px(36.0),
        _ => Val::Px(28.0),
    };

    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    min_width,
                    height,
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(0.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: button_color(action, Interaction::None, &UiGameState::default())
                    .into(),
                border_color: Color::srgba(0.25, 0.34, 0.46, 0.7).into(),
                ..default()
            },
            action,
        ))
        .with_children(|button| {
            let text_bundle = TextBundle::from_section(
                label,
                TextStyle {
                    font_size: if kind == ButtonKind::Close {
                        14.0
                    } else {
                        10.0
                    },
                    color: if kind == ButtonKind::Action {
                        TEXT_DARK
                    } else {
                        TEXT_PRIMARY
                    },
                    ..default()
                },
            );

            if action == UiButtonAction::ToggleRunning {
                button.spawn((text_bundle, StartPauseLabel));
            } else {
                button.spawn(text_bundle);
            }
        })
        .id()
}

fn handle_ui_buttons(
    mut state: ResMut<UiGameState>,
    mut interactions: Query<(&Interaction, &UiButtonAction), (Changed<Interaction>, With<Button>)>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, action) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            UiButtonAction::SelectMode(mode) => {
                state.selected_mode = *mode;
                state.is_running = false;
                state.is_defeated = false;
            }
            UiButtonAction::NewGame => {
                state.is_running = false;
                state.is_defeated = false;
            }
            UiButtonAction::ToggleRunning => {
                if !state.is_defeated {
                    state.is_running = !state.is_running;
                }
            }
            UiButtonAction::ExitGame => {
                exit.write(AppExit::Success);
            }
        }
    }
}

fn detect_player_defeat(
    mut state: ResMut<UiGameState>,
    player_query: Query<&Health, With<Player>>,
) {
    if state.selected_mode != GameMode::Arcade || state.is_defeated {
        return;
    }

    let Ok(health) = player_query.single() else {
        return;
    };

    if health.current <= 0.0 {
        state.is_running = false;
        state.is_defeated = true;
    }
}

fn handle_keyboard_exit(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn refresh_ui_buttons(
    state: Res<UiGameState>,
    mut buttons: Query<
        (
            &Interaction,
            &UiButtonAction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        With<Button>,
    >,
    mut start_labels: Query<&mut Text, With<StartPauseLabel>>,
    mut mode_labels: Query<&mut Text, (With<ModeStatusLabel>, Without<StartPauseLabel>)>,
) {
    for (interaction, action, mut background, mut border) in &mut buttons {
        *background = button_color(*action, *interaction, &state).into();
        *border = border_color(*action, *interaction, &state).into();
    }

    for mut text in &mut start_labels {
        text.0 = if state.is_defeated {
            "Start".into()
        } else if state.is_running {
            "Pause".into()
        } else {
            "Start".into()
        };
    }

    for mut text in &mut mode_labels {
        text.0 = format!("Mode: {}", state.selected_mode.label());
    }
}

fn refresh_fps_counter(
    diagnostics: Res<DiagnosticsStore>,
    mut labels: Query<&mut Text, With<FpsLabel>>,
) {
    let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    else {
        return;
    };

    for mut text in &mut labels {
        text.0 = format!("FPS: {fps:.0}");
    }
}

fn refresh_minimap(
    state: Res<UiGameState>,
    map_state: Res<WorldMapState>,
    player_query: Query<&Transform, With<Player>>,
    mut minimap_roots: Query<&mut Visibility, With<MinimapRoot>>,
    mut marker_styles: Query<&mut Style, With<MinimapMarker>>,
) {
    let minimap_visibility = if state.selected_mode == GameMode::Arcade {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };

    for mut visibility in &mut minimap_roots {
        *visibility = minimap_visibility;
    }

    if state.selected_mode != GameMode::Arcade {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let map_radius = map_state.radius.max(MAP_RADIUS);
    let minimap_position = world_to_minimap(
        player_transform.translation.truncate(),
        map_radius,
        MINIMAP_SIZE,
    );

    for mut style in &mut marker_styles {
        style.left = Val::Px(minimap_position.x - MINIMAP_MARKER_SIZE / 2.0);
        style.top = Val::Px(minimap_position.y - MINIMAP_MARKER_SIZE / 2.0);
    }
}

fn refresh_player_health_panel(
    state: Res<UiGameState>,
    player_query: Query<&Health, With<Player>>,
    mut roots: Query<&mut Visibility, With<PlayerHealthRoot>>,
    mut labels: Query<&mut Text, With<PlayerHealthLabel>>,
    mut fills: Query<&mut Style, With<PlayerHealthFill>>,
) {
    let visibility = if state.selected_mode == GameMode::Arcade {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };

    for mut root_visibility in &mut roots {
        *root_visibility = visibility;
    }

    if state.selected_mode != GameMode::Arcade {
        return;
    }

    let Ok(health) = player_query.single() else {
        return;
    };

    for mut text in &mut labels {
        text.0 = format!("HP {:.0} / {:.0}", health.current, health.max);
    }

    let width = PLAYER_HEALTH_BAR_WIDTH * health.ratio();
    for mut style in &mut fills {
        style.width = Val::Px(width);
    }
}

fn refresh_defeat_overlay(
    state: Res<UiGameState>,
    mut overlays: Query<&mut Visibility, With<DefeatOverlayRoot>>,
    mut labels: Query<&mut Text, With<DefeatOverlayLabel>>,
) {
    let show_defeat = state.selected_mode == GameMode::Arcade && state.is_defeated;
    let visibility = if show_defeat {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };

    for mut overlay_visibility in &mut overlays {
        *overlay_visibility = visibility;
    }

    for mut text in &mut labels {
        text.0 = if show_defeat {
            "Press New Game, then Start to play again.".into()
        } else {
            String::new()
        };
    }
}

fn world_to_minimap(world_position: Vec2, map_radius: f32, minimap_size: f32) -> Vec2 {
    let normalized = (world_position / map_radius).clamp_length_max(1.0);
    let half_size = minimap_size / 2.0;

    Vec2::new(
        half_size + normalized.x * half_size,
        half_size - normalized.y * half_size,
    )
}

fn button_color(action: UiButtonAction, interaction: Interaction, state: &UiGameState) -> Color {
    if matches!(action, UiButtonAction::ToggleRunning) && state.is_defeated {
        return Color::srgba(0.2, 0.23, 0.28, 0.92);
    }

    if interaction == Interaction::Pressed {
        return match action {
            UiButtonAction::NewGame => ACTION_PRESSED_BACKGROUND,
            UiButtonAction::ExitGame => CLOSE_PRESSED_BACKGROUND,
            UiButtonAction::ToggleRunning => {
                if state.is_running {
                    PAUSE_HOVER_BACKGROUND
                } else {
                    START_HOVER_BACKGROUND
                }
            }
            UiButtonAction::SelectMode(_) => BUTTON_PRESSED_BACKGROUND,
        };
    }

    match action {
        UiButtonAction::SelectMode(mode) if state.selected_mode == mode => BUTTON_ACTIVE_BACKGROUND,
        UiButtonAction::SelectMode(_) if interaction == Interaction::Hovered => {
            BUTTON_HOVER_BACKGROUND
        }
        UiButtonAction::SelectMode(_) => BUTTON_BACKGROUND,
        UiButtonAction::NewGame if interaction == Interaction::Hovered => ACTION_HOVER_BACKGROUND,
        UiButtonAction::NewGame => ACTION_BACKGROUND,
        UiButtonAction::ExitGame if interaction == Interaction::Hovered => CLOSE_HOVER_BACKGROUND,
        UiButtonAction::ExitGame => CLOSE_BACKGROUND,
        UiButtonAction::ToggleRunning
            if state.is_running && interaction == Interaction::Hovered =>
        {
            PAUSE_HOVER_BACKGROUND
        }
        UiButtonAction::ToggleRunning if state.is_running => PAUSE_BACKGROUND,
        UiButtonAction::ToggleRunning if interaction == Interaction::Hovered => {
            START_HOVER_BACKGROUND
        }
        UiButtonAction::ToggleRunning => START_BACKGROUND,
    }
}

fn border_color(action: UiButtonAction, interaction: Interaction, state: &UiGameState) -> Color {
    if matches!(action, UiButtonAction::ToggleRunning) && state.is_defeated {
        return Color::srgba(0.3, 0.35, 0.42, 0.72);
    }

    if interaction == Interaction::Hovered || interaction == Interaction::Pressed {
        return Color::srgba(0.75, 0.92, 1.0, 0.92);
    }

    match action {
        UiButtonAction::SelectMode(mode) if state.selected_mode == mode => {
            Color::srgba(0.75, 0.92, 1.0, 1.0)
        }
        _ => Color::srgba(0.24, 0.33, 0.45, 0.65),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.0001;

    fn assert_vec2_near(actual: Vec2, expected: Vec2) {
        assert!(
            actual.distance(expected) <= EPSILON,
            "expected {expected:?}, got {actual:?}"
        );
    }

    #[test]
    fn default_ui_state_starts_in_paused_arcade_mode() {
        let state = UiGameState::default();

        assert_eq!(state.selected_mode, GameMode::Arcade);
        assert!(!state.is_running);
        assert!(!state.is_defeated);
    }

    #[test]
    fn game_mode_labels_match_button_text() {
        assert_eq!(GameMode::Arcade.label(), "2D Arcade");
        assert_eq!(GameMode::SideScrolling.label(), "2D Side-Scrolling");
    }

    #[test]
    fn world_to_minimap_maps_world_center_and_edges() {
        assert_vec2_near(
            world_to_minimap(Vec2::ZERO, 100.0, 200.0),
            Vec2::new(100.0, 100.0),
        );
        assert_vec2_near(
            world_to_minimap(Vec2::new(100.0, 0.0), 100.0, 200.0),
            Vec2::new(200.0, 100.0),
        );
        assert_vec2_near(
            world_to_minimap(Vec2::new(0.0, 100.0), 100.0, 200.0),
            Vec2::new(100.0, 0.0),
        );
    }

    #[test]
    fn world_to_minimap_clamps_positions_outside_map_radius() {
        assert_vec2_near(
            world_to_minimap(Vec2::new(300.0, 0.0), 100.0, 200.0),
            Vec2::new(200.0, 100.0),
        );
    }

    #[test]
    fn button_color_reflects_state_interaction_and_action() {
        let running = UiGameState {
            selected_mode: GameMode::Arcade,
            is_running: true,
            is_defeated: false,
        };
        let defeated = UiGameState {
            selected_mode: GameMode::Arcade,
            is_running: false,
            is_defeated: true,
        };

        assert_eq!(
            button_color(
                UiButtonAction::SelectMode(GameMode::Arcade),
                Interaction::None,
                &running
            ),
            BUTTON_ACTIVE_BACKGROUND
        );
        assert_eq!(
            button_color(UiButtonAction::ToggleRunning, Interaction::None, &running),
            PAUSE_BACKGROUND
        );
        assert_eq!(
            button_color(
                UiButtonAction::ToggleRunning,
                Interaction::Hovered,
                &defeated
            ),
            Color::srgba(0.2, 0.23, 0.28, 0.92)
        );
    }

    #[test]
    fn border_color_highlights_hovered_and_selected_buttons() {
        let state = UiGameState::default();

        assert_eq!(
            border_color(UiButtonAction::NewGame, Interaction::Hovered, &state),
            Color::srgba(0.75, 0.92, 1.0, 0.92)
        );
        assert_eq!(
            border_color(
                UiButtonAction::SelectMode(GameMode::Arcade),
                Interaction::None,
                &state
            ),
            Color::srgba(0.75, 0.92, 1.0, 1.0)
        );
    }
}
