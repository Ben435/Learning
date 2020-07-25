use gl;
use gl::types::{GLuint,GLint,GLfloat};
use std::ffi::CString;
use log::error;
use cgmath::{Matrix4,Vector2};
use cgmath::prelude::*;

const MAX_LOG: usize = 1024;

#[derive(Debug)]
pub struct GlShader {
    program: GLuint,
}

impl GlShader {
    pub fn new(program: GLuint) -> GlShader {
        GlShader {
            program,
        }
    }

    pub fn builder() -> GlShaderBuilder {
        GlShaderBuilder::new()
    }

    pub fn set_uniform_2f(&self, name: String, val: Vector2<GLfloat>) {
        unsafe {
            let loc = self.get_uniform_location(name);
            gl::Uniform2f(loc, val.x, val.y);
        }
    }

    pub fn set_uniform_mat4(&self, name: String, val: Matrix4<GLfloat>) {
        unsafe {
            let loc = self.get_uniform_location(name);
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, val.as_ptr());
        }
    }

    unsafe fn get_uniform_location(&self, name: String) -> GLint {
        let cstr_name = match CString::new(name.to_string()) {
            Ok(s) => s,
            Err(e) => panic!(format!("Failed to translate String to CString: {}", e)),
        };
        let loc = gl::GetUniformLocation(self.program, cstr_name.as_ptr());

        if loc == -1 {
            panic!(format!("Failed to find location for uniform: {}", name));
        }

        return loc;
    }

    pub fn enable(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

pub struct GlShaderBuilder {
    shaders: Vec<gl::types::GLuint>,
    log_buffer: Vec<u8>,
}

impl GlShaderBuilder {
    pub fn new() -> GlShaderBuilder {
        let mut log_buffer = Vec::with_capacity(MAX_LOG);
        unsafe {
            log_buffer.set_len(MAX_LOG - 1);    // -1 for null terminator.
        }

        GlShaderBuilder {
            shaders: Vec::with_capacity(2),
            log_buffer,
        }
    }

    pub fn with_frag_shader(self, csrc: CString) -> Self {
        self.with_shader(csrc, gl::FRAGMENT_SHADER)
    }

    pub fn with_vert_shader(self, csrc: CString) -> Self {
        self.with_shader(csrc, gl::VERTEX_SHADER)
    }

    fn with_shader(mut self, csrc: CString, shader_type: gl::types::GLenum) -> Self {
        let shader_id = unsafe {
            let shader_id = gl::CreateShader(shader_type);
            gl::ShaderSource(shader_id, 1, &csrc.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);

            self.check_shader_compile_status(shader_id);

            shader_id
        };
        self.shaders.push(shader_id);

        self
    }

    /// Check for error status on GLSL compile. Panic on any error (eg: shader compile failed).
    /// Will _attempt_ to log errors, but not always successful.
    unsafe fn check_shader_compile_status(&mut self, shader_id: gl::types::GLuint) {
        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            let mut log_len = 0;
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut log_len);

            if log_len > 0 {
                gl::GetShaderInfoLog(shader_id, log_len.min(MAX_LOG as i32), std::ptr::null_mut(), self.log_buffer.as_mut_ptr() as *mut gl::types::GLchar);
                error!("Failed to compile frag: {}", String::from_utf8_lossy(&self.log_buffer));
            } else {
                error!("Failed to compile with no log?");
            }
            panic!("Failed to compile frag");
        }
    }

    pub fn build(self) -> GlShader {
        let program = unsafe {
            let tmp = gl::CreateProgram();
    
            self.shaders.iter().for_each(|shader| {
                gl::AttachShader(tmp, *shader);
            });
            
            gl::LinkProgram(tmp);
    
            let mut success = gl::FALSE as gl::types::GLint;
            gl::GetProgramiv(tmp, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                panic!("Failed to link prg");
            }
    
            tmp
        };

        GlShader::new(program)
    }
}

impl Drop for GlShaderBuilder {
    fn drop(&mut self) {
        // Cleanup linked shaders.
        unsafe {
            self.shaders.iter().for_each(|shader| {
                gl::DeleteShader(*shader);
            });
        }
    }
}
