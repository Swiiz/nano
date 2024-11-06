use crate::{Engine, Result};

pub mod window;

pub trait Platform {
    fn run(self, engine: Engine) -> Result<()>;
}
