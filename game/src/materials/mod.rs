use bevy::{
    app::{App, Plugin},
    ui::UiMaterialPlugin,
};

use self::outline_material::OutlineMaterial;

pub mod outline_material;

pub struct ShaderPlugin;
impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        // Add more shaders here
        app.add_plugins(UiMaterialPlugin::<OutlineMaterial>::default());
    }
}
