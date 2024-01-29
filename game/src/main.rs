mod constants;
mod doors;
mod materials;
mod player;
mod scenes;
mod walls;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use constants::{AppState, GameState};
use materials::ShaderPlugin;
use scenes::{DebugOverlay, MainMenu, PauseMenu, PlayerOverlay};
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
            DebugOverlay,
        ));
    }

    app.add_state::<AppState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<GameState>()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                mode: AssetMode::Processed,
                ..default()
            }),
            StylesPlugin,
            ShaderPlugin,
        ))
        .add_plugins((MainMenu, PauseMenu, PlayerOverlay))
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
            (player::move_player)
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
    asset_server: ResMut<AssetServer>,
) {
    let floor = asset_server.load::<Image>("textures/wooden-floor.png");

    commands.spawn(SpriteBundle {
        texture: floor,
        sprite: Sprite {
            custom_size: Some(Vec2::new(800., 600.)),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(player::PlayerBundle::new(Vec2::ZERO));

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
