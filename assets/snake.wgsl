#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<storage> cells: array<u32>;
@group(2) @binding(1) var<uniform> size_x: u32;
@group(2) @binding(2) var<uniform> size_y: u32;

@group(2) @binding(3) var<uniform> bg_color: vec4<f32>;
@group(2) @binding(4) var<uniform> sn_color: vec4<f32>;
@group(2) @binding(5) var<uniform> fd_color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let id = u32(mesh.uv.x * f32(size_x)) + u32(mesh.uv.y * f32(size_y)) * size_x;
    if(cells[id] == 0)
    {
        return bg_color;
    }
    if(cells[id] == 1)
    {
        return sn_color;
    }
    return fd_color;
}