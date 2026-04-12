use std::sync::Arc;
use std::time::Instant;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::camera::Camera;
use crate::config;
use crate::control::InputState;
use crate::engine::texture::create_texture_from_gltf;
use crate::light::sun::Sun;
use crate::render::Vertex;
use crate::world::generate_world;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlobalUniform {
    pub view_proj: [[f32; 4]; 4],
    pub sun_dir: [f32; 4],
    pub sun_color: [f32; 4],
    pub ambient_color: [f32; 4],
    pub camera_pos: [f32; 4],
}

pub struct State {
    pub window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    pub camera: Camera,
    sun: Sun,
    global_uniform: GlobalUniform,
    global_buffer: wgpu::Buffer,
    global_bind_group: wgpu::BindGroup,
    texture_bind_group: wgpu::BindGroup,
    depth_texture_view: wgpu::TextureView,
    pub input: InputState,
    last_frame_time: Instant,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false }).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, format: surface_caps.formats[0],
            width: size.width, height: size.height, present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0], view_formats: vec![], desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d { width: surface_config.width, height: surface_config.height, depth_or_array_layers: 1 },
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float, usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Depth Texture"), view_formats: &[],
        });
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let camera = Camera::new();
        let sun = Sun::new();

        let global_uniform = GlobalUniform {
            view_proj: camera.get_view_proj(surface_config.width as f32, surface_config.height as f32),
            sun_dir: [sun.direction.x, sun.direction.y, sun.direction.z, 0.0],
            sun_color: sun.color,
            ambient_color: sun.ambient,
            camera_pos: [camera.pos.x, camera.pos.y, camera.pos.z, 1.0],
        };

        let global_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Global Buffer"), contents: bytemuck::cast_slice(&[global_uniform]), usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let global_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }], label: Some("global_bind_group_layout"),
        });

        let global_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { layout: &global_bind_group_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: global_buffer.as_entire_binding() }], label: Some("global_bind_group") });

        let (vertices, indices, diffuse_image, normal_image, mr_image, ao_image) = generate_world();

        let diffuse_view = create_texture_from_gltf(&device, &queue, diffuse_image, "diffuse", true);
        let normal_view = create_texture_from_gltf(&device, &queue, normal_image, "normal", false);
        let mr_view = create_texture_from_gltf(&device, &queue, mr_image, "metallic_roughness", false);
        let ao_view = create_texture_from_gltf(&device, &queue, ao_image, "ambient_occlusion", false);

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat, address_mode_v: wgpu::AddressMode::Repeat, address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::MipmapFilterMode::Linear, ..Default::default()
        });

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { multisampled: false, view_dimension: wgpu::TextureViewDimension::D2, sample_type: wgpu::TextureSampleType::Float { filterable: true } }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { multisampled: false, view_dimension: wgpu::TextureViewDimension::D2, sample_type: wgpu::TextureSampleType::Float { filterable: true } }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { multisampled: false, view_dimension: wgpu::TextureViewDimension::D2, sample_type: wgpu::TextureSampleType::Float { filterable: true } }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 4, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { multisampled: false, view_dimension: wgpu::TextureViewDimension::D2, sample_type: wgpu::TextureSampleType::Float { filterable: true } }, count: None },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&diffuse_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&normal_view) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&mr_view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::TextureView(&ao_view) },
            ],
            label: Some("texture_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("Shader"), source: wgpu::ShaderSource::Wgsl(include_str!("../render/shader.wgsl").into()) });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"), bind_group_layouts: &[Some(&global_bind_group_layout), Some(&texture_bind_group_layout)], immediate_size: 0,
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"), layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs_main"), buffers: &[Vertex::desc()], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fs_main"), targets: &[Some(wgpu::ColorTargetState { format: surface_config.format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
            primitive: wgpu::PrimitiveState { front_face: wgpu::FrontFace::Ccw, cull_mode: Some(wgpu::Face::Back), ..Default::default() },
            depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: Some(true), depth_compare: Some(wgpu::CompareFunction::Less), stencil: wgpu::StencilState::default(), bias: wgpu::DepthBiasState::default() }),
            multisample: wgpu::MultisampleState::default(), multiview_mask: None, cache: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("VBuf"), contents: bytemuck::cast_slice(&vertices), usage: wgpu::BufferUsages::VERTEX });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("IBuf"), contents: bytemuck::cast_slice(&indices), usage: wgpu::BufferUsages::INDEX });

        Self { window, surface, device, queue, surface_config, render_pipeline, vertex_buffer, index_buffer, num_indices: indices.len() as u32, camera, sun, global_uniform, global_buffer, global_bind_group, texture_bind_group, depth_texture_view, input: InputState::new(), last_frame_time: Instant::now() }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width; self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
            let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor { size: wgpu::Extent3d { width: self.surface_config.width, height: self.surface_config.height, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Depth32Float, usage: wgpu::TextureUsages::RENDER_ATTACHMENT, label: None, view_formats: &[] });
            self.depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        }
    }

    pub fn update(&mut self) {
        let dt = self.last_frame_time.elapsed().as_secs_f32();
        self.last_frame_time = Instant::now();

        self.camera.update_position(dt, &self.input);

        self.global_uniform.view_proj = self.camera.get_view_proj(self.surface_config.width as f32, self.surface_config.height as f32);
        self.global_uniform.sun_dir = [self.sun.direction.x, self.sun.direction.y, self.sun.direction.z, 0.0];
        self.global_uniform.sun_color = self.sun.color;
        self.global_uniform.ambient_color = self.sun.ambient;
        self.global_uniform.camera_pos = [self.camera.pos.x, self.camera.pos.y, self.camera.pos.z, 1.0];

        self.queue.write_buffer(&self.global_buffer, 0, bytemuck::cast_slice(&[self.global_uniform]));
    }

    pub fn render(&mut self) {
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => frame,
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => { self.resize(winit::dpi::PhysicalSize::new(self.surface_config.width, self.surface_config.height)); return; }
            _ => return,
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") });
        let clear_color = wgpu::Color { r: config::CLEAR_COLOR[0], g: config::CLEAR_COLOR[1], b: config::CLEAR_COLOR[2], a: config::CLEAR_COLOR[3] };

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"), color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &view, resolve_target: None, depth_slice: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(clear_color), store: wgpu::StoreOp::Store } })], depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &self.depth_texture_view, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }), timestamp_writes: None, occlusion_query_set: None, multiview_mask: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.global_bind_group, &[]);
            render_pass.set_bind_group(1, &self.texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            
            // FIXED: Set index format to Uint32 to allow indices over 65535 limit
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}