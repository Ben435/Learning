mod lib;
mod window;
mod render;

use window::Window;
use log::{info,debug,LevelFilter};
use env_logger::{Builder};

const SCR_HEIGHT: u32 = 600;
const SCR_WIDTH: u32 = 800;

fn main() {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();
    info!("Logger initialized");

    let mut win = Window::new(
        "Hello world!",
        SCR_WIDTH,
        SCR_HEIGHT,
    ).unwrap();
    info!("Window initialized");

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
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        win.update_screen();
    }
    debug!("Exited main loop");
}
