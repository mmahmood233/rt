# CPU Ray Tracer

A simple but correct CPU ray tracer implemented in Rust that outputs PPM P3 ASCII images with realistic lighting and hard shadows.

## Features

- **Four Primitives**: Sphere, cube (AABB), flat plane, and finite cylinder with caps
- **Hard Shadows**: Realistic shadows via shadow rays with EPSILON bias to avoid self-intersection
- **Lambertian Shading**: Diffuse lighting with adjustable brightness
- **Movable Camera**: Adjustable position, target, FOV, and aspect ratio
- **PPM P3 Output**: ASCII format images, default 800×600 but configurable
- **Clean Architecture**: Modular Rust code with separate files for math, shapes, materials, etc.

## Quick Start

```bash
# Build the ray tracer
cargo build --release

# Render Scene 1 (single red sphere)
cargo run --release -- --scene 1 --width 800 --height 600 --output scene1.ppm

# Render Scene 2 (plane + cube, dimmer lighting)
cargo run --release -- --scene 2 --width 800 --height 600 --output scene2.ppm

# Render Scene 3 (all primitives)
cargo run --release -- --scene 3 --width 800 --height 600 --output scene3.ppm

# Render Scene 4 (same as Scene 3, different camera angle)
cargo run --release -- --scene 4 --width 800 --height 600 --output scene4.ppm
```

## Command Line Options

```
rt [OPTIONS]

Options:
  --width <WIDTH>          Image width in pixels [default: 800]
  --height <HEIGHT>        Image height in pixels [default: 600]
  --scene <SCENE>          Scene number (1-4) [default: 1]
  --brightness <BRIGHTNESS> Light intensity multiplier [default: 1.0]
  --fov <FOV>              Camera field of view in degrees [default: 45.0]
  --output <OUTPUT>        Output PPM file (stdout if not specified)
  --aa <AA>                Antialiasing samples per pixel (not implemented)
  --reflect                Enable reflection (not implemented)
  --mt                     Enable multithreading (not implemented)
  -h, --help               Print help
```

## Code Examples

### Creating Objects

```rust
use rt::math::Vec3;
use rt::material::Material;
use rt::shapes::{Sphere, Cube, Plane, Cylinder};

// Create a red sphere
let sphere = Sphere::new(
    Vec3::new(0.0, 0.0, -3.0),  // center
    1.0,                        // radius
    Material::red(),            // material
);

// Create a blue cube
let cube = Cube::new(
    Vec3::new(-1.0, -2.0, -4.0), // min corner
    Vec3::new(1.0, 0.0, -2.0),   // max corner
    Material::blue(),
);

// Create a horizontal plane (floor)
let plane = Plane::horizontal(-2.0, Material::gray());

// Create a green cylinder
let cylinder = Cylinder::new(
    Vec3::new(0.0, -2.0, -6.0), // center
    0.8,                        // radius
    2.0,                        // height
    Material::green(),
);
```

### Materials and Colors

```rust
use rt::material::Material;
use rt::math::Vec3;

// Predefined materials
let red_material = Material::red();
let green_material = Material::green();
let blue_material = Material::blue();
let white_material = Material::white();
let gray_material = Material::gray();

// Custom material with specific albedo (color)
let custom_material = Material::new(Vec3::new(0.8, 0.4, 0.2)); // Orange-ish

// Materials with specular highlights (for future Phong shading)
let shiny_material = Material::with_specular(
    Vec3::new(0.7, 0.7, 0.9), // albedo
    0.5,                      // specular coefficient
    32.0,                     // shininess
);
```

### Changing Brightness

```rust
use rt::scene::{Scene, Light};
use rt::math::Vec3;

let mut scene = Scene::new();

// Create a bright white light
let bright_light = Light::white_light(
    Vec3::new(2.0, 2.0, 0.0), // position
    2.0,                      // intensity (brightness)
);

// Create a dimmer light
let dim_light = Light::white_light(
    Vec3::new(2.0, 2.0, 0.0), // position
    0.5,                      // intensity (dimmer)
);

// Add lights to scene
scene.add_light(bright_light);

// Or use CLI to adjust brightness globally
// cargo run -- --scene 1 --brightness 2.0  # Twice as bright
// cargo run -- --scene 1 --brightness 0.5  # Half as bright
```

### Camera Positioning and Rotation

```rust
use rt::camera::Camera;
use rt::math::Vec3;

// Default camera (looking down -Z axis)
let camera = Camera::new(
    Vec3::new(0.0, 0.0, 0.0),   // look_from (camera position)
    Vec3::new(0.0, 0.0, -1.0),  // look_at (target point)
    Vec3::unit_y(),             // up vector (Y up)
    45.0,                       // fov (field of view in degrees)
    16.0 / 9.0,                 // aspect_ratio (width/height)
);

// Camera positioned to the side and above
let side_camera = Camera::new(
    Vec3::new(4.0, 2.0, -1.0),  // position: to the right and above
    Vec3::new(0.0, -1.0, -4.0), // target: looking at scene center
    Vec3::unit_y(),             // up vector
    36.0,                       // narrower field of view
    16.0 / 9.0,                 // aspect ratio
);

// Camera looking from behind and above
let overhead_camera = Camera::new(
    Vec3::new(0.0, 5.0, 2.0),   // position: high and behind
    Vec3::new(0.0, -2.0, -4.0), // target: looking down at scene
    Vec3::unit_y(),             // up vector
    60.0,                       // wider field of view
    4.0 / 3.0,                  // different aspect ratio
);
```

## Scene Descriptions

### Scene 1: Single Sphere
- **Objects**: One red sphere floating in space
- **Lighting**: Bright directional light creating clear shadows
- **Camera**: Standard front view
- **Purpose**: Demonstrates basic sphere intersection and shading

### Scene 2: Plane + Cube (Dimmer)
- **Objects**: Gray horizontal plane (floor) with blue cube
- **Lighting**: Dimmer lighting (50% of Scene 1 brightness)
- **Camera**: Same angle as Scene 1
- **Purpose**: Shows cube intersection, plane rendering, and brightness control

### Scene 3: All Primitives
- **Objects**: White plane, red sphere, green cube, blue cylinder
- **Lighting**: Standard brightness with realistic shadows
- **Camera**: Angled to show all objects clearly
- **Purpose**: Demonstrates all four primitive types working together

### Scene 4: Different Perspective
- **Objects**: Identical to Scene 3
- **Lighting**: Same as Scene 3
- **Camera**: Different position and angle for new perspective
- **Purpose**: Shows camera positioning and how viewpoint affects the scene

## Technical Implementation

### Ray Tracing Pipeline
1. **Ray Generation**: Camera generates rays through each pixel
2. **Intersection Testing**: Test ray against all scene objects
3. **Shading**: Calculate Lambertian diffuse lighting at hit points
4. **Shadow Rays**: Cast rays toward lights to check for occlusion
5. **Color Output**: Convert final color to RGB and write to PPM

### Coordinate System
- **Right-handed**: X right, Y up, Z forward (toward viewer)
- **Camera Default**: Positioned at origin, looking down -Z axis
- **World Units**: Arbitrary units, objects positioned in negative Z

### Intersection Algorithms
- **Sphere**: Analytic solution using quadratic formula
- **Plane**: Point-normal form intersection
- **Cube**: Slab method for axis-aligned bounding boxes (AABB)
- **Cylinder**: Finite cylinder with caps, quadratic + linear intersections

### Shadow Implementation
- **Hard Shadows**: Binary shadow test (fully lit or fully shadowed)
- **Shadow Rays**: Cast from hit points toward each light source
- **EPSILON Bias**: 1e-4 offset to prevent self-intersection artifacts
- **Ambient Light**: Small ambient term (10%) prevents completely black shadows

## Performance Notes

- **Single-threaded**: Clean, readable implementation
- **No Optimization**: Focus on correctness over speed
- **Memory Usage**: Minimal allocations, stack-based ray tracing
- **Render Times**: ~1-2 seconds for 800×600 on modern hardware

## File Structure

```
src/
├── main.rs           # CLI interface and scene definitions
├── math.rs           # Vec3, Ray, and mathematical operations
├── camera.rs         # Camera with adjustable position/FOV
├── material.rs       # Material properties and predefined colors
├── ppm.rs            # PPM P3 format writer
├── render.rs         # Ray tracing and shading logic
├── scene.rs          # Scene management and lighting
└── shapes/
    ├── mod.rs        # Shape traits and transforms
    ├── sphere.rs     # Sphere primitive
    ├── plane.rs      # Plane primitive
    ├── cube.rs       # Cube (AABB) primitive
    └── cylinder.rs   # Cylinder primitive
```

## Limitations

- **Single-threaded**: No parallel ray tracing
- **No Antialiasing**: Aliasing artifacts may be visible
- **No Reflection/Refraction**: Only diffuse materials
- **No Textures**: Solid colors only
- **No Acceleration**: Brute force intersection testing

## Future Enhancements

The codebase is designed for easy extension:

- **Multithreading**: Add Rayon for parallel pixel rendering
- **Antialiasing**: Implement supersampling with multiple rays per pixel
- **Reflection**: Add recursive ray tracing for mirrors
- **Textures**: UV mapping and procedural textures
- **Acceleration**: BVH or spatial partitioning for complex scenes

## Dependencies

- `clap`: Command-line argument parsing
- Standard library only (no external math or image libraries)

## License

This project is a learning implementation of ray tracing fundamentals.
