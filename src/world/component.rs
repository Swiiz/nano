use std::{any::Any, sync::RwLock};

use super::archetypes::Column;

pub trait Component: 'static + Send + Sync + Any {
    fn make_column(&self) -> Box<dyn Column>;
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
}
impl<T: Clone> Component for T
where
    T: Send + Sync + 'static,
{
    fn make_column(&self) -> Box<dyn Column> {
        Box::new(RwLock::new(Vec::<T>::new()))
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }
}
