mod resources;

use glfw::{Action,Context,Key};
use std::path::Path;
use std::ptr;
use std::sync::mpsc::Receiver;
use cgmath::prelude::*;
use cgmath::{Matrix4,Deg,Rad,vec3,perspective};
use std::ffi::CString;

use crate::resources::ResourceLoader;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;


fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(SCR_WIDTH, SCR_HEIGHT, "OpenGL 3D", glfw::WindowMode::Windowed)
        .expect("Failed to init glfw window");

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
    
    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    let mut info_log = Vec::with_capacity(512);
    unsafe {
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
    }

    let triangle_vertices: [f32; 40] = [
        // positions       // texture coords
         0.5,  0.5, -0.5,   1.0, 1.0,   // back top right
         0.5, -0.5, -0.5,   1.0, 0.0,   // back bottom right
        -0.5, -0.5, -0.5,   0.0, 0.0,   // back bottom left
        -0.5,  0.5, -0.5,   0.0, 1.0,   // back top left 

         0.5,  0.5,  0.5,   1.0, 1.0,   // front top right
         0.5, -0.5,  0.5,   1.0, 0.0,   // front bottom right
        -0.5, -0.5,  0.5,   0.0, 0.0,   // front bottom left
        -0.5,  0.5,  0.5,   0.0, 1.0    // front top left 
    ];

    let indices = [
        // Back face
        0, 1, 3,
        1, 2, 3,
        // Top Face
        0, 3, 4,
        3, 4, 7,
        // Bottom Face
        1, 2, 5,
        2, 5, 6,
        // Left Face
        2, 3, 6,
        3, 6, 7,
        // Right face
        0, 1, 5,
        0, 5, 6,
        // Front Face
        4, 5, 7,
        5, 6, 7,
    ];

    let resource_loader = ResourceLoader::from_relative_exe_path(Path::new("assets")).unwrap();

    let vertex_shader_source = resource_loader.load_cstring("shaders/triangle.vert").unwrap();
    let vertex_shader = unsafe {
        let tmp = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(tmp, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(tmp);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(tmp, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetShaderInfoLog(tmp, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
            println!("ERROR::SHADER::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
        }

        tmp
    };

    let fragment_shader_source = resource_loader.load_cstring("shaders/triangle.frag").unwrap();
    let fragment_shader = unsafe {
        let tmp = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(tmp, 1, &fragment_shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(tmp);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(tmp, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetShaderInfoLog(tmp, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
            println!("ERROR::SHADER::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
        }

        tmp
    };

    let shader_program = unsafe {
        let tmp = gl::CreateProgram();
        gl::AttachShader(tmp, fragment_shader);
        gl::AttachShader(tmp, vertex_shader);

        gl::LinkProgram(tmp);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetProgramiv(tmp, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetProgramInfoLog(tmp, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
        }

        // Cleanup shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        tmp
    };


    let mut vbo: gl::types::GLuint = 0;
    let mut ebo: gl::types::GLuint = 0;
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            (triangle_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            triangle_vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER, 
            (indices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        let stride = (5 * std::mem::size_of::<f32>()) as gl::types::GLsizei;
        
        // Position attrib
        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            stride, 
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // Texture attrib
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
        gl::EnableVertexAttribArray(1);
    }

    let texture = unsafe {
        let img = image::open(&Path::new("assets/textures/container.jpg")).expect("Failed to load texture");
        let img_buffer = img.into_rgb();
        let (width, height) = (img_buffer.width(), img_buffer.height());
        let raw_data = img_buffer.into_raw();

        let mut tmp = 0;
        gl::GenTextures(1, &mut tmp);
        gl::BindTexture(gl::TEXTURE_2D, tmp);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            width as i32,
            height as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            raw_data.as_ptr() as *const gl::types::GLvoid,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        tmp
    };
    

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::BindTexture(gl::TEXTURE_2D, texture);
            
            gl::UseProgram(shader_program);

            let model: Matrix4<f32> = Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(), Rad(-55.0 as f32));
            let view: Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, -3.0));
            let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);

            let model_loc = gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr());
            let view_loc = gl::GetUniformLocation(shader_program, CString::new("view").unwrap().as_ptr());
            let projection_loc = gl::GetUniformLocation(shader_program, CString::new("projection").unwrap().as_ptr());

            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr());

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    unsafe {
        gl::DeleteProgram(shader_program);
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(&events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            },
            _ => {},
        }
    }
}
