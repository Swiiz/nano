use nano::{
    prelude::ParallelExecutor,
    world::{prototype::Prototype, query::Query, World},
};

//TODO: Add documentation for this example

fn main() {
    let mut world = World::new();

    let proto1 = Prototype::new().with(1u32).with(2u64).with(3u128);
    let proto2 = Prototype::new().with(1u32).with(2u64);
    let proto3 = Prototype::new().with(1u32);

    world.insert(proto1);
    world.insert(proto2);
    world.insert(proto3);

    let ex = ParallelExecutor::new().with(&test_sys);

    for _ in 0..10 {
        ex.run(&world);
    }
}

fn test_sys<'a>(query: Query<'a, (&'a u32, &'a mut u64, Option<&'a u128>)>) {
    for (a, b, c) in query {
        println!("{} {} {}", a, b, c.map_or(0, |c| *c));
        *b += 1;
    }
}
