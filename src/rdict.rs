use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::{RVersioner, Unmarshaler};
/// Mod rdict contains the definition of ROOT streamers and facilities
/// to generate new streamers meta data from user types.
use crate::{factotry_all_for_register_impl, rbase};
use anyhow::ensure;
use anyhow::Result;
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
use crate::rtypes::factory::{Factory, FactoryBuilder};
use crate::rvers;

use num;

#[derive(Default)]
pub struct StreamerInfo {
    named: rbase::Named,
    chksum: u32,
    clsver: i32,
    // objarr: Box<rcont::objarray::ObjArray>,
    objarr: rcont::objarray::ObjArray,
    // elems: Vec<Box<dyn rbytes::StreamerElement>>,
    elems: Vec<Box<dyn FactoryItem>>,
}

impl rbytes::RVersioner for StreamerInfo {
    fn rversion() -> i16 {
        rvers::StreamerInfo
    }
}

impl Unmarshaler for StreamerInfo {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        let hdr = r.read_header(self.class())?;

        trace!("hdr = {:?}", hdr);

        ensure!(
            !(hdr.vers > rvers::StreamerInfo),
            "rdict: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::List
        );

        r.read_object(&mut self.named)?;

        trace!("named = {:?}", self.named);

        self.chksum = r.read_u32()?;
        self.clsver = r.read_i32()?;

        let objs: Box<dyn FactoryItem> = r.read_object_any_into()?.expect("something is wrong");

        // let objs: Box<rcont::objarray::ObjArray> =

        self.objarr = *objs.downcast::<rcont::objarray::ObjArray>().unwrap();

        // self.objarr = r.read_object_into::<rcont::objarray::ObjArray>()?;

        self.elems.append(&mut self.objarr.objs);

        Ok(())

        // todo!()
    }
}

factotry_all_for_register_impl!(StreamerInfo, "TStreamerInfo");

pub struct Element {
    name: rbase::Named,
    // Type:   rmeta.Enum, // element type
    /// size of element
    size: i32,
    /// cumulative size of all array dims
    arr_len: i32,
    /// number of array dimensions
    arr_dim: i32,
    /// maximum array index for array dimension "dim"
    max_idx: [i32; 5],
    /// element offset in class
    offset: i32,
    /// data type name of data member
    ename: String,
    /// minimum of data member if a range is specified [xmin.xmax.nbits]
    xmin: f64,
    /// maximum of data member if a range is specified [xmin.xmax.nbits]
    xmax: f64,
    /// conversion factor if a range is specified. factor = (1<<nbits/(xmax-xmin))
    factor: f64,
}

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
    offset: i32,
    /// data type name of data member
    ename: String,
    /// minimum of data member if a range is specified [xmin.xmax.nbits]
    xmin: f64,
    /// maximum of data member if a range is specified [xmin.xmax.nbits]
    xmax: f64,
    /// conversion factor if a range is specified. factor = (1<<nbits/(xmax-xmin))
    factor: f64,
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
        info!("StreamerElement:unmarshal");
        let hdr = r.read_header(self.class())?;

        ensure!(
            hdr.vers <= rvers::StreamerElement,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::StreamerElement
        );

        r.read_object(&mut self.named)?;

        // = Enum::from_i32();

        // self.etype = num::FromPrimitive::from_i32(r.read_i32()?).unwrap();
        self.etype = Enum::from_i32(r.read_i32()?);

        trace!("StreamerElement:unmarshal self.etype = {:?}", self.etype);

        self.esize = r.read_i32()?;
        self.arr_len = r.read_i32()?;
        self.arr_dim = r.read_i32()?;

        if hdr.vers == 1 {
            todo!();
        } else {
            r.read_array_i32(&mut self.max_idx)?;
        }

        trace!(
            "StreamerElement:unmarshal self.max_idx = {:?}",
            self.max_idx
        );

        self.ename = r.read_string()?.to_string();
        trace!("StreamerElement:unmarshal self.ename = {:?}", self.ename);

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
        trace!("StreamerBase:unmarshal");
        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::StreamerBase,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::StreamerBase
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

impl Unmarshaler for StreamerString {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("StreamerString:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::StreamerString,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::StreamerString
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
        trace!("StreamerString:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::StreamerBasicType,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::StreamerBasicType
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
            todo!();
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
        info!("StreamerObject:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::StreamerObject,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::StreamerObject
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
        info!("StreamerObjectPointer:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::StreamerObjectPointer,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::StreamerObjectPointer
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
        info!("StreamerObjectAny:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::StreamerObjectAny,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::StreamerObjectAny
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
        info!("StreamerObjectAny:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::StreamerBasicPointer,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::StreamerBasicPointer
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
        rvers::StreamerSTL
    }
}

impl Unmarshaler for StreamerSTL {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        info!("StreamerSTL:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::StreamerSTL,
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
