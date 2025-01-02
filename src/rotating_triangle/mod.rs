use eframe::{egui_glow, glow::{self, HasContext}};
use crate::webgl_util::create_shader;

pub struct RotatingTriangle {
    program: glow::Program,
    vertex_array: glow::VertexArray,
}

impl RotatingTriangle {
    pub fn new(gl: &glow::Context) -> Result<Self,String> {
        
        let shader_version = egui_glow::ShaderVersion::get(gl);
        unsafe {
            let program = gl
                .create_program()
                .expect("Cannot create program");
            
            if !shader_version.is_new_shader_interface() {
                let errmsg = format!("Custom 3D painting hasn't been ported to {:?}", shader_version);
                return Err(errmsg);
            }
            let vertex_shader = create_shader(
                gl, &program,glow::VERTEX_SHADER, 
                include_str!("shader/rotating_triangle_vertex.glsl")
            )?;
            let fragment_shader = create_shader(
                gl, &program,glow::FRAGMENT_SHADER,
                include_str!("shader/rotating_triangle_fragment.glsl")
            )?;
            
            gl.link_program(program);
            assert!(
                gl.get_program_link_status(program),
                "Failed to link custom_3d_glow program: {}",
                gl.get_program_info_log(program)
            );

            gl.detach_shader(program, vertex_shader);
            gl.delete_shader(vertex_shader);
            gl.detach_shader(program, fragment_shader);
            gl.delete_shader(fragment_shader);
            
            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            Ok(RotatingTriangle {
                program: program,
                vertex_array: vertex_array,
            })
        }

    }
    
    pub fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }
    
    pub fn paint(&mut self, gl: &glow::Context, angle: f32) {
        use glow::HasContext as _;
        unsafe {
            gl.use_program(Some(self.program));
            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_angle").as_ref(),
                angle
            );
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}