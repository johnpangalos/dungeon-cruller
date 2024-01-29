use bevy::prelude::*;

pub const PLAYER_SIZE: Vec2 = Vec2::new(64.0, 64.0);
pub const PLAYER_SPEED: f32 = 500.0;

pub const WIDTH: f32 = 1920.;
pub const HEIGHT: f32 = 1080.;

pub const FLOOR_WIDTH: f32 = 640.;
pub const FLOOR_HEIGHT: f32 = 480.;

pub const WALL_THICKNESS: f32 = 320.0;
pub const LEFT_WALL: f32 = -FLOOR_WIDTH / 2. - WALL_THICKNESS;
pub const RIGHT_WALL: f32 = FLOOR_WIDTH / 2. + WALL_THICKNESS;
pub const BOTTOM_WALL: f32 = -FLOOR_HEIGHT / 2. - WALL_THICKNESS;
pub const TOP_WALL: f32 = FLOOR_HEIGHT / 2. + WALL_THICKNESS;

pub const WALL_HEIGHT: f32 = FLOOR_HEIGHT / 2. + WALL_THICKNESS - DOOR_WIDTH;
pub const WALL_WIDTH: f32 = FLOOR_WIDTH / 2. + WALL_THICKNESS - DOOR_WIDTH;

pub const DOOR_WIDTH: f32 = 100.;
pub const DOOR_THICKNESS: f32 = 2.;

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
