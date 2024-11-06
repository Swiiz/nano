use anyhow::Context;
use better_any::Tid;
use better_any::TidAble;
use winit::event_loop::ActiveEventLoop;
use winit::{
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop},
};

pub use winit;

use super::Platform;
use crate::{Engine, Result};

#[derive(Default)]
pub struct WindowPlatform;

impl Platform for WindowPlatform {
    fn run(self, mut engine: Engine) -> Result<()> {
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(&mut engine).context("Failed to run app")
    }
}

#[derive(Tid)]
pub struct WindowPlatformEvent<'a> {
    pub event_loop: &'a ActiveEventLoop,
    pub event: WindowPlatformEventContent,
}

pub enum WindowPlatformEventContent {
    DeviceEvent {
        content: winit::event::DeviceEvent,
        device_id: winit::event::DeviceId,
    },
    WindowEvent {
        content: winit::event::WindowEvent,
        window_id: winit::window::WindowId,
    },
    AboutToWait,
    Suspended,
    Exiting,
    MemoryWarning,
    Resumed,
}

impl ApplicationHandler for Engine {
    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.run(WindowPlatformEvent {
            event_loop,
            event: WindowPlatformEventContent::DeviceEvent {
                content: event,
                device_id,
            },
        })
        .unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.run(WindowPlatformEvent {
            event_loop,
            event: WindowPlatformEventContent::WindowEvent {
                content: event,
                window_id,
            },
        })
        .unwrap();
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.run(WindowPlatformEvent {
            event_loop,
            event: WindowPlatformEventContent::AboutToWait,
        })
        .unwrap()
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.run(WindowPlatformEvent {
            event_loop,
            event: WindowPlatformEventContent::Suspended,
        })
        .unwrap()
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.run(WindowPlatformEvent {
            event_loop,
            event: WindowPlatformEventContent::Exiting,
        })
        .unwrap()
    }

    fn memory_warning(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.run(WindowPlatformEvent {
            event_loop,
            event: WindowPlatformEventContent::MemoryWarning,
        })
        .unwrap()
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.run(WindowPlatformEvent {
            event_loop,
            event: WindowPlatformEventContent::Resumed,
        })
        .unwrap()
    }
}
