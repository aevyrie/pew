let gas_constant: f32 = 8.3143;
// pow(1/(700, 530, 440), 4)
let scattering_coeff: vec3<f32> = vec3<f32>(0.41649313e-11, 1.2673499e-11, 2.6680213e-11);

// Uses ideal gas law to simulate atmospheric density and scattering
struct AtmosphereMaterial {
    sun_dir: vec3<f32>,
    surface_radius: f32, // m
    radius: f32, // m
    gravity: f32, // m/s^2
    surface_temperature: f32, // K
    surface_pressure: f32, // kPa
    molar_mass: f32, // kg/mol
};

@group(1) @binding(0)
var<uniform> material: AtmosphereMaterial;

#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::utils

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
    if discriminant <= 0.0 { return vec2(-1.0); }

    let s = sqrt(discriminant);
    let dist_near = (-b - s);
    let dist_far = (-b + s);
    return vec2(dist_near, dist_far); // whereeeeeever you are
}

// Input is altitude above the surface
fn density_above_surface(altitude: f32) -> f32 {
    let p_0 = material.surface_pressure;
    let t_0 = material.surface_temperature;
    let m_m = material.molar_mass;
    let g = material.gravity;

    let p = p_0 * exp(-(m_m * g)/(gas_constant * t_0) * max(altitude, 0.0));
    let density = (m_m * p)/(gas_constant * t_0);
    return density;
}

fn optical_depth(
    atmosphere_origin: vec3<f32>, 
    ray_pos: vec3<f32>, 
    ray_dir: vec3<f32>, 
    ray_length: f32, 
    samples: i32
) -> f32 {
    var optical_depth = 0.0;
    let sample_spacing = ray_length / f32(samples - 1);

    for ( var i: i32 = 0; i < samples; i++ ) {
        let sample_pos = ray_pos + (ray_dir * sample_spacing * f32(i));
        let altitude = distance(sample_pos, atmosphere_origin) - material.surface_radius;
        let density = density_above_surface(altitude);
        optical_depth += density * sample_spacing;
    }

    return optical_depth;
}

fn light_reaching_eye(
    atmosphere_origin: vec3<f32>, 
    eye_pos: vec3<f32>, 
    eye_dir: vec3<f32>,
    eye_atmosphere_depth: f32, // how much atmosphere between the eye and the edge of the atmosphere
    samples: i32
) -> vec4<f32> {
    var light_reaching_eye = vec3<f32>(0.0);
    let sample_spacing = eye_atmosphere_depth / f32(samples - 1);

    for ( var i: i32 = 0; i < samples; i++ ) {
        let sample_pos = eye_pos + (eye_dir * sample_spacing * f32(i));
        // Find the distance from the sample point to the sun through the atmosphere
        let sun_depth = ray_sphere_intersection(
            atmosphere_origin, 
            material.radius, 
            sample_pos, 
            material.sun_dir
        ).y;

        let sun_to_sample_optical_depth = optical_depth(atmosphere_origin, sample_pos, material.sun_dir, sun_depth, 15);
        let sample_to_eye_optical_depth = optical_depth(atmosphere_origin, eye_pos, eye_dir, sample_spacing * f32(i), 15);
        let transmittance = exp(
                -(sun_to_sample_optical_depth + sample_to_eye_optical_depth) 
                * normalize(scattering_coeff) * 0.01
            );

        let altitude = distance(sample_pos, atmosphere_origin) - material.surface_radius;
        let density = density_above_surface(altitude);

        // TODO: this should use the angle of the sun ray to the eye ray for proper mie scattering, using the "peanut" function.(8:45 in video)
        light_reaching_eye += density * transmittance * normalize(scattering_coeff) * 0.01 * sample_spacing;
    }
    return vec4<f32>(light_reaching_eye, length(light_reaching_eye));
}


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
    
    if atmosphere_depth > 10.0 {
        let first_point_in_atm = ray_origin + ray_dir * near;
        let light = light_reaching_eye(
            sphere_pos.xyz,
            first_point_in_atm, 
            ray_dir,
            atmosphere_depth,
            10
        );
        return vec4<f32>(light);
    }
    return vec4<f32>(0.0);
}