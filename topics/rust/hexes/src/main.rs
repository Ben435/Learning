mod point_gen;

use cgmath::{Point3};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use wgpu::{
    Instance,
    TextureFormat,
    BackendBit,
    RequestAdapterOptions,
};
use wgpu_subscriber::initialize_default_subscriber;
use futures::executor::block_on;

const CAMERA: Point3<f32> = Point3 {
    x: -100.0,
    y: 50.0,
    z: 100.0,
};

#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
struct Uniforms {
    view: [f32; 16],
    projection: [f32; 16],
    time_size_width: [f32; 4],
}

struct HexesState {
    terrain_vertex_buf: wgpu::Buffer,
    terrain_vertex_count: usize,
    terrain_normal_bind_group: wgpu::BindGroup,
    terrain_pipeline: wgpu::RenderPipeline,

    current_frame: usize,
}

async fn run(event_loop: EventLoop<()>, window: Window) {
    let swapchain_format = TextureFormat::Bgra8UnormSrgb;
    let size = window.inner_size();
    let instance = Instance::new(BackendBit::PRIMARY);
    let surface = unsafe {
        instance.create_surface(&window)
    };

    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find appropriate adapter");

    let (device, queue) = adapter
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

    let vs_module = device.create_shader_module(wgpu::include_spirv!("./assets/shader.vert.spv"));
    let fs_module = device.create_shader_module(wgpu::include_spirv!("./assets/shader.frag.spv"));

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: None,
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[swapchain_format.into()],
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    event_loop.run(move |event, _, control_flow| {
        let _ = (
            &instance,
            &adapter,
            &vs_module,
            &fs_module,
            &pipeline_layout,
        );

        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                sc_desc.width = size.width;
                sc_desc.height = size.height;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            },
            Event::RedrawRequested(_) => {
                let frame = swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.draw(0..12, 0..1);
                }

                queue.submit(Some(encoder.finish()));
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {},
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    initialize_default_subscriber(None);
    block_on(run(event_loop, window));
}
