use deps::ReadDeps;
use nano7::*;
use platform::{
    window::{
        winit::{event::WindowEvent, window::WindowAttributes},
        WindowPlatform, WindowPlatformEvent, WindowPlatformEventContent,
    },
    Platform,
};

fn main() -> Result<()> {
    let mut engine = Engine::new();

    engine.add_module(ExampleModule::default());

    WindowPlatform.run(engine)
}

#[derive(Default)]
pub struct ExampleModule {
    window: Option<winit::window::Window>,
}

impl Module for ExampleModule {
    type Input<'a> = WindowPlatformEvent<'a>;
    type Dependencies = ();

    fn run<'a>(&mut self, input: Input<'a, Self>, deps: ReadDeps<Self>) -> Output<'a> {
        match input.event {
            WindowPlatformEventContent::Resumed => {
                self.window.replace(
                    input
                        .event_loop
                        .create_window(WindowAttributes::default().with_title("Nano example!"))
                        .unwrap(),
                );
            }
            WindowPlatformEventContent::WindowEvent {
                content,
                window_id: _, // Single window so not needed
            } => {
                let Some(window) = self.window.as_mut() else {
                    return Output::default();
                };

                match content {
                    WindowEvent::CloseRequested => {
                        input.event_loop.exit();
                    }
                    WindowEvent::RedrawRequested => {
                        // Render here...

                        window.request_redraw();
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        Output::default() //TODO: output
    }
}
