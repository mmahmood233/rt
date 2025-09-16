pub mod sphere;
pub mod plane;
pub mod cube;
pub mod cylinder;

use crate::math::{Vec3, Ray};
use crate::material::Material;

/// Hit information for ray-object intersections
#[derive(Debug, Clone)]
pub struct HitInfo {
    pub t: f64,           // Ray parameter at hit point
    pub point: Vec3,      // Hit point in world space
    pub normal: Vec3,     // Surface normal at hit point
    pub material: Material, // Material at hit point
}

/// Trait for objects that can be intersected by rays
pub trait Intersectable {
    /// Test ray intersection, return closest hit if any
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>;
}

/// Transform matrix for positioning/rotating/scaling objects
#[derive(Debug, Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Vec3,    // Euler angles in radians
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            translation: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
    
    pub fn with_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
    
    /// Apply transform to a point
    pub fn apply_to_point(&self, point: Vec3) -> Vec3 {
        // For now, just apply translation and uniform scale
        // TODO: Add proper rotation matrix support
        Vec3::new(
            point.x * self.scale.x + self.translation.x,
            point.y * self.scale.y + self.translation.y,
            point.z * self.scale.z + self.translation.z,
        )
    }
    
    /// Apply inverse transform to a ray (for object-space intersection)
    pub fn inverse_transform_ray(&self, ray: &Ray) -> Ray {
        // For now, just handle translation and uniform scale
        // TODO: Add proper inverse matrix support
        let inv_scale = Vec3::new(1.0 / self.scale.x, 1.0 / self.scale.y, 1.0 / self.scale.z);
        Ray::new(
            Vec3::new(
                (ray.origin.x - self.translation.x) * inv_scale.x,
                (ray.origin.y - self.translation.y) * inv_scale.y,
                (ray.origin.z - self.translation.z) * inv_scale.z,
            ),
            Vec3::new(
                ray.direction.x * inv_scale.x,
                ray.direction.y * inv_scale.y,
                ray.direction.z * inv_scale.z,
            ),
        )
    }
}

pub use sphere::Sphere;
pub use plane::Plane;
pub use cube::Cube;
pub use cylinder::Cylinder;
