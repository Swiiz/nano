use std::{any::TypeId, collections::BTreeMap};

use crate::component::Component;

pub struct Prototype {
    pub components: BTreeMap<TypeId, Box<dyn Component>>,
}

impl Prototype {
    pub fn new() -> Self {
        Self {
            components: BTreeMap::new(),
        }
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
