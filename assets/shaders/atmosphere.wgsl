// Takes a normalized ray direction
fn ray_sphere_intersection(sphere_origin: vec3<f32>, sphere_radius: f32, ray_origin: vec3<f32>, ray_dir: vec3<f32>) -> vec2<f32> {
    let origin_vec = ray_origin - sphere_origin;
    let b = dot(origin_vec, ray_dir);
    let c = dot(origin_vec, origin_vec) - sphere_radius * sphere_radius;
    let discriminant = b * b - c;
    // Value of the discriminant determines if the ray intersects the sphere.
    // d < 0  -> none
    // d = 0  -> one - only occurs when the ray is tangent with the sphere
    // d > 0  -> two - when the ray origin is inside or outside the sphere
    //           when inside the sphere, one intersection will be behind the camera
    if discriminant <= 0.0 { return vec2(0.0); }

    let s = sqrt(discriminant);
    let dist_near = (-b - s);
    let dist_far = (-b + s);
    return vec2(dist_near, dist_far); // whereeeeeever you are
}

struct AtmosphereMaterial {
    color: vec4<f32>,
    radius: f32,
};

@group(1) @binding(0)
var<uniform> material: AtmosphereMaterial;

#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::utils

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    @builtin(sample_index) sample_index: u32,
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let ray_origin = view.world_position.xyz;
    let ray_dir = normalize(world_position.xyz - ray_origin);

    // Build the prepass depth into a point in ndc space
    let prepass_ndc_depth = prepass_depth(position, sample_index);
    let frag_uv = coords_to_viewport_uv(position.xy, view.viewport);
    let prepass_ndc_xy = frag_uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0);
    // Transform the point into clip space
    let prepass_clip = vec4<f32>(prepass_ndc_xy, prepass_ndc_depth, 1.0);
    // Transform the point into view space, note the perspective divide
    let view_undiv = view.inverse_projection * prepass_clip;
    let prepass_view = view_undiv / view_undiv.w;
    // Transform the point into world space to find the distance to the camera
    let prepass_world = view.inverse_view * prepass_view;
    let prepass_dist = length(prepass_world.xyz - ray_origin);
    // Note the following is approximately the same as the above block, however accuracy drops off
    // from the center, because depth (-z "distance") is not the same as distance from the camera.
    // let prepass_dist = 0.1 + 0.1 / prepass_depth(position, sample_index);

    let sphere_pos = mesh.model * vec4(0.0, 0.0, 0.0, 1.0);
    let intersect = ray_sphere_intersection(sphere_pos.xyz, material.radius, ray_origin, ray_dir);
    let sphere_near = intersect.x;
    let sphere_far = intersect.y;

    let near = max(sphere_near, 0.0);
    let far = min(sphere_far, prepass_dist);
    let atmosphere_depth = far - near;
    
    let d = atmosphere_depth / (material.radius * 2.0) * vec3<f32>(1.0);
    return vec4(d, 1.0);
}