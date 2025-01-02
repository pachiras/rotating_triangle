use eframe::glow::{self, HasContext};

pub unsafe fn check_for_gl_errors(gl: &glow::Context, msg: &str) {
    while let Some(err) = match gl.get_error() {
        glow::NO_ERROR => {
            None
        },
        err => Some(err),
    } {
        log::warn!("{}: GL ERROR {} ({:#X})", msg, err, err);
    }
}

pub fn create_shader(gl: &glow::Context, program: &glow::Program, shader_type: u32, shader_src: &str) -> Result<glow::Shader, String>{
    unsafe {
        let shader = gl.create_shader(shader_type)?;
        gl.shader_source(shader,&shader_src);
        gl.compile_shader(shader);
        assert!(
            gl.get_shader_compile_status(shader),
            "Failed to compile custom_3d_glow {shader_type}: {}",
            gl.get_shader_info_log(shader)
        );
        gl.attach_shader(*program, shader);
        check_for_gl_errors(gl, "attach shader");

        
        Ok(shader)
    }
}
