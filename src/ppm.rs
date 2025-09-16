/// PPM P3 format writer for ASCII image output
pub struct PpmWriter {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl PpmWriter {
    /// Create a new PPM writer with specified dimensions
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: Vec::with_capacity((width * height * 3) as usize),
        }
    }
    
    /// Write a single pixel with RGB values (0-255)
    pub fn write_pixel(&mut self, r: u8, g: u8, b: u8) {
        self.pixels.push(r);
        self.pixels.push(g);
        self.pixels.push(b);
    }
    
    /// Convert to PPM P3 format string
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        
        // PPM P3 header
        result.push_str("P3\n");
        result.push_str(&format!("{} {}\n", self.width, self.height));
        result.push_str("255\n");
        
        // Write pixels - one per line as "R G B"
        for chunk in self.pixels.chunks(3) {
            if chunk.len() == 3 {
                result.push_str(&format!("{} {} {}\n", chunk[0], chunk[1], chunk[2]));
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ppm_writer() {
        let mut writer = PpmWriter::new(2, 2);
        writer.write_pixel(255, 0, 0);    // Red
        writer.write_pixel(0, 255, 0);    // Green
        writer.write_pixel(0, 0, 255);    // Blue
        writer.write_pixel(255, 255, 255); // White
        
        let output = writer.to_string();
        assert!(output.starts_with("P3\n2 2\n255\n"));
        assert!(output.contains("255 0 0\n"));
        assert!(output.contains("0 255 0\n"));
        assert!(output.contains("0 0 255\n"));
        assert!(output.contains("255 255 255\n"));
    }
}
