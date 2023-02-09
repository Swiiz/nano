//! Any function for which all parameters can be provided by a single provider,
//! is considered a system and has the System trait.
//! Systems can be executed using the System::run method or using an executor.

pub mod executor;

/// Abstract trait implemented by data containers to provide data to systems.
pub trait Provider<'a, T: 'a> {
    /// Provides the data.
    /// The lifetime of the data is the lifetime of the provider.
    fn provide(&'a self) -> T;
}

/// System trait.
/// This trait is implemented by all systems.
/// 'a is the lifetime of the provider.
/// P is the provider type.
/// Params is the tuple of parameters of the system.
pub trait System<'a, P, Params> {
    /// Runs the system.
    /// The parameters are provided by the provider.
    fn run(&self, provider: &'a P);
}

macro_rules! _impl {
    ($($name:ident),*) => {
        #[allow(non_snake_case)]
        impl<'a, _F, P, $($name),*> System<'a, P, ($($name,)*)> for _F
        where
            _F: Fn($($name),*),
            $(P: Provider<'a, $name>,)*
            $($name: 'a,)*
        {
            fn run(&self, provider: &'a P)
            {
                self(
                    $(<P as Provider<'a, $name>>::provide(&provider),)*
                )
            }
        }


    };
}

_impl!(A);
_impl!(A, B);
_impl!(A, B, C);
_impl!(A, B, C, D);
_impl!(A, B, C, D, E);
_impl!(A, B, C, D, E, F);
_impl!(A, B, C, D, E, F, G);
_impl!(A, B, C, D, E, F, G, H);
