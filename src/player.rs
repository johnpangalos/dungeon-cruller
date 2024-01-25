use crate::constants;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
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

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
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
