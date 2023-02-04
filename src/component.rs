use std::{any::Any, sync::RwLock};

use crate::archetype::{Column, UntypedColumn};

pub trait Component {
    fn new_column(&self) -> Box<RwLock<dyn UntypedColumn>>;
    fn downcast(self: Box<Self>) -> Box<dyn Any>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> Component for T
where
    T: Any + 'static,
{
    fn new_column(&self) -> Box<RwLock<dyn UntypedColumn>> {
        Box::new(RwLock::new(Column::<T> { data: Vec::new() }))
    }

    fn downcast(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
