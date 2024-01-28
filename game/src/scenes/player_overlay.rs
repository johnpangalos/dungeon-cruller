use crate::constants::AppState;
use crate::player::Life;
use crate::player::Player;
use bevy::prelude::*;

use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use styles::elements::*;
use styles::stylesheet::*;
use styles::*;

#[derive(Component)]

pub struct PlayerOverlay;

impl Plugin for PlayerOverlay {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup)
            .add_plugins(UiMaterialPlugin::<CustomMaterial>::default())
            .add_plugins(UiMaterialPlugin::<OutlineMaterial>::default())
            .add_systems(Update, (update_life).run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_recursively::<PlayerOverlay>);
    }
}

fn update_life(
    player: Query<&Life, (With<Player>, Changed<Life>)>,
    hearts: Query<(&Heart, &Handle<OutlineMaterial>), With<Handle<OutlineMaterial>>>,
    mut ui_materials: ResMut<Assets<OutlineMaterial>>,
) {
    if let Ok(Life(life)) = player.get_single() {
        for (heart, handle) in &hearts {
            let material = ui_materials.get_mut(handle).unwrap();
            if life < &heart.0 {
                material.image_tint = Color::BLACK;
            } else {
                material.image_tint = Color::WHITE;
            }
        }
    }
}

#[derive(Component, Clone, Debug)]
struct Heart(u32);

fn mat_heart<T: UiMaterial>(n: u32, material: Handle<T>) -> Element {
    Heart(n).as_el(mat(cn!(h_16, w_16), material.clone()))
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<OutlineMaterial>>,
) {
    let full = asset_server.load::<Image>("textures/heart.png");

    // let base = CustomMaterial {
    //     color: Color::WHITE,
    //     color_texture: full.clone(),
    // };

    let base = OutlineMaterial {
        outline_color: Color::WHITE,
        image_tint: Color::WHITE,
        thickness: 0.04,
        color_texture: full.clone(),
    };

    let heart_1 = materials.add(base.clone());
    let heart_2 = materials.add(base.clone());
    let heart_3 = materials.add(base.clone());

    let tree = div(
        cn!(flex),
        [
            mat_heart(1, heart_1),
            mat_heart(2, heart_2),
            mat_heart(3, heart_3),
        ],
    );

    spawn_root_element(&mut commands, PlayerOverlay, tree);
}

fn despawn_recursively<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct OutlineMaterial {
    #[uniform(0)]
    outline_color: Color,
    #[uniform(0)]
    image_tint: Color,
    #[uniform(0)]
    thickness: f32,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

impl UiMaterial for OutlineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline_shader.wgsl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

impl UiMaterial for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/heart_shader.wgsl".into()
    }
}
