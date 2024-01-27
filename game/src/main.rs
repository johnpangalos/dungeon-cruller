mod constants;
mod doors;
mod player;
mod scenes;
mod walls;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use constants::{AppState, GameState};
use scenes::{DebugPlugin, PausePlugin, SplashPlugin};
use styles::elements::StylesPlugin;

fn main() {
    let mut app = App::new();
    #[cfg(debug_assertions)] // debug/dev builds only
    {
        use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
        use bevy::diagnostic::LogDiagnosticsPlugin;
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            DebugPlugin,
        ));
    }

    app.add_state::<AppState>()
        .add_state::<GameState>()
        .add_plugins((DefaultPlugins, StylesPlugin))
        .add_plugins((SplashPlugin, PausePlugin))
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::SetupGame), setup_game)
        .add_systems(
            Update,
            (
                player::pause_game.run_if(in_state(GameState::Running)),
                player::unpause_game.run_if(in_state(GameState::Paused)),
            )
                .run_if(in_state(AppState::Game))
                .run_if(input_just_pressed(KeyCode::Escape)),
        )
        .add_systems(
            FixedUpdate,
            (
                player::move_player,
                doors::check_door_collisions,
                doors::print_collision,
            )
                .chain()
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)),
        )
        .add_event::<doors::CollisionEvent>();

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_game(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
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

    app_state.set(AppState::Game);
}
