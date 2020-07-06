mod resources;

use glfw::{Action,Context,Key};
use std::path::Path;
use std::ptr;
use std::sync::mpsc::Receiver;

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
    
    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    let mut info_log = Vec::with_capacity(512);
    unsafe {
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
    }

    let triangle_vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0,
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
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            (triangle_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            triangle_vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        
        gl::VertexAttribPointer(
            0, 
            3 as gl::types::GLsizei, 
            gl::FLOAT, 
            gl::FALSE, 
            (3 * std::mem::size_of::<f32>()) as gl::types::GLsizei, 
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);
    }
    

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        process_events(&mut window, &events);

        unsafe {
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
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
