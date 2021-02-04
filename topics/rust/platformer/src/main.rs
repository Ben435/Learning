mod camera;
mod uniforms;
mod vertex;
mod texture;
mod camera_controller;
mod player;
mod sprite_sheet;
mod rect;
mod timer;

use camera::Camera;
use camera_controller::CameraController;
use uniforms::Uniforms;
use vertex::Vertex;
use texture::Texture;
use player::Player;
use sprite_sheet::{SpriteSheet,SpriteSheetFactory};
use rect::Rect;
use timer::Timer;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window,WindowBuilder},
    dpi::LogicalSize,
};
use wgpu::{
    Instance,
    TextureFormat,
    BackendBit,
    RequestAdapterOptions,
    include_spirv,
    util::DeviceExt,
};
use futures::executor::block_on;
use log::{info,error};
use cgmath::{Point3,Vector3,Matrix4};
use std::time::Duration;
use std::collections::VecDeque;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    
    sprite_sheet: SpriteSheet,
    sprite_sheet_factory: SpriteSheetFactory,

    camera: Camera,
    camera_controller: CameraController,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    sprite_num: usize,
}

impl State {
    async fn new(window: &Window) -> Self {
        let swapchain_format = TextureFormat::Bgra8UnormSrgb;
        let size = window.inner_size();
        let instance = Instance::new(BackendBit::PRIMARY);

        let surface = unsafe {
            instance.create_surface(window)
        };

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find appropriate adapter");

        let (device, queue): (wgpu::Device, wgpu::Queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
    
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let sprite_sheet_bytes = include_bytes!("./assets/example-sprite-sheet.png");
        let sprite_sheet_texture = Texture::from_bytes(&device, &queue, sprite_sheet_bytes, Some("Example Sprite Sheet Texture")).unwrap();

        let sprite_sheet_factory = SpriteSheetFactory::new(&device);
        let sprite_sheet = sprite_sheet_factory
            .new_spritesheet()
            .for_texture(sprite_sheet_texture)
            .add_clip_rect(Rect::new(0.0, 0.0, 0.25, 0.25))
            .add_clip_rect(Rect::new(0.25, 0.0, 0.25, 0.25))
            .add_clip_rect(Rect::new(0.5, 0.0, 0.25, 0.25))
            .add_clip_rect(Rect::new(0.75, 0.0, 0.25, 0.25))
            .build(&device)
            .unwrap();
        

        let vs_module = device.create_shader_module(include_spirv!("./assets/shaders/shader.vert.spv"));
        let fs_module = device.create_shader_module(include_spirv!("./assets/shaders/shader.frag.spv"));

        let camera = Camera::new(
            Point3::new(0.0, 0.0, 1.0),
            Vector3::unit_y(),
            sc_desc.width as f32 / sc_desc.height as f32,
            0.1,
            100.0,
        );
        let camera_controller = CameraController::new(1.0);

        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);

        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        );

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("Uniform Bind Group Layout")
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
                }
            ],
            label: Some("Uniform Bind Group"),
        });

        let render_pipeline = State::create_pipeline(
            &device, &sc_desc,
            vs_module, fs_module,
            &sprite_sheet_factory.bind_group_layout, &uniform_bind_group_layout,
        );

        State {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,

            render_pipeline,

            sprite_sheet,
            sprite_sheet_factory,

            camera,
            camera_controller,
            uniforms,
            uniform_buffer,
            uniform_bind_group,

            sprite_num: 0,
        }
    }

    fn create_pipeline(
        device: &wgpu::Device,
        sc_desc: &wgpu::SwapChainDescriptor,
        vs_module: wgpu::ShaderModule,
        fs_module: wgpu::ShaderModule,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    texture_bind_group_layout,
                    uniform_bind_group_layout,
                ],
                push_constant_ranges: &[],
            }
        );

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(
                wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                    clamp_depth: false,
                }
            ),
            color_states: &[
                wgpu::ColorStateDescriptor {
                    format: sc_desc.format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }
            ],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[
                    Vertex::descriptor()
                ],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        })
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            _ if self.camera_controller.process_event(event) => true,
            WindowEvent::KeyboardInput {
                input,
                ..
            } => {
                match input {
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    } => {
                        self.sprite_num = (self.sprite_num + 1) % 4;
                        self.sprite_sheet.set_current_sprite(self.sprite_num);
                        error!("Current sprite: {}", self.sprite_num);
                        true
                    }
                    _ => false,
                }
            }
            _ => false
        }
    }

    fn update(&mut self, delta_frame: Duration) {
        self.camera_controller.update_camera(&mut self.camera, delta_frame);
        self.uniforms.update_view_proj(&self.camera);
        self.uniforms.update_model_proj(Matrix4::from_scale(0.5));
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
    }

    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain
            .get_current_frame()?
            .output;
        
        let mut encoder: wgpu::CommandEncoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass: wgpu::RenderPass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            let sprite = self.sprite_sheet.get_sprite_representation();
            render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, sprite.vertex_buffer.slice(..));
            render_pass.set_index_buffer(sprite.index_buffer.slice(..));
            render_pass.set_bind_group(0, sprite.bind_group, &[]);
            render_pass.draw_indexed(0..sprite.indices_len, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let init_timer = Timer::new();

    let mut state = block_on(State::new(&window));

    info!("Time to initialize: {}ms", init_timer.elapsed().as_millis());

    let mut time_since_last_render_time_log = Timer::new();
    let max_rolling_frame_size = 500;
    let mut rolling_render_times = VecDeque::<u128>::with_capacity(max_rolling_frame_size + 1);
    let mut render_timer = Timer::new();
    let mut frame_timer = Timer::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() =>
                match event {
                    WindowEvent::Resized(size) => state.resize(*size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    e if state.input(e) => {}
                    WindowEvent::KeyboardInput {
                        input,
                        ..
                    } => {
                        match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            Event::RedrawRequested(_) => {
                render_timer.reset();
                let delta_frame = frame_timer.elapsed();
                frame_timer.reset();

                state.update(delta_frame);
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                    Err(wgpu::SwapChainError::OutOfMemory) => {
                        error!("Out of memory, closing");
                        *control_flow = ControlFlow::Exit;
                    },
                    Err(e) => error!("{:?}", e),
                }

                let time_to_render = render_timer.elapsed().as_millis();
                rolling_render_times.push_back(time_to_render);
                rolling_render_times.truncate(max_rolling_frame_size);

                if time_since_last_render_time_log.elapsed().as_millis() > 1000 {
                    let sum: u128 = rolling_render_times.iter().sum();
                    let avg_frame_time = sum / rolling_render_times.len() as u128;
                    time_since_last_render_time_log.reset();
                    info!("Avg render time: {}ms", avg_frame_time);
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    })
}
    
