//! # Nano
//! Is a simple but flexible and powerful ECS framework.
//! It is designed to be used in games and other applications.
//!
//! ## Features
//! - Simple and easy to use
//! - Flexible
//! - Powerful
//! - Fast
//! - Parallel
//! - No dependencies (except rayon for the `parallel` feature)
//!
//! ## Examples
//!
//! ### Simple
//!
//! ```rust
//! use std::sync::atomic::AtomicUsize;
//!
//! use nano::prelude::*;
//!
//! fn main() {
//!   // Create a new ressources container
//!   let mut resources = Resources::new();
//!   // Insert a resource into the container
//!   resources.insert(AtomicUsize::new(0));
//!   
//!   // Create a new executor and add a system to it
//!   // We could also use test_sys.run(&resources) directly
//!   let ex = ParallelExecutor::new().with(&test_sys);
//!   
//!   ex.run(&resources);
//! }
//!
//! fn test_sys(counter: Res<AtomicUsize>) {
//!   counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
//!   println!(
//!     "1 + {} = {}",
//!     counter.load(std::sync::atomic::Ordering::Relaxed),
//!     counter.load(std::sync::atomic::Ordering::Relaxed) + 1
//!   );
//! }
#![warn(missing_docs)]

pub mod resources;
pub mod runtime;
pub mod systems;
pub mod world;

/// An easy way to import all the types you might need in a classic use case.
pub mod prelude {
    pub use crate::resources::{Res, ResMut, Resources, TLRes, TLResMut, TLResources};
    pub use crate::systems::executor::{ParallelExecutor, SequentialExecutor};
    //TODO: pub use crate::world::{Archetype, Archetypes, Entity, World};
}
