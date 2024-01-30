use std::ops::Mul;

use crate::{
    constants::{self, GameState, PLAYER_SPEED},
    doors::Door,
    inventory::{use_active_item, Inventory, ItemEvent},
    scenes::console_log,
};
use bevy::prelude::*;
use bevy_rapier2d::{
    control::KinematicCharacterController, geometry::Collider, plugin::RapierContext,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    life: Life,
    player: Player,
    character_controller: KinematicCharacterController,
    collider: Collider,
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
            character_controller: KinematicCharacterController::default(),
            collider: Collider::ball(constants::PLAYER_SIZE.x / 4.0),
            speed: Speed(PLAYER_SPEED),
        }
    }
}

#[derive(Component)]
pub struct Life(pub u32);

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
) -> Option<Vec2> {
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

    if axis != Vec2::ZERO {
        return Some(axis.normalize());
    }
    return None;
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut KinematicCharacterController, &Speed), With<Player>>,
) {
    let (mut controller, Speed(speed)) = player_query.single_mut();

    let axis = input_as_axis(
        keyboard_input,
        KeyCode::A,
        KeyCode::D,
        KeyCode::W,
        KeyCode::S,
    );

    controller.translation = axis.map(|ax| ax.mul(*speed) * time.delta_seconds());
}

pub fn use_item_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut inventory: Query<&mut Inventory, With<Player>>,
    mut writer: EventWriter<ItemEvent>,
) {
    if let Ok(mut inventory) = inventory.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            use_active_item(&mut inventory, &mut writer);
        }
    }
}

pub fn read_touching_door_system(
    context: Res<RapierContext>,
    query_player: Query<Entity, With<Player>>,
    query_door: Query<Entity, With<Door>>,
) {
    console_log("Touching door", "false");

    if let Ok(player) = query_player.get_single() {
        let pairs = context.intersection_pairs_with(player);

        for (_, entity, _) in pairs {
            if query_door.contains(entity) {
                console_log("Touching door", "true");
            }
        }
    }
}
