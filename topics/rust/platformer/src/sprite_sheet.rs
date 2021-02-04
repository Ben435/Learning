use wgpu::{
    util::DeviceExt,
};
use crate::rect::Rect;
use crate::texture::Texture;
use crate::vertex::Vertex;
use log::{info};

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
    index_buffers: Vec<Vec<wgpu::Buffer>>, // 1 vec per row, 1 buffer per clipped rect in each row
    vertices: Vec<Vertex>,
    indices: Vec<Vec<Vec<u16>>>,
}

impl SpriteSheet {
    pub fn get_sprite_representation(&self, row: usize, frame: usize) -> SpriteRepresentation {
        let index_buffer = self.index_buffers.get(row).map(|row_buffers| row_buffers.get(frame)).flatten().unwrap();
        let indices_len = self.indices.get(row).map(|row| row.get(frame)).flatten().map(|indices| indices.len()).unwrap();
        return SpriteRepresentation {
            bind_group: &self.texture_atlas_bind_group,
            vertex_buffer: & self.vertex_buffer,
            index_buffer: &index_buffer,
            indices_len: indices_len as u32,
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
    rows: Vec<Vec<Rect>>,
    texture: Option<Texture>,
    label: Option<String>,
}

pub struct SpriteSheetRowBuilder {
    clip_rects: Vec<Rect>,
}

impl <'a> SpriteSheetRowBuilder {
    pub fn new() -> SpriteSheetRowBuilder {
        SpriteSheetRowBuilder {
            clip_rects: Vec::new(),
        }
    }

    pub fn divide_to_columns(row_index: u32, cell_height: f32, columns: usize) -> SpriteSheetRowBuilder {
        let height_offset = row_index as f32 * cell_height;
        let cell_width = 1.0 / columns as f32;
        SpriteSheetRowBuilder {
            clip_rects: (0..columns).map(|index| Rect::new(index as f32 * cell_width, height_offset, cell_width, cell_height)).collect(),
        }
    }

    pub fn add_clip_rect(mut self, clip_rect: Rect) -> SpriteSheetRowBuilder {
        self.clip_rects.push(clip_rect);
        self
    }

    pub fn finish_row(self) -> Vec<Rect> {
        self.clip_rects
    }
}

impl <'a> SpriteSheetBuilder<'a> {
    pub fn new(parent_factory: &'a SpriteSheetFactory) -> SpriteSheetBuilder<'a> {
        SpriteSheetBuilder {
            parent_factory,
            rows: Vec::new(),
            texture: None,
            label: None,
        }
    }

    pub fn add_row(mut self, row: Vec<Rect>) -> Self {
        self.rows.push(row);
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
        } else if self.rows.len() <= 0 {
            return None;
        }

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<Vec<Vec<u16>>> = Vec::new();
        let mut index_buffers: Vec<Vec<wgpu::Buffer>> = Vec::new();
        for (row_index, row) in self.rows.iter().enumerate() {
            let mut row_indices = Vec::new();
            let mut row_index_buffers = Vec::new();
            for (col_index, rect) in row.iter().enumerate() {
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
                let offset = 
                    (row_index as u16 * 4 * 4) // TODO: 4 cols per row is a hardcode hack. Extend to more sprite sheets.
                    + (col_index as u16 * 4); // 4 vertices per rect.
                let mut new_indices = Vec::new();
                new_indices.extend([
                    offset, offset + 2, offset + 1,
                    offset, offset + 3, offset + 2,
                ].iter());

                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Index Buffer[{}][{}]", row_index, col_index)),
                    contents: bytemuck::cast_slice(new_indices.as_slice()),
                    usage: wgpu::BufferUsage::INDEX,
                });
    
                row_indices.push(new_indices);
                row_index_buffers.push(index_buffer);
            }
            indices.push(row_indices);
            index_buffers.push(row_index_buffers);
        }


        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });

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

        info!("Creating sprite sheet with {} rows", indices.len());

        Some(SpriteSheet {
            texture_atlas: texture,
            texture_atlas_bind_group,
            vertex_buffer,
            index_buffers,
            vertices,
            indices,
        })
    }
}
