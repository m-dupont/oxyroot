use crate::rbase;
use crate::root::{objects, traits};
use std::any::Any;

use crate::rtypes::factory::{Factory, FactoryBuilder};

#[derive(Default)]
pub struct List {
    name: String,
    obj: rbase::Object,
    objs: Vec<objects::Object>,
}

impl List {
    pub fn new() -> List {
        List {
            ..Default::default()
        }
    }
}

impl traits::Object for List {
    fn class(&self) -> Option<String> {
        if self.name == "" {
            return Some(self.name.to_string());
        } else {
            return Some("TList".to_string());
        }
    }
}

impl FactoryBuilder for List {
    // fn make_factory_builder() -> crate::rtypes::factory::FactoryBuilderValue {
    //     let f = || {
    //         let v: List = List::new();
    //         let b: Box<dyn Any> = Box::new(v);
    //         b
    //     };
    //
    //     f
    // }
    //
    // fn make_factory_name() -> &'static str {
    //     "TList"
    // }

    fn register(factory: &mut Factory) {
        let f = || {
            let v: List = List::new();
            let b: Box<dyn Any> = Box::new(v);
            b
        };

        factory.add("TList", f);
    }
}

pub fn plop() {
    let f = List::new;
}
