// Takes a normalized ray direction
fn ray_sphere_intersection(sphere_origin: vec3<f32>, sphere_radius: f32, ray_origin: vec3<f32>, ray_dir: vec3<f32>) -> vec2<f32> {
    let origin_dist = ray_origin - sphere_origin;
    let b = dot(origin_dist, ray_dir);
    let c = dot(origin_dist, origin_dist) - sphere_radius * sphere_radius;
    let discriminant = b * b - c;
    // Value of the discriminant determines if the ray intersects the sphere.
    // d < 0  -> none
    // d = 0  -> one - only occurs when the ray is tangent with the sphere
    // d > 0  -> two - when the ray origin is inside or outside the sphere
    if discriminant <= 0.0 { return vec2(0.0); }

    let s = sqrt(discriminant);
    let dist_near = (-b - s) / 2.0;
    let dist_far = (-b + s) / 2.0;
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
    @builtin(position) frag_coord: vec4<f32>,
    @builtin(sample_index) sample_index: u32,
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let depth = 0.1 / prepass_depth(frag_coord, sample_index);
    // let depth = min((1.0 / d) - 1.0, 10000000000000000000.0);
    let ray_origin = view.world_position.xyz;
    let ray_dir = normalize(world_position.xyz - ray_origin);

    let sphere_pos = mesh.model * vec4(0.0, 0.0, 0.0, 1.0);

    let sphere_dist = ray_sphere_intersection(sphere_pos.xyz, material.radius, ray_origin, ray_dir);
    let near = max(sphere_dist.x, 0.0);
    let far = min(sphere_dist.y, depth); // todo this multiplied const isn't right. Is it because inf reverse projection is nonlinear in world space? (it's linear in screen space) something to do with the w divide
    // yup, see https://learnopengl.com/Advanced-OpenGL/Depth-testing
    let atm_dist = far - near;
    
    let r = atm_dist / (material.radius * 2.0);
    return vec4(r, r, r, 1.0);

    // let l = depth / (material.radius * 4.0);
    // return vec4(l, l, l, 1.0);
}