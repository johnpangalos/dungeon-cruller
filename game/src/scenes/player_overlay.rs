use crate::constants::AppState;
use bevy::app::AppExit;
use bevy::ecs::system::Insert;
use bevy::prelude::*;

use styles::elements::*;
use styles::stylesheet::*;
use styles::*;

#[derive(Component)]

pub struct PlayerOverlay;

impl Plugin for PlayerOverlay {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup)
            .add_systems(Update, (update_life).run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_recursively::<PlayerOverlay>);
    }
}

fn update_life() {
    ()
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let heart_image = asset_server.load::<Image>("textures/heart.png");

    let tree = div(
        cn!(flex),
        [
            img(cn!(h_16, w_16), heart_image.clone()),
            img(cn!(h_16, w_16), heart_image.clone()),
            img(cn!(h_16, w_16), heart_image.clone()),
        ],
    );

    render_root(&mut commands, PlayerOverlay, tree);
}

fn despawn_recursively<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
