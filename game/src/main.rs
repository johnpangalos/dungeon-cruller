mod constants;
mod doors;
mod dungeon;
mod inventory;
mod materials;
mod player;
mod rooms;
mod scenes;

use bevy::{
    input::common_conditions::input_just_pressed, prelude::*, render::camera::ScalingMode,
    window::*,
};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use constants::{AppState, GameState};
use inventory::{ConsoleItem, Inventory, Item};
use materials::ShaderPlugin;
use scenes::{DebugOverlay, MainMenu, PauseMenu, PlayerOverlay};
use styles::elements::StylesPlugin;

fn main() {
    let mut app = App::new();
    #[cfg(debug_assertions)] // debug/dev builds only
    {
        use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
        use bevy::diagnostic::LogDiagnosticsPlugin;
        use bevy_rapier2d::render::RapierDebugRenderPlugin;
        app.add_plugins((
            RapierDebugRenderPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            DebugOverlay,
        ));
    }

    app.add_state::<AppState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<GameState>()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(constants::WIDTH, constants::HEIGHT),
                        ..default()
                    }),
                    ..default()
                }),
            RapierPhysicsPlugin::<NoUserData>::default(),
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
            (
                inventory::use_console_item,
                player::move_player,
                player::read_touching_door_system,
                player::use_item_player,
            )
                .chain()
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)),
        );

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: constants::WIDTH,
                min_height: constants::HEIGHT,
            },
            near: -1000.0,
            far: 1000.0,
            ..default()
        },
        ..default()
    });
}

fn setup_game(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    asset_server: ResMut<AssetServer>,
) {
    let floor = asset_server.load::<Image>("textures/wooden-floor.png");
    let player = asset_server.load::<Image>("textures/cat.png");

    let console_item = commands
        .spawn((ConsoleItem("yallo".to_string()), Item))
        .id();

    let mut player = commands.spawn(player::PlayerBundle::new(Vec2::ZERO, player));

    player.insert(Inventory::OneHanded(Some(console_item)));

    let entrance = dungeon::get_dungeon()[0];

    // Walls
    commands.spawn(rooms::WallBundle::new(rooms::WallLocation::TopRight));
    commands.spawn(rooms::WallBundle::new(rooms::WallLocation::TopLeft));
    commands.spawn(rooms::WallBundle::new(rooms::WallLocation::BottomRight));
    commands.spawn(rooms::WallBundle::new(rooms::WallLocation::BottomLeft));

    // Floor
    commands.spawn(SpriteBundle {
        texture: floor,
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        sprite: Sprite {
            custom_size: Some(Vec2::new(entrance.width, entrance.height)),
            ..Default::default()
        },
        ..Default::default()
    });

    // Doors
    commands.spawn(doors::DoorBundle::new(doors::Door::Left));
    commands.spawn(doors::DoorBundle::new(doors::Door::Right));
    commands.spawn(doors::DoorBundle::new(doors::Door::Bottom));
    commands.spawn(doors::DoorBundle::new(doors::Door::Top));

    app_state.set(AppState::Game);
}
