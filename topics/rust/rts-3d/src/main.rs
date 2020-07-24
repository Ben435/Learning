mod lib;
mod window;
mod render;
mod resources;

use window::Window;
use log::{info,debug,error,LevelFilter};
use env_logger::{Builder};
use render::{Renderable,Vertex};
use render::sprite::Sprite;
use std::ffi::CString;
use resources::ResourceLoader;
use std::path::Path;

const SCR_HEIGHT: u32 = 600;
const SCR_WIDTH: u32 = 800;

fn main() {
    Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();
    info!("Logger initialized");

    let mut win = Window::new(
        "Hello world!",
        SCR_WIDTH,
        SCR_HEIGHT,
    ).unwrap();
    info!("Window initialized");

    // let mut renderer: render::SimpleRenderer<render::sprite::Sprite> = render::SimpleRenderer::new();

    let sp: Sprite = Sprite::square();

    let mut vbo = render::GlBuffer::new(sp.get_vertices().as_ref(), sp.get_vertices().len());
    let mut ibo = render::GlIndexBuffer::new(sp.get_indices().as_ref());

    let vao = unsafe {
        let mut tmp = 0;
        gl::GenVertexArrays(1, &mut tmp);
        gl::BindVertexArray(tmp);

        vbo.bind();
        ibo.bind();

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, std::mem::size_of::<Vertex>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        
        gl::BindVertexArray(0);

        tmp
    };

    let cap = 1024;
    let mut msg_buffer = Vec::with_capacity(cap);
    unsafe {
        msg_buffer.set_len(cap-1);
    }

    let loader = ResourceLoader::from_relative_exe_path(Path::new("assets")).unwrap();

    let frag = unsafe {
        let csrc = loader.load_cstring("shaders/shader.frag").unwrap();    // TODO: Better handling

        let tmp = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(tmp, 1, &csrc.as_ptr(), std::ptr::null());
        gl::CompileShader(tmp);

        // TODO: From here to bottom, wrap for error handling.
        let err = gl::GetError();
        if err != gl::NO_ERROR {
            match err {
                gl::INVALID_ENUM => error!("Invalid Enum!"),
                gl::INVALID_VALUE => error!("Invalid Value!"),
                gl::INVALID_OPERATION => error!("Invalid Op!"),
                _ => error!("Unknown error!"),
            }
            panic!(format!("Gl error set!: {}", err));
        }

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(tmp, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            let mut log_len = 0;
            gl::GetShaderiv(tmp, gl::INFO_LOG_LENGTH, &mut log_len);

            if log_len > 0 {
                gl::GetShaderInfoLog(tmp, cap as i32, std::ptr::null_mut(), msg_buffer.as_mut_ptr() as *mut gl::types::GLchar);
                info!("Found: len={}, {:?}", log_len, msg_buffer);
                error!("Failed to compile frag: {}", String::from_utf8_lossy(&msg_buffer));
            } else {
                error!("Failed to compile with no log?");
            }
            panic!("Failed to compile frag");
        }

        tmp
    };

    let vert = unsafe {
        let src = include_str!("../assets/shaders/shader.vert");
        let csrc = CString::new(src).unwrap();    // TODO: Better handling

        let tmp = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(tmp, 1, &csrc.as_ptr(), std::ptr::null());
        gl::CompileShader(tmp);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(tmp, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            panic!("Failed to compile vert");
        }

        tmp
    };

    let prg = unsafe {
        let tmp = gl::CreateProgram();

        gl::AttachShader(tmp, frag);
        gl::AttachShader(tmp, vert);
        gl::LinkProgram(tmp);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetProgramiv(tmp, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            panic!("Failed to link prg");
        }

        tmp
    };
    info!("Program initialized");

    debug!("Beginning main loop");
    while !win.should_close() {
        // Process events
        for (_, event) in win.flush_events() {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    debug!("Resize to {}, {}", width, height);
                    unsafe { gl::Viewport(0, 0, width, height) }
                },
                e => debug!("Unrecognized event: {:?}", e),
            }
        };

        // Render
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(prg);

            gl::BindVertexArray(vao);

            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_SHORT, std::ptr::null());
        }

        win.update_screen();
    }
    debug!("Exited main loop");
}
