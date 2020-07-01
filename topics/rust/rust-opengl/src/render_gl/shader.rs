use crate::resources::ResourceLoader;
use std::ffi::{CString, CStr};

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_res(loader: &ResourceLoader, name: &str) -> Result<Shader, String> {
        const POSSIBLE_EXTENSIONS: [(&str, gl::types::GLenum); 2] = [
            (".vert", gl::VERTEX_SHADER),
            (".frag", gl::FRAGMENT_SHADER),
        ];

        let shader_kind = POSSIBLE_EXTENSIONS.iter()
            .find(|&&(file_extension, _)| name.ends_with(file_extension))
            .map(|&(_, kind)| kind)
            .ok_or_else(|| format!("Failed to resolve shader type for resource: '{}'", name))?;

        let source = loader
            .load_cstring(name)
            .map_err(|e| format!("Error loading resource '{}': {:?}", name, e))?;

        Shader::from_source(&source, shader_kind)
    }

    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    match success {
        0 => {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                )
            }

            Err(error.to_string_lossy().into_owned())
        },
        _ => Ok(id),
    }
}

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // Length + \0 end
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));

    let cstr = unsafe { CString::from_vec_unchecked(buffer) };

    return cstr;
}

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_res(res: &ResourceLoader, name: &str) -> Result<Program, String> {
        const POSSIBLE_EXTENSIONS: [&str; 2] = [
            ".vert",
            ".frag",
        ];
        let shaders = POSSIBLE_EXTENSIONS
            .iter()
            .map(|extension| Shader::from_res(res, &format!("{}{}", name, extension)))
            .collect::<Result<Vec<Shader>, String>>()?;

        Program::from_shaders(&shaders[..])
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        shaders.iter().for_each(|shader| {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        });

        unsafe { gl::LinkProgram(program_id); }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                )
            }

            return Err(error.to_string_lossy().into_owned());
        }

        shaders.iter().for_each(|shader| {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        });

        Ok(Program { id: program_id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
