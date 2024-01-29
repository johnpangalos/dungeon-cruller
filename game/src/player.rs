use std::ops::Mul;

use crate::{
    constants::{self, GameState, PLAYER_SPEED},
    scenes::console_log,
};
use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{RigidBody, Velocity},
    geometry::Collider,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    life: Life,
    player: Player,
    body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    speed: Speed,
}

impl PlayerBundle {
    pub fn new(position: Vec2, image: Handle<Image>) -> PlayerBundle {
        // Rectangle

        PlayerBundle {
            player: Player,
            life: Life(2),
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(position.extend(3.)),
                texture: image,
                sprite: Sprite {
                    custom_size: Some(constants::PLAYER_SIZE),
                    ..default()
                },
                ..default()
            },
            body: RigidBody::KinematicVelocityBased,
            collider: Collider::ball(constants::PLAYER_SIZE.x / 4.0),
            velocity: Velocity::default(),
            speed: Speed(PLAYER_SPEED),
        }
    }
}

#[derive(Component)]
pub struct Life(pub u32);

pub struct Item(pub String);

#[derive(Component)]
pub struct Inventory(pub Vec<Item>);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Player;

pub fn pause_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Paused);
}

pub fn unpause_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Running);
}

fn input_as_axis(
    keyboard_input: Res<Input<KeyCode>>,
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,
) -> Vec2 {
    let mut axis = Vec2::ZERO;

    if keyboard_input.pressed(left) {
        axis.x -= 1.0;
    }

    if keyboard_input.pressed(right) {
        axis.x += 1.0;
    }

    if keyboard_input.pressed(up) {
        axis.y += 1.0;
    }

    if keyboard_input.pressed(down) {
        axis.y -= 1.0;
    }

    axis
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &Speed), With<Player>>,
) {
    let (mut velocity, Speed(speed)) = player_query.single_mut();

    let axis = input_as_axis(
        keyboard_input,
        KeyCode::A,
        KeyCode::D,
        KeyCode::W,
        KeyCode::S,
    );
    velocity.linvel = axis.mul(*speed);
}
