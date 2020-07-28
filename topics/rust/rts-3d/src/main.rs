mod lib;
mod window;
mod render;
mod resources;
mod camera;
mod timer;

use window::Window;
use log::{info,debug,error,LevelFilter};
use env_logger::{Builder};
use render::sprite::Sprite;
use render::GlShader;
use resources::ResourceLoader;
use std::path::Path;
use cgmath::{vec3,vec2,Matrix4};
use std::collections::HashMap;
use camera::Camera;

const SCR_HEIGHT: u32 = 600;
const SCR_WIDTH: u32 = 800;

struct GameState {
    pub wireframe_mode: bool,
}

struct MouseState {
    pub prev_x: f32,
    pub prev_y: f32,
    pub first_mouse: bool,
}

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

    let sprites: Vec<Sprite> = (0..16)
        .flat_map(|x| (0..9).map(move |y| (x, y)))
        .map(|(x, y)| Sprite::square(&shader, vec3(x as f32, y as f32, -10.0), 0.9))
        .collect();

    let mut camera = Camera::default();

    let mut renderer: render::SimpleRenderer<Sprite> = render::SimpleRenderer::new();

    let mut key_state = HashMap::new();
    let mut mouse_state = MouseState{
        prev_x: 0.0,
        prev_y: 0.0,
        first_mouse: true,
    };
    let mut gamestate = GameState{
        wireframe_mode: false,
    };

    let start_time = win.get_time();;
    let mut frame_timer = timer::Timer::new(start_time);
    let mut fps_timer = timer::Timer::new(start_time);
    let mut frame_count = 0;
    debug!("Beginning main loop");
    while !win.should_close() {
        let cur_time = win.get_time();
        let time_progressed = frame_timer.elapsed(cur_time);
        frame_timer.reset(cur_time);

        // Calculate fps
        frame_count += 1;
        let elapsed = fps_timer.elapsed(cur_time);
        if elapsed > 1.0 {
            info!("{}fps", frame_count);
            fps_timer.reset(cur_time);
            frame_count = 0;
        }

        // Process events
        for (_, event) in win.flush_events() {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    debug!("Resize to {}, {}", width, height);
                    unsafe { gl::Viewport(0, 0, width, height) }
                },
                glfw::WindowEvent::Scroll(_x, y) => camera.update_scroll(y as f32),
                e => debug!("Unrecognized event: {:?}", e),
            }
        };
        process_keys(time_progressed, &mut win, &mut key_state, &mut gamestate, &mut camera);
        
        let (cursor_x, cursor_y) = win.window.get_cursor_pos();
        renderer.light_pos = vec2(
            (cursor_x as f32 / (SCR_WIDTH/2) as f32) - 1.0,
            (-cursor_y as f32 / (SCR_HEIGHT/2) as f32) + 1.0,
        );
        
        // Render
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if gamestate.wireframe_mode {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
            
            renderer.begin();

            sprites.iter().for_each(|sp| {
                renderer.submit(sp);
            });

            renderer.end();

            renderer.present(&camera, SCR_WIDTH, SCR_HEIGHT);

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

fn process_keys(elapsed_time: f64, win: &mut Window, key_states: &mut HashMap<glfw::Key, bool>, gamestate: &mut GameState, camera: &mut Camera) {
    match win.window.get_key(glfw::Key::GraveAccent) {
        glfw::Action::Press => {
            let prev_pressed = key_states.insert(glfw::Key::GraveAccent, true).unwrap_or(false);
            if !prev_pressed {
                info!("Toggling wireframe mode");
                gamestate.wireframe_mode = !gamestate.wireframe_mode;
            }
        }
        glfw::Action::Release => {
            key_states.insert(glfw::Key::GraveAccent, false);
        },
        _ => {},
    };

    match win.window.get_key(glfw::Key::Escape) {
        glfw::Action::Press => {
            win.set_should_close(true);
        },
        _ => {},
    };

    // Primitive user input for panning.
    let player_speed = 10.0;
    let effective_speed = player_speed * elapsed_time as f32;
    if win.window.get_key(glfw::Key::Right) == glfw::Action::Press {
        camera.transform(Matrix4::from_translation(vec3(effective_speed, 0.0, 0.0)));
    }
    if win.window.get_key(glfw::Key::Left) == glfw::Action::Press {
        camera.transform(Matrix4::from_translation(vec3(-effective_speed, 0.0, 0.0)));
    }
    if win.window.get_key(glfw::Key::Up) == glfw::Action::Press {
        camera.transform(Matrix4::from_translation(vec3(0.0, effective_speed, 0.0)));
    }
    if win.window.get_key(glfw::Key::Down) == glfw::Action::Press {
        camera.transform(Matrix4::from_translation(vec3(0.0, -effective_speed, 0.0)));
    }
}
