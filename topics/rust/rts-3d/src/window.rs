use glfw;
use glfw::Context;
use gl;
use std::sync::mpsc::{Receiver};
use log::{info,Level,log_enabled};
use std::ffi::CStr;

pub struct Window {
    glfw: glfw::Glfw,
    pub window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(title: &str, scr_width: u32, scr_height: u32) -> Result<Window, String> {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS);
        if glfw.is_err() {
            return Err(format!("Init glfw error: {}", glfw.err().unwrap()))
        }
        let mut glfw = glfw.unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
       
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    
        let window = glfw.create_window(scr_width, scr_height, title, glfw::WindowMode::Windowed);
    
        if window.is_none() {
            return Err(String::from("Failed to load window"));
        }
        let (mut window, events) = window.expect("Unexpected failed to load window");

        window.make_current();

        // Setup polling for events
        window.set_framebuffer_size_polling(true);
        // window.set_scroll_polling(true);

        let mut res = Window{
            glfw,
            window,
            events,
        };
        res.init_gl();

        Ok(res)
    }

    fn init_gl(&mut self) {
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);

        if log_enabled!(Level::Info) {
            let renderer = get_gl_rstring(gl::RENDERER).unwrap();
            let glsl_ver = get_gl_rstring(gl::SHADING_LANGUAGE_VERSION).unwrap();
            info!("{} {}", renderer, glsl_ver);
        }

        unsafe {

            // Clear any existing data.
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
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

    pub fn set_should_close(&mut self, v: bool) {
        self.window.set_should_close(v);
    }
}

fn get_gl_rstring(name: gl::types::GLenum) -> Option<String> {
    let cs = unsafe {
        let ptr = gl::GetString(name);
        if ptr.is_null() {
            return None
        }
        
        Some(CStr::from_ptr(ptr as *const i8))
    }?;
    
    Some(String::from_utf8_lossy(cs.to_bytes()).to_string())
}
