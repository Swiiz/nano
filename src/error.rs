use thiserror::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Window creation error: {0}")]
    WindowCreationError(winit::error::OsError),
    #[error("This error is not from Nano: {0}")]
    Extern(Box<dyn std::error::Error + Send + Sync>),
    #[error("Asset loading error: {0}")]
    AssetLoading(std::io::Error),
}

pub fn exterr(err: impl std::error::Error + Send + Sync + 'static) -> Error {
    Error::Extern(Box::new(err))
}
