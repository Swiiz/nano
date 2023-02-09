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
//! use nano::{
//!   resources::{Res, Ressources},  
//!   systems::executor::ParallelExecutor,
//! };
//!
//! fn main() {
//!   let mut resources = Ressources::new();
//!   resources.insert(AtomicUsize::new(0));
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
pub mod systems;
pub mod world;

/// An easy way to import all the types you might need in a classic use case.
pub mod prelude {
    pub use crate::resources::{Res, ResMut, Ressources, TLRes, TLResMut, TLRessources};
    pub use crate::systems::executor::{ParallelExecutor, SequentialExecutor};
    //TODO: pub use crate::world::{Archetype, Archetypes, Entity, World};
}
