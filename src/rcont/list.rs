use crate::rbase;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{
    ensure_maximum_supported_version, ensure_minimum_supported_version, Marshaler, Unmarshaler,
};
use crate::root::traits;
use crate::root::traits::Object;
use crate::rvers;
use log::trace;

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
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, rvers::LIST, self.class())?;

        ensure_minimum_supported_version(hdr.vers, 3, self.class())?;

        r.read_object(&mut self.obj)?;

        self.name = Some(r.read_string()?.to_string());
        let size = r.read_i32()?;

        for _ in 0..size {
            let obj = r.read_object_any_into()?.expect("something is wrong");
            self.objs.push(obj);

            let n = r.read_u8()?;

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

impl Marshaler for List {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        trace!(";List.marshal.call.w.pos:{:?}", w.pos());
        let beg = w.pos();

        let hdr = w.write_header(self.class(), rvers::LIST)?;

        self.obj.marshal(w)?;

        w.write_string(self.name.as_ref().unwrap_or(&String::new()))?;

        w.write_i32(self.objs.len() as i32)?;

        for obj in &self.objs {
            unimplemented!("List.marshal");
            w.write_object_any(obj)?;
        }

        trace!(";List.marshal.buf.value:{:?}", w.p());
        trace!(";List.marshal.buf.pos:{:?}", w.pos());

        w.set_header(hdr)?;

        Ok(w.pos() - beg)
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

// impl FactoryItem for LIST {}

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
