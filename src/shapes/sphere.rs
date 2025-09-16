use crate::math::{Vec3, Ray};
use crate::material::Material;
use super::{HitInfo, Intersectable, Transform};

/// Sphere primitive
#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
    pub transform: Transform,
}

impl Sphere {
    /// Create a new sphere
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
            transform: Transform::new(),
        }
    }
    
    /// Create a sphere with transform
    pub fn with_transform(center: Vec3, radius: f64, material: Material, transform: Transform) -> Self {
        Self {
            center,
            radius,
            material,
            transform,
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        // Transform ray to object space if needed
        let local_ray = if self.transform.translation == Vec3::zero() && self.transform.scale == Vec3::new(1.0, 1.0, 1.0) {
            ray.clone()
        } else {
            self.transform.inverse_transform_ray(ray)
        };
        
        // Ray-sphere intersection using quadratic formula
        // Ray: P(t) = origin + t * direction
        // Sphere: |P - center|² = radius²
        // Substitute: |origin + t * direction - center|² = radius²
        
        let oc = local_ray.origin - self.center;
        let a = local_ray.direction.dot(&local_ray.direction);
        let b = 2.0 * oc.dot(&local_ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return None; // No intersection
        }
        
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);
        
        // Choose the closest positive intersection
        let t = if t1 > 1e-4 {
            t1
        } else if t2 > 1e-4 {
            t2
        } else {
            return None; // Both intersections behind ray origin
        };
        
        let hit_point = local_ray.at(t);
        let normal = (hit_point - self.center).normalize();
        
        // Transform back to world space if needed
        let world_hit_point = if self.transform.translation == Vec3::zero() && self.transform.scale == Vec3::new(1.0, 1.0, 1.0) {
            hit_point
        } else {
            self.transform.apply_to_point(hit_point)
        };
        
        Some(HitInfo {
            t,
            point: world_hit_point,
            normal,
            material: self.material.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sphere_intersection() {
        let sphere = Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Material::red(),
        );
        
        // Ray pointing directly at sphere center
        let ray = Ray::new(Vec3::zero(), Vec3::new(0.0, 0.0, -1.0));
        let hit = sphere.intersect(&ray);
        
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!((hit.t - 0.5).abs() < 1e-10);
        assert_eq!(hit.point, Vec3::new(0.0, 0.0, -0.5));
    }
    
    #[test]
    fn test_sphere_no_intersection() {
        let sphere = Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Material::red(),
        );
        
        // Ray pointing away from sphere
        let ray = Ray::new(Vec3::zero(), Vec3::new(0.0, 0.0, 1.0));
        let hit = sphere.intersect(&ray);
        
        assert!(hit.is_none());
    }
}
