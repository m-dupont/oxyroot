use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::Unmarshaler;
use crate::rbytes::{ensure_maximum_supported_version, RVersioner};
use crate::root::traits;
use crate::root::traits::Object;
use crate::rvers;
use crate::{rbase, Marshaler};
use log::trace;

use crate::rtypes::factory::{Factory, FactoryBuilder, FactoryItemWrite};
use crate::rtypes::FactoryItemRead;

#[derive(Default, Debug)]
pub struct ReaderObjArray {
    obj: rbase::Object,
    name: Option<String>,
    pub objs: Vec<Box<dyn FactoryItemRead>>,
    _last: i32,
    low: i32,
}
#[derive(Default, Debug)]
pub struct WriterObjArray<'a> {
    obj: rbase::Object,
    name: Option<String>,
    objs: Vec<(&'a dyn FactoryItemWrite, usize)>,
    _last: i32,
    low: i32,
}

impl<'a> WriterObjArray<'a> {
    pub fn new() -> Self {
        Self {
            objs: Vec::new(),
            ..Default::default()
        }
    }

    pub(crate) fn push<'b>(&mut self, obj: &'a (dyn FactoryItemWrite + 'b), ptr: usize) {
        self.objs.push((obj, ptr));
    }
}

impl ReaderObjArray {
    pub fn new() -> Self {
        Self {
            objs: Vec::new(),
            ..Default::default()
        }
    }

    // pub fn len(&self) -> usize {
    //     self.objs.len()
    // }
    //
    // pub fn at(&self, i: usize) -> &Box<dyn FactoryItem> {
    //     return self.objs.get(i).unwrap();
    // }

    pub fn take_objs(&mut self) -> Vec<Box<dyn FactoryItemRead>> {
        std::mem::take(&mut self.objs)
    }

    // pub fn into_iter_as(&mut self) -> Iter
}

impl Object for ReaderObjArray {
    fn class(&self) -> &'_ str {
        "TObjArray"
    }
}
impl Object for WriterObjArray<'_> {
    fn class(&self) -> &'_ str {
        "TObjArray"
    }
}

impl traits::Named for WriterObjArray<'_> {
    fn name(&self) -> &'_ str {
        match &self.name {
            None => "TObjArray",
            Some(s) => s,
        }
    }

    fn title(&self) -> &'_ str {
        "An array of objects"
    }
}
impl traits::Named for ReaderObjArray {
    fn name(&self) -> &'_ str {
        match &self.name {
            None => "TObjArray",
            Some(s) => s,
        }
    }

    fn title(&self) -> &'_ str {
        "An array of objects"
    }
}

impl Unmarshaler for ReaderObjArray {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let _beg = r.pos();
        trace!(";ObjArray.unmarshal.beg: {}", _beg);
        trace!(";ObjArray.unmarshal.{}.beg: {}", _beg, _beg);

        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, rvers::OBJ_ARRAY, self.class())?;

        trace!(";ObjArray.unmarshal.{}.hdr.vers: {}", _beg, hdr.vers);
        if hdr.vers > 2 {
            r.read_object(&mut self.obj)?;
        }

        if hdr.vers > 1 {
            self.name = Some(r.read_string()?.to_string());
        }

        trace!(";ObjArray.unmarshal.{}.name: {:?}", _beg, self.name);

        let nobjs = r.read_i32()?;

        //trace!(";ObjArray.unmarshal.{}.beg.nobjs: {}", _beg, _beg);
        trace!(";ObjArray.unmarshal.{}.nobjs: {}", _beg, nobjs);
        self.low = r.read_i32()?;

        for i in 0..nobjs {
            trace!(";ObjArray.unmarshal.{}.i: {}", _beg, i);
            let pos = r.pos();
            trace!(";ObjArray.unmarshal.{}.objs.{}.pos: {}", _beg, i, pos);
            let obj = r.read_object_any_into()?;
            if let Some(obj) = obj {
                self.objs.push(obj);
            }
        }

        trace!(";ObjArray.unmarshal.{}.objs.len: {}", _beg, self.objs.len());

        let end = r.pos();
        trace!(";ObjArray.unmarshal.{}.end: {}", _beg, end);

        Ok(())
    }
}

impl FactoryBuilder for ReaderObjArray {
    fn register(factory: &mut Factory) {
        let f = || {
            let v: Self = Self::new();
            let b: Box<dyn FactoryItemRead> = Box::new(v);
            b
        };

        factory.add("TObjArray", f);
    }
}

impl RVersioner for WriterObjArray<'_> {
    fn rversion(&self) -> i16 {
        rvers::OBJ_ARRAY
    }
}

impl Marshaler for WriterObjArray<'_> {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        w.write_object(&self.obj)?;

        // trace!(";WriterObjArray.marshal.name: {:?}", self.name());

        match &self.name {
            None => {
                w.write_string("")?;
            }
            Some(s) => {
                w.write_string(s)?;
            }
        }

        trace!(";WriterObjArray.marshal.buf.value:{:?}", w.p());

        w.write_i32(self.objs.len().try_into()?)?;
        w.write_i32(self.low)?;
        trace!(";WriterObjArray.marshal.buf.value:{:?}", w.p());

        for (obj, addr) in self.objs.iter() {
            trace!(";WriterObjArray.marshal.buf.pos:{:?}", w.pos());
            w.write_object_any(*obj, *addr)?;
        }

        w.set_header(hdr)
    }
}
