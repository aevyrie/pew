// struct AtmosphereMaterial {
//     color: vec4<f32>,
//     radius: f32,
// };

// @group(1) @binding(0)
// var<uniform> material: AtmosphereMaterial;

// #import bevy_pbr::mesh_types
// #import bevy_pbr::mesh_view_bindings
// #import bevy_pbr::utils

// @fragment
// fn fragment(
//     @builtin(position) frag_coord: vec4<f32>,
//     @builtin(sample_index) sample_index: u32,
//     #import bevy_pbr::mesh_vertex_output
// ) -> @location(0) vec4<f32> {
//     // let depth = prepass_depth(frag_coord, sample_index);
//     // return vec4(depth, depth, depth, 1.0);
//     return material.color;
// }


#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::utils

@fragment
fn fragment(
    @builtin(position) frag_coord: vec4<f32>,
    @builtin(sample_index) sample_index: u32,
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    // let depth = prepass_depth(frag_coord, sample_index);
    // return vec4(depth, depth, depth, 1.0);
    return vec4(1.0, 0.0, 0.0, 0.10);
}
