use crate::math::{Vec3, Ray};
use crate::material::Material;
use super::{HitInfo, Intersectable, Transform};

/// Finite cylinder primitive (along Y axis)
#[derive(Debug, Clone)]
pub struct Cylinder {
    pub center: Vec3,     // Center of the cylinder
    pub radius: f64,      // Radius
    pub height: f64,      // Height along Y axis
    pub material: Material,
    pub transform: Transform,
}

impl Cylinder {
    /// Create a new cylinder
    pub fn new(center: Vec3, radius: f64, height: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            height,
            material,
            transform: Transform::new(),
        }
    }
}

impl Intersectable for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        // Cylinder intersection (infinite cylinder + caps)
        // Cylinder equation: (x - cx)² + (z - cz)² = r²
        // Ray: P(t) = origin + t * direction
        
        let oc = ray.origin - self.center;
        
        // Solve quadratic for infinite cylinder (ignoring Y)
        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        let b = 2.0 * (oc.x * ray.direction.x + oc.z * ray.direction.z);
        let c = oc.x * oc.x + oc.z * oc.z - self.radius * self.radius;
        
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return None; // No intersection with infinite cylinder
        }
        
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);
        
        let mut closest_t = None;
        let mut closest_normal = Vec3::zero();
        
        // Check both intersections with cylinder walls
        for &t in &[t1, t2] {
            if t > 1e-4 {
                let hit_point = ray.at(t);
                let y = hit_point.y;
                
                // Check if hit is within cylinder height
                if y >= self.center.y - self.height / 2.0 && y <= self.center.y + self.height / 2.0 {
                    if closest_t.is_none() || t < closest_t.unwrap() {
                        closest_t = Some(t);
                        // Normal for cylinder wall
                        let normal_x = (hit_point.x - self.center.x) / self.radius;
                        let normal_z = (hit_point.z - self.center.z) / self.radius;
                        closest_normal = Vec3::new(normal_x, 0.0, normal_z).normalize();
                    }
                }
            }
        }
        
        // Check intersection with top and bottom caps
        let cap_y_top = self.center.y + self.height / 2.0;
        let cap_y_bottom = self.center.y - self.height / 2.0;
        
        for &cap_y in &[cap_y_top, cap_y_bottom] {
            if ray.direction.y.abs() > 1e-6 {
                let t = (cap_y - ray.origin.y) / ray.direction.y;
                if t > 1e-4 {
                    let hit_point = ray.at(t);
                    let dx = hit_point.x - self.center.x;
                    let dz = hit_point.z - self.center.z;
                    
                    // Check if hit is within cap radius
                    if dx * dx + dz * dz <= self.radius * self.radius {
                        if closest_t.is_none() || t < closest_t.unwrap() {
                            closest_t = Some(t);
                            // Normal for cap
                            closest_normal = if cap_y == cap_y_top {
                                Vec3::unit_y()
                            } else {
                                -Vec3::unit_y()
                            };
                        }
                    }
                }
            }
        }
        
        if let Some(t) = closest_t {
            let hit_point = ray.at(t);
            Some(HitInfo {
                t,
                point: hit_point,
                normal: closest_normal,
                material: self.material.clone(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cylinder_intersection() {
        let cylinder = Cylinder::new(
            Vec3::new(0.0, 0.0, -2.0),
            0.5,
            2.0,
            Material::green(),
        );
        
        // Ray pointing at cylinder center
        let ray = Ray::new(Vec3::zero(), Vec3::new(0.0, 0.0, -1.0));
        let hit = cylinder.intersect(&ray);
        
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!(hit.t > 0.0);
    }
}
