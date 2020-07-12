mod resources;
mod camera;
mod shader;

use glfw::{Action,Context,Key};
use std::path::Path;
use std::ptr;
use std::sync::mpsc::Receiver;
use cgmath::prelude::*;
use cgmath::{Matrix4,Deg,vec3,perspective,Point3};

use crate::resources::ResourceLoader;
use crate::camera::*;
use crate::shader::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(SCR_WIDTH, SCR_HEIGHT, "OpenGL 3D", glfw::WindowMode::Windowed)
        .expect("Failed to init glfw window");

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
    
    // Make the window's context current
    window.make_current();

    // Capture mouse
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // Set polling for events
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_pos_polling(true);

    let triangle_vertices: [f32; 180] = [
        // positions       // texture coords
        -0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,

        -0.5, -0.5,  0.5,  0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,

        -0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 0.0,

         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5,  0.5,  0.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  1.0, 1.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,

        -0.5,  0.5, -0.5,  0.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0
    ];

    let resource_loader = ResourceLoader::from_relative_exe_path(Path::new("assets")).unwrap();

    let shader_program = ProgramBuilder::new(&resource_loader)
        .with_vertex_shader("shaders/camera.vert").unwrap()
        .with_fragment_shader("shaders/texture.frag").unwrap()
        .with_uniform(&String::from("model")).unwrap()
        .with_uniform(&String::from("view")).unwrap()
        .with_uniform(&String::from("projection")).unwrap()
        .build().unwrap();

    let mut vbo: gl::types::GLuint = 0;
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            (triangle_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            triangle_vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        let stride = (5 * std::mem::size_of::<f32>()) as gl::types::GLsizei;
        
        // Position attrib
        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            stride, 
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // Texture attrib
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
        gl::EnableVertexAttribArray(1);
    }

    let texture = unsafe {
        let img = image::open(&Path::new("assets/textures/container.jpg")).expect("Failed to load texture");
        let img_buffer = img.into_rgb();
        let (width, height) = (img_buffer.width(), img_buffer.height());
        let raw_data = img_buffer.into_raw();

        let mut tmp = 0;
        gl::GenTextures(1, &mut tmp);
        gl::BindTexture(gl::TEXTURE_2D, tmp);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            width as i32,
            height as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            raw_data.as_ptr() as *const gl::types::GLvoid,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        tmp
    };

    let cube_positions = [
        vec3(0.0, 0.0, 0.0),
        vec3(2.0, 5.0, -15.0),
        vec3(-1.5, -2.2, -2.5),
        vec3(-3.8, -2.0, -12.3),
        vec3(2.4, -0.4, -3.5),
        vec3(-1.7, 3.0, -7.5),
        vec3(1.3, -2.0, -2.5),
        vec3(1.5, 2.0, -2.5),
        vec3(1.5, 0.2, -1.5),
        vec3(-1.3, 1.0, -1.5)
    ];

    let mut last_frame_time: f32 = 0.0;
    let mut delta_time: f32;

    let mut camera = Camera {
        position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    let mut first_mouse = true;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.0;

    while !window.should_close() {
        let current_frame_time = glfw.get_time() as f32;
        delta_time = current_frame_time - last_frame_time;
        last_frame_time = current_frame_time;

        process_events(&events, &mut first_mouse, &mut last_x, &mut last_y, &mut camera);

        process_input(&mut window, delta_time, &mut camera);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::BindTexture(gl::TEXTURE_2D, texture);
            
            shader_program.use_program();

            let view: Matrix4<f32> = camera.get_view_matrix();

            let projection: Matrix4<f32> = perspective(Deg(camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);

            shader_program.set_uniform_matrix4("view", &view);
            shader_program.set_uniform_matrix4("projection", &projection);

            gl::BindVertexArray(vao);
            for (i, position) in cube_positions.iter().enumerate() {
                let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
                let angle = 2.0*i as f32;
                model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));

                shader_program.set_uniform_matrix4("model", &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(
    events: &Receiver<(f64, glfw::WindowEvent)>,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    camera: &mut Camera,
) {
    for (_, event) in glfw::flush_messages(&events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                camera.process_mouse_scroll(yoffset as f32);
            }
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                let (xpos, ypos) = (xpos as f32, ypos as f32);
                if *first_mouse {
                    *last_x = xpos;
                    *last_x = ypos;
                    *first_mouse = false;
                }

                let xoffset = xpos - *last_x;
                let yoffset = *last_y - ypos;

                *last_x = xpos;
                *last_y = ypos;

                camera.process_mouse_movements(xoffset, yoffset, true);
            }
            _ => {},
        }
    }
}

fn process_input(window: &mut glfw::Window, delta_time: f32, camera: &mut Camera) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }

    if window.get_key(Key::W) == Action::Press {
        camera.process_keyboard(CameraMovement::FORWARD, delta_time);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.process_keyboard(CameraMovement::BACKWARD, delta_time);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.process_keyboard(CameraMovement::LEFT, delta_time);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.process_keyboard(CameraMovement::RIGHT, delta_time);
    }
}
