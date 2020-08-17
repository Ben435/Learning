mod resources;
mod camera;
mod shader;
mod vertex_constants;
mod images;

use glfw::{Action,Context,Key};
use std::path::Path;
use std::ptr;
use std::sync::mpsc::Receiver;
use std::os::raw::c_void;
use cgmath::prelude::*;
use cgmath::{Matrix4,Deg,vec3,perspective,Point3};

use crate::vertex_constants::*;
use crate::resources::ResourceLoader;
use crate::camera::*;
use crate::shader::*;
use crate::images::*;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

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

    let resource_loader = ResourceLoader::from_relative_exe_path(Path::new("assets")).unwrap();

    let lamp_shader = ProgramBuilder::new(&resource_loader)
        .with_vertex_shader("shaders/camera.vert").unwrap()
        .with_uniform(&String::from("model")).unwrap()
        .with_uniform(&String::from("view")).unwrap()
        .with_uniform(&String::from("projection")).unwrap()
        .with_fragment_shader("shaders/lamp.frag").unwrap()
        .build().unwrap();
    let lighting_shader = ProgramBuilder::new(&resource_loader)
        .with_vertex_shader("shaders/light_map.vert").unwrap()
        .with_uniform(&String::from("model")).unwrap()
        .with_uniform(&String::from("view")).unwrap()
        .with_uniform(&String::from("projection")).unwrap()
        .with_fragment_shader("shaders/light_map.frag").unwrap()
        .with_uniform(&String::from("viewPos")).unwrap()
        .with_uniform(&String::from("material.diffuse")).unwrap()
        .with_uniform(&String::from("material.specular")).unwrap()
        .with_uniform(&String::from("material.shininess")).unwrap()
        .with_uniform(&String::from("light.position")).unwrap()
        .with_uniform(&String::from("light.ambient")).unwrap()
        .with_uniform(&String::from("light.diffuse")).unwrap()
        .with_uniform(&String::from("light.specular")).unwrap()
        .build().unwrap();
    
    println!("Loaded shaders!");

    let vertices = CUBE_VERTICES_WITH_NORM_AND_TEX;

    let mut vbo: gl::types::GLuint = 0;
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::BindVertexArray(vao);
        let stride = (8 * std::mem::size_of::<f32>()) as gl::types::GLsizei;
        
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

        // Normal attrib
        gl::VertexAttribPointer(
            1, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            stride, 
            (3 * std::mem::size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        // Tex attrib
        gl::VertexAttribPointer(
            2, 
            2, 
            gl::FLOAT, 
            gl::FALSE, 
            stride, 
            (6 * std::mem::size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);
    }

    println!("Fancy cube loaded!");

    let cube_vertices = CUBE_VERTICES;
    let mut lamp_vbo: gl::types::GLuint = 0;
    let mut lamp_vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut lamp_vao);
        gl::GenBuffers(1, &mut lamp_vbo);

        gl::BindVertexArray(lamp_vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, lamp_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            (cube_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            cube_vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            (3 * std::mem::size_of::<f32>()) as i32, 
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }

    println!("Loaded lamp!");

    let diffuse_map = unsafe {
        load_texture(&resource_loader.resolve_path("textures/container2.png").unwrap())
    };

    let specular_map = unsafe {
        load_texture(&resource_loader.resolve_path("textures/container2_specular.png").unwrap())
    };

    println!("Loaded diffuse map!");

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

    let lamp_position = vec3(2.0, 0.0, 1.0);

    let mut last_frame_time: f32 = 0.0;
    let mut delta_time: f32;

    let mut camera = Camera {
        position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    let mut first_mouse = true;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.0;

    println!("Initialized!");

    while !window.should_close() {
        let current_frame_time = glfw.get_time() as f32;
        delta_time = current_frame_time - last_frame_time;
        last_frame_time = current_frame_time;

        process_events(&events, &mut first_mouse, &mut last_x, &mut last_y, &mut camera);

        process_input(&mut window, delta_time, &mut camera);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let view: Matrix4<f32> = camera.get_view_matrix();
            let projection: Matrix4<f32> = perspective(Deg(camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);

            {   // Block to isolate lamp_shader usage
                lamp_shader.use_program();
                lamp_shader.set_uniform_matrix4("projection", &projection);
                lamp_shader.set_uniform_matrix4("view", &view);
                let model = Matrix4::from_translation(lamp_position) * Matrix4::from_scale(0.2);
                lamp_shader.set_uniform_matrix4("model", &model);

                gl::BindVertexArray(lamp_vao);
    
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            
            lighting_shader.use_program();

            lighting_shader.set_uniform_vec3("viewPos", &camera.position.to_vec());

            lighting_shader.set_uniform_vec3("light.position", &lamp_position);
            lighting_shader.set_uniform_vec3("light.ambient", &vec3(0.2, 0.2, 0.2));
            lighting_shader.set_uniform_vec3("light.diffuse", &vec3(0.5, 0.5, 0.5));
            lighting_shader.set_uniform_vec3("light.specular", &vec3(1.0, 1.0, 1.0));

            lighting_shader.set_uniform_int("material.diffuse", 0);
            lighting_shader.set_uniform_int("material.specular", 1);
            lighting_shader.set_uniform_float("material.shininess", 64.0);

            lighting_shader.set_uniform_matrix4("projection", &projection);
            lighting_shader.set_uniform_matrix4("view", &view);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, diffuse_map);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, specular_map);

            gl::BindVertexArray(vao);
            for (i, position) in cube_positions.iter().enumerate() {
                let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
                let angle = 2.0*i as f32;
                model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));

                lighting_shader.set_uniform_matrix4("model", &model);
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
