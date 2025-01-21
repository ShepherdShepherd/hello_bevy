use bevy::{asset::Asset, color::LinearRgba, pbr::Material, reflect::TypePath, render::render_resource::{AsBindGroup,  ShaderRef}};

#[derive(Asset,TypePath,AsBindGroup,Clone)]
pub struct DisplayMaterial
{
    #[storage(0,read_only)]
    pub cells:Vec<u32>,
    #[uniform(1)]
    pub size_x:u32,
    #[uniform(2)]
    pub size_y:u32,

    #[uniform(3)]
    pub background_color: LinearRgba,
    #[uniform(4)]
    pub snake_color: LinearRgba,
    #[uniform(5)]
    pub food_color: LinearRgba
}

impl Material for  DisplayMaterial {
    fn fragment_shader() -> ShaderRef {
        "snake.wgsl".into()
    }
}