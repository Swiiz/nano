mod error;
mod host;
mod input;
mod window;

pub use error::*;
pub type Result<T> = std::result::Result<T, Error>;

pub use host::*;
pub use input::*;
pub use window::*;
pub type Event<'a> = winit::event::Event<'a, ()>;
pub use winit::event::WindowEvent;
pub use winit::event_loop::ControlFlow;

pub struct Context<'a> {
    pub control_flow: &'a mut ControlFlow,
    pub start: std::time::Instant,
    pub inputs: &'a Inputs,
}

pub trait Game: Sized + 'static {
    fn new(host: &Host) -> Result<Self>;
    fn on(&mut self, context: Context, event: &Event) -> Result<()>;
}

pub fn run<T: Game>() -> Result<()> {
    let engine = Host::new();
    let mut user_state = T::new(&engine)?;
    let start = std::time::Instant::now();
    let mut inputs = Inputs::default();
    engine.window_host.run(move |event, control_flow| {
        inputs.handle(event);
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::CloseRequested = event {
                *control_flow = ControlFlow::Exit;
            }
        }
        user_state.on(
            Context {
                control_flow,
                start,
                inputs: &inputs,
            },
            &event,
        )?;
        Ok(())
    })
}
