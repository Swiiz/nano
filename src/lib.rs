pub mod archetype;
pub mod component;
pub mod entity;
pub mod prototype;
pub mod query;
pub mod ressource;
pub mod system;
pub mod world;

pub mod prelude {
    pub use crate::entity::Entity;
    pub use crate::prototype::Prototype;
    pub use crate::query::Query;
    pub use crate::ressource::{Res, ResMut};
    pub use crate::system::{Result, System};
    pub use crate::world::World;
}
