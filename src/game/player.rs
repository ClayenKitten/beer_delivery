use bevy::{prelude::*, transform::TransformSystem};

use crate::GameState;

use super::GameSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Game)))
            .add_systems(
                (
                    movement.in_base_set(CoreSet::FixedUpdate),
                    animate_player,
                    follow_player
                        .in_base_set(CoreSet::PostUpdate)
                        .before(TransformSystem::TransformPropagate),
                )
                    .in_set(GameSystemSet),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player.single_mut();

    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::A) {
        direction += Vec3::NEG_X;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::X;
    }
    if keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::Y;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction += Vec3::NEG_Y;
    }
    if direction != Vec3::ZERO {
        direction = direction.normalize()
    }

    let z = transform.translation.z;
    transform.translation += time.delta_seconds() * direction * 50.;
    transform.translation.z = z;
}

fn animate_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut MovementAnimation,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    mut offset: Local<usize>,
) {
    let (mut animation, mut timer, mut sprite) = query.single_mut();

    let new_animation;
    if keyboard_input.pressed(KeyCode::S) {
        new_animation = MovementAnimation::Walking(Direction::Down);
    } else if keyboard_input.pressed(KeyCode::W) {
        new_animation = MovementAnimation::Walking(Direction::Up);
    } else if keyboard_input.pressed(KeyCode::D) {
        new_animation = MovementAnimation::Walking(Direction::Right);
    } else if keyboard_input.pressed(KeyCode::A) {
        new_animation = MovementAnimation::Walking(Direction::Left);
    } else {
        *offset = 0;
        new_animation = MovementAnimation::Standing(animation.direction());
    }

    let mut should_change = false;
    timer.tick(time.delta());
    if animation.ne(&new_animation) {
        *animation = new_animation;
        should_change = true;
        timer.reset();
    }
    should_change |= timer.just_finished();

    if should_change {
        sprite.index = animation.index(*offset);
        if *offset < animation.animation_len() {
            *offset += 1;
        } else {
            *offset = 0;
        }
    }
}

fn follow_player(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera = camera.single_mut();
    let player = player.single();

    let z = camera.translation.z;
    camera.translation = player.translation;
    camera.translation.z = z;
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("city_tiles.png");
    let movement_animation = MovementAnimation::Standing(Direction::Down);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 27, 18, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(movement_animation.index(0)),
            transform: Transform::from_scale(Vec3::splat(1.))
                .with_translation(Vec3::new(0., 0., 2.)),
            ..default()
        },
        Player,
        movement_animation,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum MovementAnimation {
    Standing(Direction),
    Walking(Direction),
}

impl MovementAnimation {
    pub fn animation_len(&self) -> usize {
        match self {
            MovementAnimation::Standing(_) => 1,
            MovementAnimation::Walking(_) => 4,
        }
    }

    pub fn index(&self, offset: usize) -> usize {
        match self {
            MovementAnimation::Standing(dir) => match dir {
                Direction::Left => 23,
                Direction::Down => 24,
                Direction::Up => 25,
                Direction::Right => 26,
            },
            MovementAnimation::Walking(dir) => {
                let main = match dir {
                    Direction::Up => 25,
                    Direction::Down => 24,
                    Direction::Left => 23,
                    Direction::Right => 26,
                };
                let offset = match offset % self.animation_len() {
                    0 => 27,
                    1 => 0,
                    2 => 27 * 2,
                    3 => 0,
                    _ => unreachable!(),
                };
                main + offset
            }
        }
    }

    pub fn direction(&self) -> Direction {
        match self {
            MovementAnimation::Standing(dir) => *dir,
            MovementAnimation::Walking(dir) => *dir,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
