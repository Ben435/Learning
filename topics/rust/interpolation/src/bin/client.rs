use std::net::UdpSocket;
use std::io::Result;
use interpolation::network::messages::{ServerUpdate,ClientUpdate};
use interpolation::network::serialization::{Message};
use interpolation::network::constants::{SERVER_ADDR,MAX_MESSAGE_SIZE};
use bincode::{serialize,deserialize};
use simple_opengl_renderer::render::*;
use simple_opengl_renderer::window::*;
use simple_opengl_renderer::camera::*;
use log::{debug,info,LevelFilter};
use env_logger::{Builder};
use std::ffi::CString;
use gl;
use cgmath::{Matrix4,vec3,Vector3,ElementWise};
use glfw::{self,Key,Action};

pub fn main() -> Result<()> {
    Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();
    info!("Logger initialized");

    let mut window = Window::new("Simple Renderer", 800, 600).expect("Failed to init window");
    let cam = Camera::default();
    let renderer = SimpleRenderer::<GlMesh>::new();

    let frag_shader = include_str!("../assets/shader.frag");
    let vert_shader = include_str!("../assets/shader.vert");

    let shader = GlShader::builder()
        .with_frag_shader(CString::new(frag_shader).expect("Failed to convert frag shader to CString"))
        .with_vert_shader(CString::new(vert_shader).expect("Failed to convert vert shader to CString"))
        .build();

    let cube = GlMesh::cube();
    let mut client_cube_position: Vector3<f32> = vec3(0.0, 0.0, -10.0);
    let mut server_cube_position: Vector3<f32> = vec3(0.0, 0.0, -10.0);
    let mut cur_time = window.get_time();

    let sock = UdpSocket::bind("127.0.0.1:0")?;

    sock.connect(SERVER_ADDR)?;

    while !window.should_close() {
        for (_, event) in window.flush_events() {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    debug!("Resize to {}, {}", width, height);
                    unsafe { gl::Viewport(0, 0, width, height) }
                },
                _ => {},
            }
        };

        let new_time = window.get_time();
        let frame_delta_time = new_time - cur_time;
        cur_time = new_time;

        client_cube_position = update_from_keys(&window, client_cube_position, frame_delta_time);

        // Send msg
        let msg = ClientUpdate{
            position: client_cube_position,
        };

        let wrapped_msg = Message::client_update(msg);

        let serial_msg = serialize(&wrapped_msg).expect("Failed to serialize");

        sock.send(serial_msg.as_slice())?;

        // Receive response
        let mut recv_buf: [u8; MAX_MESSAGE_SIZE] = [0; MAX_MESSAGE_SIZE];
        let (_amount_read, _sender) = sock.recv_from(&mut recv_buf)?;

        let recv_msg = deserialize::<Message>(&recv_buf).unwrap();

        match recv_msg {
            Message::ServerUpdate(server_update) => {
                // Explicit move, just to get it working.
                server_cube_position = server_update.positions.get(0).unwrap().position;
            },
            _ => panic!("Wrong message from server!"),
        }
        

        {
            let mut ctx = renderer.begin();

            ctx.submit(&cube, Matrix4::from_translation(server_cube_position), &shader);

            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            ctx.present(&cam);
        }

        gl_errors::check_gl_error();

        window.update_screen();
    }

    Ok(())
}

fn update_from_keys(window: &Window, prev_pos: Vector3<f32>, frame_delta_time: f64) -> Vector3<f32> {
    let new_pos = prev_pos;
    let time_scale = frame_delta_time as f32;
    let move_speed = 10.0;

    let new_pos = match window.window.get_key(Key::Up) {
        Action::Press => new_pos.add_element_wise(vec3(0.0, move_speed * time_scale, 0.0)),
        _ => new_pos,
    };

    let new_pos = match window.window.get_key(Key::Down) {
        Action::Press => new_pos.add_element_wise(vec3(0.0, -move_speed * time_scale, 0.0)),
        _ => new_pos,
    };

    let new_pos = match window.window.get_key(Key::Right) {
        Action::Press => new_pos.add_element_wise(vec3(move_speed * time_scale, 0.0, 0.0)),
        _ => new_pos,
    };

    let new_pos = match window.window.get_key(Key::Left) {
        Action::Press => new_pos.add_element_wise(vec3(-move_speed * time_scale, 0.0, 0.0)),
        _ => new_pos,
    };
    
    new_pos
}
