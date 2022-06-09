use crate::rbase;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::root::traits::Object;
use crate::root::{objects, traits};
use crate::rvers;
use anyhow::ensure;
use log::{debug, trace};
use std::any::Any;

use crate::rtypes::factory::{Factory, FactoryBuilder, FactoryItem};

#[derive(Default)]
pub struct List {
    name: Option<String>,
    obj: rbase::Object,
    objs: Vec<Box<dyn FactoryItem>>,
}

impl List {
    pub fn new() -> Self {
        Self {
            objs: Vec::new(),
            ..Default::default()
        }
    }

    pub fn len(&self) -> usize {
        self.objs.len()
    }

    pub fn at(&mut self, i: usize) -> Box<dyn FactoryItem> {
        self.objs.remove(i)
    }
}

impl traits::Object for List {
    fn class(&self) -> &'_ str {
        match &self.name {
            None => "TList",
            Some(s) => s,
        }
    }
}

impl Unmarshaler for List {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("List:unmarshal");

        let hdr = r.read_header(self.class())?;

        ensure!(
            hdr.vers <= rvers::List,
            "rcont: invalid TList version={} > {}",
            hdr.vers,
            rvers::List
        );

        ensure!(
            hdr.vers > 3,
            "rcont: invalid TList version, too old= ({} < {})",
            hdr.vers,
            3
        );

        r.read_object(&mut self.obj)?;

        self.name = Some(r.read_string()?.to_string());
        let size = r.read_i32()?;

        trace!("name = {}, size = {}", self.name.as_ref().unwrap(), size);

        for i in 0..size {
            debug!("List:unmarshal: {}", i);
            let obj = r.read_object_any_into()?.expect("something is wrong");
            self.objs.push(obj);

            let n = r.read_u8()?;
            trace!("n = {}", n);

            if n > 0 {
                unimplemented!()
            }

            // todo!()
        }

        r.check_header(&hdr)?;
        Ok(())

        // ensure!()
    }
}

impl traits::Named for List {
    fn name(&self) -> &'_ str {
        match &self.name {
            None => "TList",
            Some(s) => s,
        }
    }

    fn title(&self) -> &'_ str {
        "Doubly linked list"
    }
}

// impl FactoryItem for List {}

impl FactoryBuilder for List {
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
    // let f = List::unmarshal_into;
}
