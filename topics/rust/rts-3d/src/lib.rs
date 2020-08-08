pub mod window;
mod render;
pub mod resources;
pub mod camera;
pub mod timer;
mod render_system;
mod components;

use log::{info,debug,error};
use std::path::Path;
use cgmath::{vec3,Matrix3,Quaternion,Matrix4,Deg,Rad};
use cgmath::prelude::*;
use std::collections::HashMap;
use wavefront_obj::obj::{parse,Primitive};
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
    pub wireframe_mode: bool,
    pub cam_rotate_mode: bool,
    pub frame: u32,
}

impl GameState {
    pub fn new(window: Window, config: Config) -> GameState {
        let mut gs = GameState {
            ecs: World::new(),
            window,
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
            .with_mesh(GlMesh::cube(vec3(0.0, 0.0, -12.0), Quaternion::from(Matrix3::from_value(0.0)), 1.0))
            .build();
        let demo_card = GlModel::builder()
            .with_mesh(GlMesh::square(vec3(-1.0, -1.0, -5.0), Quaternion::from(Matrix3::from_angle_x(Deg(90.0))), 0.9))
            .build();
        let terrain = GlModel::builder()
            .with_obj_file(loader.load_string("models/terrain.obj").unwrap())
            .build();

        
        self.ecs.create_entity()
            .with(Pos{ x: 0.0, y: 0.0, z: -12.0 })
            .with(Renderable3D{ model: demo_box, shader })
            .build();

        self.ecs.create_entity()
            .with(Pos{ x: -5.0, y: 5.0, z: -12.0 })
            .with(Renderable3D{ model: demo_card, shader })
            .build();

        self.ecs.create_entity()
            .with(Pos{ x: -5.0, y: 5.0, z: -12.0 })
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
            let t = self.ecs.fetch::<Timer>();
            t.elapsed(cur_time)
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

    fn process_keys(&self, delta_time: f64) {

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

    let loader = ResourceLoader::from_relative_exe_path(Path::new("assets")).unwrap();
    // let shader = GlShader::builder()
    //     .with_frag_shader(loader.load_cstring("shaders/shader.frag").unwrap())
    //     .with_vert_shader(loader.load_cstring("shaders/shader.vert").unwrap())
    //     .build();
    // info!("Shader initialized");

    let mesh = parse(
        loader.load_string("models/terrain.obj").unwrap()
    ).unwrap();

    // // Janky, but proves it works?
    // let obj = mesh.objects.get(0).unwrap();
    // let indices: Vec<Index> = obj.geometry
    //     .iter()
    //     .flat_map(|g| g.shapes.iter())
    //     .map(|shape| shape.primitive)
    //     .fold(Vec::new(), |mut v, primitive| {
    //         let to_append = match primitive {
    //             Primitive::Point(v) => [v.0, v.0, v.0],
    //             Primitive::Line(v1, v2) => [v1.0, v2.0, v2.0],
    //             Primitive::Triangle(v1, v2, v3) => [v1.0, v2.0, v3.0],
    //         };

    //         v.push(to_append[0] as Index);
    //         v.push(to_append[1] as Index);
    //         v.push(to_append[2] as Index);

    //         v
    //     });
    // let verts: Vec<Vertex> = obj.vertices
    //     .iter()
    //     .zip(&obj.normals)
    //     .map(|(v, n)| Vertex::from_coords(
    //         v.x as f32, v.y as f32, v.z as f32, 
    //         n.x as f32, n.y as f32, n.z as f32,
    //     )).collect();
    // let model = Mesh::from_vertices(verts, indices, vec3(0.0, 0.0, -12.0), Quaternion::from(Matrix3::from_value(0.0)), 1.0);

    // let sprites: Vec<Mesh> = (0..16)
    //     .flat_map(|x| (0..9).map(move |y| (x, y)))
    //     .map(|(x, y)| Mesh::square(&shader, vec3(x as f32, y as f32, -10.0), Quaternion::from(Matrix3::from_value(0.0)), 0.9))
    //     .collect();

    // let demo_card = Mesh::square(&shader, vec3(-1.0 as f32, 4.0 as f32, -5.0), Quaternion::from(Matrix3::from_angle_x(Deg(90.0))), 0.9);

    // let mut key_state = HashMap::new();
    let mut mouse_state = MouseState{
        prev_x: 0.0,
        prev_y: 0.0,
        first_mouse: true,
    };

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
