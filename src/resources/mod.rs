//! This module contains the resources system.
//!
//! Each resource can be considered as a global variable/singleton
//! inside the "System Providing Context".
//!
//! Two containers are provided:
//! - `TLRessources` is a thread-local container.
//! - `Ressources` is a container that can be shared between threads.
//!
//! `TLRessources` yields `TLRes` and `TLResMut` which are thread-local
//! references to the resources.
//!
//! `Ressources` yields `Res` and `ResMut` which are references to the
//! resources.
//!
//! `TLRes`, `TLResMut`, `Res` and `ResMut` are all provided by their respective
//! containers. So that you can use them inside systems.

use std::{
    any::{Any, TypeId},
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::systems::Provider;

/// A thread-local container for resources.
pub struct TLResources {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

#[cfg(feature = "parallel")]
/// A container for resources.
pub struct Resources {
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl TLResources {
    /// Creates a new `TLRessources`.
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    /// Inserts a resource into the container.
    pub fn insert<T: Any>(&mut self, resource: T) {
        self.resources
            .insert(TypeId::of::<T>(), Box::new(RefCell::new(resource)));
    }

    /// Gets a reference to a resource.
    /// Returns `None` if the resource is not found.
    /// Panics if the resource is already borrowed.
    pub fn get<T: Any>(&self) -> Option<TLRes<T>> {
        Some(TLRes {
            resource: self
                .resources
                .get(&TypeId::of::<T>())?
                .downcast_ref::<RefCell<T>>()
                .unwrap()
                .try_borrow()
                .expect("Resource already borrowed"),
        })
    }

    /// Gets a mutable reference to a resource.
    /// Returns `None` if the resource is not found.
    /// Panics if the resource is already borrowed.
    pub fn get_mut<T: Any>(&self) -> Option<TLResMut<T>> {
        Some(TLResMut {
            resource: self
                .resources
                .get(&TypeId::of::<T>())?
                .downcast_ref::<RefCell<T>>()
                .unwrap()
                .try_borrow_mut()
                .expect("Resource already borrowed"),
        })
    }
}

#[cfg(feature = "parallel")]
impl Resources {
    /// Creates a new `Ressources`.
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    /// Inserts a resource into the container.
    pub fn insert<T: Any + Send + Sync>(&mut self, resource: T) {
        self.resources
            .insert(TypeId::of::<T>(), Box::new(RwLock::new(resource)));
    }

    /// Gets a reference to a resource.
    /// Returns `None` if the resource is not found.
    /// Panics if the resource is already borrowed.
    pub fn get<T: Any + Send + Sync>(&self) -> Option<Res<T>> {
        Some(Res {
            resource: self
                .resources
                .get(&TypeId::of::<T>())?
                .downcast_ref::<RwLock<T>>()
                .unwrap()
                .try_read()
                .expect("Failed to read resource, maybe it is already borrowed?"),
        })
    }

    /// Gets a mutable reference to a resource.
    /// Returns `None` if the resource is not found.
    /// Panics if the resource is already borrowed.
    pub fn get_mut<T: Any + Send + Sync>(&self) -> Option<ResMut<T>> {
        Some(ResMut {
            resource: self
                .resources
                .get(&TypeId::of::<T>())?
                .downcast_ref::<RwLock<T>>()
                .unwrap()
                .try_write()
                .expect("Failed to write resource, maybe it is already borrowed?"),
        })
    }
}

/// A reference to a resource.
pub struct Res<'a, T: 'a + Send + Sync> {
    resource: RwLockReadGuard<'a, T>,
}

/// A mutable reference to a resource.
pub struct ResMut<'a, T: 'a + Send + Sync> {
    resource: RwLockWriteGuard<'a, T>,
}

/// A reference to a thread-local resource.
pub struct TLRes<'a, T: 'a> {
    resource: Ref<'a, T>,
}

/// A mutable reference to a thread-local resource.
pub struct TLResMut<'a, T: 'a> {
    resource: RefMut<'a, T>,
}

impl<'a, T: 'a + Send + Sync> Borrow<T> for Res<'a, T> {
    fn borrow(&self) -> &T {
        &self.resource
    }
}

impl<'a, T: 'a + Send + Sync> Borrow<T> for ResMut<'a, T> {
    fn borrow(&self) -> &T {
        &self.resource
    }
}

impl<'a, T: 'a> Borrow<T> for TLRes<'a, T> {
    fn borrow(&self) -> &T {
        &self.resource
    }
}

impl<'a, T: 'a> Borrow<T> for TLResMut<'a, T> {
    fn borrow(&self) -> &T {
        &self.resource
    }
}

impl<'a, T: 'a + Send + Sync> AsRef<T> for Res<'a, T> {
    fn as_ref(&self) -> &T {
        &self.resource
    }
}

impl<'a, T: 'a + Send + Sync> AsRef<T> for ResMut<'a, T> {
    fn as_ref(&self) -> &T {
        &self.resource
    }
}

impl<'a, T: 'a> AsRef<T> for TLRes<'a, T> {
    fn as_ref(&self) -> &T {
        &self.resource
    }
}

impl<'a, T: 'a> AsRef<T> for TLResMut<'a, T> {
    fn as_ref(&self) -> &T {
        &self.resource
    }
}

impl<'a, T: 'a + Send + Sync> AsMut<T> for ResMut<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.resource
    }
}

impl<'a, T: 'a> AsMut<T> for TLResMut<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.resource
    }
}

impl<'a, T: 'a + Send + Sync> Deref for Res<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl<'a, T: 'a + Send + Sync> Deref for ResMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl<'a, T: 'a> Deref for TLRes<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl<'a, T: 'a> Deref for TLResMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl<'a, T: 'a + Send + Sync> DerefMut for ResMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.resource
    }
}

impl<'a, T: 'a> DerefMut for TLResMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.resource
    }
}

impl<'a, T: 'static + Send + Sync> Provider<'a, Res<'a, T>> for Resources {
    fn provide(&'a self) -> Res<'a, T> {
        self.get().expect("Failed to get resource")
    }
}

impl<'a, T: 'static + Send + Sync> Provider<'a, ResMut<'a, T>> for Resources {
    fn provide(&'a self) -> ResMut<'a, T> {
        self.get_mut().expect("Failed to get resource")
    }
}

impl<'a, T: 'static> Provider<'a, TLRes<'a, T>> for TLResources {
    fn provide(&'a self) -> TLRes<'a, T> {
        self.get().expect("Failed to get resource")
    }
}

impl<'a, T: 'static> Provider<'a, TLResMut<'a, T>> for TLResources {
    fn provide(&'a self) -> TLResMut<'a, T> {
        self.get_mut().expect("Failed to get resource")
    }
}
