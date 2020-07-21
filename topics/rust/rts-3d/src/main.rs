mod lib;
mod window;
mod log;
mod render;

use window::Window;

const SCR_HEIGHT: u32 = 600;
const SCR_WIDTH: u32 = 800;

fn main() {
    let mut logger = log::Logger::new_with_level("main", log::LogLevel::INFO);

    logger.info("Logger initialized");

    let mut win = Window::new(
        "Hello world!",
        SCR_WIDTH,
        SCR_HEIGHT,
    ).unwrap();

    logger.info("Window initialized");

    logger.debug("Beginning main loop");
    while !win.should_close() {
        // Process events
        for (_, event) in win.flush_events() {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    logger.debug(&format!("Resize to {}, {}", width, height));
                    unsafe { gl::Viewport(0, 0, width, height) }
                },
                e => logger.debug(&format!("Unrecognized event: {:?}", e)),
            }
        };

        // Render
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        win.update_screen();
    }
    logger.debug("Exited main loop");
}
