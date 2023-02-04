use std::{
    fmt::Debug,
    sync::{RwLockReadGuard, RwLockWriteGuard},
};

pub struct Res<'a, T> {
    _ref: RwLockReadGuard<'a, T>,
}

impl<'a, T> Res<'a, T> {
    pub fn new(_ref: RwLockReadGuard<'a, T>) -> Self {
        Self { _ref }
    }
}

impl<'a, T: 'static> std::ops::Deref for Res<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self._ref
    }
}

impl<'a, T: 'static + Debug> Debug for Res<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Resource Read")
            .field("value", &self._ref)
            .finish()
    }
}

pub struct ResMut<'a, T> {
    _ref: RwLockWriteGuard<'a, T>,
}

impl<'a, T> ResMut<'a, T> {
    pub fn new(_ref: RwLockWriteGuard<'a, T>) -> Self {
        Self { _ref }
    }
}

impl<'a, T: 'static> std::ops::Deref for ResMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self._ref
    }
}

impl<'a, T: 'static> std::ops::DerefMut for ResMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._ref
    }
}

impl<'a, T: 'static + Debug> Debug for ResMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Resource Write")
            .field("value", &self._ref)
            .finish()
    }
}
