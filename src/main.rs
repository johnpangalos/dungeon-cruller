use bevy::prelude::*;

#[derive(Component)]
struct Player;

const PLAYER_COLOR: Color = Color::rgb(0.0, 0.9, 0.0);
const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);
const PLAYER_SPEED: f32 = 500.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_player).chain())
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
                color: PLAYER_COLOR,
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..default()
        },
        Player,
    ));
}

fn move_player(
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

    let new_player_x_position =
        player_transform.translation.x + x_direction * PLAYER_SPEED * time.delta_seconds();
    let new_player_y_position =
        player_transform.translation.y + y_direction * PLAYER_SPEED * time.delta_seconds();

    player_transform.translation.x = new_player_x_position;
    player_transform.translation.y = new_player_y_position;
}
