use gl;
use std::ptr;
use std::ffi::CString;
use std::collections::{HashMap,HashSet};
use cgmath::{Matrix4,Vector3};
use cgmath::prelude::*;
use crate::resources::ResourceLoader;

#[derive(Debug)]
pub enum ProgramBuilderError {
    NoVertexShader,
    NoFragmentShader,
    ShaderCompileError{
        reason: String,
    },
    AlreadyBuilt,
}

pub struct Program {
    program: gl::types::GLuint,
    uniform_map: HashMap<String, gl::types::GLint>,
}

impl Program {
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.program);
    }

    pub fn set_uniform_matrix4(&self, name: &str, mat4: &Matrix4<f32>) {
        let loc = self.uniform_map.get(name);

        debug_assert!(loc.is_some(), format!("Uniform name not found: {}", name));

        unsafe {
            gl::UniformMatrix4fv(*loc.unwrap(), 1, gl::FALSE, mat4.as_ptr());
        }
    }

    pub fn set_uniform_vec3(&self, name: &str, vec3: &Vector3<f32>) {
        let loc = self.uniform_map.get(name);

        debug_assert!(loc.is_some(), format!("Uniform name not found: {}", name));

        unsafe {
            gl::Uniform3fv(*loc.unwrap(), 1, vec3.as_ptr());
        }
    }

    pub fn set_uniform_float(&self, name: &str, float: f32) {
        let loc = self.uniform_map.get(name);

        debug_assert!(loc.is_some(), format!("Uniform name not found: {}", name));

        unsafe {
            gl::Uniform1f(*loc.unwrap(), float);
        }
    }

    pub fn set_uniform_int(&self, name: &str, int: i32) {
        let loc = self.uniform_map.get(name);

        debug_assert!(loc.is_some(), format!("Uniform name not found: {}", name));

        unsafe {
            gl::Uniform1i(*loc.unwrap(), int);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}

pub struct ProgramBuilder <'a> {
    resource_loader: &'a ResourceLoader,

    vertex_shader: gl::types::GLuint,
    fragment_shader: gl::types::GLuint,

    known_uniforms: HashSet<String>,

    log: Vec<u8>,
    built: bool,
}

impl<'a> ProgramBuilder<'a> {
    pub fn new(resource_loader: &'a ResourceLoader) -> ProgramBuilder<'a> {
        let mut info_log = Vec::with_capacity(512);
        unsafe {
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
        }

        ProgramBuilder {
            resource_loader,
            vertex_shader: 0,
            fragment_shader: 0,
            known_uniforms: HashSet::new(),
            log: info_log,
            built: false,
        }
    }

    pub fn with_vertex_shader(mut self, path: &str) -> Result<Self, ProgramBuilderError> {
        let vertex_shader_source = self.resource_loader.load_cstring(path).unwrap();

        self.vertex_shader = unsafe {
            let tmp = gl::CreateShader(gl::VERTEX_SHADER);

            gl::ShaderSource(tmp, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(tmp);
    
            let mut success = gl::FALSE as gl::types::GLint;
            gl::GetShaderiv(tmp, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl::GetShaderInfoLog(tmp, 512, ptr::null_mut(), self.log.as_mut_ptr() as *mut gl::types::GLchar);
                let error_msg = format!("ERROR::SHADER::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&self.log));
                return Err(ProgramBuilderError::ShaderCompileError{
                    reason: error_msg,
                })
            }
    
            tmp
        };

        Ok(self)
    }

    pub fn with_fragment_shader(mut self, path: &str) -> Result<Self, ProgramBuilderError> {
        let fragment_shader_source = self.resource_loader.load_cstring(path).unwrap();

        self.fragment_shader = unsafe {
            let tmp = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(tmp, 1, &fragment_shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(tmp);

            let mut success = gl::FALSE as gl::types::GLint;
            gl::GetShaderiv(tmp, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl::GetShaderInfoLog(tmp, 512, ptr::null_mut(), self.log.as_mut_ptr() as *mut gl::types::GLchar);
                let error_msg = format!("ERROR::SHADER::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&self.log));
                return Err(ProgramBuilderError::ShaderCompileError{
                    reason: error_msg,
                })
            }

            tmp
        };

        Ok(self)
    }

    pub fn with_uniform(mut self, uniform_name: &String) -> Result<Self, ProgramBuilderError> {
        let exists = self.known_uniforms.insert(uniform_name.to_string());

        debug_assert!(exists, format!("Uniform_name registered twice: {}", &uniform_name));

        Ok(self)
    }

    pub fn build(mut self) -> Result<Program, ProgramBuilderError> {
        if self.built {
            return Err(ProgramBuilderError::AlreadyBuilt);
        }
        
        if self.vertex_shader == 0 {
            return Err(ProgramBuilderError::NoVertexShader);
        }
        if self.fragment_shader == 0 {
            return Err(ProgramBuilderError::NoFragmentShader);
        }
        
        let program = unsafe {
            let tmp = gl::CreateProgram();
            gl::AttachShader(tmp, self.fragment_shader);
            gl::AttachShader(tmp, self.vertex_shader);
    
            gl::LinkProgram(tmp);
    
            let mut success = gl::FALSE as gl::types::GLint;
            gl::GetProgramiv(tmp, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl::GetProgramInfoLog(tmp, 512, ptr::null_mut(), self.log.as_mut_ptr() as *mut gl::types::GLchar);
                let error_msg = format!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&self.log));
                return Err(ProgramBuilderError::ShaderCompileError{
                    reason: error_msg,
                });
            }
    
            tmp
        };

        let uniform_map: HashMap<String, gl::types::GLint> = self.known_uniforms
            .iter()
            .fold(HashMap::new(), |mut map, uniform_name| {
                let cstr_name = CString::new(uniform_name.to_string()).unwrap();
                let loc = unsafe {
                    gl::GetUniformLocation(program, cstr_name.as_ptr())
                };

                map.insert(uniform_name.to_string(), loc);

                map
            });

        // Cleanup
        self.built = true;
        unsafe {
            // Cleanup shaders
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.fragment_shader);
        }

        Ok(Program {
            program,
            uniform_map,
        })
    }
}
