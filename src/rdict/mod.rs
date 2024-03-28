use crate::rbase::AttFill;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::{ensure_maximum_supported_version, RVersioner, Unmarshaler};
/// Mod rdict contains the definition of ROOT streamers and facilities
/// to generate new streamers meta data from user types.
use crate::{factory_all_for_register_impl, factory_fn_register_impl, rbase, Marshaler};
use downcast::Any;
use log::trace;
use std::iter::{empty, once};
use std::ptr::addr_of;

use crate::rbytes;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::Error::Misc;
use crate::rcont;
use crate::rmeta;
use crate::rmeta::{ESTLType, Enum, EnumNamed};
use crate::root;
use crate::root::traits;
use crate::root::traits::Named;
use crate::root::traits::Object;
use crate::rtypes::factory::FactoryItemRead;
use crate::rvers;

#[derive(Debug, Clone)]
pub enum Streamer {
    String(StreamerString),
    STLstring(StreamerSTLstring),
    BasicType(StreamerBasicType),
    BasicPointer(StreamerBasicPointer),
    ObjectAny(StreamerObjectAny),
    Stl(StreamerSTL),
    Base(StreamerBase),
    Object(StreamerObject),
    ObjectPointer(StreamerObjectPointer),
}

impl TryFrom<Box<dyn FactoryItemRead>> for Streamer {
    // TODO: change to rdict specific error
    type Error = crate::rbytes::Error;

    fn try_from(value: Box<dyn FactoryItemRead>) -> Result<Self, Self::Error> {
        let ret = match value.class() {
            "TStreamerBasicType" => {
                Streamer::BasicType(*value.downcast::<StreamerBasicType>().unwrap())
            }
            "TStreamerString" => Streamer::String(*value.downcast::<StreamerString>().unwrap()),
            "TStreamerSTL" => Streamer::Stl(*value.downcast::<StreamerSTL>().unwrap()),
            "TStreamerBase" => Streamer::Base(*value.downcast::<StreamerBase>().unwrap()),
            "TStreamerObject" => Streamer::Object(*value.downcast::<StreamerObject>().unwrap()),
            "TStreamerObjectPointer" => {
                Streamer::ObjectPointer(*value.downcast::<StreamerObjectPointer>().unwrap())
            }
            "TStreamerSTLstring" => {
                Streamer::STLstring(*value.downcast::<StreamerSTLstring>().unwrap())
            }
            "TStreamerBasicPointer" => {
                Streamer::BasicPointer(*value.downcast::<StreamerBasicPointer>().unwrap())
            }
            "TStreamerObjectAny" => {
                Streamer::ObjectAny(*value.downcast::<StreamerObjectAny>().unwrap())
            }
            _ => {
                return Err(Misc(format!(
                    "Unknow type or write code for {}",
                    value.class()
                )))
            }
        };
        Ok(ret)
    }
}

impl Streamer {
    pub fn name(&self) -> &'_ str {
        match self {
            Streamer::String(a) => a.element.name(),
            Streamer::STLstring(a) => a.streamer_stl.element.name(),
            Streamer::BasicType(a) => a.element.name(),
            Streamer::BasicPointer(a) => a.element.name(),
            Streamer::ObjectAny(a) => a.element.name(),
            Streamer::Stl(a) => a.element.name(),
            Streamer::Base(a) => a.element.name(),
            Streamer::Object(a) => a.element.name(),
            Streamer::ObjectPointer(a) => a.element.name(),
        }
    }

    fn element(&self) -> &StreamerElement {
        match self {
            Streamer::String(a) => &a.element,
            Streamer::STLstring(a) => &a.streamer_stl.element,
            Streamer::BasicType(a) => &a.element,
            Streamer::BasicPointer(a) => &a.element,
            Streamer::ObjectAny(a) => &a.element,
            Streamer::Stl(a) => &a.element,
            Streamer::Base(a) => &a.element,
            Streamer::Object(a) => &a.element,
            Streamer::ObjectPointer(a) => &a.element,
        }
    }

    pub fn item_type_name(&self) -> &str {
        self.element().ename.as_str()
    }
}

impl Marshaler for Streamer {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        match self {
            Streamer::String(a) => a.marshal(w),
            Streamer::STLstring(a) => a.marshal(w),
            Streamer::BasicType(a) => a.marshal(w),
            Streamer::BasicPointer(a) => a.marshal(w),
            Streamer::ObjectAny(a) => a.marshal(w),
            Streamer::Stl(a) => a.marshal(w),
            Streamer::Base(a) => a.marshal(w),
            Streamer::Object(a) => a.marshal(w),
            Streamer::ObjectPointer(a) => a.marshal(w),
        }
    }
}

impl Object for Streamer {
    fn class(&self) -> &'_ str {
        match self {
            Streamer::String(_) => "TStreamerString",
            Streamer::STLstring(_) => "TStreamerSTLstring",
            Streamer::BasicType(_) => "TStreamerBasicType",
            Streamer::BasicPointer(_) => "TStreamerBasicPointer",
            Streamer::ObjectAny(_) => "TStreamerObjectAny",
            Streamer::Stl(_) => "TStreamerSTL",
            Streamer::Base(_) => "TStreamerBase",
            Streamer::Object(_) => "TStreamerObject",
            Streamer::ObjectPointer(_) => "TStreamerObjectPointer",
        }
    }
}

impl Named for Streamer {}

impl RVersioner for Streamer {
    fn rversion(&self) -> i16 {
        todo!()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Streamers {
    obj: rbase::Object,
    pub(crate) list: Vec<Streamer>,
    low: i32,
}

impl Object for Streamers {
    fn class(&self) -> &'_ str {
        "TObjArray"
    }
}

impl traits::Named for Streamers {
    fn name(&self) -> &'_ str {
        "TObjArray"
    }

    fn title(&self) -> &'_ str {
        "An array of objects"
    }
}

impl RVersioner for Streamers {
    fn rversion(&self) -> i16 {
        rvers::OBJ_ARRAY
    }
}

impl Marshaler for Streamers {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len() - 1;
        let beg = w.pos();
        trace!(";Streamers.marshal.a{beg}.beg:{}", beg);
        trace!(";Streamers.marshal.a{beg}.arr.len:{}", self.list.len());
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        w.write_object(&self.obj)?;

        w.write_string("")?;

        //trace!(";Streamers.marshal.buf.value:{:?}", w.p());

        w.write_i32(self.list.len().try_into()?)?;
        w.write_i32(self.low)?;
        //trace!(";Streamers.marshal.buf.value:{:?}", w.p());

        trace!(
            ";Streamers.marshal.before_write_array.buf.pos:{:?}",
            w.pos()
        );
        trace!(
            ";Streamers.marshal.before_write_array.buf.value:{:?}",
            &w.p()[len..]
        );

        for (i, obj) in self.list.iter().enumerate() {
            trace!(";Streamers.marshal.writing_array.buf.pos:{:?}", w.pos());
            trace!(
                ";Streamers.marshal.a{beg}.writing_array.{i}.pos:{}",
                w.pos()
            );
            w.write_object_any(obj, obj.element().id)?;
        }

        w.set_header(hdr)
    }
}

impl Streamers {
    pub fn push(&mut self, value: Streamer) {
        self.list.push(value);
    }

    pub fn get(&self, name: &str) -> Option<&Streamer> {
        self.list.iter().find(|&s| s.name() == name)
    }
}

#[derive(Default, Debug)]
pub struct StreamerInfo {
    named: rbase::Named,
    chksum: u32,
    clsver: i32,
    // objarr: rcont::objarray::ObjArray,
    // wobjs: WriterObjArray,
    pub(crate) elems: Streamers,
    pub(crate) id: usize,
}

impl Clone for StreamerInfo {
    fn clone(&self) -> Self {
        let mut s = StreamerInfo::default();
        s.named = self.named.clone();
        s.chksum = self.chksum;
        s.clsver = self.clsver;
        s.elems = self.elems.clone();
        s.id = ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        s
    }
}

impl StreamerInfo {
    pub fn get(&self, name: &str) -> Option<&Streamer> {
        self.elems.get(name)
    }
    pub(crate) fn new(name: &str, chksum: u32, clsver: i32) -> Self {
        trace!(";StreamerInfo.new.name:{}", name);
        trace!(";StreamerInfo.new.chksum:{}", chksum);
        trace!(";StreamerInfo.new.clsver:{}", clsver);
        let mut s = StreamerInfo::default();
        s.named = s.named.with_name(name.to_string());
        s.chksum = chksum;
        s.clsver = clsver;
        s
    }
    pub fn clsver(&self) -> i32 {
        self.clsver
    }

    pub(crate) fn new_from_streamerq(name: String, elems: Vec<Streamer>) -> Self {
        let mut s = StreamerInfo::default();
        s.named = s.named.with_name(name);
        s.elems.list = elems;
        s
    }
}

impl Named for StreamerInfo {
    fn name(&self) -> &'_ str {
        self.named.name()
    }
}

impl rbytes::RVersioner for StreamerInfo {
    fn rversion(&self) -> i16 {
        rvers::STREAMER_INFO
    }
}

impl Unmarshaler for StreamerInfo {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        if hdr.vers > rvers::STREAMER_INFO {
            return Err(crate::rbytes::Error::VersionTooHigh {
                class: self.class().into(),
                version_read: hdr.vers,
                max_expected: rvers::STREAMER_INFO,
            });
        }

        r.read_object(&mut self.named)?;

        self.chksum = r.read_u32()?;
        self.clsver = r.read_i32()?;

        let objs: Box<dyn FactoryItemRead> = r.read_object_any_into()?.expect("something is wrong");

        let objarr = *objs.downcast::<rcont::objarray::ReaderObjArray>().unwrap();
        //
        // let objarr = r.read_object_into::<rcont::objarray::ObjArray>()?;

        for elem in objarr.objs {
            self.elems.push(elem.try_into()?);
        }

        // self.elems.append(&mut self.objarr.objs);

        Ok(())

        // todo!()
    }
}

impl Marshaler for StreamerInfo {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len() - 1;
        let beg = w.pos();
        trace!(";StreamerInfo.marshal.a{beg}.beg:{}", beg);

        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        // trace!(";StreamerInfo.marshal.buf.value:{:?}", &w.p()[len..]);

        trace!(";StreamerInfo.marshal.a{beg}.clsver:{}", self.clsver);
        trace!(";StreamerInfo.marshal.a{beg}.chksum:{}", self.chksum);

        w.write_object(&self.named)?;
        w.write_u32(self.chksum)?;
        w.write_i32(self.clsver)?;
        trace!(
            ";StreamerInfo.marshal.a{}.before_array.buf.value:{:?}",
            beg,
            &w.p()[len..]
        );
        trace!(
            ";StreamerInfo.marshal.a{}.before_array.buf.pos:{:?}",
            beg,
            w.pos()
        );
        trace!(
            ";StreamerInfo.marshal.a{}.before_array.buf.len:{:?}",
            beg,
            &w.p()[len..].len()
        );
        w.write_object_any(&self.elems, self.id)?;
        trace!(
            ";StreamerInfo.marshal.a{}.after_array.buf.len:{:?}",
            beg,
            &w.p()[len..].len()
        );

        w.set_header(hdr)
    }
}

factory_fn_register_impl!(StreamerInfo, "TStreamerInfo");

pub(crate) struct Visitor<F>
where
    F: FnMut(i32, &Streamer),
{
    visited: Vec<usize>,
    f: F,
}

impl<F> Visitor<F>
where
    F: FnMut(i32, &Streamer),
{
    pub(crate) fn new(f: F) -> Self {
        Visitor {
            visited: Vec::new(),
            f: f,
        }
    }

    pub(crate) fn run(&mut self, depth: i32, si: &StreamerInfo) -> Result<(), Error> {
        if si.name() == "TArray" {
            trace!("dep");
        }
        trace!(";StreamerInfo.run.si.name:{} depth={} ", si.name(), depth,);
        for s in si.elems.list.iter() {
            trace!(
                ";StreamerInfo.run.for_loop.{}.name:{} depth={} ",
                si.name(),
                s.name(),
                depth,
            );

            self.visit(depth, s)?
        }
        Ok(())
    }

    pub(crate) fn visit(&mut self, depth: i32, se: &Streamer) -> Result<(), Error> {
        if depth == 10 {
            todo!()
        }
        trace!(";StreamerInfo.visit.se.name:{} depth={} ", se.name(), depth,);
        if se.name() == "TArray" {
            trace!("de");
        }

        if self.visited.contains(&se.element().id) {
            trace!(
                ";StreamerInfo.visit.se.name:{} depth={}=visited",
                se.name(),
                depth,
            );

            return Ok(());
        }

        self.visited.push(se.element().id);

        (self.f)(depth, &se);

        let name = se.name().to_string();
        let tname = se.element().ename.clone();
        if tname == "TVirtualIndex" || tname == "TVirtualIndex*" {
            return Ok(());
        }

        //

        match se {
            Streamer::String(_) => {}
            Streamer::STLstring(_) => {}
            Streamer::BasicType(_) => {}
            Streamer::BasicPointer(_) => {}
            Streamer::ObjectAny(se) => {
                let tname = &se.element.ename;
                // visited.push(tname.to_string());
                let si = streamer_info(tname, -1)?;
                self.run(depth + 1, &si)?
                // itt.push(Box::new(X));
            }
            Streamer::Stl(stl) => {
                match &stl.vtype {
                    ESTLType::STLvector => {
                        let etn = se.item_type_name();
                        trace!(";StreamerInfo.visit.se.etn:{} {}", etn, depth);
                        // itt.push(Box::new(empty::<StreamerInfo>()));
                        // todo!("Streamer::Stl");
                    }
                    _ => {
                        todo!("Streamer::Stl, vtype = {:?}", &stl.vtype);
                    }
                }
            }
            Streamer::Base(se) => {
                // visited.push(name.to_string());
                let si = streamer_info(&name, -1)?;
                self.run(depth + 1, &si)?;
                // itt.push(Box::new());
            }
            Streamer::Object(se) => {
                let tname = &se.element.ename;
                // visited.push(tname.to_string());
                let si = streamer_info(tname, -1)?;
                self.run(depth + 1, &si)?;
                // itt.push(Box::new());
            }
            Streamer::ObjectPointer(se) => {
                let tname = se.element.ename.trim_end_matches('*');
                // visited.push(tname.to_string());
                let si = streamer_info(tname, -1)?;
                self.run(depth + 1, &si)?;
                // itt.push(Box::new());
            }
        }

        Ok(())
        // Ok(itt.into_iter().flatten())
    }
}

#[derive(Default, Debug)]
pub(crate) struct StreamerElement {
    named: rbase::Named,
    etype: rmeta::Enum,
    // element type
    /// size of element
    esize: i32,
    /// cumulative size of all array dims
    arr_len: i32,
    /// number of array dimensions
    arr_dim: i32,
    /// maximum array index for array dimension "dim"
    max_idx: [i32; 5],
    /// element offset in class
    _offset: i32,
    /// data type name of data member
    pub(crate) ename: String,
    /// minimum of data member if a range is specified [xmin.xmax.nbits]
    xmin: f64,
    /// maximum of data member if a range is specified [xmin.xmax.nbits]
    xmax: f64,
    /// conversion factor if a range is specified. factor = (1<<nbits/(xmax-xmin))
    factor: f64,
    id: usize,
}

impl Clone for StreamerElement {
    fn clone(&self) -> Self {
        let mut s = StreamerElement::default();
        s.named = self.named.clone();
        s.etype = self.etype.clone();
        s.esize = self.esize;
        s.arr_len = self.arr_len;
        s.arr_dim = self.arr_dim;
        s.max_idx = self.max_idx;
        s.ename = self.ename.clone();
        s.xmin = self.xmin;
        s.xmax = self.xmax;
        s.factor = self.factor;
        s.id = ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        s
    }
}

impl StreamerElement {
    pub fn etype(&self) -> &rmeta::Enum {
        &self.etype
    }
    pub(crate) fn new(name: &str, etype: Enum, esize: i32, id: usize) -> Self {
        trace!(";StreamerElement.new.{}:call", name);
        //trace!(";StreamerElement.new.{}.etype:{:?}", name, etype);
        let mut s = StreamerElement::default();
        s.named = s.named.with_name(name.to_string());
        s.etype = etype;
        s.esize = esize;
        s.id = id;
        s
    }
}

impl traits::Object for StreamerElement {
    fn class(&self) -> &'_ str {
        "TStreamerElement"
    }
}

impl root::traits::Named for StreamerElement {
    fn name(&self) -> &'_ str {
        self.named.name()
    }

    fn title(&self) -> &'_ str {
        self.named.title()
    }
}

impl Unmarshaler for StreamerElement {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        if hdr.vers > rvers::STREAMER_ELEMENT {
            return Err(crate::rbytes::Error::VersionTooHigh {
                class: self.class().into(),
                version_read: hdr.vers,
                max_expected: rvers::STREAMER_ELEMENT,
            });
        }

        r.read_object(&mut self.named)?;

        // = Enum::from_i32();

        // self.etype = num::FromPrimitive::from_i32(r.read_i32()?).unwrap();
        self.etype = Enum::from_i32(r.read_i32()?);

        self.esize = r.read_i32()?;
        self.arr_len = r.read_i32()?;
        self.arr_dim = r.read_i32()?;

        if hdr.vers == 1 {
            todo!();
        } else {
            r.read_array_i32(&mut self.max_idx)?;
        }

        self.ename = r.read_string()?.to_string();

        if let Enum::Named(EnumNamed::UChar) = &self.etype {
            if self.ename == "Bool_t" || self.ename == "bool" {
                self.etype = Enum::Named(EnumNamed::Bool);
            }
        }

        if hdr.vers == 3 {
            self.xmin = r.read_f64()?;
            todo!()
        } else if hdr.vers > 3 {
            (self.xmin, self.xmax, self.factor) = streamer_types::get_range(self.title());
        } else {
            self.xmin = 0.;
            self.xmax = 0.;
            self.factor = 0.;
        }

        r.check_header(&hdr)?;

        Ok(())

        // todo!()
    }
}

impl Marshaler for StreamerElement {
    fn marshal(&self, w: &mut WBuffer) -> rbytes::Result<i64> {
        let len = w.len() - 1;
        let beg = w.pos();

        let etype_i32 = match self.etype {
            Enum::Named(EnumNamed::TNamed) => 0,
            Enum::Named(EnumNamed::TObject) => 0,
            _ => self.etype.to_i32(),
        };

        trace!(";StreamerElement.marshal.a{}.beg:{}", beg, beg);
        trace!(";StreamerElement.marshal.a{}.etype:{:?}", beg, self.etype);
        trace!(
            ";StreamerElement.marshal.a{}.etype_i32:{:?}",
            beg,
            etype_i32
        );
        trace!(";StreamerElement.marshal.a{}.ename:{:?}", beg, self.ename);
        trace!(
            ";StreamerElement.marshal.a{}.named.name:{:?}",
            beg,
            self.named.name()
        );
        trace!(
            ";StreamerElement.marshal.a{}.named.title:{:?}",
            beg,
            self.named.title()
        );
        trace!(
            ";StreamerElement.marshal.a{beg}.buf.value:{:?}",
            &w.p()[len..]
        );
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        trace!(
            ";StreamerElement.marshal.a{beg}.buf.len_after_hdr:{:?}",
            &w.p()[len..].len()
        );
        trace!(
            ";StreamerElement.marshal.a{beg}.buf.value:{:?}",
            &w.p()[len..]
        );
        w.write_object(&self.named)?;
        trace!(
            ";StreamerElement.marshal.a{beg}.buf.len_after_named:{:?}",
            &w.p()[len..].len()
        );
        trace!(
            ";StreamerElement.marshal.a{beg}.buf.value:{:?}",
            &w.p()[len..]
        );

        w.write_i32(etype_i32)?;
        w.write_i32(self.esize)?;
        trace!(
            ";StreamerElement.marshal.a{beg}.buf.value:{:?}",
            &w.p()[len..]
        );
        w.write_i32(self.arr_len)?;
        w.write_i32(self.arr_dim)?;
        trace!(
            ";StreamerElement.marshal.a{beg}.buf.value:{:?}",
            &w.p()[len..]
        );
        w.write_array_i32(&self.max_idx)?;
        trace!(
            ";StreamerElement.marshal.a{beg}.buf.value:{:?}",
            &w.p()[len..]
        );
        w.write_string(&self.ename)?;
        if self.rversion() == 3 {
            w.write_f64(self.xmin)?;
            w.write_f64(self.xmax)?;
            w.write_f64(self.factor)?;
        } else if self.rversion() > 3 {
        }

        trace!(
            ";StreamerElement.marshal.a{beg}.buf.len_end:{:?}",
            &w.p()[len..].len()
        );
        w.set_header(hdr)
    }
}

impl RVersioner for StreamerElement {
    fn rversion(&self) -> i16 {
        rvers::STREAMER_ELEMENT
    }
}

pub mod streamers;

mod error;

use crate::rcont::objarray::{ReaderObjArray, WriterObjArray};
use crate::rdict::streamers::db::streamer_info_from;
use crate::rdict::streamers::db::{streamer_info, ID};
use crate::rtree::leaf::Leaf;
pub use error::Error;
use streamers::streamer_types;
use streamers::streamer_types::{
    StreamerBase, StreamerBasicPointer, StreamerBasicType, StreamerObject, StreamerObjectAny,
    StreamerObjectPointer, StreamerSTL, StreamerSTLstring, StreamerString,
};
