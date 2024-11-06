use nano7::*;

fn main() -> Result<()> {
    let mut engine = Engine::new();

    engine.add_module(ExampleMessageModule::default());
    engine.add_module(ExamplePrintModule::default());

    engine.run(StartEvent)
}

#[derive(Tid)]
pub struct StartEvent;
#[derive(Tid)]
pub struct PrintEvent;

#[derive(Default)]
pub struct ExampleMessageModule {
    pub message: String,
}

impl Module for ExampleMessageModule {
    type Input<'a> = StartEvent;
    type Dependencies = ();

    fn run<'a>(&mut self, _: Input<Self>, _: Deps<Self>) -> Output<'a> {
        self.message = "Hello world!".to_string();
        Output::default().with(PrintEvent)
    }
}

#[derive(Default)]
pub struct ExamplePrintModule;

impl Module for ExamplePrintModule {
    type Input<'a> = PrintEvent;
    type Dependencies = (ExampleMessageModule,);

    fn run<'a>(&mut self, _: Input<Self>, deps: Deps<Self>) -> Output<'a> {
        println!(
            "Using dependecy type (small overhead):\n{}",
            deps.get::<ExampleMessageModule>().message
        );

        let (msg_module,) = deps;
        println!("Using dependency tuple order:\n{}", msg_module.message);
        Output::default()
    }
}
