mod error;
mod event;
mod host;
mod input;
mod window;

pub use error::*;
pub type Result<T> = std::result::Result<T, Error>;

pub use event::*;
pub use host::*;
pub use input::*;
pub use window::*;
pub use winit::event_loop::ControlFlow;

pub struct Context<'a> {
    pub control_flow: &'a mut ControlFlow,
    pub start: std::time::Instant,
    /// Is equal to start if the first update has not been called yet.
    pub last_update: std::time::Instant,
    /// Is equal to start if the first draw has not been called yet.
    pub last_draw: std::time::Instant,
    pub inputs: &'a Inputs,
}

pub trait Game: Sized + 'static {
    fn new(host: &Host) -> Result<Self>;
    fn on(&mut self, context: Context, event: &Event) -> Result<()>;
}

pub fn run<T: Game>() -> Result<()> {
    let engine = Host::new();
    let mut user_state = T::new(&engine)?;
    let (start, mut last_update, mut last_draw) = (
        std::time::Instant::now(),
        std::time::Instant::now(),
        std::time::Instant::now(),
    );
    let mut inputs = Inputs::default();
    engine.window_host.run(move |wevent, control_flow| {
        inputs.handle(wevent);
        Ok(if let Some(event) = Event::maybe_from(wevent) {
            user_state.on(
                Context {
                    control_flow,
                    start,
                    last_update,
                    last_draw,
                    inputs: &inputs,
                },
                &event,
            )?;
            if let Some(to_refresh) = match event {
                Event::Update => Some(&mut last_update),
                Event::Draw => Some(&mut last_draw),
                _ => None,
            } {
                *to_refresh = std::time::Instant::now();
            };
        } else {
            //println!("Unknown event: {:?}", wevent);
        })
    })
}
