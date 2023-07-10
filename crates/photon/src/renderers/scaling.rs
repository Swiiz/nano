
use crate::{Color, Instance, Canvas};

pub struct ScalingRenderer2d {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    texture: wgpu::Texture,
    texture_bind_group: wgpu::BindGroup,
    pub config: ScalingRenderer2dConfig,
}

pub struct ScalingRenderer2dConfig {
    pub background_color: Color,
    /// How many time smaller should the texture_source len be compared to width * height
    pub upsampling_ratio: u32,
    pub texture_source: Option<Box<[Color]>>
}

impl Default for ScalingRenderer2dConfig {
    fn default() -> Self {
        Self {
            background_color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            upsampling_ratio: 1,
            texture_source: None,
        }
    }
}

impl ScalingRenderer2d {
    pub fn new(ctx: &Instance, mut config: ScalingRenderer2dConfig) -> Result<Self, crate::Error> {
        let shader = wgpu::include_wgsl!("scaling.wgsl");
        let module = ctx.gpu.device.create_shader_module(shader);

        let (texture, texture_bind_group_layout, texture_bind_group) = Self::create_texture(ctx, &mut config);

        let vertex_data: [f32; 16] = [
            //[x, y, u, v]
            -1.0, -1.0, 0.0, 1.0, // bottom left
            1.0, -1.0, 1.0, 1.0, // bottom right
            1.0, 1.0, 1.0, 0.0, // top right
            -1.0, 1.0, 0.0, 0.0, // top left
        ];

        let index_data = [0u16, 1, 2, 0, 2, 3];

        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            &ctx.gpu.device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("scaling_renderer_2d_vertex_buffer"),
                contents: 
                // SAFETY: Safe as long as vertex_data is [f32]
                unsafe {
                    std::slice::from_raw_parts(
                        vertex_data.as_ptr() as *const u8,
                        vertex_data.len() * std::mem::size_of::<f32>(),
                    )
                },
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let index_buffer = wgpu::util::DeviceExt::create_buffer_init(
            &ctx.gpu.device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("scaling_renderer_2d_index_buffer"),
                contents: 
                // SAFETY: Safe as long as index_data is [u16]
                unsafe {
                    std::slice::from_raw_parts(
                        index_data.as_ptr() as *const u8,
                        index_data.len() * std::mem::size_of::<u16>(),
                    )
                },
                usage: wgpu::BufferUsages::INDEX,
            },
        );

        let render_pipeline_layout =
            ctx.gpu
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("scaling_renderer_2d_pipeline_layout"),
                    bind_group_layouts: &[&texture_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            ctx.gpu
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
                            format: ctx.surface_texture_format,
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
            config,
        })
    }

    fn need_resize_size(ctx: &Instance, config: &ScalingRenderer2dConfig) -> (bool, (u32, u32)) {
        let (w_width, w_height): (u32, u32) = ctx.window.inner_size().into();
        let tex_dims = (w_width / config.upsampling_ratio, w_height / config.upsampling_ratio);
        let pixel_count = (tex_dims.0 * tex_dims.1) as usize;
        (config.texture_source.is_none() || config.texture_source.as_ref().unwrap().len() != pixel_count, tex_dims)
    }

    fn create_texture(ctx: &Instance, config: &mut ScalingRenderer2dConfig) -> (wgpu::Texture, wgpu::BindGroupLayout, wgpu::BindGroup) {
        let (need_resize, (width, height)) = Self::need_resize_size(ctx, config);
        let pixel_count = (width * height) as usize;
        match &mut config.texture_source {
            None => {
                config.texture_source = Some(vec![Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }; pixel_count].into_boxed_slice());
                
            }
            Some(array) => {
                if need_resize {
                    let mut vec = array.clone().into_vec();
                    vec.resize(pixel_count, Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0});
                    *array = vec.into_boxed_slice();
                }
            }
        }
        
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture = ctx.gpu.device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba32Float, // Might not work on every device? We need this to avoid conversion between texture_source and the texture when uploading
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("scaling_renderer_2d_texture"),
                view_formats: &[],
            }
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = ctx.gpu.device.create_sampler(&wgpu::SamplerDescriptor {
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
            ctx.gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

            let bind_group = ctx.gpu.device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        }
                    ],
                    label: Some("scaling_renderer_2d_texture_bind_group"),
                }
            );

        (texture, bind_group_layout, bind_group)
    }


    pub fn draw(&mut self, ctx: &Instance, encoder: &mut wgpu::CommandEncoder, target: &wgpu::TextureView) {
        let (need_resize, (width, height)) = Self::need_resize_size(ctx, &self.config);
        if need_resize {
            self.texture = Self::create_texture(ctx, &mut self.config).0;
        }
        let texture_source = self.config.texture_source.as_ref().expect("Self::create_texture should initialize self.config.texture_source to Some()");
        ctx.gpu.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // SAFETY: Safe as long as self.config.texture_source is an array of the same size as the texture and is made of 4x 32 float color
            unsafe {
                std::slice::from_raw_parts(
                    texture_source.as_ptr() as *const u8,
                    texture_source.len() * std::mem::size_of::<Color>(),
                )
            },
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * std::mem::size_of::<f32>() as u32 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scaling_renderer_2d_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.config.background_color.into()),
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

    pub fn canvas(&mut self, ctx: &Instance) -> Canvas {
        let (need_resize, (width, height)) = Self::need_resize_size(ctx, &self.config);
        if need_resize {
            self.texture = Self::create_texture(ctx, &mut self.config).0;
        }
        let texture_source = self.config.texture_source.as_mut().expect("Self::create_texture should initialize self.config.texture_source to Some()");
        Canvas::from_texture_source(texture_source, width, height)
    }
}
