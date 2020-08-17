pub mod window;
mod render;
pub mod resources;
pub mod camera;
pub mod timer;
mod render_system;
mod components;

use log::{info,debug,error};
use std::path::Path;
use cgmath::{vec3,Matrix3,Quaternion,Matrix4,Deg};
use std::collections::HashMap;
use specs::prelude::*;

use camera::Camera;
use timer::Timer;
use render::{GlShader,GlMesh,SimpleRenderer,GlModel};
use resources::ResourceLoader;
use render_system::RenderSystem;
use components::*;
use window::Window;

pub const SCR_HEIGHT: u32 = 600;
pub const SCR_WIDTH: u32 = 800;

struct GameState {
    pub ecs: World,
    pub window: Window,
    pub key_state: HashMap<glfw::Key, bool>,
    pub mouse_state: MouseState,

    pub wireframe_mode: bool,
    pub cam_rotate_mode: bool,

    pub frame: u32,
}

impl GameState {
    pub fn new(window: Window, config: Config) -> GameState {
        let mut gs = GameState {
            ecs: World::new(),
            window,
            key_state: HashMap::new(),
            mouse_state: MouseState{
                prev_x: 0.0,
                prev_y: 0.0,
                first_mouse: true,
            },
            wireframe_mode: false,
            cam_rotate_mode: false,
            frame: 0,
        };

        gs.init(config);

        gs
    }

    fn init(&mut self, config: Config) {
        let init_time = self.window.get_time();
        let frame_timer = Timer::new(init_time);
        let loader = ResourceLoader::from_relative_exe_path(Path::new(&config.asset_base_path)).unwrap();
        let renderer = SimpleRenderer::<GlMesh>::new();

        register(&mut self.ecs);

        let shader = GlShader::builder()
                .with_frag_shader(loader.load_cstring("shaders/shader.frag").unwrap())
                .with_vert_shader(loader.load_cstring("shaders/shader.vert").unwrap())
                .build();

        let demo_box = GlModel::builder()
            .with_mesh(GlMesh::cube())
            .build();
        let terrain = GlModel::builder()
            .with_obj_file(loader.load_string("models/terrain.obj").unwrap())
            .build();
        
        self.ecs.create_entity()
            .with(Pos{ x: 0.0, y: 0.0, z: -12.0 })
            .with(Renderable3D{ model: demo_box, shader })
            .build();

        self.ecs.create_entity()
            .with(Pos{ x: 0.0, y: 0.0, z: -12.0 })
            .with(Rot{ quaternion: Quaternion::from(Matrix3::from_angle_x(Deg::<f32>(90.0))) })
            .with(Renderable3D{ model: terrain, shader })
            .build();

        self.ecs.insert(frame_timer);
        self.ecs.insert(Camera::default());
        self.ecs.insert(loader);
        self.ecs.insert(renderer);
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn tick(&mut self) {
        // Process events
        for (_, event) in self.window.flush_events() {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    debug!("Resize to {}, {}", width, height);
                    unsafe { gl::Viewport(0, 0, width, height) }
                },
                glfw::WindowEvent::Scroll(_x, y) => {
                    let mut camera = self.ecs.write_resource::<Camera>();
                    camera.update_scroll(y as f32);
                },
                e => debug!("Unrecognized event: {:?}", e),
            }
        };

        let cur_time = self.window.get_time();
        let delta_time = {
            let mut t = self.ecs.fetch_mut::<Timer>();
            let res = t.elapsed(cur_time);
            t.reset(cur_time);

            res
        };

        self.process_keys(delta_time);
        self.process_mouse(delta_time);

        // Rotate based on current time, just to show lighting
        // cube.rotation = Quaternion::from(Matrix3::from_angle_x(Rad::<f32>((win.get_time() as f32).sin() * 2.0)));
 
        self.run_systems();
        unsafe {
            check_gl_error();
        }

        self.window.update_screen();
    }

    fn process_keys(&mut self, delta_time: f64) {
        // let (elapsed_time: f64, win: &mut Window, key_states: &mut HashMap<glfw::Key, bool>, gamestate: &mut GameState, camera: &mut Camera);
        match self.window.window.get_key(glfw::Key::GraveAccent) {
            glfw::Action::Press => {
                let prev_pressed = self.key_state.insert(glfw::Key::GraveAccent, true).unwrap_or(false);
                if !prev_pressed {
                    info!("Toggling wireframe mode");
                    self.wireframe_mode = !self.wireframe_mode;
                }
            }
            glfw::Action::Release => {
                self.key_state.insert(glfw::Key::GraveAccent, false);
            },
            _ => {},
        };
    
        match self.window.window.get_key(glfw::Key::Escape) {
            glfw::Action::Press => {
                self.window.set_should_close(true);
            },
            _ => {},
        };
    
        // Primitive user input for panning.
        let player_speed = 10.0;
        let effective_speed = player_speed * delta_time as f32;

        let mut camera = self.ecs.fetch_mut::<Camera>();
        if self.window.window.get_key(glfw::Key::Right) == glfw::Action::Press {
            camera.transform(Matrix4::from_translation(vec3(effective_speed, 0.0, 0.0)));
        }
        if self.window.window.get_key(glfw::Key::Left) == glfw::Action::Press {
            camera.transform(Matrix4::from_translation(vec3(-effective_speed, 0.0, 0.0)));
        }
        if self.window.window.get_key(glfw::Key::Up) == glfw::Action::Press {
            camera.transform(Matrix4::from_translation(vec3(0.0, effective_speed, 0.0)));
        }
        if self.window.window.get_key(glfw::Key::Down) == glfw::Action::Press {
            camera.transform(Matrix4::from_translation(vec3(0.0, -effective_speed, 0.0)));
        }
    }

    fn process_mouse(&self, delta_time: f64) {

    }

    fn run_systems(&mut self) {
        let mut render_sys = RenderSystem{};
        render_sys.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

struct MouseState {
    pub prev_x: f64,
    pub prev_y: f64,
    pub first_mouse: bool,
}

struct Config {
    asset_base_path: String,
}

pub fn run() {
    let win = Window::new(
        "Hello world!",
        SCR_WIDTH,
        SCR_HEIGHT,
    ).unwrap();
    info!("Window initialized");

    let config = Config{
        asset_base_path: String::from("assets"),
    };
    let mut gamestate = GameState::new(win, config);

    debug!("Entering main loop");
    while !gamestate.should_close() {
        gamestate.tick();
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

fn process_mouse(elapsed_time: f64, win: &mut Window, mouse_state: &mut MouseState, gamestate: &mut GameState, camera: &mut Camera) {
    match win.window.get_mouse_button(glfw::MouseButton::Button2) {
        glfw::Action::Press => {
            if !gamestate.cam_rotate_mode {
                gamestate.cam_rotate_mode = true;
            }
        },
        glfw::Action::Release => {
            gamestate.cam_rotate_mode = false;
        },
        _ => {},
    };

    let (cur_x, cur_y) = win.window.get_cursor_pos();
    let min_move = 0.01;
    let rotate_speed = 10.0;
    if gamestate.cam_rotate_mode {
        let x_mov = mouse_state.prev_x - cur_x;
        let y_mov = mouse_state.prev_y - cur_y;

        if x_mov.abs() > min_move {
            let x_delta = x_mov * elapsed_time * rotate_speed;
            camera.yaw += x_delta as f32;
        }

        if y_mov.abs() > min_move {
            let y_delta = y_mov * elapsed_time * rotate_speed;
            camera.pitch = (camera.pitch + y_delta as f32).min(89.0).max(-89.0);
        }

        camera.update_camera_vectors();
    }

    mouse_state.prev_x = cur_x;
    mouse_state.prev_y = cur_y;
}
