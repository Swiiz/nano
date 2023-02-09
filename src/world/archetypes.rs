use std::sync::RwLock;

pub struct Archetype {
    columns: Vec<RwLock<Box<dyn Column>>>,
    alives: Vec<usize>,
    dead: Vec<usize>,
}

impl Archetype {}

pub trait Column {}
