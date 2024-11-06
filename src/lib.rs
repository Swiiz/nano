mod engine;

pub mod deps;
pub mod platform;

use std::any::Any;

pub use anyhow::*;
pub use better_any::{Tid, TidAble};
pub use deps::ReadDeps;
pub use engine::Engine;

pub trait Module: engine::DynModule + Any + Sized {
    type Input<'a>: TidAble<'a>;
    type Dependencies: deps::Dependencies<Self>;

    fn run<'a>(&mut self, input: Input<'a, Self>, deps: Deps<Self>) -> Output<'a>;
}

pub type Input<'a, M> = <M as Module>::Input<'a>;
pub type Deps<'a, T> = <<T as Module>::Dependencies as deps::Dependencies<T>>::ReadOnly<'a>;

#[derive(Default)]
pub struct Output<'a> {
    events: Vec<Box<dyn Tid<'a> + 'a>>,
}

impl<'a> Output<'a> {
    pub(crate) fn iter(self) -> impl Iterator<Item = Box<dyn Tid<'a> + 'a>> {
        self.events.into_iter()
    }

    pub fn with<T: TidAble<'a>>(mut self, input: T) -> Output<'a> {
        self.events.push(Box::new(input) as Box<dyn Tid<'a> + 'a>);
        self
    }
}
