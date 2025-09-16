use clap::Parser;
use std::io;

mod math;
mod ppm;
mod camera;
mod material;
mod shapes;
mod scene;
mod render;

use math::Vec3;
use camera::Camera;
use material::Material;
use shapes::{Sphere, Plane, Cube, Cylinder};
use scene::{Scene, Light};
use render::Renderer;
use ppm::PpmWriter;

#[derive(Parser)]
#[command(name = "rt")]
#[command(about = "A CPU ray tracer that outputs PPM images")]
struct Args {
    #[arg(long, default_value_t = 800)]
    width: u32,
    
    #[arg(long, default_value_t = 600)]
    height: u32,
    
    #[arg(long, default_value_t = 1)]
    scene: u32,
    
    #[arg(long, default_value_t = 1.0)]
    brightness: f64,
    
    #[arg(long, default_value_t = 45.0)]
    fov: f64,
    
    #[arg(long)]
    output: Option<String>,
    
    #[arg(long)]
    aa: Option<u32>,
    
    #[arg(long)]
    reflect: bool,
    
    #[arg(long)]
    mt: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    // Create scene based on scene number
    let mut scene = Scene::new();
    let camera;
    
    match args.scene {
        1 => {
            // Scene 1: Single sphere
            scene.add_object(Box::new(Sphere::new(
                Vec3::new(0.0, 0.0, -3.0),
                1.0,
                Material::red(),
            )));
            
            scene.add_light(Light::white_light(
                Vec3::new(2.0, 2.0, 0.0),
                args.brightness,
            ));
            
            camera = Camera::new(
                Vec3::new(0.0, 0.0, 0.0),   // look_from
                Vec3::new(0.0, 0.0, -1.0),  // look_at
                Vec3::unit_y(),             // up
                args.fov,                   // fov
                args.width as f64 / args.height as f64, // aspect_ratio
            );
        }
        2 => {
            // Scene 2: Plane + cube (dimmer)
            scene.add_object(Box::new(Plane::horizontal(-2.0, Material::gray())));
            scene.add_object(Box::new(Cube::new(
                Vec3::new(-1.0, -2.0, -4.0), // min corner
                Vec3::new(1.0, 0.0, -2.0),   // max corner
                Material::blue(),
            )));
            
            scene.add_light(Light::white_light(
                Vec3::new(2.0, 2.0, 0.0),
                args.brightness * 0.5, // Dimmer than scene 1
            ));
            
            camera = Camera::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::unit_y(),
                args.fov,
                args.width as f64 / args.height as f64,
            );
        }
        3 => {
            // Scene 3: All primitives (sphere, cube, plane, cylinder)
            scene.add_object(Box::new(Plane::horizontal(-3.0, Material::white())));
            
            scene.add_object(Box::new(Sphere::new(
                Vec3::new(-2.0, -1.0, -4.0),
                1.0,
                Material::red(),
            )));
            
            scene.add_object(Box::new(Cube::new(
                Vec3::new(0.5, -3.0, -4.5),
                Vec3::new(2.5, -1.0, -2.5),
                Material::green(),
            )));
            
            scene.add_object(Box::new(Cylinder::new(
                Vec3::new(0.0, -2.0, -6.0),
                0.8,
                2.0,
                Material::blue(),
            )));
            
            scene.add_light(Light::white_light(
                Vec3::new(3.0, 3.0, -1.0),
                args.brightness,
            ));
            
            camera = Camera::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, -1.0, -4.0),
                Vec3::unit_y(),
                args.fov,
                args.width as f64 / args.height as f64,
            );
        }
        4 => {
            // Scene 4: Same as Scene 3 but different camera position/angle
            scene.add_object(Box::new(Plane::horizontal(-3.0, Material::white())));
            
            scene.add_object(Box::new(Sphere::new(
                Vec3::new(-2.0, -1.0, -4.0),
                1.0,
                Material::red(),
            )));
            
            scene.add_object(Box::new(Cube::new(
                Vec3::new(0.5, -3.0, -4.5),
                Vec3::new(2.5, -1.0, -2.5),
                Material::green(),
            )));
            
            scene.add_object(Box::new(Cylinder::new(
                Vec3::new(0.0, -2.0, -6.0),
                0.8,
                2.0,
                Material::blue(),
            )));
            
            scene.add_light(Light::white_light(
                Vec3::new(3.0, 3.0, -1.0),
                args.brightness,
            ));
            
            // Different camera position and angle for Scene 4
            camera = Camera::new(
                Vec3::new(4.0, 2.0, -1.0),  // Different position
                Vec3::new(0.0, -1.0, -4.0), // Same target
                Vec3::unit_y(),
                args.fov * 0.8, // Slightly different FOV
                args.width as f64 / args.height as f64,
            );
        }
        _ => {
            // Default to scene 1
            scene.add_object(Box::new(Sphere::new(
                Vec3::new(0.0, 0.0, -3.0),
                1.0,
                Material::red(),
            )));
            
            scene.add_light(Light::white_light(
                Vec3::new(2.0, 2.0, 0.0),
                args.brightness,
            ));
            
            camera = Camera::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::unit_y(),
                args.fov,
                args.width as f64 / args.height as f64,
            );
        }
    }
    
    // Render the scene
    let renderer = Renderer::new();
    let writer = renderer.render(&scene, &camera, args.width, args.height);
    
    // Output to stdout or file
    match args.output {
        Some(filename) => {
            std::fs::write(filename, writer.to_string())?;
        }
        None => {
            print!("{}", writer.to_string());
        }
    }
    
    Ok(())
}
