use crate::{Engine, Module};

pub type ReadDeps<T> = <<T as Module>::Dependencies as Dependencies>::ReadOnly;

pub trait Dependencies {
    type ReadOnly;
    fn read_deps(engine: &Engine) -> Self::ReadOnly;
}

impl Dependencies for () {
    type ReadOnly = ();
    fn read_deps(_: &Engine) -> Self::ReadOnly {}
}
