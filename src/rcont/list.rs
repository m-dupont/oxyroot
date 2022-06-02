use crate::rbase;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::{Unmarshaler, Unmarshaler2};
use crate::root::{objects, traits};
use std::any::Any;

use crate::rtypes::factory::{Factory, FactoryBuilder, FactoryItem};

#[derive(Default)]
pub struct List {
    name: String,
    obj: rbase::Object,
    objs: Vec<objects::Object>,
}

impl List {
    pub fn new() -> List {
        List {
            objs: Vec::new(),
            ..Default::default()
        }
    }
}

impl traits::Object for List {
    fn class(&self) -> Option<String> {
        if self.name != "" {
            return Some(self.name.to_string());
        } else {
            return Some("TList".to_string());
        }
    }
}

impl Unmarshaler for List {
    type Item = List;

    fn unmarshal_root(r: &mut RBuffer) -> anyhow::Result<Self::Item> {
        todo!()
    }
}

impl Unmarshaler2 for List {
    fn unmarshal_root2(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        todo!()
    }
}

impl traits::Named for List {
    fn name(&self) -> &'_ str {
        if self.name == "" {
            return "TList";
        }
        &self.name
    }

    fn title(&self) -> &'_ str {
        "Doubly linked list"
    }
}

// impl FactoryItem for List {}

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
            let b: Box<dyn FactoryItem> = Box::new(v);
            b
        };

        factory.add("TList", f);
    }
}

pub fn plop() {
    let f = List::new;
    let f = List::unmarshal_root;
}
