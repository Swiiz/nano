use std::{
    any::{Any, TypeId},
    collections::{BTreeSet, HashMap},
    sync::{RwLock},
};

use crate::{
    archetype::Archetype,
    entity::Entity,
    prototype::Prototype,
    ressource::{Res, ResMut},
    system::System,
};

pub struct World {
    data: HashMap<DataKind, Box<dyn Any>>,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
pub enum DataKind {
    Resource(TypeId),
    Archetype(BTreeSet<TypeId>),
}

impl World {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert_resource<T: Any + Send>(&mut self, resource: T) {
        self.data.insert(
            DataKind::Resource(TypeId::of::<T>()),
            Box::new(RwLock::new(resource)),
        );
    }

    pub fn get_resource<T: Any>(&self) -> Option<Res<'_, T>> {
        let resource = self.data.get(&DataKind::Resource(TypeId::of::<T>()))?;

        let resource = resource
            .downcast_ref::<RwLock<T>>()
            .unwrap()
            .try_read()
            .expect("Resource is already mutably borrowed");

        Some(Res::<T>::new(resource))
    }

    pub fn get_resource_mut<T: Any>(&self) -> Option<ResMut<'_, T>> {
        let resource = self.data.get(&DataKind::Resource(TypeId::of::<T>()))?;

        let resource = resource
            .downcast_ref::<RwLock<T>>()
            .unwrap()
            .try_write()
            .expect("Resource is already borrowed");

        Some(ResMut::<T>::new(resource))
    }

    pub fn get_archetype(&self, components: &[TypeId]) -> Option<&Archetype> {
        self.data
            .get(&DataKind::Archetype(components.iter().copied().collect()))
            .and_then(|archetype| archetype.downcast_ref::<Archetype>())
    }

    fn lazy_mut_archetype(&mut self, components: BTreeSet<TypeId>) -> &mut Archetype {
        self.data
            .entry(DataKind::Archetype(components))
            .or_insert_with(|| Box::new(Archetype::new()))
            .downcast_mut::<Archetype>()
            .unwrap()
    }

    pub fn create_entity(&mut self, prototype: Prototype) -> Entity {
        let components = || prototype.components.keys().copied().collect();

        let archetype = self.lazy_mut_archetype(components());

        Entity::new(components(), archetype.push(prototype))
    }

    pub fn run<'a, P>(&'a self, mut system: impl System<'a, P>) -> crate::system::Result {
        system.run(self)
    }
}
