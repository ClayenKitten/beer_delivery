pub mod map;
pub mod player;

use bevy::prelude::*;

use crate::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player::PlayerPlugin)
            .add_plugin(map::MapPlugin)
            .configure_set(GameSystemSet.run_if(in_state(GameState::Game)))
            .add_system(game_setup.in_schedule(OnEnter(GameState::Game)))
            .add_system(game_cleanup.in_schedule(OnExit(GameState::Game)));
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct GameSystemSet;

fn game_setup(mut _commands: Commands, _asset_server: Res<AssetServer>) {}

fn game_cleanup(entities: Query<Entity, With<OnGameScreen>>, mut commands: Commands) {
    entities.for_each(|entity| commands.entity(entity).despawn_recursive());
}

/// Tag component used to tag entities added on the game screen.
#[derive(Component)]
struct OnGameScreen;
