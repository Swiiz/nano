use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::RwLock,
};

use crate::prototype::Prototype;

pub struct Archetype {
    columns: HashMap<TypeId, Box<RwLock<dyn UntypedColumn>>>,
    entries: Vec<ArchetypeEntry>,
    free_list: Vec<usize>,
}

pub struct Column<T> {
    pub data: Vec<T>,
}

pub trait UntypedColumn {
    fn get(&self, index: usize) -> &dyn Any;
    fn get_mut(&mut self, index: usize) -> &mut dyn Any;
    fn push(&mut self, value: Box<dyn Any>);
    fn remove(&mut self, index: usize);
}

impl<T> UntypedColumn for Column<T>
where
    T: Any + 'static,
{
    fn get(&self, index: usize) -> &dyn Any {
        &self.data[index]
    }

    fn get_mut(&mut self, index: usize) -> &mut dyn Any {
        &mut self.data[index]
    }

    fn push(&mut self, value: Box<dyn Any>) {
        self.data.push(*value.downcast().unwrap());
    }

    fn remove(&mut self, index: usize) {
        self.data.swap_remove(index);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArchetypeEntry {
    generation: u32,
    index: usize,
}

impl Archetype {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
            entries: Vec::new(),
            free_list: Vec::new(),
        }
    }

    pub fn push(&mut self, prototype: Prototype) -> ArchetypeEntry {
        let index = if let Some(index) = self.free_list.pop() {
            index
        } else {
            self.entries.len()
        };

        for (type_id, value) in prototype.components {
            let column = self
                .columns
                .entry(type_id)
                .or_insert_with(|| value.new_column());

            column
                .try_write()
                .expect("Column is already borrowed")
                .push(value.downcast());
        }

        let entry = ArchetypeEntry {
            generation: 0,
            index,
        };

        self.entries.push(entry);

        entry
    }
}
