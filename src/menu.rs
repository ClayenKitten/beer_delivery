use bevy::{app::AppExit, prelude::*};

use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .configure_set(MenuSystemSet.run_if(in_state(GameState::MainMenu)))
            .add_system(main_menu_setup.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(main_menu_cleanup.in_schedule(OnExit(GameState::MainMenu)))
            .add_systems((button_system, menu_action).in_set(MenuSystemSet));
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct MenuSystemSet;

// State used for the current menu screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, States, Hash, Default)]
enum MenuState {
    Main,
    Settings,
    #[default]
    Disabled,
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    Quit,
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            spawn_button(parent, &asset_server, "Play", MenuButtonAction::Play);
            spawn_button(parent, &asset_server, "Settings", MenuButtonAction::Settings);
            spawn_button(parent, &asset_server, "Quit", MenuButtonAction::Quit);
        });
}

fn spawn_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    action: MenuButtonAction,
) {
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    parent
        .spawn((
            ButtonBundle {
                style: button_style,
                background_color: NORMAL_BUTTON.into(),
                ..Default::default()
            },
            action,
        ))
        .with_children(|parent| spawn_text(parent, asset_server, text));
}

fn spawn_text(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, text: &str) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let button_text_style = TextStyle {
        font,
        font_size: 40.0,
        color: Color::rgb_u8(100, 100, 100),
    };

    parent.spawn(TextBundle::from_section(text, button_text_style));
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
            }
        }
    }
}

fn main_menu_cleanup(entities: Query<Entity, With<OnMainMenuScreen>>, mut commands: Commands) {
    entities.for_each(|entity| commands.entity(entity).despawn_recursive());
}
