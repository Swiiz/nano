use crate::window::*;

pub struct Host {
    pub(crate) window_host: WindowHost,
}

impl Host {
    pub fn new() -> Self {
        Self {
            window_host: WindowHost::new(),
        }
    }

    pub fn create_window(
        &self,
        wbuilder: impl Fn(WindowBuilder) -> WindowBuilder,
    ) -> crate::Result<Window> {
        Ok(Window::new(&self.window_host, wbuilder)?)
    }
}
