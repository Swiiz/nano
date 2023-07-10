use thiserror::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Window creation error: {0}")]
    SurfaceCreation(wgpu::CreateSurfaceError),
    #[error("Device acquisition error: {0}")]
    DeviceAcquisition(wgpu::RequestDeviceError),
    #[error("Surface texture acquisition error: {0}")]
    SurfaceTextureAcquisition(wgpu::SurfaceError),
}

impl From<self::Error> for nano::Error {
    fn from(value: self::Error) -> Self {
        nano::Error::Extern(Box::new(value))
    }
}
