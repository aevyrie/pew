# Production Ready Sky and Atmosphere Rendering

Multiple scatter is important to prevent a yellowish atmosphere. This is also what makes clouds look white and puffy, single scattering would result in clouds that look like smoke.

Single scattering: light interacts with media, scattering toward camera
Multiple scattering: light interacts with media, scatters, interacts with more media, then scatters toward camera

Light can scatter thousands of times withing a cloud or the atmosphere before reaching the camera

## Planet atmosphere:
 - Rayleigh scattering
 - Mie scattering and absorption
 - Ozone absorption

## Insights
- If the media in the atmosphere is low frequency (e.g. just air with no clouds), the distant sky is low frequency
- Mie scattering (the light blob around the sun) is smooth and low frequency
- High frequency changes occur at the horizon (sudden color transitions in sunsets) and in volumetric shadows
- Multiple scattering is important for believable results

## Contributions
- New LUT (look up table) rendering of distant sky and aerial perspective (between camera and landscape)
  - Atmosphere LUTs are not new, but previous work had high dimensional LUTs (3d/4d) whereas this is 2D.
- LUTs can be low res, and maintain high frequency features
- Multiple scattering LUT!
- Decouples atmosphere rendering from screen resolution!
  - Perf depends on LUT resolution and raymarching sample count

## How to render the atmosphere?
- Volumetric raymarching, with the sample count adjusted based on distance
- For each sample evaluate
  - Single Scattering: 
    - light transmitted through the atmosphere from sun to sample point
    - the Mie/Rayleigh phase function: how much light scatters toward the camera based on the angle between sun and camera at the sampel point
    - we then know the amount of light scattered toward the camera and transmittance according to the medium
  - Multiple Scattering
    - Normally too expensive, as we need to repeat this process recursively on rays from the sample point
    - The "order" of multiple scattering is the depth of the recursion (how many levels of branches from the "tree" of sample rays)
    - Use a new LUT to approximate

### Transmittance LUT
- Same as [Bruneton 08]
- Stores colored transmittance from a point in the atmosphere to outer space
- Optimized for views from the ground
  - Single transmittance value from the top of the atmosphere to the ground
  - *For space views, need to use per-pixel transmittance!*
  - This will result in a more accurate terminator
  - Allows planet to cast shadows onto moons

### Sky-View LUT
- New in this paper, used for the *distant* sky
- Latitude-longitude mapping, which is then mapped non-linearly in the texture to assign more texture detail at the horizon where there is more high-frequency detail without interpolation artifacts
- Can be used to store the contribution for any number of suns!

### Aerial Perspective LUT
- Froxel volume texture mapped onto the camera frustum [Wronski 14, Hillaire 16]
- RGB stores luminance reaching the camera from the camera to froxel center
- Applied onto opaque and translucent surfaces

### Multiple Scattering LUT
- Approximates an infinite amount of scattering orders! i.e. approximates an offline reference path tracer result
- 2D LUT, stores the isotropic multiple scattering contribution, altitude vs. sun zenith angle

### Compositing the scene
- Terrain
- Apply sky-view LUT for distant terrain (behind terrain)
- Add the sun disk
- Add the aerial perspective LUT to add distance fog to objects in scene
- Add multiple scattering LUT result
