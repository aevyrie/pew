struct AtmosphereMaterial {
    color: vec4<f32>,
    radius: f32,
};

@group(1) @binding(0)
var<uniform> material: AtmosphereMaterial;

@fragment
fn fragment(
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    return material.color
}