use nano::prelude::*;

pub struct Counter {
    value: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn display(&self) {
        println!("Counter: {}", self.value);
    }
}

fn main() {
    let mut world = World::new();

    world.insert_resource(Counter::new());

    let entity = world.create_entity(Prototype::new().with(0u32));
    println!("{:?}", entity);

    loop {
        let result = world.run(increment_until_10);

        if let Err(error) = result {
            println!("App terminated: {}", error);
            break;
        }
    }
}

fn increment_until_10(mut counter: ResMut<Counter>) -> Result {
    if counter.value() != 10 {
        counter.increment();
        counter.display();
        Ok(())
    } else {
        Err("Counter is at 10".into())
    }
}
