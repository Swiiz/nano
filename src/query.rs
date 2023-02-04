use std::sync::RwLockReadGuard;

use crate::archetype::Archetype;

pub struct Query<'a, Q: Queryable, F: QueryFilter> {
    _archetypes: Vec<RwLockReadGuard<'a, Archetype>>, //TODO: Use this
    _marker: std::marker::PhantomData<(Q, F)>,
}

impl<'a, Q: Queryable, F: QueryFilter> QueryFilter for Query<'a, Q, F> {
    fn filter(archetype: &Archetype) -> bool {
        F::filter(archetype) && Q::filter(archetype)
    }
}

pub trait QueryFilter {
    fn filter(archetype: &Archetype) -> bool;
}

pub trait Queryable: QueryFilter {}
