use nano::*;

fn main() -> Result<()> {
    nano::run::<ExampleGame>()
}

struct ExampleGame {
    window: nano::Window,
}
impl nano::Game for ExampleGame {
    fn new(host: &nano::Host) -> nano::Result<Self> {
        let window = host.create_window(|wb| wb)?;

        Ok(Self { window })
    }

    fn on(&mut self, context: Context, event: &nano::Event) -> nano::Result<()> {
        match event {
            nano::Event::Draw => {
                println!("Draw!");
            }
            nano::Event::Update => {
                println!("Update!");
            }
            nano::Event::CloseRequested { window_id } => {
                if window_id == &self.window.id() {
                    context.control_flow.set_exit();
                }
            }
            _ => {}
        }
        Ok(())
    }
}
