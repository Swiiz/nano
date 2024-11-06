use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use better_any::{Tid, TidAble, TidExt};

use crate::{deps::Dependencies, Module, Output};

pub trait DynModule: Any {
    fn run_dyn<'a>(&mut self, input: Box<dyn Tid<'a> + 'a>, engine: &Engine) -> Output<'a>;
    fn as_any(&self) -> &dyn Any;
}
impl<T: Module> DynModule for T {
    fn run_dyn<'a>(&mut self, input: Box<dyn Tid<'a> + 'a>, engine: &Engine) -> Output<'a> {
        self.run(
            *input.downcast_box::<T::Input<'a>>().unwrap_or_else(|_| {
                panic!("Invalid input type");
            }),
            T::Dependencies::read_deps(engine),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Engine {
    /// TypeId of Module -> Module
    pub modules: HashMap<TypeId, Box<dyn DynModule>>,
    /// betterany TypeId of Module::Input -> TypeId of Module
    pub modules_input_tid: HashMap<TypeId, TypeId>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            modules_input_tid: HashMap::new(),
        }
    }

    pub fn add_module<M: Module>(&mut self, module: M) {
        let module = Box::new(module) as Box<dyn DynModule>;
        let tid: TypeId = TypeId::of::<M>();

        self.modules
            .insert(tid, module)
            .is_some()
            .then(|| panic!("Module already exists"));

        self.modules_input_tid.insert(M::Input::<'_>::id(), tid);
    }

    pub fn run<'a, T: TidAble<'a>>(&mut self, input: T) -> crate::Result<()> {
        let mut inputs = vec![Box::new(input) as Box<dyn Tid<'a> + 'a>];
        loop {
            let output = inputs
                .into_iter()
                .filter_map(|input| {
                    let input_tid = (&*input).self_id();
                    let module_tid = self
                        .modules_input_tid
                        .get(&input_tid)
                        .expect("Event is missing handler in engine modules");
                    let mut module = self.modules.remove(&module_tid);
                    let output = module.as_mut().map(|m| m.run_dyn(input, self).iter());
                    if module.is_some() {
                        self.modules.insert(module_tid.clone(), module.unwrap());
                    }
                    output
                })
                .flatten();

            inputs = output.collect();
            if inputs.len() == 0 {
                return Ok(());
            }
        }
    }

    pub fn read_module<M: Module>(&self) -> Option<&M> {
        self.modules
            .get(&TypeId::of::<M>())
            .map(|m| m.as_any().downcast_ref::<M>().unwrap())
    }
}
