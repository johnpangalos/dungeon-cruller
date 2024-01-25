use bevy::prelude::*;

pub const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);
pub const PLAYER_SPEED: f32 = 500.0;

pub const WALL_THICKNESS: f32 = 10.0;
pub const LEFT_WALL: f32 = -450.;
pub const RIGHT_WALL: f32 = 450.;
pub const BOTTOM_WALL: f32 = -300.;
pub const TOP_WALL: f32 = 300.;

pub const DOOR_WIDTH: f32 = 50.;
pub const DOOR_THICKNESS: f32 = 10.;

pub const PLAYER_COLOR: Color = Color::rgb(0.0, 0.9, 0.0);
pub const WALL_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
pub const DOOR_COLOR: Color = Color::rgb(9.0, 0.0, 0.0);
