use crate::math::{Vec3, Ray};

/// Pinhole camera with adjustable position, target, and field of view
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    /// Create a new camera
    /// - look_from: camera position
    /// - look_at: point camera is looking at
    /// - up: up vector (usually Vec3::unit_y())
    /// - fov: vertical field of view in degrees
    /// - aspect_ratio: width / height
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        
        let w = (look_from - look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);
        
        let origin = look_from;
        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;
        
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    
    /// Get ray for given screen coordinates (u, v in [0, 1])
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_camera_creation() {
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.0),  // look_from
            Vec3::new(0.0, 0.0, -1.0), // look_at
            Vec3::unit_y(),            // up
            90.0,                      // fov
            16.0 / 9.0,               // aspect_ratio
        );
        
        // Test that we can generate rays
        let ray = camera.get_ray(0.5, 0.5);
        assert_eq!(ray.origin, Vec3::new(0.0, 0.0, 0.0));
        
        // Ray should point roughly down -Z
        assert!(ray.direction.z < 0.0);
    }
}
