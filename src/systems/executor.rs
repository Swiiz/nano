//! This module contains the executor for the systems.
//! An Executor is used to run batches of systems.
//!
//! This crate currently provides two executors:
//! - `SequentialExecutor` runs the systems sequentially.
//! - `ParallelExecutor` runs the systems in parallel.
//!
//! The `ParallelExecutor` is only available if the `parallel` feature is enabled.
//! Also note that the `ParallelExecutor` will only work for Send and Sync Systems / "System Providing Context".

use super::System;

/// A sequential executor for systems.
/// The systems will be run in the order they were added.
pub struct SequentialExecutor<'a, Pr> {
    systems: Vec<Box<dyn Fn(&'a Pr)>>,
}

impl<'a, Pr> SequentialExecutor<'a, Pr> {
    /// Creates a new `SequentialExecutor`.
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    /// Adds a system to the executor.
    /// The system will be run when `run` is called.
    pub fn with<Sys, Params>(mut self, system: &'static Sys) -> Self
    where
        Sys: System<'a, Pr, Params>,
    {
        self.systems.push(Box::new(|pr| system.run(pr)));
        self
    }

    /// Runs the systems.
    /// The systems will be run in the order they were added.
    pub fn run(&self, provider: &'a Pr) {
        for system in &self.systems {
            system(provider);
        }
    }
}

#[cfg(feature = "parallel")]
/// A parallel executor for systems.
/// The systems will be run in parallel in DIFFERENT threads.
pub struct ParallelExecutor<'a, Pr: Send + Sync> {
    systems: Vec<Box<dyn Fn(&'a Pr) + Send + Sync>>,
}

#[cfg(feature = "parallel")]
impl<'a, Pr: Send + Sync> ParallelExecutor<'a, Pr> {
    /// Creates a new `ParallelExecutor`.
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    /// Adds a system to the executor.
    /// The system will be run when `run` is called.
    pub fn with<Sys, Params>(mut self, system: &'static Sys) -> Self
    where
        Sys: System<'a, Pr, Params> + Send + Sync,
    {
        self.systems.push(Box::new(|pr| Sys::run(system, pr)));
        self
    }

    /// Runs the systems.
    /// The systems will be run in parallel in DIFFERENT threads.
    pub fn run(&self, provider: &'a Pr) {
        rayon::prelude::ParallelIterator::for_each(
            rayon::prelude::IntoParallelRefIterator::par_iter(&self.systems),
            |system| system(provider.clone()),
        );
    }
}
