//! Work in progress... //TODO: Remove this comment when the module is done.

use self::{archetypes::Archetype, prototype::Prototype};

pub(crate) mod archetypes;
pub(crate) mod component;
pub mod prototype;

#[derive(Default)]
pub struct World {
    archetypes: Vec<Archetype>,
}

impl World {
    /// Creates a new `World`.
    pub fn new() -> Self {
        Self {
            archetypes: Vec::new(),
        }
    }

    /// Returns a reference to the world's archetypes.
    pub fn archetypes(&self) -> &[Archetype] {
        &self.archetypes
    }

    /// Returns a mutable reference to the world's archetypes.
    pub fn archetypes_mut(&mut self) -> &mut [Archetype] {
        &mut self.archetypes
    }

    /// Finds an archetype that matches the given predicate.
    /// Returns a reference to the archetype if found.
    /// Returns `None` if no archetype matches the predicate.
    pub fn find_archetype(&self, predicate: impl Fn(&Archetype) -> bool) -> Option<&Archetype> {
        self.archetypes()
            .iter()
            .find(|archetype| predicate(archetype))
    }

    /// Finds an archetype that matches the given predicate.
    /// Returns a mutable reference to the archetype if found.
    /// Returns `None` if no archetype matches the predicate.
    pub fn find_archetype_mut(
        &mut self,
        predicate: impl Fn(&Archetype) -> bool,
    ) -> Option<&mut Archetype> {
        self.archetypes_mut()
            .iter_mut()
            .find(|archetype| predicate(archetype))
    }

    /// Inserts a new entity into the world.
    /// The entity is created from the given prototype.
    pub fn insert(&mut self, prototype: Prototype) {
        let mut prototype = Some(prototype);
        self.find_archetype_mut(|archetype| {
            archetype.layout() == &prototype.as_ref().unwrap().layout()
        })
        .map(|archetype| archetype.insert(prototype.take().unwrap()))
        .unwrap_or_else(|| {
            let mut archetype = Archetype::new(prototype.as_ref().unwrap().layout());
            archetype.insert(prototype.take().unwrap());
            self.archetypes.push(archetype);
        });
    }
}
