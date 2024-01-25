use bevy::prelude::*;

#[derive(Component)]
struct Player;

const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);
const PLAYER_SPEED: f32 = 500.0;

const WALL_THICKNESS: f32 = 10.0;
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const PLAYER_COLOR: Color = Color::rgb(0.0, 0.9, 0.0);
const WALL_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }
    fn size(&self) -> Vec2 {
        let areana_height = TOP_WALL - BOTTOM_WALL;
        let areana_width = RIGHT_WALL - LEFT_WALL;

        assert!(areana_height > 0.0);
        assert!(areana_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, areana_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(areana_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

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

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
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

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.x / 2.0;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.x / 2.0;
    let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.y / 2.0;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.y / 2.0;

    player_transform.translation.x = new_player_x_position.clamp(left_bound, right_bound);
    player_transform.translation.y = new_player_y_position.clamp(bottom_bound, top_bound);
}
