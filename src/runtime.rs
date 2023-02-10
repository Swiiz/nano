//! A Runtime is a container of data that can be used to run systems.
//! The Runtime provides the data to the systems using the `Provider` trait.

use crate::{
    resources::{Resources, TLResources},
    world::World,
};

#[macro_export]
/// Implements `Provider` for resources for $type using the $variable field.
macro_rules! impl_resource_provider {
    ($variable:ident, $ty:ty) => {
        impl<'a, T: 'static + Send + Sync> crate::systems::Provider<'a, crate::prelude::Res<'a, T>>
            for $ty
        {
            fn provide(&'a self) -> crate::prelude::Res<'a, T> {
                self.$variable.provide()
            }
        }
    };
}

#[macro_export]
/// Implements `Provider` for thread local resources for $type using the $variable field.
macro_rules! impl_tlresource_provider {
    ($variable:ident, $ty:ty) => {
        impl<'a, T: 'static> crate::systems::Provider<'a, crate::prelude::TLRes<'a, T>> for $ty {
            fn provide(&'a self) -> crate::prelude::TLRes<'a, T> {
                self.$variable.provide()
            }
        }
    };
}

/// A thread safe Runtime, used for systems that are marked as `Send + Sync`.
pub struct Runtime {
    /// The resources.
    pub resources: Resources,
    /// The world.
    pub world: World,
}
impl_resource_provider!(resources, Runtime);
//TODO: impl_world_provider!(world, Runtime);

/// A thread-local Runtime, used for systems that are not marked as `Send + Sync`.
pub struct TLRuntime {
    /// The thread-local resources.
    pub resources: TLResources,
    /// The world.
    pub world: World,
}
impl_tlresource_provider!(resources, TLRuntime);
//TODO: impl_world_provider!(world, TLRuntime);
