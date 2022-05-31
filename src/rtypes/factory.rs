use crate::rbase;
use lazy_static::lazy_static;
use std::any::Any;
use std::collections::HashMap;

use crate::rcont::list::List;
use crate::root::objects;

enum Value {
    Tlist(List),
    NotInitialized,
}

pub(crate) type FactoryBuilderValue = fn() -> Box<dyn Any>;

pub trait FactoryBuilder {
    // fn make_factory_builder() -> FactoryBuilderValue;
    // fn make_factory_name() -> &'static str;

    fn register(factory: &mut Factory);
}

pub struct Factory<'a> {
    map: HashMap<&'a str, FactoryBuilderValue>,
}

impl<'a> Factory<'a> {
    pub fn new() -> Factory<'a> {
        Factory {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, s: &'a str, f: FactoryBuilderValue) {
        self.map.insert(s, f);
    }

    pub fn get(&self, s: &'a str) -> Option<&FactoryBuilderValue> {
        self.map.get(s)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

lazy_static! {
    pub static ref FACTORY: Factory<'static> = {
        let mut f = Factory::new();
        // f.add(List::make_factory_name(), List::make_factory_builder());
        List::register(&mut f);

        f
    };
}

#[cfg(test)]
mod tests {
    use crate::rtypes::factory::{Factory, FACTORY};
    use std::any::Any;

    #[test]
    fn gen_factory() {
        let mut factory = Factory::new();
        let f = || {
            let v: Vec<i32> = Vec::new();
            let b: Box<dyn Any> = Box::new(v);
            b
        };
        factory.add("VEC", f);

        let fct = factory.get("VE");
        assert!(fct.is_none());
        let fct = factory.get("VEC");
        assert!(fct.is_some());

        assert_eq!(factory.len(), 1);

        let vec = fct.unwrap();
        let vec = vec();

        let mut vec: Box<Vec<i32>> = vec.downcast().unwrap();

        vec.push(43);
        vec.push(43);
        vec.push(43);

        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn factory_static() {
        assert_eq!(FACTORY.len(), 1);
        assert!(FACTORY.get("TList").is_some());
    }
}
