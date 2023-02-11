//! The fastest way to access lot of components is using a query.
//! A query match a predicate against the components of an entity.
//! If the predicate matches, the requested components are returned
//! with their respective access (& or &mut).
//! The query can then be used as an iterator to access the components
//! of all the entities that match the predicate.

//TODO: add support for Option<&T> and Option<&mut T> in queries

use std::{
    any::TypeId,
    collections::HashSet,
    marker::PhantomData,
    sync::{RwLockReadGuard, RwLockWriteGuard},
};

use crate::systems::Provider;

use super::{archetypes::Archetype, component::Component, World};

/// A query item is a component or a tuple of references (& or/and &mut) to components.
pub trait QueryItem<'a>: QueryPredicate + 'a {
    type ColumnsAccessor: ColumnsAccessor<'a, Item = Self>;
}

pub trait ColumnsAccessor<'a> {
    type Item;
    fn resolve(archetype: &'a Archetype) -> Self;
    unsafe fn get(&mut self, index: usize) -> Self::Item;
    fn size(&self) -> usize;
}
/// A query predicate can statically be built using tuple of With<T> or Without<T>
pub trait QueryPredicate {
    /// Returns true if the predicate matches the layout.
    fn matches(layout: &HashSet<TypeId>) -> bool;
}

impl<'a, T: Component> QueryPredicate for &'a T {
    fn matches(layout: &HashSet<TypeId>) -> bool {
        layout.contains(&TypeId::of::<T>())
    }
}
impl<'a, T: Component> QueryPredicate for &'a mut T {
    fn matches(layout: &HashSet<TypeId>) -> bool {
        layout.contains(&TypeId::of::<T>())
    }
}
impl<'a, T: Component> QueryPredicate for Option<&'a T> {
    fn matches(_layout: &HashSet<TypeId>) -> bool {
        true
    }
}
impl<'a, T: Component> QueryPredicate for Option<&'a mut T> {
    fn matches(_layout: &HashSet<TypeId>) -> bool {
        true
    }
}
impl<'a, T: Component> QueryItem<'a> for &'a T {
    type ColumnsAccessor = RwLockReadGuard<'a, Vec<T>>;
}
impl<'a, T: Component> QueryItem<'a> for &'a mut T {
    type ColumnsAccessor = RwLockWriteGuard<'a, Vec<T>>;
}
impl<'a, T: Component> QueryItem<'a> for Option<&'a T> {
    type ColumnsAccessor = Option<RwLockReadGuard<'a, Vec<T>>>;
}
impl<'a, T: Component> QueryItem<'a> for Option<&'a mut T> {
    type ColumnsAccessor = Option<RwLockWriteGuard<'a, Vec<T>>>;
}

impl<'a, T: Component> ColumnsAccessor<'a> for RwLockReadGuard<'a, Vec<T>> {
    type Item = &'a T;
    fn resolve(archetype: &'a Archetype) -> Self {
        archetype.get_column::<T>().unwrap()
    }
    unsafe fn get(&mut self, index: usize) -> Self::Item {
        &*(&self[index] as *const T)
    }
    fn size(&self) -> usize {
        self.len()
    }
}

impl<'a, T: Component> ColumnsAccessor<'a> for RwLockWriteGuard<'a, Vec<T>> {
    type Item = &'a mut T;
    fn resolve(archetype: &'a Archetype) -> Self {
        archetype.get_column_mut().unwrap()
    }
    unsafe fn get(&mut self, index: usize) -> Self::Item {
        &mut *(&mut self[index] as *mut T)
    }
    fn size(&self) -> usize {
        self.len()
    }
}

impl<'a, T: Component> ColumnsAccessor<'a> for Option<RwLockReadGuard<'a, Vec<T>>> {
    type Item = Option<&'a T>;
    fn resolve(archetype: &'a Archetype) -> Self {
        archetype.get_column::<T>()
    }
    unsafe fn get(&mut self, index: usize) -> Self::Item {
        self.as_ref().map(|c| &*(&c[index] as *const T))
    }
    fn size(&self) -> usize {
        self.as_ref().map(|c| c.len()).unwrap_or(0)
    }
}

impl<'a, T: Component> ColumnsAccessor<'a> for Option<RwLockWriteGuard<'a, Vec<T>>> {
    type Item = Option<&'a mut T>;
    fn resolve(archetype: &'a Archetype) -> Self {
        archetype.get_column_mut()
    }
    unsafe fn get(&mut self, index: usize) -> Self::Item {
        self.as_mut().map(|c| &mut *(&mut c[index] as *mut T))
    }
    fn size(&self) -> usize {
        self.as_ref().map(|c| c.len()).unwrap_or(0)
    }
}

/// Query predicate that matches if the entity has the component T.
pub struct With<T>(PhantomData<T>);
impl<T: Component> QueryPredicate for With<T> {
    fn matches(layout: &HashSet<TypeId>) -> bool {
        layout.contains(&TypeId::of::<T>())
    }
}
/// Query predicate that matches if the entity does not have the component T.
pub struct Without<T>(PhantomData<T>);
impl<T: Component> QueryPredicate for Without<T> {
    fn matches(layout: &HashSet<TypeId>) -> bool {
        !layout.contains(&TypeId::of::<T>())
    }
}
impl QueryPredicate for () {
    fn matches(_layout: &HashSet<TypeId>) -> bool {
        true
    }
}
impl<'a, I: QueryItem<'a>, P: QueryPredicate> QueryPredicate for Query<'a, I, P> {
    fn matches(layout: &HashSet<TypeId>) -> bool {
        I::matches(layout) && P::matches(layout)
    }
}

macro_rules! _impl {
    ($($t:ident $idx:tt),*) => {
        impl<'a, $($t),*> QueryPredicate for ($($t),*) where $($t: QueryPredicate),* {
            fn matches(layout: &HashSet<TypeId>) -> bool {
                $($t::matches(layout) &&)* true
            }
        }
        impl<'a, $($t),*> QueryItem<'a> for ($($t),*) where $($t: QueryItem<'a>),* {
            type ColumnsAccessor = ($($t::ColumnsAccessor),*);
        }

        impl<'a, $($t),*> ColumnsAccessor<'a> for ($($t),*) where $($t: ColumnsAccessor<'a>),* {
            type Item = ($($t::Item),*);
            fn resolve(archetype: &'a Archetype) -> Self {
                ($($t::resolve(archetype)),*)
            }
            unsafe fn get(&mut self, index: usize) -> Self::Item {
                ($($t::get(&mut self.$idx, index)),*)
            }
            fn size(&self) -> usize {
                // Longest column
                let mut size = 0;
                $(
                    let s = $t::size(&self.$idx);
                    if s > size {
                        size = s;
                    }
                )*
                size
            }
        }
    };
}

_impl!(T1 0, T2 1);
_impl!(T1 0, T2 1, T3 2);
_impl!(T1 0, T2 1, T3 2, T4 3);
_impl!(T1 0, T2 1, T3 2, T4 3, T5 4);
_impl!(T1 0, T2 1, T3 2, T4 3, T5 4, T6 5);
_impl!(T1 0, T2 1, T3 2, T4 3, T5 4, T6 5, T7 6);
_impl!(T1 0, T2 1, T3 2, T4 3, T5 4, T6 5, T7 6, T8 7);

/// An iterator matching a predicate against the components of an entity.
pub struct Query<'a, I: QueryItem<'a>, P: QueryPredicate = ()> {
    _marker: std::marker::PhantomData<(I, P)>,
    columns: Vec<I::ColumnsAccessor>,
    current_index: usize,
}

impl<'a, I: QueryItem<'a>, P: QueryPredicate> Query<'a, I, P> {
    pub(crate) fn new(archetypes: &'a Vec<Archetype>) -> Self {
        let r = Self {
            _marker: std::marker::PhantomData,
            columns: archetypes
                .into_iter()
                .filter(|archetype| Self::matches(archetype.layout()))
                .map(|archetype| I::ColumnsAccessor::resolve(archetype))
                .collect(),
            current_index: 0,
        };
        r
    }
}

impl<'a, I: QueryItem<'a>, P: QueryPredicate> Iterator for Query<'a, I, P> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if self.columns.is_empty() {
            return None;
        }

        let index = self.current_index;
        let last = self.columns.last_mut().unwrap();
        if index >= last.size() {
            self.columns.pop();
            self.current_index = 0;
            return self.next();
        }
        self.current_index += 1;

        Some(unsafe {
            // This is safe because we get every element only once. (Iterator yield each element only once)
            last.get(index)
        })
    }
}

impl<'a, I: QueryItem<'a>, P: QueryPredicate + 'a> Provider<'a, Query<'a, I, P>> for World {
    fn provide(&'a self) -> Query<'a, I, P> {
        Query::new(&self.archetypes)
    }
}
