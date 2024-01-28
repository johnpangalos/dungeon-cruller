use bevy::{
    asset::{Asset, Handle},
    reflect::TypePath,
    render::{
        color::Color,
        render_resource::{AsBindGroup, ShaderRef},
        texture::Image,
    },
    ui::UiMaterial,
};

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct OutlineMaterial {
    #[uniform(0)]
    pub outline_color: Color,
    #[uniform(0)]
    pub image_tint: Color,
    #[uniform(0)]
    pub thickness: f32,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
}

impl UiMaterial for OutlineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline-shader.wgsl".into()
    }
}
