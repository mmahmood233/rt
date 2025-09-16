use crate::math::{Vec3, Ray};
use crate::material::Material;
use super::{HitInfo, Intersectable, Transform};

/// Infinite plane primitive
#[derive(Debug, Clone)]
pub struct Plane {
    pub point: Vec3,      // Point on the plane
    pub normal: Vec3,     // Plane normal (should be normalized)
    pub material: Material,
    pub transform: Transform,
}

impl Plane {
    /// Create a new plane from point and normal
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Self {
            point,
            normal: normal.normalize(),
            material,
            transform: Transform::new(),
        }
    }
    
    /// Create a horizontal plane at given Y coordinate
    pub fn horizontal(y: f64, material: Material) -> Self {
        Self::new(Vec3::new(0.0, y, 0.0), Vec3::unit_y(), material)
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        // Ray-plane intersection
        // Plane equation: (P - point) · normal = 0
        // Ray: P(t) = origin + t * direction
        // Substitute: (origin + t * direction - point) · normal = 0
        // Solve for t: t = (point - origin) · normal / (direction · normal)
        
        let denom = ray.direction.dot(&self.normal);
        
        // Check if ray is parallel to plane
        if denom.abs() < 1e-6 {
            return None;
        }
        
        let t = (self.point - ray.origin).dot(&self.normal) / denom;
        
        // Check if intersection is behind ray origin
        if t < 1e-4 {
            return None;
        }
        
        let hit_point = ray.at(t);
        
        Some(HitInfo {
            t,
            point: hit_point,
            normal: self.normal,
            material: self.material.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plane_intersection() {
        let plane = Plane::horizontal(-1.0, Material::gray());
        
        // Ray pointing down at the plane
        let ray = Ray::new(Vec3::zero(), Vec3::new(0.0, -1.0, 0.0));
        let hit = plane.intersect(&ray);
        
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!((hit.t - 1.0).abs() < 1e-10);
        assert_eq!(hit.point, Vec3::new(0.0, -1.0, 0.0));
    }
}
