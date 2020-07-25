mod lib;
mod window;
mod render;
mod resources;

use window::Window;
use log::{info,debug,error,LevelFilter};
use env_logger::{Builder};
use render::sprite::Sprite;
use render::GlShader;
use resources::ResourceLoader;
use std::path::Path;
use cgmath::vec3;

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

    let loader = ResourceLoader::from_relative_exe_path(Path::new("assets")).unwrap();
    let shader = GlShader::builder()
        .with_frag_shader(loader.load_cstring("shaders/shader.frag").unwrap())
        .with_vert_shader(loader.load_cstring("shaders/shader.vert").unwrap())
        .build();
    info!("Shader initialized");

    let sp = Sprite::square(&shader, vec3(5.0, 5.0, 0.0));
    info!("Loading sprite: {:?}", &sp);

    let mut renderer: render::SimpleRenderer<render::sprite::Sprite> = render::SimpleRenderer::new();

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
