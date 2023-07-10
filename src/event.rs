use winit::window::WindowId;

pub enum Event {
    Draw,
    Update,
    WindowResize {
        window_id: WindowId,
        new_size: (u32, u32),
    },
    CloseRequested {
        window_id: WindowId,
    },
}

impl Event {
    pub(crate) fn maybe_from(event: &winit::event::Event<()>) -> Option<Self> {
        match event {
            winit::event::Event::MainEventsCleared => Some(Self::Update),
            winit::event::Event::RedrawRequested(_) => Some(Self::Draw),
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                window_id,
                ..
            } => Some(Self::CloseRequested {
                window_id: *window_id,
            }),
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::Resized(new_size),
                window_id,
                ..
            } => Some(Self::WindowResize {
                window_id: *window_id,
                new_size: (new_size.width, new_size.height),
            }),
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::ScaleFactorChanged { .. },
                window_id,
                ..
            } => Some(Self::WindowResize {
                window_id: *window_id,
                new_size: (0, 0),
            }),
            _ => None,
        }
    }
}
