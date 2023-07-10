use std::ops::Deref;

use winit::{event::Event, event_loop::ControlFlow};

pub struct WindowHost {
    event_loop: winit::event_loop::EventLoop<()>,
}

pub use winit::window::WindowId;
pub struct Window {
    window: winit::window::Window,
}

impl WindowHost {
    pub fn new() -> Self {
        Self {
            event_loop: winit::event_loop::EventLoop::new(),
        }
    }

    pub fn run(
        self,
        mut handle_event: impl (FnMut(&Event<()>, &mut ControlFlow) -> crate::Result<()>) + 'static,
    ) -> crate::Result<()> {
        self.event_loop.run(move |event, _, control_flow| {
            handle_event(&event, control_flow).unwrap_or_else(|e| {
                eprintln!("Your game errored: {:?}", e);
                *control_flow = ControlFlow::Exit;
            });
        })
    }
}

pub type WindowBuilder = winit::window::WindowBuilder;

impl Window {
    pub(crate) fn new(
        host: &WindowHost,
        wbuilder: impl Fn(WindowBuilder) -> WindowBuilder,
    ) -> crate::Result<Self> {
        let window = wbuilder(WindowBuilder::new())
            .with_title("Made with Nano!")
            .build(&host.event_loop)
            .map_err(crate::Error::WindowCreationError)?;
        Ok(Self { window })
    }

    pub fn inner(&self) -> &winit::window::Window {
        &self.window
    }
}

impl Deref for Window {
    type Target = winit::window::Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
