use std::any::TypeId;

use crate::{
    ressource::{Res, ResMut},
    world::World,
};

pub trait SystemData<'a> {
    fn fetch(world: &'a World) -> Self;
    #[cfg(debug_assertions)]
    fn claimed() -> Vec<WorldDataAccess>;
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// Made to avoid deadlocks
pub enum WorldDataAccess {
    All,
    ReadRes(TypeId),
    WriteRes(TypeId),
}

impl<'a> SystemData<'a> for () {
    fn fetch(_: &'a World) -> Self {}
    #[cfg(debug_assertions)]
    fn claimed() -> Vec<WorldDataAccess> {
        Vec::new()
    }
}

impl<'a, T: 'static> SystemData<'a> for Option<Res<'a, T>> {
    fn fetch(world: &'a World) -> Self {
        world.get_resource()
    }

    #[cfg(debug_assertions)]
    fn claimed() -> Vec<WorldDataAccess> {
        vec![WorldDataAccess::ReadRes(TypeId::of::<T>())]
    }
}

impl<'a, T: 'static> SystemData<'a> for Res<'a, T> {
    fn fetch(world: &'a World) -> Self {
        <Option<Res<'a, T>>>::fetch(world).expect("Resource not found")
    }

    #[cfg(debug_assertions)]
    fn claimed() -> Vec<WorldDataAccess> {
        vec![WorldDataAccess::ReadRes(TypeId::of::<T>())]
    }
}

impl<'a, T: 'static> SystemData<'a> for Option<ResMut<'a, T>> {
    fn fetch(world: &'a World) -> Self {
        world.get_resource_mut()
    }

    #[cfg(debug_assertions)]
    fn claimed() -> Vec<WorldDataAccess> {
        vec![WorldDataAccess::WriteRes(TypeId::of::<T>())]
    }
}

impl<'a, T: 'static> SystemData<'a> for ResMut<'a, T> {
    fn fetch(world: &'a World) -> Self {
        <Option<ResMut<'a, T>>>::fetch(world).expect("Resource not found")
    }

    #[cfg(debug_assertions)]
    fn claimed() -> Vec<WorldDataAccess> {
        vec![WorldDataAccess::WriteRes(TypeId::of::<T>())]
    }
}

pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub trait System<'a, P> {
    fn run(&mut self, world: &'a World) -> Result;
}

macro_rules! impl_system {
    ($($name:ident),*) => {
      #[allow(unused_parens, non_snake_case)]
        impl<'a, $($name: SystemData<'a>),*, _F> System<'a, ($($name,)* ())> for _F
        where
            _F: FnMut($($name),*),
        {
            fn run(&mut self, world: &'a World) -> Result{
                #[cfg(debug_assertions)]
                check_system_data([$($name::claimed()),*].concat().iter());

                let ($($name,)*) = ($($name::fetch(world),)*);
                self($($name),*);
                Ok(())
            }
        }

        #[allow(unused_parens, non_snake_case)]
        impl<'a, $($name: SystemData<'a>),*, _F> System<'a, ($($name,)* Result)> for _F
        where
            _F: FnMut($($name),*) -> Result,
        {
          fn run(&mut self, world: &'a World) -> Result{
                #[cfg(debug_assertions)]
                check_system_data([$($name::claimed()),*].concat().iter());

                let ($($name,)*) = ($($name::fetch(world),)*);
                self($($name),*)
            }
          }
    };
}

#[cfg(debug_assertions)]
fn check_system_data<'a>(claimed: impl Iterator<Item = &'a WorldDataAccess>) {
    // If there is duplicate claimed data, it means that the system is trying to
    // access the same data twice. This is not allowed.
    let mut claimed = claimed.collect::<Vec<_>>();
    claimed.sort_unstable();
    let original_len = claimed.len();
    claimed.dedup();
    if claimed.len() != original_len {
        panic!("System is trying to access the same data twice");
    }
}

impl<'a, _F> System<'a, ()> for _F
where
    _F: FnMut(),
{
    fn run(&mut self, _: &'a World) -> Result {
        self();
        Ok(())
    }
}

impl<'a, _F> System<'a, Result> for _F
where
    _F: FnMut() -> Result,
{
    fn run(&mut self, _: &'a World) -> Result {
        self()
    }
}

impl_system!(A);
impl_system!(A, B);
impl_system!(A, B, C);
impl_system!(A, B, C, D);
impl_system!(A, B, C, D, E);
impl_system!(A, B, C, D, E, F);
impl_system!(A, B, C, D, E, F, G);
impl_system!(A, B, C, D, E, F, G, H);
