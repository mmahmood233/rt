use crate::math::{Vec3, Ray};
use crate::camera::Camera;
use crate::scene::Scene;
use crate::ppm::PpmWriter;

/// Ray tracer renderer
pub struct Renderer {
    pub max_depth: u32,
    pub epsilon: f64,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            max_depth: 10,
            epsilon: 1e-4,
        }
    }
    
    /// Render a scene to a PPM writer
    pub fn render(&self, scene: &Scene, camera: &Camera, width: u32, height: u32) -> PpmWriter {
        let mut writer = PpmWriter::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let u = x as f64 / width as f64;
                let v = (height - 1 - y) as f64 / height as f64; // Flip Y coordinate
                
                let ray = camera.get_ray(u, v);
                let color = self.trace_ray(&ray, scene, 0);
                
                // Convert color to RGB bytes
                let r = (255.0 * color.x.min(1.0).max(0.0)) as u8;
                let g = (255.0 * color.y.min(1.0).max(0.0)) as u8;
                let b = (255.0 * color.z.min(1.0).max(0.0)) as u8;
                
                writer.write_pixel(r, g, b);
            }
        }
        
        writer
    }
    
    /// Trace a ray through the scene
    fn trace_ray(&self, ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
        if depth >= self.max_depth {
            return Vec3::zero();
        }
        
        if let Some(hit) = scene.intersect(ray) {
            // Lambertian shading with hard shadows
            let mut color = Vec3::zero();
            
            for light in &scene.lights {
                let light_dir = (light.position - hit.point).normalize();
                let light_distance = (light.position - hit.point).length();
                let light_intensity = hit.normal.dot(&light_dir).max(0.0);
                
                // Only add light contribution if surface faces the light
                if light_intensity > 0.0 {
                    // Cast shadow ray to check for occlusion
                    let shadow_ray_origin = hit.point + hit.normal * self.epsilon; // Bias to avoid self-intersection
                    let shadow_ray = Ray::new(shadow_ray_origin, light_dir);
                    
                    let mut in_shadow = false;
                    
                    // Check if shadow ray hits any object before reaching the light
                    if let Some(shadow_hit) = scene.intersect(&shadow_ray) {
                        // If we hit something closer than the light, we're in shadow
                        if shadow_hit.t < light_distance - self.epsilon {
                            in_shadow = true;
                        }
                    }
                    
                    // Only add light contribution if not in shadow
                    if !in_shadow {
                        let light_contribution = Vec3::new(
                            hit.material.albedo.x * light.color.x,
                            hit.material.albedo.y * light.color.y,
                            hit.material.albedo.z * light.color.z,
                        ) * light.intensity * light_intensity;
                        color = color + light_contribution;
                    }
                }
            }
            
            // Add small ambient light to prevent completely black shadows
            let ambient = Vec3::new(
                hit.material.albedo.x * 0.1,
                hit.material.albedo.y * 0.1,
                hit.material.albedo.z * 0.1,
            );
            color = color + ambient;
            
            color
        } else {
            scene.background_color
        }
    }
}
