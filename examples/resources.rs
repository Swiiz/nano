use nano::prelude::*;

fn main() {
    // Resources are a way to store data that can be accessed by systems.
    // They are similar to global variables or singletons.
    // As there can be only one instance of a resource,
    // you can't have multiple resources of the same type in the same Resources instance.

    let mut resources = Resources::new();

    resources.insert("Hello world!"); // Insert the &str resource into the container

    let ex = ParallelExecutor::new() // Create a new executor and add a system to it
        .with(&test_sys); // We could also use test_sys.run(&resources) directly

    // Run the executor
    ex.run(&resources);
}

// We can use the Res<T> or ResMut<T> types to access resources
// Accessing the same resource multiple times will panic at runtime
fn test_sys(value: ResMut<&str>) {
    println!("{}", *value);
}
