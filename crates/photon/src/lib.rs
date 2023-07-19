mod canvas;
mod error;

use std::ops::Deref;

pub use canvas::*;
pub use error::*;
pub use wgpu;
pub mod renderers;

pub struct Gpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub type SyncInstance = Instance<std::sync::Arc<nano::Window>>;
pub struct Instance<W: std::ops::Deref<Target = nano::Window> = std::rc::Rc<nano::Window>> {
    pub gpu: Gpu,
    window: W,
    surface: wgpu::Surface,
    surface_texture_format: wgpu::TextureFormat,
    surface_caps: wgpu::SurfaceCapabilities,
}

impl<W: std::ops::Deref<Target = nano::Window>> Instance<W> {
    pub fn new(window: W) -> Result<Self, Error> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        // SAFETY: This is safe as long as the window pointer (W) is alive.
        let surface =
            unsafe { instance.create_surface(window.inner()) }.map_err(Error::SurfaceCreation)?;
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        ))
        .map_err(Error::DeviceAcquisition)?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_texture_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        Ok(Self {
            gpu: Gpu { device, queue },
            window,
            surface,
            surface_caps,
            surface_texture_format,
        })
    }

    pub fn resize_surface(&self, new_size: (u32, u32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.surface.configure(
                &self.gpu.device,
                &wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: self.surface_texture_format,
                    width: new_size.0,
                    height: new_size.1,
                    present_mode: self.surface_caps.present_modes[0],
                    alpha_mode: self.surface_caps.alpha_modes[0],
                    view_formats: vec![],
                },
            );
        }
    }

    pub fn render(&mut self, renderfunc: impl FnOnce(&Instance<W>, Frame<W>)) -> Result<(), Error> {
        let (w, h): (u32, u32) = self.window.inner_size().into();
        if w == 0 || h == 0 {
            return Ok(());
        }
        let surface_texture = self.surface.get_current_texture().or_else(|e| {
            match e {
                wgpu::SurfaceError::OutOfMemory => {
                    panic!("NANO PHOTON: The system is out of memory!")
                }
                _ => {
                    self.resize_surface(self.window.inner().inner_size().into());
                }
            }
            self.surface
                .get_current_texture()
                .map_err(Error::SurfaceTextureAcquisition)
        })?;
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        (renderfunc)(
            self,
            Frame {
                encoder: &mut encoder,
                view: &view,
                context: self,
            },
        );
        self.gpu.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
        Ok(())
    }
}

pub struct Frame<'a, W: Deref<Target = nano::Window>> {
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub view: &'a wgpu::TextureView,
    pub context: &'a Instance<W>,
}
