use thiserror::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Window creation error: {0}")]
    SurfaceCreation(wgpu::CreateSurfaceError),
    #[error("Device acquisition error: {0}")]
    DeviceAcquisition(wgpu::RequestDeviceError),
    #[error("Surface texture acquisition error: {0}")]
    SurfaceTextureAcquisition(wgpu::SurfaceError),
    #[error("Image loading error: {0}")]
    ImageLoading(image::ImageError),
    #[cfg(feature = "imgui")]
    #[error("Imgui error: {0}")]
    Imgui(imgui_wgpu::RendererError),
    #[error("Extern error: {0}")]
    Extern(Box<dyn std::error::Error + Send + Sync>),
}

impl From<self::Error> for nano::Error {
    fn from(value: self::Error) -> Self {
        nano::Error::Extern(Box::new(value))
    }
}

impl From<nano::Error> for self::Error {
    fn from(value: nano::Error) -> Self {
        match value {
            nano::Error::Extern(e) => Self::Extern(e),
            _ => unreachable!(),
        }
    }
}
