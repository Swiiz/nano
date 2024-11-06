mod engine;

pub mod deps;
pub mod platform;

use std::any::Any;

pub use anyhow::*;
pub use better_any::{Tid, TidAble};
use deps::ReadDeps;
pub use engine::Engine;

pub type Input<'a, M> = <M as Module>::Input<'a>;

pub trait Module: engine::DynModule + Any {
    type Input<'a>: TidAble<'a>;
    type Dependencies: deps::Dependencies;

    fn run<'a>(&mut self, input: Input<'a, Self>, deps: ReadDeps<Self>) -> Output<'a>;
}

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

#[derive(Tid)]
pub struct NoEvent;
pub use NoEvent as __;
impl From<()> for NoEvent {
    fn from(_: ()) -> Self {
        NoEvent
    }
}
