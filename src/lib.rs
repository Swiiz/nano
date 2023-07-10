mod error;
mod event;
mod host;
mod window;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub use host::Host;

pub use window::Window;

pub use event::Event;

pub use winit::event_loop::ControlFlow;

pub struct Context<'a> {
    pub control_flow: &'a mut ControlFlow,
}

pub trait Game: Sized + 'static {
    fn new(host: &Host) -> Result<Self>;
    fn on(&mut self, context: Context, event: Event) -> Result<()>;
}

pub fn run<T: Game>() -> Result<()> {
    let engine = Host::new();
    let mut user_state = T::new(&engine)?;
    engine.window_host.run(move |wevent, control_flow| {
        Ok(if let Some(event) = Event::maybe_from(wevent) {
            user_state.on(Context { control_flow }, event)?
        } else {
            //println!("Unknown event: {:?}", wevent);
        })
    })
}
