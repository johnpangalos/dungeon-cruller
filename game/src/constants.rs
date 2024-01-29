use bevy::prelude::*;

pub const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);
pub const PLAYER_SPEED: f32 = 500.0;

pub const WALL_THICKNESS: f32 = 175.0;
pub const LEFT_WALL: f32 = -600.;
pub const RIGHT_WALL: f32 = 600.;
pub const BOTTOM_WALL: f32 = -420.;
pub const TOP_WALL: f32 = 420.;

pub const HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
pub const WIDTH: f32 = RIGHT_WALL - LEFT_WALL;

pub const DOOR_WIDTH: f32 = 100.;
pub const DOOR_THICKNESS: f32 = 2.;

pub const PLAYER_COLOR: Color = Color::rgb(0.0, 0.9, 0.0);
pub const WALL_COLOR: Color = Color::rgb(0.01, 0.01, 0.01);
pub const DOOR_COLOR: Color = Color::rgb(9.0, 0.0, 0.0);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    SetupGame,
    Game,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}
