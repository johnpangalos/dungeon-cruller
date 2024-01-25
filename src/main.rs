mod constants;
mod player;
mod walls;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (player::move_player).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Rectangle
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: constants::PLAYER_COLOR,
                custom_size: Some(constants::PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..default()
        },
        player::Player,
    ));

    // Walls
    commands.spawn(walls::WallBundle::new(walls::WallLocation::Left));
    commands.spawn(walls::WallBundle::new(walls::WallLocation::Right));
    commands.spawn(walls::WallBundle::new(walls::WallLocation::Bottom));
    commands.spawn(walls::WallBundle::new(walls::WallLocation::Top));
}
