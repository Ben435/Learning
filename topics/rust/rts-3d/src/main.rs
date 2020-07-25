mod lib;
mod window;
mod render;
mod resources;

use window::Window;
use log::{info,debug,error,LevelFilter};
use env_logger::{Builder};
use render::sprite::Sprite;
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

    let sp = Sprite::square();
    info!("Loading sprite: {:?}", &sp);

    let mut renderer: render::SimpleRenderer<render::sprite::Sprite> = render::SimpleRenderer::new();


    let cap = 512;
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
        let csrc = loader.load_cstring("shaders/shader.vert").unwrap();

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

        gl::DeleteShader(frag);
        gl::DeleteShader(frag);

        tmp
    };
    info!("Program initialized");

    debug!("Beginning main loop");

    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE)
    }

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
            renderer.begin();

            renderer.submit(&sp);

            renderer.end();
            renderer.present();

            check_gl_error();
        }

        win.update_screen();
    }
    debug!("Exited main loop");
}

unsafe fn check_gl_error() {
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
}
