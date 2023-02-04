use std::{any::TypeId, collections::BTreeSet};

use crate::archetype::ArchetypeEntry;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entity {
    components: BTreeSet<TypeId>,
    entry: ArchetypeEntry,
}

impl Entity {
    pub fn new(components: BTreeSet<TypeId>, entry: ArchetypeEntry) -> Self {
        Self { components, entry }
    }

    pub fn components(&self) -> impl Iterator<Item = &TypeId> {
        self.components.iter()
    }
}
