use std::{
    any::{Any, TypeId},
    collections::{BTreeSet, HashMap, HashSet},
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use super::{component::Component, prototype::Prototype};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ArchetypeEntity {
    index: usize,
    generation: u32,
}

pub struct ArchetypeEntry {
    index: usize,
    generation: u32,
}

// Those are the same structs but they don't have the same purpose.
// ArchetypeEntity is used to identify an entity in an archetype.
// ArchetypeEntry is an entry in the archetype's entity table.

#[derive(Default)]
pub struct Archetype {
    columns: HashMap<TypeId, Box<dyn Column>>,
    entities: Vec<ArchetypeEntity>,
    dead_entities: BTreeSet<usize>,
    layout: HashSet<TypeId>,
}

impl Archetype {
    pub fn new(layout: HashSet<TypeId>) -> Self {
        Self {
            columns: HashMap::new(),
            entities: Vec::new(),
            dead_entities: BTreeSet::new(),
            layout,
        }
    }

    pub fn layout(&self) -> &HashSet<TypeId> {
        &self.layout
    }

    pub fn insert(&mut self, prototype: Prototype) {
        assert_eq!(
            prototype.layout(),
            self.layout,
            "Prototype layout does not match archetype layout"
        );

        // Create a new entity.
        if let Some(entity) = self.dead_entities.iter().next() {
            let entity = ArchetypeEntity {
                index: *entity,
                generation: self.entities[*entity].generation + 1,
            };
            self.entities[entity.index] = entity;
        } else {
            let entity = ArchetypeEntity {
                index: self.entities.len(),
                generation: 0,
            };
            self.entities.push(entity);
        }

        // Insert the components.
        let components = prototype.components();
        for component in components {
            let type_id = (&*component).type_id();
            assert!(
                self.layout.contains(&type_id),
                "Component type not in archetype layout"
            );
            let column = self
                .columns
                .entry(type_id)
                .or_insert_with(|| (*component).make_column());

            column.push(component);

            assert_eq!(
                column.len(),
                self.entities.len() - self.dead_entities.len(),
                "Column length is not equal to total number of entities in archetype"
            );
        }
    }

    pub fn len(&self) -> usize {
        self.entities.len() - self.dead_entities.len()
    }

    pub fn entities(&self) -> impl Iterator<Item = ArchetypeEntity> + '_ {
        self.entities
            .iter()
            .enumerate()
            .filter_map(move |(index, entity)| {
                if self.dead_entities.contains(&index) {
                    None
                } else {
                    Some(*entity)
                }
            })
    }

    pub fn is_alive(&self, entry: ArchetypeEntry) -> bool {
        self.entities[entry.index].generation == entry.generation
    }

    pub fn get_column<'a, T: Component>(&'a self) -> Option<RwLockReadGuard<'a, Vec<T>>> {
        Some(
            self.columns
                .get(&TypeId::of::<T>())?
                .as_any()
                .downcast_ref::<RwLock<Vec<T>>>()
                .unwrap()
                .try_read()
                .expect("Column already borrowed"),
        )
    }

    pub fn get_column_mut<'a, T: Component>(&'a self) -> Option<RwLockWriteGuard<'a, Vec<T>>> {
        Some(
            self.columns
                .get(&TypeId::of::<T>())?
                .as_any()
                .downcast_ref::<RwLock<Vec<T>>>()
                .unwrap()
                .try_write()
                .expect("Column already borrowed"),
        )
    }
}

pub trait Column: 'static + Send + Sync {
    fn len(&self) -> usize;
    fn push(&mut self, component: Box<dyn Component>);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
impl<T: 'static + Component> Column for RwLock<Vec<T>> {
    fn push(&mut self, component: Box<dyn Component>) {
        assert_eq!(
            TypeId::of::<T>(),
            (&*component).type_id(),
            "Component type does not match column type"
        );
        self.try_write()
            .unwrap()
            .push(*component.as_any_box().downcast::<T>().unwrap())
    }

    fn len(&self) -> usize {
        self.try_read().unwrap().len()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
