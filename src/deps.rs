use crate::{Engine, Module};
use std::any::{Any, TypeId};

pub trait Dependencies<T> {
    type ReadOnly<'a>: ReadDeps;
    fn read_deps<'a>(engine: &'a Engine) -> Self::ReadOnly<'a>;
}

pub trait ReadDeps {
    fn get<M: Module>(&self) -> &M;
}

impl<T> Dependencies<T> for () {
    type ReadOnly<'a> = ();
    fn read_deps<'a>(_: &'a Engine) -> Self::ReadOnly<'a> {}
}

impl ReadDeps for () {
    fn get<M: Module>(&self) -> &M {
        panic!("Dependencies need to be defined in module before use")
    }
}

macro_rules! impl_deps {
    ($($name:tt $num:tt),*) => {
        impl<T, $($name: Module),*> Dependencies<T> for ($($name,)*) {
            type ReadOnly<'a> = ($(&'a $name,)*);

            fn read_deps<'a>(engine: &'a Engine) -> Self::ReadOnly<'a> {
                ($(engine.read_module::<$name>().unwrap_or_else(|| panic!("Module {} not found, but is required by module {}", std::any::type_name::<$name>(), std::any::type_name::<T>())),)*)
            }
        }

        impl<'a, $($name: Module),*> ReadDeps for ($(&$name,)*) {
            fn get<M: Module>(&self) -> &M {
                match TypeId::of::<M>() {
                    $(
                        t if t == TypeId::of::<$name>() => {
                            (self.$num as &dyn Any).downcast_ref().unwrap()
                        }
                    )*
                    _ => panic!("Module {} not found in module dependencies!", std::any::type_name::<M>())
                }
            }
        }
    };
}

impl_deps!(A 0);
impl_deps!(A 0, B 1);
impl_deps!(A 0, B 1, C 2);
impl_deps!(A 0, B 1, C 2, D 3);
impl_deps!(A 0, B 1, C 2, D 3, E 4);
impl_deps!(A 0, B 1, C 2, D 3, E 4, F 5);
impl_deps!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
impl_deps!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
