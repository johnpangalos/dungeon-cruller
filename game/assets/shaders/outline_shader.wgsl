#import bevy_ui::ui_vertex_output UiVertexOutput

struct OutlineMaterial {
    outline_color: vec4<f32>,
    image_tint: vec4<f32>,
    thickness: f32
};
@group(1) @binding(0)
var<uniform> material: OutlineMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

fn get_sample(
    probe: vec2<f32>
) -> f32 {
    return textureSample(base_color_texture, base_color_sampler, probe).a;
}

@fragment
fn fragment(
    in: UiVertexOutput,
) -> @location(0) vec4<f32> {
    var uv = in.uv;
    var outline: f32 = get_sample(uv + vec2<f32>(material.thickness, 0.0));
    outline += get_sample(uv + vec2<f32>(-material.thickness, 0.0));
    outline += get_sample(uv + vec2<f32>(0.0, material.thickness));
    outline += get_sample(uv + vec2<f32>(0.0, -material.thickness));
    outline += get_sample(uv + vec2<f32>(material.thickness, -material.thickness));
    outline += get_sample(uv + vec2<f32>(-material.thickness, material.thickness));
    outline += get_sample(uv + vec2<f32>(material.thickness, material.thickness));
    outline += get_sample(uv + vec2<f32>(-material.thickness, -material.thickness));
    outline = min(outline, 1.0);
    var color: vec4<f32> = textureSample(base_color_texture, base_color_sampler, uv);
    return mix(color, material.outline_color, outline - color.a) - color * (vec4<f32>(1.0, 1.0, 1.0, 1.0) - material.image_tint);
}