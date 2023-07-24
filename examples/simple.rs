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

    fn on(&mut self, _: Context, event: &nano::Event) -> nano::Result<()> {
        match event {
            nano::Event::RedrawRequested(_) => {
                println!("Draw!");
            }
            nano::Event::MainEventsCleared => {
                println!("Update!");
            }
            _ => {}
        }
        Ok(())
    }
}
