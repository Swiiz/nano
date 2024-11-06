use deps::ReadDeps;
use nano7::*;

fn main() -> Result<()> {
    let mut engine = Engine::new();

    engine.add_module(ExampleModule::default());

    engine.run(StartEvent)
}

#[derive(Tid)]
pub struct StartEvent;

#[derive(Default)]
pub struct ExampleModule {}

impl Module for ExampleModule {
    type Input<'a> = StartEvent;
    type Dependencies = ();

    fn run<'a>(&mut self, _input: Input<'a, Self>, deps: ReadDeps<Self>) -> Output<'a> {
        println!("Hello World!");
        Output::default()
    }
}
