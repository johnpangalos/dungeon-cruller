use std::ops::Mul;

use crate::{
    constants::{self, AppSet, AppState, GameState, PLAYER_SPEED},
    doors::Door,
    input::input_as_axis,
    inventory::Inventory,
    items::components::ItemEvent,
    scenes::console_log,
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_rapier2d::{
    control::KinematicCharacterController, geometry::Collider, plugin::RapierContext,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                pause_game.run_if(in_state(GameState::Running)),
                unpause_game.run_if(in_state(GameState::Paused)),
            )
                .run_if(in_state(AppState::Game))
                .run_if(input_just_pressed(KeyCode::Escape)),
        )
        .add_systems(Update, use_item_player.in_set(AppSet::Player))
        .add_systems(
            FixedUpdate,
            (move_player, read_touching_door_system)
                .chain()
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)),
        );
    }
}

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

fn pause_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Paused);
}

fn unpause_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Running);
}

fn move_player(
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

fn use_item_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut inventory: Query<(&mut Inventory, &Transform), With<Player>>,
    mut writer: EventWriter<ItemEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok((mut inventory, transform)) = inventory.get_single_mut() {
            let position = transform.translation.truncate();
            use_active_item(&mut inventory, &position, &transform.rotation, &mut writer);
        }
    }
}

fn read_touching_door_system(
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

fn use_active_item(
    inventory: &mut Inventory,
    position: &Vec2,
    rotation: &Quat,
    writer: &mut EventWriter<ItemEvent>,
) {
    let event = |entity: &Entity| ItemEvent::Used {
        entity: entity.clone(),
        position: position.clone(),
        rotation: rotation.clone(),
    };

    match inventory {
        Inventory::DoubleHanded(Some(entity), _) => {
            writer.send(event(entity));
        }
        Inventory::Revolver(entities) => {
            if let Some(entity) = entities.front() {
                writer.send(event(entity));
            }

            entities.rotate_left(1);
        }
        Inventory::OneHanded(Some(entity)) => {
            writer.send(event(entity));
        }
        Inventory::DoubleHanded(None, _) | Inventory::OneHanded(None) => {}
    };
}

fn r(
    context: Res<RapierContext>,
    query_player: Query<Entity, With<Player>>,
    query_door: Query<(&Door, Entity), With<Door>>,
    // query_dungeon: Query<Entity, With<Player>>,
) {
    console_log("Touching door", "false");

    if let Ok(player) = query_player.get_single() {
        let pairs = context.intersection_pairs_with(player);

        for (_, entity, _) in pairs {
            if let Ok((door, _)) = query_door.get(entity) {
                console_log("Touching door", "true");
            }
        }
    }
}
