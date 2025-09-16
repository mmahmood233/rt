use crate::math::Vec3;

/// Material properties for shading
#[derive(Debug, Clone)]
pub struct Material {
    pub albedo: Vec3,      // Base color (diffuse reflectance)
    pub specular: f64,     // Specular reflection coefficient
    pub shininess: f64,    // Phong shininess exponent
    pub reflectivity: f64, // Mirror reflection coefficient (0.0 = no reflection, 1.0 = perfect mirror)
}

impl Material {
    /// Create a new material with diffuse properties
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo,
            specular: 0.0,
            shininess: 1.0,
            reflectivity: 0.0,
        }
    }
    
    /// Create a material with specular highlights (Phong shading)
    pub fn with_specular(albedo: Vec3, specular: f64, shininess: f64) -> Self {
        Self {
            albedo,
            specular,
            shininess,
            reflectivity: 0.0,
        }
    }
    
    /// Create a reflective material (mirror-like)
    pub fn with_reflection(albedo: Vec3, reflectivity: f64) -> Self {
        Self {
            albedo,
            specular: 0.0,
            shininess: 1.0,
            reflectivity,
        }
    }
    
    /// Predefined materials
    pub fn red() -> Self {
        Self::new(Vec3::new(0.8, 0.2, 0.2))
    }
    
    pub fn green() -> Self {
        Self::new(Vec3::new(0.2, 0.8, 0.2))
    }
    
    pub fn blue() -> Self {
        Self::new(Vec3::new(0.2, 0.2, 0.8))
    }
    
    pub fn white() -> Self {
        Self::new(Vec3::new(0.8, 0.8, 0.8))
    }
    
    pub fn gray() -> Self {
        Self::new(Vec3::new(0.5, 0.5, 0.5))
    }
    
    pub fn mirror() -> Self {
        Self::with_reflection(Vec3::new(0.9, 0.9, 0.9), 0.9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_material_creation() {
        let mat = Material::red();
        assert_eq!(mat.albedo, Vec3::new(0.8, 0.2, 0.2));
        assert_eq!(mat.reflectivity, 0.0);
        
        let mirror = Material::mirror();
        assert_eq!(mirror.reflectivity, 0.9);
    }
}
