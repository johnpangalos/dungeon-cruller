mod constants;
mod doors;
mod dungeon;
mod input;
mod inventory;
mod items;
mod materials;
mod player;
mod rooms;
mod scenes;

use bevy::{prelude::*, render::camera::ScalingMode, window::*};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use constants::{AppState, GameState};
use inventory::Inventory;
use items::{components::Item, ConsoleItem, ItemsPlugin};
use materials::ShaderPlugin;
use player::PlayerPlugin;
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

    let default_plugin = DefaultPlugins
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
        });

    app.add_state::<AppState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<GameState>()
        .add_plugins((
            default_plugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            StylesPlugin,
            ShaderPlugin,
        ))
        .add_plugins((
            MainMenu,
            PauseMenu,
            PlayerOverlay,
            ItemsPlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::SetupGame), setup_game);

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
    let player = asset_server.load::<Image>("textures/cat.png");

    let console_item = commands
        .spawn((Item, ConsoleItem("yallo".to_string())))
        .id();

    let mut player = commands.spawn(player::PlayerBundle::new(Vec2::ZERO, player));

    player.insert(Inventory::OneHanded(Some(console_item)));

    let d = dungeon::Dungeon::new();

    d.layout[0].spawn(&mut commands, asset_server);

    app_state.set(AppState::Game);
}
