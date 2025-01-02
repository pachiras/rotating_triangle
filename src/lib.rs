
use eframe::glow::{self};
mod rotating_triangle;
use rotating_triangle::*;
mod webgl_util;

pub struct Scene {
    rotating_triangle: RotatingTriangle,
}

#[allow(unsafe_code)]
impl Scene {
    pub fn new(gl: &glow::Context) -> Result<Self,String> {
        
        let rotating_triangle = RotatingTriangle::new(gl)?;

        Ok(Scene {
            rotating_triangle,
        })
    } 
    
    pub fn paint(&mut self, gl: &glow::Context, angle: f32) {
        self.rotating_triangle.paint(gl, angle);
    }
    
    pub fn destroy(&self, gl: &glow::Context) {
        self.rotating_triangle.destroy(gl);
    }
}
