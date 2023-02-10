use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    sync::RwLock,
};

use super::{component::Component, prototype::Prototype};

pub struct Archetype {
    columns: HashMap<TypeId, RwLock<Box<dyn Column>>>,
    layout: HashSet<TypeId>,
}

impl Archetype {
    pub fn new(layout: HashSet<TypeId>) -> Self {
        Self {
            columns: HashMap::new(),
            layout,
        }
    }

    pub fn layout(&self) -> &HashSet<TypeId> {
        &self.layout
    }

    pub fn insert(&mut self, prototype: Prototype) {
        let components = prototype.components();
        for component in components {
            let type_id = component.type_id();
            if !self.layout.contains(&type_id) {
                continue;
            }
            let column = self
                .columns
                .entry(type_id)
                .or_insert_with(|| RwLock::new(component.make_column()));
            let mut column = column.try_write().expect("Column already borrowed");
            column.push(component);
        }
    }
}

pub trait Column {
    fn push(&mut self, component: Box<dyn Component>);
}
impl<T: 'static> Column for Vec<T> {
    fn push(&mut self, component: Box<dyn Component>) {
        let component = component.as_any_box().downcast::<T>().unwrap();
        self.push(*component);
    }
}
