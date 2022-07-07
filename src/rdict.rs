use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::{RVersioner, Unmarshaler};
/// Mod rdict contains the definition of ROOT streamers and facilities
/// to generate new streamers meta data from user types.
use crate::{factotry_all_for_register_impl, factotry_fn_register_impl, rbase};
use anyhow::bail;
use anyhow::ensure;
use log::{info, trace};

use crate::rbytes;
use crate::rcont;
use crate::rmeta;
use crate::rmeta::{ESTLType, Enum, EnumNamed};
use crate::root;
use crate::root::traits;
use crate::root::traits::Named;
use crate::root::traits::Object;
use crate::rtypes::factory::FactoryItem;
use crate::rvers;

pub enum Streamer {
    String(StreamerString),
    STLstring(StreamerSTLstring),
    BasicType(StreamerBasicType),
    BasicPointer(StreamerBasicPointer),
    ObjectAny(StreamerObjectAny),
    STL(StreamerSTL),
    Base(StreamerBase),
    Object(StreamerObject),
    ObjectPointer(StreamerObjectPointer),
}

impl TryFrom<Box<dyn FactoryItem>> for Streamer {
    type Error = anyhow::Error;

    fn try_from(value: Box<dyn FactoryItem>) -> anyhow::Result<Self> {
        let ret = match value.class() {
            "TStreamerBasicType" => {
                Streamer::BasicType(*value.downcast::<StreamerBasicType>().unwrap())
            }
            "TStreamerString" => Streamer::String(*value.downcast::<StreamerString>().unwrap()),
            "TStreamerSTL" => Streamer::STL(*value.downcast::<StreamerSTL>().unwrap()),
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
            _ => anyhow::bail!("Unknow type or write code for {}", value.class()),
        };
        Ok(ret)
    }
}

impl Streamer {
    fn name(&self) -> &'_ str {
        match self {
            Streamer::String(a) => a.element.name(),
            Streamer::STLstring(a) => a.streamer_stl.element.name(),
            Streamer::BasicType(a) => a.element.name(),
            Streamer::BasicPointer(a) => a.element.name(),
            Streamer::ObjectAny(a) => a.element.name(),
            Streamer::STL(a) => a.element.name(),
            Streamer::Base(a) => a.element.name(),
            Streamer::Object(a) => a.element.name(),
            Streamer::ObjectPointer(a) => a.element.name(),
        }
    }
}

#[derive(Default)]
pub struct Streamers {
    list: Vec<Streamer>,
}

impl Streamers {
    pub fn push(&mut self, value: Streamer) {
        self.list.push(value);
    }

    pub fn get(&self, name: &str) -> Option<&Streamer> {
        for s in self.list.iter() {
            trace!("looking for name = {}, s.name = {}", name, s.name());
            if s.name() == name {
                return Some(s);
            }
        }
        None
    }
}

#[derive(Default)]
pub struct StreamerInfo {
    named: rbase::Named,
    chksum: u32,
    clsver: i32,
    // objarr: rcont::objarray::ObjArray,
    elems: Streamers,
}

impl StreamerInfo {
    pub fn get(&self, name: &str) -> Option<&Streamer> {
        self.elems.get(name)
    }
}

impl Named for StreamerInfo {
    fn name(&self) -> &'_ str {
        self.named.name()
    }
}

impl rbytes::RVersioner for StreamerInfo {
    fn rversion() -> i16 {
        rvers::STREAMER_INFO
    }
}

impl Unmarshaler for StreamerInfo {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        let hdr = r.read_header(self.class())?;

        trace!("hdr = {:?}", hdr);

        ensure!(
            !(hdr.vers > rvers::STREAMER_INFO),
            "rdict: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::LIST
        );

        r.read_object(&mut self.named)?;

        trace!("named = {:?}", self.named);

        self.chksum = r.read_u32()?;
        self.clsver = r.read_i32()?;

        let objs: Box<dyn FactoryItem> = r.read_object_any_into()?.expect("something is wrong");

        let objarr = *objs.downcast::<rcont::objarray::ObjArray>().unwrap();

        // self.objarr = r.read_object_into::<rcont::objarray::OBJ_ARRAY>()?;

        objarr
            .objs
            .iter()
            .enumerate()
            .for_each(|(i, elem)| trace!("i = {i}, elem class = {}", elem.class()));

        for elem in objarr.objs {
            self.elems.push(elem.try_into()?);
        }

        // self.elems.append(&mut self.objarr.objs);

        Ok(())

        // todo!()
    }
}

factotry_fn_register_impl!(StreamerInfo, "TStreamerInfo");

#[derive(Default, Debug)]
pub struct StreamerElement {
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
    ename: String,
    /// minimum of data member if a range is specified [xmin.xmax.nbits]
    xmin: f64,
    /// maximum of data member if a range is specified [xmin.xmax.nbits]
    xmax: f64,
    /// conversion factor if a range is specified. factor = (1<<nbits/(xmax-xmin))
    factor: f64,
}

impl StreamerElement {
    pub fn etype(&self) -> &rmeta::Enum {
        &self.etype
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

fn get_range(s: &str) -> (f64, f64, f64) {
    trace!("get_range on {s}");
    let (xmin, xmax, factor) = (0., 0., 0.);

    if s == "" {
        return (xmin, xmax, factor);
    }

    let beg = s.rfind("[");

    if beg.is_none() {
        return (xmin, xmax, factor);
    }

    let beg = beg.unwrap();

    if beg > 0 {
        todo!()
    }

    let end = s.rfind("]");

    if end.is_none() {
        return (xmin, xmax, factor);
    }

    let end = end.unwrap();

    let s = &s[beg + 1..end];

    trace!("s = {s}");

    if s.rfind(",").is_none() {
        return (xmin, xmax, factor);
    }

    todo!()

    // return (xmin, xmax, factor);
}

impl Unmarshaler for StreamerElement {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        info!("STREAMER_ELEMENT:unmarshal");
        let hdr = r.read_header(self.class())?;

        ensure!(
            hdr.vers <= rvers::STREAMER_ELEMENT,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::STREAMER_ELEMENT
        );

        r.read_object(&mut self.named)?;

        // = Enum::from_i32();

        // self.etype = num::FromPrimitive::from_i32(r.read_i32()?).unwrap();
        self.etype = Enum::from_i32(r.read_i32()?);

        trace!("STREAMER_ELEMENT:unmarshal self.etype = {:?}", self.etype);

        self.esize = r.read_i32()?;
        self.arr_len = r.read_i32()?;
        self.arr_dim = r.read_i32()?;

        if hdr.vers == 1 {
            todo!();
        } else {
            r.read_array_i32(&mut self.max_idx)?;
        }

        trace!(
            "STREAMER_ELEMENT:unmarshal self.max_idx = {:?}",
            self.max_idx
        );

        self.ename = r.read_string()?.to_string();
        trace!("STREAMER_ELEMENT:unmarshal self.ename = {:?}", self.ename);

        match &self.etype {
            Enum::Named(ty) => match ty {
                EnumNamed::UChar => {
                    if self.ename == "Bool_t" || self.ename == "bool" {
                        self.etype = Enum::Named(EnumNamed::Bool);
                    }
                }

                _ => {}
            },
            _ => {}
        }

        if hdr.vers == 3 {
            self.xmin = r.read_f64()?;
            todo!()
        } else if hdr.vers > 3 {
            trace!("title = {}", self.title());

            (self.xmin, self.xmax, self.factor) = get_range(self.title());
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

#[derive(Default)]
pub struct StreamerBase {
    element: StreamerElement,
    vbase: i32,
}

impl Unmarshaler for StreamerBase {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("STREAMER_BASE:unmarshal");
        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::STREAMER_BASE,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::STREAMER_BASE
        );

        r.read_object(&mut self.element)?;

        if hdr.vers > 2 {
            self.vbase = r.read_i32()?;
        }

        r.check_header(&hdr)?;

        Ok(())
    }
}

factotry_all_for_register_impl!(StreamerBase, "TStreamerBase");

#[derive(Default)]
pub struct StreamerString {
    element: StreamerElement,
}

impl StreamerString {
    pub fn element(&self) -> &StreamerElement {
        &self.element
    }
}

impl Unmarshaler for StreamerString {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("STREAMER_STRING:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::STREAMER_STRING,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::STREAMER_STRING
        );

        r.read_object(&mut self.element)?;
        r.check_header(&hdr)?;

        Ok(())
    }
}

factotry_all_for_register_impl!(StreamerString, "TStreamerString");

#[derive(Default)]
pub struct StreamerBasicType {
    element: StreamerElement,
}

impl Unmarshaler for StreamerBasicType {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("STREAMER_STRING:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::STREAMER_BASIC_TYPE,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::STREAMER_BASIC_TYPE
        );

        r.read_object(&mut self.element)?;

        let mut etype = self.element.etype.to_i32();

        trace!("etype = {etype}");

        if EnumNamed::OffsetL.to_i32()? < etype && etype < EnumNamed::OffsetP.to_i32()? {
            etype -= EnumNamed::OffsetL.to_i32()?;
        }
        trace!("etype = {etype}");

        let mut basic = true;

        let etype = EnumNamed::from_i32(etype)?;

        match etype {
            EnumNamed::Bool | EnumNamed::UChar | EnumNamed::Char => {
                self.element.esize = 1;
            }
            EnumNamed::Short | EnumNamed::UShort => {
                self.element.esize = 2;
            }
            EnumNamed::Bits | EnumNamed::UInt | EnumNamed::Int | EnumNamed::Counter => {
                self.element.esize = 4;
            }
            EnumNamed::ULong | EnumNamed::ULong64 | EnumNamed::Long | EnumNamed::Long64 => {
                self.element.esize = 8;
            }
            EnumNamed::Float | EnumNamed::Float16 => {
                self.element.esize = 4;
            }
            EnumNamed::Double | EnumNamed::Double32 => {
                self.element.esize = 8;
            }
            EnumNamed::CharStar => {
                // unimplemented!()
                self.element.esize = 8;
                // self.element.esize =
            }

            _ => {
                basic = false;
            }
        }

        if basic && self.element.arr_len > 0 {
            self.element.esize *= self.element.arr_len;
        }

        trace!("esize = {}", self.element.esize);

        // todo!();

        r.check_header(&hdr)?;

        Ok(())
    }
}

factotry_all_for_register_impl!(StreamerBasicType, "TStreamerBasicType");

#[derive(Default)]
pub struct StreamerObject {
    element: StreamerElement,
}

impl Unmarshaler for StreamerObject {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        info!("STREAMER_OBJECT:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::STREAMER_OBJECT,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::STREAMER_OBJECT
        );

        r.read_object(&mut self.element)?;
        r.check_header(&hdr)?;
        Ok(())
    }
}

factotry_all_for_register_impl!(StreamerObject, "TStreamerObject");

#[derive(Default)]
pub struct StreamerObjectPointer {
    element: StreamerElement,
}

impl Unmarshaler for StreamerObjectPointer {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        info!("STREAMER_OBJECT_POINTER:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::STREAMER_OBJECT_POINTER,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::STREAMER_OBJECT_POINTER
        );

        r.read_object(&mut self.element)?;
        r.check_header(&hdr)?;
        Ok(())
    }
}

factotry_all_for_register_impl!(StreamerObjectPointer, "TStreamerObjectPointer");

#[derive(Default)]
pub struct StreamerObjectAny {
    element: StreamerElement,
}

impl Unmarshaler for StreamerObjectAny {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        info!("STREAMER_OBJECT_ANY:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::STREAMER_OBJECT_ANY,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::STREAMER_OBJECT_ANY
        );

        r.read_object(&mut self.element)?;
        r.check_header(&hdr)?;
        Ok(())
    }
}

factotry_all_for_register_impl!(StreamerObjectAny, "TStreamerObjectAny");

#[derive(Default)]
pub struct StreamerBasicPointer {
    element: StreamerElement,
    /// version number of the class with the counter
    cvers: i32,
    /// name of data member holding the array count
    cname: String,
    /// name of the class with the counter
    ccls: String,
}

factotry_all_for_register_impl!(StreamerBasicPointer, "TStreamerBasicPointer");

impl Unmarshaler for StreamerBasicPointer {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        info!("STREAMER_OBJECT_ANY:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::STREAMER_BASIC_POINTER,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::STREAMER_BASIC_POINTER
        );

        r.read_object(&mut self.element)?;

        self.cvers = r.read_i32()?;
        self.cname = r.read_string()?.to_string();
        self.ccls = r.read_string()?.to_string();

        trace!("ccls = {}", self.ccls);

        r.check_header(&hdr)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct StreamerSTL {
    element: StreamerElement,
    /// type of STL vector
    vtype: rmeta::ESTLType,
    /// STL contained type
    ctype: rmeta::Enum,
}

factotry_all_for_register_impl!(StreamerSTL, "TStreamerSTL");

impl RVersioner for StreamerSTL {
    fn rversion() -> i16 {
        rvers::STREAMER_STL
    }
}

impl Unmarshaler for StreamerSTL {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        info!("STREAMER_STL:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::STREAMER_STL,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            StreamerSTL::rversion()
        );

        r.read_object(&mut self.element)?;

        self.vtype = rmeta::ESTLType::from_i32(r.read_i32()?).unwrap();
        self.ctype = Enum::from_i32(r.read_i32()?);

        match self.vtype {
            ESTLType::STLmultimap | ESTLType::STLset => {
                if self.element.name().starts_with("std::set")
                    || self.element.name().starts_with("set")
                {
                    self.vtype = ESTLType::STLset;
                }

                if self.element.name().starts_with("std::multimap")
                    || self.element.name().starts_with("multimap")
                {
                    self.vtype = ESTLType::STLmultimap;
                }
            }
            _ => {}
        }

        trace!("self.vtype = {:?}", self.vtype);
        trace!("self.ctype = {:?}", self.ctype);

        r.check_header(&hdr)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct StreamerSTLstring {
    streamer_stl: StreamerSTL,
}

factotry_all_for_register_impl!(StreamerSTLstring, "TStreamerSTLstring");

impl Unmarshaler for StreamerSTLstring {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        info!("StreamerSTLstring:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::StreamerSTLstring,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::StreamerSTLstring
        );

        r.read_object(&mut self.streamer_stl)?;
        r.check_header(&hdr)?;
        Ok(())
    }
}
