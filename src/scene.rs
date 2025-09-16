use crate::math::{Vec3, Ray};
use crate::material::Material;
use crate::shapes::{HitInfo, Intersectable};

/// Light source for illumination
#[derive(Debug, Clone)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
    pub color: Vec3,
}

impl Light {
    pub fn new(position: Vec3, intensity: f64, color: Vec3) -> Self {
        Self { position, intensity, color }
    }
    
    pub fn white_light(position: Vec3, intensity: f64) -> Self {
        Self::new(position, intensity, Vec3::new(1.0, 1.0, 1.0))
    }
}

/// Scene containing objects and lights
pub struct Scene {
    pub objects: Vec<Box<dyn Intersectable>>,
    pub lights: Vec<Light>,
    pub background_color: Vec3,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            background_color: Vec3::new(0.2, 0.3, 0.5), // Sky blue background
        }
    }
    
    pub fn add_object(&mut self, object: Box<dyn Intersectable>) {
        self.objects.push(object);
    }
    
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
    
    /// Find closest intersection with any object in the scene
    pub fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        let mut closest_hit = None;
        let mut closest_t = f64::INFINITY;
        
        for object in &self.objects {
            if let Some(hit) = object.intersect(ray) {
                if hit.t < closest_t {
                    closest_t = hit.t;
                    closest_hit = Some(hit);
                }
            }
        }
        
        closest_hit
    }
}
