//! A prototype is a collection of components, representing the composition of an entity
//! before it is created.
//! A prototype can thus be used to create an entity.

use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
};

use super::component::Component;

/// A prototype for an entity.
pub struct Prototype {
    components: HashMap<TypeId, Box<dyn Component>>,
}

impl Prototype {
    /// Creates a new `Prototype`.
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    /// Adds a component to the prototype.
    pub fn with<T: Component>(mut self, component: T) -> Self {
        self.components
            .insert(TypeId::of::<T>(), Box::new(component));
        self
    }

    /// Returns the current layout of the prototype.
    pub fn layout(&self) -> HashSet<TypeId> {
        self.components.keys().map(|id| *id).collect()
    }

    /// Consumes the prototype and returns its components.
    pub(crate) fn components(self) -> Vec<Box<dyn Component>> {
        self.components.into_values().collect()
    }
}
