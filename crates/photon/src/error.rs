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
}

impl From<self::Error> for nano::Error {
    fn from(value: self::Error) -> Self {
        nano::Error::Extern(Box::new(value))
    }
}
