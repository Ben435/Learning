pub mod resources;
pub mod render_gl;
extern crate sdl2;
extern crate gl;

#[macro_use] extern crate render_gl_derive as render_gl_derive;

use std::path::Path;
use render_gl::{Program};
use render_gl::data;
use resources::ResourceLoader;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pub pos: data::f32x3,
    #[location = "1"]
    clr: data::f32x3,
}


fn main() {
    let sdl = sdl2::init().unwrap();

    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let mut timer_subsystem = sdl.timer().unwrap();

    let window = video_subsystem
        .window("Triangle", 512, 512)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    video_subsystem
        .gl_set_swap_interval(sdl2::video::SwapInterval::VSync)
        .unwrap();

    // Viewport setup
    unsafe {
        gl::Viewport(0, 0, 512, 512);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let resource_loader = ResourceLoader::from_relative_exe_path(Path::new("assets")).unwrap();

    // Shaders
    let shader_program = Program::from_res(&resource_loader, "shaders/triangle").unwrap();

    // Example triangle
    let mut vertices: Vec<Vertex> = vec![
        Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },
        Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() },
        Vertex { pos: (0.0,  0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() },
    ];

    // Inital bind
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::DYNAMIC_DRAW,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // Bind to vao
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        Vertex::vertex_attrib_pointers();

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let mut prev_ticks = 0;
    'main: loop {
        let ticks = timer_subsystem.ticks();
        let ticks_delta = ticks - prev_ticks;
        prev_ticks = ticks;

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                ev => println!("Unknown event: {:?}", ev),
            }
        }
        

        unsafe {
            gl::BindVertexArray(0);
            gl::Clear(gl::COLOR_BUFFER_BIT)
        }
        // Update triangle
        for vert in vertices.iter_mut() {
            vert.pos.d0 += 0.1 * (ticks_delta as f32 / 1000.0);
        }

        // Update vbo
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                3,
            )
        }

        window.gl_swap_window();
    };
}
