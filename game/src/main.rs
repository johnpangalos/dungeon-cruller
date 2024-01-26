mod constants;
mod doors;
mod player;
mod scenes;
mod walls;

use bevy::prelude::*;
use constants::GameState;
use scenes::RootMainMenu;
use styles::elements::StylesPlugin;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((DefaultPlugins, StylesPlugin))
        .add_plugins(RootMainMenu)
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Game), setup_game)
        .add_systems(
            FixedUpdate,
            (
                player::move_player,
                doors::check_door_collisions,
                doors::print_collision,
            )
                .chain()
                .run_if(in_state(GameState::Game)),
        )
        .add_event::<doors::CollisionEvent>()
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_game(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Rectangle
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: constants::PLAYER_COLOR,
                custom_size: Some(constants::PLAYER_SIZE),
                ..default()
            },
            ..default()
        },
        player::Player,
    ));

    // Walls
    commands.spawn(walls::WallBundle::new(walls::WallLocation::Left));
    commands.spawn(walls::WallBundle::new(walls::WallLocation::Right));
    commands.spawn(walls::WallBundle::new(walls::WallLocation::Bottom));
    commands.spawn(walls::WallBundle::new(walls::WallLocation::Top));

    // Doors
    commands.spawn(doors::DoorBundle::new(doors::DoorLocation::Left));
    commands.spawn(doors::DoorBundle::new(doors::DoorLocation::Right));
    commands.spawn(doors::DoorBundle::new(doors::DoorLocation::Bottom));
    commands.spawn(doors::DoorBundle::new(doors::DoorLocation::Top));
}
