mod lib;
mod window;
mod render;
mod resources;
mod camera;
mod timer;

use window::Window;
use log::{info,debug,error,LevelFilter};
use env_logger::{Builder};
use render::mesh::Mesh;
use render::GlShader;
use render::{Index,Vertex};
use resources::ResourceLoader;
use std::path::Path;
use cgmath::{vec3,Matrix3,Quaternion,Matrix4,Deg,Rad};
use cgmath::prelude::*;
use std::collections::HashMap;
use camera::Camera;
use wavefront_obj::obj::{parse,Primitive};

pub const SCR_HEIGHT: u32 = 600;
pub const SCR_WIDTH: u32 = 800;

struct GameState {
    pub wireframe_mode: bool,
    pub cam_rotate_mode: bool,
}

struct MouseState {
    pub prev_x: f64,
    pub prev_y: f64,
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

    let mesh = parse(
        loader.load_string("models/terrain.obj").unwrap()
    ).unwrap();

    // Janky, but proves it works?
    let obj = mesh.objects.get(0).unwrap();
    let indices: Vec<Index> = obj.geometry
        .iter()
        .flat_map(|g| g.shapes.iter())
        .map(|shape| shape.primitive)
        .fold(Vec::new(), |mut v, primitive| {
            let to_append = match primitive {
                Primitive::Point(v) => [v.0, v.0, v.0],
                Primitive::Line(v1, v2) => [v1.0, v2.0, v2.0],
                Primitive::Triangle(v1, v2, v3) => [v1.0, v2.0, v3.0],
            };

            v.push(to_append[0] as Index);
            v.push(to_append[1] as Index);
            v.push(to_append[2] as Index);

            v
        });
    let verts: Vec<Vertex> = obj.vertices
        .iter()
        .zip(&obj.normals)
        .map(|(v, n)| Vertex::from_coords(
            v.x as f32, v.y as f32, v.z as f32, 
            n.x as f32, n.y as f32, n.z as f32,
        )).collect();
    let model = Mesh::from_vertices(verts, indices, &shader, vec3(0.0, 0.0, -12.0), Quaternion::from(Matrix3::from_value(0.0)), 1.0);

    let sprites: Vec<Mesh> = (0..16)
        .flat_map(|x| (0..9).map(move |y| (x, y)))
        .map(|(x, y)| Mesh::square(&shader, vec3(x as f32, y as f32, -10.0), Quaternion::from(Matrix3::from_value(0.0)), 0.9))
        .collect();

    let demo_card = Mesh::square(&shader, vec3(-1.0 as f32, 4.0 as f32, -5.0), Quaternion::from(Matrix3::from_angle_x(Deg(90.0))), 0.9);

    let mut cube = Mesh::cube(&shader, vec3(-5.0, 5.0, -12.0), Quaternion::from(Matrix3::from_value(0.0)), 1.0);

    let mut camera = Camera::default();

    let renderer: render::SimpleRenderer<Mesh> = render::SimpleRenderer::new();

    let mut key_state = HashMap::new();
    let mut mouse_state = MouseState{
        prev_x: 0.0,
        prev_y: 0.0,
        first_mouse: true,
    };
    let mut gamestate = GameState{
        wireframe_mode: false,
        cam_rotate_mode: false,
    };

    let start_time = win.get_time();
    let mut frame_timer = timer::Timer::new(start_time);
    let mut fps_timer = timer::Timer::new(start_time);
    let mut frame_count = 0;
    debug!("Beginning main loop");
    while !win.should_close() {
        let cur_time = win.get_time();
        let elapsed = frame_timer.elapsed(cur_time);
        frame_timer.reset(cur_time);

        // Calculate fps
        frame_count += 1;
        if fps_timer.elapsed(cur_time) > 1.0 {
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

        process_keys(elapsed, &mut win, &mut key_state, &mut gamestate, &mut camera);
        process_mouse(elapsed, &mut win, &mut mouse_state, &mut gamestate, &mut camera);

        // Rotate based on current time, just to show lighting
        cube.rotation = Quaternion::from(Matrix3::from_angle_x(Rad::<f32>((win.get_time() as f32).sin() * 2.0)));
        
        // Render
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            if gamestate.wireframe_mode {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
            
            let mut context = renderer.begin();

            sprites.iter().for_each(|sp| {
                context.submit(sp);
            });
            context.submit(&model);
            context.submit(&cube);
            context.submit(&demo_card);

            context.present(&camera);

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
