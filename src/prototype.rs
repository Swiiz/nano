use std::{any::TypeId, collections::BTreeMap};

use crate::component::Component;

#[derive(Default)]
pub struct Prototype {
    pub(super) components: BTreeMap<TypeId, Box<dyn Component>>,
}

impl Prototype {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with<T: Component + 'static>(mut self, component: T) -> Self {
        self.components
            .insert(TypeId::of::<T>(), Box::new(component));
        self
    }
    pub fn components(&self) -> impl Iterator<Item = &TypeId> {
        self.components.keys()
    }
}
