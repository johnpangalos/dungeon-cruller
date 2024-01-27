use crate::constants::{self, GameState};
use crate::doors::Collider;
use bevy::render::primitives::Aabb;
use bevy::{prelude::*, sprite::collide_aabb::collide};

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    life: Life,
    player: Player,
}

impl PlayerBundle {
    pub fn new(position: Vec2) -> PlayerBundle {
        // Rectangle

        PlayerBundle {
            player: Player,
            life: Life(2),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.),
                    ..default()
                },
                sprite: Sprite {
                    color: constants::PLAYER_COLOR,
                    custom_size: Some(constants::PLAYER_SIZE),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct Life(pub u32);

pub struct Item {
    pub name: String,
}

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Item>,
}

#[derive(Component)]
pub struct Player;

pub fn pause_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Paused);
}

pub fn unpause_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Running);
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    // query: Query<&mut Transform, With<Player>>,
    mut player_query: Query<(&mut Transform, &Aabb), With<Player>>,
    collider_query: Query<(&Transform, &Aabb), (With<Collider>, Without<Player>)>,
    time: Res<Time>,
) {
    let (mut player_transform, player_aabb) = player_query.single_mut();
    let mut y_direction = 0.0;
    let mut x_direction = 0.0;

    if keyboard_input.pressed(KeyCode::W) {
        y_direction += 1.0
    }
    if keyboard_input.pressed(KeyCode::S) {
        y_direction -= 1.0
    }
    if keyboard_input.pressed(KeyCode::D) {
        x_direction += 1.0
    }
    if keyboard_input.pressed(KeyCode::A) {
        x_direction -= 1.0
    }

    let new_player_x_position = player_transform.translation.x
        + x_direction * constants::PLAYER_SPEED * time.delta_seconds();
    let new_player_y_position = player_transform.translation.y
        + y_direction * constants::PLAYER_SPEED * time.delta_seconds();

    for (_i, (transform, aabb)) in collider_query.iter().enumerate() {
        let collision = collide(
            player_transform.translation + Vec3::from(player_aabb.center),
            player_aabb.half_extents.truncate() * Vec2::splat(2.),
            transform.translation + Vec3::from(aabb.center),
            aabb.half_extents.truncate() * Vec2::splat(2.),
        );

        if let Some(_collision) = collision {
            player_transform.translation.x = new_player_x_position;
            player_transform.translation.y = new_player_y_position;
            return;
        }
    }

    let left_bound =
        constants::LEFT_WALL + constants::WALL_THICKNESS / 2.0 + constants::PLAYER_SIZE.x / 2.0;
    let right_bound =
        constants::RIGHT_WALL - constants::WALL_THICKNESS / 2.0 - constants::PLAYER_SIZE.x / 2.0;
    let top_bound =
        constants::TOP_WALL - constants::WALL_THICKNESS / 2.0 - constants::PLAYER_SIZE.y / 2.0;
    let bottom_bound =
        constants::BOTTOM_WALL + constants::WALL_THICKNESS / 2.0 + constants::PLAYER_SIZE.y / 2.0;

    player_transform.translation.x = new_player_x_position.clamp(left_bound, right_bound);
    player_transform.translation.y = new_player_y_position.clamp(bottom_bound, top_bound);
}
