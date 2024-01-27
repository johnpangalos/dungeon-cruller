use crate::constants::AppState;
use crate::player::Life;
use crate::player::Player;
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

fn update_life(
    player: Query<&Life, (With<Player>, Changed<Life>)>,
    mut query: Query<(&Heart, &mut UiImage)>,
) {
    if let Ok(Life(life)) = player.get_single() {
        for (heart, mut image) in &mut query {
            if life < &heart.0 {
                image.texture = heart.2.clone();
            } else {
                image.texture = heart.1.clone();
            }
        }
    }
}

#[derive(Component, Clone)]
struct Heart(u32, Handle<Image>, Handle<Image>);
render!(Heart, |heart, _| { img(cn!(h_16, w_16), heart.1.clone()) });

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let full = asset_server.load::<Image>("textures/heart.png");
    let missing = asset_server.load::<Image>("textures/heart-missing.png");

    let tree = div(
        cn!(flex),
        [
            Heart(1, full.clone(), missing.clone()).el(),
            Heart(2, full.clone(), missing.clone()).el(),
            Heart(3, full.clone(), missing.clone()).el(),
        ],
    );

    render_root(&mut commands, PlayerOverlay, tree);
}

fn despawn_recursively<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
