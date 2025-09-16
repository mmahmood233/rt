use crate::math::{Vec3, Ray};
use crate::material::Material;
use super::{HitInfo, Intersectable, Transform};

/// Axis-aligned bounding box (cube) primitive
#[derive(Debug, Clone)]
pub struct Cube {
    pub min: Vec3,        // Minimum corner
    pub max: Vec3,        // Maximum corner
    pub material: Material,
    pub transform: Transform,
}

impl Cube {
    /// Create a new cube from min and max corners
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Self {
        Self {
            min,
            max,
            material,
            transform: Transform::new(),
        }
    }
    
    /// Create a unit cube centered at origin
    pub fn unit(material: Material) -> Self {
        Self::new(
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(0.5, 0.5, 0.5),
            material,
        )
    }
}

impl Intersectable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        // Slab method for AABB intersection
        let mut t_min = f64::NEG_INFINITY;
        let mut t_max = f64::INFINITY;
        let mut normal = Vec3::zero();
        let mut hit_min_face = true; // Track if we hit the min face or max face
        
        // Check intersection with each pair of parallel planes (X, Y, Z slabs)
        for i in 0..3 {
            let (axis_origin, axis_direction, axis_min, axis_max) = match i {
                0 => (ray.origin.x, ray.direction.x, self.min.x, self.max.x),
                1 => (ray.origin.y, ray.direction.y, self.min.y, self.max.y),
                _ => (ray.origin.z, ray.direction.z, self.min.z, self.max.z),
            };
            
            if axis_direction.abs() < 1e-6 {
                // Ray is parallel to the slab
                if axis_origin < axis_min || axis_origin > axis_max {
                    return None; // Ray misses the slab entirely
                }
            } else {
                let t1 = (axis_min - axis_origin) / axis_direction;
                let t2 = (axis_max - axis_origin) / axis_direction;
                
                let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };
                
                if t_near > t_min {
                    t_min = t_near;
                    hit_min_face = t1 < t2; // True if we hit the min face
                    
                    // Set normal based on which face and axis we hit
                    normal = match i {
                        0 => if hit_min_face { Vec3::new(-1.0, 0.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) },
                        1 => if hit_min_face { Vec3::new(0.0, -1.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) },
                        _ => if hit_min_face { Vec3::new(0.0, 0.0, -1.0) } else { Vec3::new(0.0, 0.0, 1.0) },
                    };
                }
                
                if t_far < t_max {
                    t_max = t_far;
                }
                
                // Early exit if slabs don't overlap
                if t_min > t_max {
                    return None;
                }
            }
        }
        
        // Choose the closest positive intersection
        let t = if t_min > 1e-4 {
            t_min
        } else if t_max > 1e-4 {
            t_max
            // If we're using t_max, we need to recalculate the normal
            // This happens when the ray starts inside the box
        } else {
            return None; // Both intersections behind ray origin
        };
        
        let hit_point = ray.at(t);
        
        Some(HitInfo {
            t,
            point: hit_point,
            normal,
            material: self.material.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cube_intersection() {
        let cube = Cube::unit(Material::blue());
        
        // Ray pointing at cube center
        let ray = Ray::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = cube.intersect(&ray);
        
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!((hit.t - 0.5).abs() < 1e-10);
        assert_eq!(hit.point, Vec3::new(0.0, 0.0, 0.5));
    }
}
