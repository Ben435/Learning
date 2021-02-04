use wgpu::{
    util::DeviceExt,
};
use crate::rect::Rect;
use crate::texture::Texture;
use crate::vertex::Vertex;

pub struct SpriteSheetFactory {
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl SpriteSheetFactory {
    pub fn new(device: &wgpu::Device) -> SpriteSheetFactory {
        SpriteSheetFactory {
            bind_group_layout: device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: wgpu::TextureViewDimension::D2,
                                component_type: wgpu::TextureComponentType::Uint,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler {
                                comparison: false,
                            },
                            count: None,
                        }
                    ],
                    label: Some("SpriteSheet Texture Bind Layout")
                }
            ),
        }
    }

    pub fn new_spritesheet(&self) -> SpriteSheetBuilder {
        SpriteSheetBuilder::new(&self)
    }
}

pub struct SpriteSheet {
    texture_atlas: Texture,
    texture_atlas_bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer, // bulk buffer for all vertices
    index_buffers: Vec<wgpu::Buffer>, // 1 buffer per clipped rect
    vertices: Vec<Vertex>,
    indices: Vec<Vec<u16>>,
    current_sprite_index: usize,
}

impl SpriteSheet {
    pub fn set_current_sprite(&mut self, index: usize) {
        self.current_sprite_index = index;
    }

    pub fn get_sprite_representation(&self) -> SpriteRepresentation {
        return SpriteRepresentation {
            bind_group: &self.texture_atlas_bind_group,
            vertex_buffer: & self.vertex_buffer,
            index_buffer: &self.index_buffers.get(self.current_sprite_index).unwrap(),
            indices_len: self.indices.get(self.current_sprite_index).map(|vec| vec.len()).unwrap() as u32,
        };
    }
}

pub struct SpriteRepresentation<'a> {
    pub bind_group: &'a wgpu::BindGroup,
    pub vertex_buffer: &'a wgpu::Buffer,
    pub index_buffer: &'a wgpu::Buffer,
    pub indices_len: u32,
}

pub struct SpriteSheetBuilder<'a> {
    parent_factory: &'a SpriteSheetFactory,
    clip_rects: Vec<Rect>,
    texture: Option<Texture>,
    label: Option<String>,
}

pub struct SpriteSheetRow {
    
}

impl <'a> SpriteSheetBuilder<'a> {
    pub fn new(parent_factory: &'a SpriteSheetFactory) -> SpriteSheetBuilder<'a> {
        SpriteSheetBuilder {
            parent_factory,
            clip_rects: Vec::new(),
            texture: None,
            label: None,
        }
    }

    pub fn add_clip_rect(mut self, clip_rect: Rect) -> SpriteSheetBuilder<'a> {
        self.clip_rects.push(clip_rect);
        self
    }

    pub fn for_texture(mut self, texture: Texture) -> SpriteSheetBuilder<'a> {
        self.texture = Some(texture);
        self
    }

    pub fn build(self, device: &wgpu::Device) -> Option<SpriteSheet> {
        // Validation
        if self.texture.is_none() {
            return None;
        } else if self.clip_rects.len() <= 0 {
            return None;
        }

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<Vec<u16>> = Vec::new();
        for (index, rect) in self.clip_rects.iter().enumerate() {
            let top_left = Vertex {
                position: [-1.0, 1.0, 0.0],
                tex_coords: [rect.origin.x, rect.origin.y],
            };

            let top_right = Vertex {
                position: [1.0, 1.0, 0.0],
                tex_coords: [rect.origin.x + rect.width, rect.origin.y],
            };

            let bottom_right = Vertex {
                position: [1.0, -1.0, 0.0],
                tex_coords: [rect.origin.x + rect.width, rect.origin.y + rect.height],
            };

            let bottom_left = Vertex {
                position: [-1.0, -1.0, 0.0],
                tex_coords: [rect.origin.x, rect.origin.y + rect.height],
            };

            vertices.extend([top_left, top_right, bottom_right, bottom_left].iter());
            let offset = index as u16 * 4; // 4 vertices per rect.
            let mut new_indices = Vec::new();
            new_indices.extend([
                offset, offset + 2, offset + 1,
                offset, offset + 3, offset + 2,
            ].iter());
            indices.push(new_indices);
        }


        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let mut index_buffers: Vec<wgpu::Buffer> = Vec::new();
        for rect_indices in indices.iter() {
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(rect_indices.as_slice()),
                usage: wgpu::BufferUsage::INDEX,
            });

            index_buffers.push(index_buffer);
        }

        let texture = self.texture.unwrap();

        let texture_atlas_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &self.parent_factory.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    },
                ],
                label: Some("Tree Texture Bind Group")
            }
        );

        Some(SpriteSheet {
            texture_atlas: texture,
            texture_atlas_bind_group,
            vertex_buffer,
            index_buffers,
            vertices,
            indices,
            current_sprite_index: 0,
        })
    }
}
