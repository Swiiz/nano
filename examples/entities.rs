use nano::{
    prelude::ParallelExecutor,
    world::{prototype::Prototype, query::Query, World},
};

fn main() {
    let mut world = World::new();

    let proto = Prototype::new().with(1u32).with(2u64).with(3u128);

    world.insert(proto);

    let ex = ParallelExecutor::new().with(&test_sys);

    for _ in 0..10 {
        ex.run(&world);
    }
}

fn test_sys<'a>(query: Query<'a, (&'a u32, &'a mut u64)>) {
    for (a, b) in query {
        println!("{} {}", a, b);
        *b += 1;
    }
}
