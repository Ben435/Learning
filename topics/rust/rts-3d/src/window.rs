use glfw;
use glfw::Context;
use gl;
use std::sync::mpsc::{Receiver};
use crate::log::Logger;

pub struct Window<'a> {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    logger: Logger<'a>,
}

impl <'a> Window<'a> {
    pub fn new(title: &str, scr_width: u32, scr_height: u32) -> Result<Window, String> {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS);
        if glfw.is_err() {
            return Err(format!("Init glfw error: {}", glfw.err().unwrap()))
        }
        let mut glfw = glfw.unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
        let window = glfw.create_window(scr_width, scr_height, title, glfw::WindowMode::Windowed);
    
        if window.is_none() {
            return Err(String::from("Failed to load window"));
        }
        let (mut window, events) = window.expect("Unexpected failed to load window");

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // Setup polling for events
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        // window.set_scroll_polling(true);
        // window.set_cursor_pos_polling(true);

        let logger = Logger::new("window");

        Ok(Window{
            glfw,
            window,
            events,
            logger,
        })
    }

    pub fn update_screen(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
    }

    pub fn flush_events(&mut self) -> glfw::FlushedMessages<(f64, glfw::WindowEvent)> {
        glfw::flush_messages(&self.events)
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
}
