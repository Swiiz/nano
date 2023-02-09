//! Work in progress... //TODO: Remove this comment when the module is done.

use std::{any::TypeId, collections::HashSet};

use self::archetypes::Archetype;

pub(crate) mod archetypes;

#[derive(Default)]
pub struct World {
    archetypes: Vec<Archetype>,
    layouts: Vec<HashSet<TypeId>>,
}
