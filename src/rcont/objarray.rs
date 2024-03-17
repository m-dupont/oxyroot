use crate::rbase::AttFill;
use crate::rbytes::ensure_maximum_supported_version;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::Unmarshaler;
use crate::root::traits;
use crate::root::traits::Object;
use crate::rvers;
use crate::{rbase, Marshaler};
use log::trace;

use crate::rtypes::factory::{Factory, FactoryBuilder, FactoryItem};

#[derive(Default, Debug)]
pub struct ObjArray {
    obj: rbase::Object,
    name: Option<String>,
    pub objs: Vec<Box<dyn FactoryItem>>,
    _last: i32,
    low: i32,
}

impl ObjArray {
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

    pub fn take_objs(&mut self) -> Vec<Box<dyn FactoryItem>> {
        std::mem::take(&mut self.objs)
    }

    // pub fn into_iter_as(&mut self) -> Iter
}

impl Object for ObjArray {
    fn class(&self) -> &'_ str {
        "TObjArray"
    }
}

impl traits::Named for ObjArray {
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

impl Unmarshaler for ObjArray {
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

impl Marshaler for ObjArray {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        todo!()
    }
}

impl FactoryBuilder for ObjArray {
    fn register(factory: &mut Factory) {
        let f = || {
            let v: Self = Self::new();
            let b: Box<dyn FactoryItem> = Box::new(v);
            b
        };

        factory.add("TObjArray", f);
    }
}
