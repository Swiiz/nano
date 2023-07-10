use crate::{Canvas, Color, Instance};

pub struct ScalingRenderer2d {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    texture: wgpu::Texture,
    texture_bind_group: wgpu::BindGroup,
    pub canvas: Canvas,
    pub background_color: Color,
}

impl ScalingRenderer2d {
    pub fn new(
        graphics: &Instance,
        canvas: Canvas,
        background_color: Color,
    ) -> Result<Self, crate::Error> {
        let shader = wgpu::include_wgsl!("scaling.wgsl");
        let module = graphics.gpu.device.create_shader_module(shader);

        let (texture, texture_bind_group_layout, texture_bind_group) =
            Self::create_texture(graphics, canvas.width, canvas.height);

        let vertex_data: [f32; 16] = [
            //[x, y, u, v]
            -1.0, -1.0, 0.0, 1.0, // bottom left
            1.0, -1.0, 1.0, 1.0, // bottom right
            1.0, 1.0, 1.0, 0.0, // top right
            -1.0, 1.0, 0.0, 0.0, // top left
        ];

        let index_data = [0u16, 1, 2, 0, 2, 3];

        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            &graphics.gpu.device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("scaling_renderer_2d_vertex_buffer"),
                contents: unsafe {
                    // SAFETY: Safe as long as vertex_data is [f32]
                    std::slice::from_raw_parts(
                        vertex_data.as_ptr() as *const u8,
                        vertex_data.len() * std::mem::size_of::<f32>(),
                    )
                },
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let index_buffer = wgpu::util::DeviceExt::create_buffer_init(
            &graphics.gpu.device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("scaling_renderer_2d_index_buffer"),
                contents: unsafe {
                    // SAFETY: Safe as long as index_data is [u16]
                    std::slice::from_raw_parts(
                        index_data.as_ptr() as *const u8,
                        index_data.len() * std::mem::size_of::<u16>(),
                    )
                },
                usage: wgpu::BufferUsages::INDEX,
            },
        );

        let render_pipeline_layout =
            graphics
                .gpu
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("scaling_renderer_2d_pipeline_layout"),
                    bind_group_layouts: &[&texture_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            graphics
                .gpu
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("scaling_renderer_2d_render_pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &module,
                        entry_point: "vs_main",
                        buffers: &[wgpu::VertexBufferLayout {
                            array_stride: 4 * std::mem::size_of::<f32>() as u64,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[
                                wgpu::VertexAttribute {
                                    format: wgpu::VertexFormat::Float32x2,
                                    offset: 0,
                                    shader_location: 0,
                                },
                                wgpu::VertexAttribute {
                                    format: wgpu::VertexFormat::Float32x2,
                                    offset: 2 * std::mem::size_of::<f32>() as u64,
                                    shader_location: 1,
                                },
                            ],
                        }],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: graphics.surface_texture_format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

        Ok(Self {
            vertex_buffer,
            index_buffer,
            render_pipeline,
            texture,
            texture_bind_group,
            canvas,
            background_color,
        })
    }

    fn create_texture(
        graphics: &Instance,
        width: u32,
        height: u32,
    ) -> (wgpu::Texture, wgpu::BindGroupLayout, wgpu::BindGroup) {
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture = graphics
            .gpu
            .device
            .create_texture(&wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba32Float, // Might not work on every device? We need this to avoid conversion between texture_source and the texture when uploading
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("scaling_renderer_2d_texture"),
                view_formats: &[],
            });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = graphics
            .gpu
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                label: Some("scaling_renderer_2d_sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                lod_min_clamp: 0.0,
                lod_max_clamp: 1.0,
                compare: None,
                anisotropy_clamp: 1,
                border_color: None,
            });

        let bind_group_layout =
            graphics
                .gpu
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                            count: None,
                        },
                    ],
                    label: Some("scaling_renderer_2d_texture_bind_group_layout"),
                });

        let bind_group = graphics
            .gpu
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
                label: Some("scaling_renderer_2d_texture_bind_group"),
            });

        (texture, bind_group_layout, bind_group)
    }

    pub fn resize_canvas(&mut self, graphics: &Instance, width: u32, height: u32) {
        if !self.canvas.size_matches(width, height) {
            self.canvas.resize(width, height, self.background_color);
            let (texture, _, bind_group) = Self::create_texture(graphics, width, height);
            self.texture = texture;
            self.texture_bind_group = bind_group;
        }
    }

    pub fn draw(
        &mut self,
        graphics: &Instance,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
    ) {
        graphics.gpu.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // SAFETY: Safe as long as canvas.data is an array of the same size as the texture and is made of 4x 32 float color
            unsafe {
                std::slice::from_raw_parts(
                    self.canvas.data.as_ptr() as *const u8,
                    self.canvas.data.len() * std::mem::size_of::<Color>(),
                )
            },
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * std::mem::size_of::<f32>() as u32 * self.canvas.width),
                rows_per_image: Some(self.canvas.height),
            },
            wgpu::Extent3d {
                width: self.canvas.width,
                height: self.canvas.height,
                depth_or_array_layers: 1,
            },
        );

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scaling_renderer_2d_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.background_color.into()),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        render_pass.draw_indexed(0..6, 0, 0..1);
    }
}
