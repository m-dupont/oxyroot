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

use crate::rtypes::factory::{Factory, FactoryBuilder, FactoryItemRead, FactoryItemWrite};

#[derive(Default)]
pub struct ReaderList {
    name: Option<String>,
    obj: rbase::Object,
    objs: Vec<Box<dyn FactoryItemRead>>,
}

#[derive(Default, Debug)]
pub struct WriterList<'a> {
    name: Option<String>,
    obj: rbase::Object,
    objs: Vec<(&'a dyn FactoryItemWrite, usize)>,
}

impl<'a> WriterList<'a> {
    pub fn new() -> Self {
        Self {
            objs: Vec::new(),
            ..Default::default()
        }
    }

    pub(crate) fn push(&mut self, obj: &'a dyn FactoryItemWrite, ptr: usize) {
        self.objs.push((obj, ptr));
    }
}

impl ReaderList {
    pub fn new() -> Self {
        Self {
            objs: Vec::new(),
            ..Default::default()
        }
    }

    pub fn len(&self) -> usize {
        self.objs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objs.is_empty()
    }

    pub fn remove(&mut self, i: usize) -> Box<dyn FactoryItemRead> {
        self.objs.remove(i)
    }

    pub fn at<T: 'static>(&self, i: usize) -> &T {
        let v = &self.objs[i].downcast_ref::<T>().unwrap();
        v
    }
}

impl traits::Object for ReaderList {
    fn class(&self) -> &'_ str {
        match &self.name {
            None => "TList",
            Some(s) => s,
        }
    }
}

impl traits::Object for WriterList<'_> {
    fn class(&self) -> &'_ str {
        match &self.name {
            None => "TList",
            Some(s) => s,
        }
    }
}

impl Unmarshaler for ReaderList {
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

impl traits::Named for ReaderList {
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
impl traits::Named for WriterList<'_> {
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

impl FactoryBuilder for ReaderList {
    fn register(factory: &mut Factory) {
        let f = || {
            let v: ReaderList = ReaderList::new();
            let b: Box<dyn FactoryItemRead> = Box::new(v);
            b
        };

        factory.add("TList", f);
    }
}

impl Marshaler for WriterList<'_> {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        trace!(";List.marshal.call.w.pos:{:?}", w.pos());
        let beg = w.pos();

        let hdr = w.write_header(self.class(), rvers::LIST)?;

        self.obj.marshal(w)?;

        w.write_string(self.name.as_ref().unwrap_or(&String::new()))?;

        w.write_i32(self.objs.len() as i32)?;

        trace!(";List.marshal.buf.value:{:?}", w.p());
        for (obj, addr) in self.objs.iter() {
            trace!(";List.marshal.buf.pos:{:?}", w.pos());
            w.write_object_any(*obj, *addr)?;
            w.write_string("")?;
        }

        trace!(";List.marshal.buf.value:{:?}", w.p());
        trace!(";List.marshal.buf.pos:{:?}", w.pos());

        w.set_header(hdr)?;

        Ok(w.pos() - beg)
    }
}
