#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use game::GamePlugin;
use menu::MenuPlugin;

mod game;
mod map;
mod menu;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    Game,
}

fn main() {
    const TITLE: &str = "Beer Delivery";

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from(TITLE),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<GameState>()
        .add_plugin(TilemapPlugin)
        .add_startup_system(setup)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    bundle.projection.scale = 0.25;
    commands.spawn(bundle);
}
