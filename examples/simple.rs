use std::sync::atomic::AtomicUsize;

use nano::{
    resources::{Res, Ressources},
    systems::executor::ParallelExecutor,
};

fn main() {
    let mut resources = Ressources::new();
    resources.insert(AtomicUsize::new(0));

    let ex = ParallelExecutor::new().with(&test_sys);

    ex.run(&resources);
}

fn test_sys(counter: Res<AtomicUsize>) {
    counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    println!(
        "1 + {} = {}",
        counter.load(std::sync::atomic::Ordering::Relaxed),
        counter.load(std::sync::atomic::Ordering::Relaxed) + 1
    );
}
