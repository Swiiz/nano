use thiserror::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Window creation error: {0}")]
    WindowCreationError(winit::error::OsError),
    #[error("This error is not from Nano: {0}")]
    Extern(Box<dyn std::error::Error>),
    #[error("Asset loading error: {0}")]
    AssetLoading(std::io::Error),
}
