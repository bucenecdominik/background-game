//! UI module.

use bevy::prelude::*;

const TOP_BAR_HEIGHT: f32 = 126.0;
const TOP_BAR_WIDTH: f32 = 456.0;
const TOP_BAR_MARGIN: f32 = 6.0;
const BAR_BACKGROUND: Color = Color::srgba(0.02, 0.035, 0.07, 0.94);
const PANEL_BACKGROUND: Color = Color::srgba(0.05, 0.075, 0.13, 0.96);
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
            .add_systems(Startup, spawn_top_bar)
            .add_systems(
                Update,
                (handle_keyboard_exit, handle_ui_buttons, refresh_ui_buttons),
            );
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiGameState {
    pub selected_mode: GameMode,
    pub is_running: bool,
}

impl Default for UiGameState {
    fn default() -> Self {
        Self {
            selected_mode: GameMode::Arcade,
            is_running: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Arcade,
    SideScrolling,
    IdleOverlay,
}

impl GameMode {
    fn label(self) -> &'static str {
        match self {
            Self::Arcade => "2D Arcade",
            Self::SideScrolling => "2D Side-Scrolling",
            Self::IdleOverlay => "Idle Overlay",
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
            z_index: ZIndex::Global(10),
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
                    ..default()
                },
                background_color: BAR_BACKGROUND.into(),
                border_color: Color::srgba(0.28, 0.38, 0.52, 0.75).into(),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            })
            .with_children(|bar| {
                spawn_header_row(bar);
                spawn_action_row(bar);
            });
        });
}

fn spawn_header_row(parent: &mut ChildBuilder) {
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

fn spawn_action_row(parent: &mut ChildBuilder) {
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

fn spawn_brand_panel(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(124.0),
                height: Val::Percent(100.0),
                padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: PANEL_BACKGROUND.into(),
            border_radius: BorderRadius::all(Val::Px(14.0)),
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
                "overlay control",
                TextStyle {
                    font_size: 8.0,
                    color: TEXT_MUTED,
                    ..default()
                },
            ));
        });
}

fn spawn_status_panel(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                flex_grow: 1.0,
                padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: PANEL_BACKGROUND.into(),
            border_radius: BorderRadius::all(Val::Px(14.0)),
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

fn spawn_mode_panel(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                flex_grow: 1.0,
                padding: UiRect::axes(Val::Px(8.0), Val::Px(7.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: PANEL_BACKGROUND.into(),
            border_radius: BorderRadius::all(Val::Px(14.0)),
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
                    for mode in [
                        GameMode::Arcade,
                        GameMode::SideScrolling,
                        GameMode::IdleOverlay,
                    ] {
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

fn spawn_control_panel(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(128.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(7.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(6.0),
                ..default()
            },
            background_color: PANEL_BACKGROUND.into(),
            border_radius: BorderRadius::all(Val::Px(14.0)),
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
    parent: &mut ChildBuilder,
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
                    ..default()
                },
                background_color: button_color(action, Interaction::None, &UiGameState::default())
                    .into(),
                border_color: Color::srgba(0.25, 0.34, 0.46, 0.7).into(),
                border_radius: BorderRadius::all(Val::Px(10.0)),
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
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            UiButtonAction::SelectMode(mode) => state.selected_mode = *mode,
            UiButtonAction::NewGame => state.is_running = false,
            UiButtonAction::ToggleRunning => state.is_running = !state.is_running,
            UiButtonAction::ExitGame => {
                exit.send(AppExit::Success);
            }
        }
    }
}

fn handle_keyboard_exit(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
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
        text.sections[0].value = if state.is_running {
            "Pause".into()
        } else {
            "Start".into()
        };
    }

    for mut text in &mut mode_labels {
        text.sections[0].value = format!("Mode: {}", state.selected_mode.label());
    }
}

fn button_color(action: UiButtonAction, interaction: Interaction, state: &UiGameState) -> Color {
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
