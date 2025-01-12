use bevy::{
    asset::{Asset, Handle},
    prelude::Image,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct CustomMaterial2d {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material2d for CustomMaterial2d {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        bevy::render::render_resource::ShaderRef::Default
    }
}
