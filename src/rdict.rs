/// Mod rdict contains the definition of ROOT streamers and facilities
/// to generate new streamers meta data from user types.
use crate::rbase;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use anyhow::ensure;
use anyhow::Result;
use log::trace;

use crate::rbytes;
use crate::rcont;
use crate::rmeta;
use crate::rmeta::Enum;
use crate::root;
use crate::root::traits;
use crate::root::traits::Named;
use crate::root::traits::Object;
use crate::rtypes::factory::FactoryItem;
use crate::rtypes::factory::{Factory, FactoryBuilder};
use crate::rvers;
use crate::rvers::StreamerElement;

#[derive(Default)]
pub struct StreamerInfo {
    named: rbase::Named,
    chksum: u32,
    clsver: i32,
    // objarr: Box<rcont::objarray::ObjArray>,
    objarr: rcont::objarray::ObjArray,
    elems: Vec<Box<dyn rbytes::StreamerElement>>,
}

impl StreamerInfo {
    pub fn new() -> StreamerInfo {
        StreamerInfo {
            ..Default::default()
        }
    }
}

impl root::traits::Object for StreamerInfo {
    fn class(&self) -> &'_ str {
        "TStreamerInfo"
    }
}

impl root::traits::Named for StreamerInfo {
    fn name(&self) -> &'_ str {
        self.named.name()
    }

    fn title(&self) -> &'_ str {
        self.named.title()
    }
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

        let objs: Box<dyn FactoryItem> = r.read_object_any_into()?;

        // let objs: Box<rcont::objarray::ObjArray> =

        self.objarr = *objs.downcast::<rcont::objarray::ObjArray>().unwrap();

        if self.objarr.len() > 0 {
            for i in 0..self.objarr.len() {
                let elem = self.objarr.at(i);
                let elem = elem.downcast_ref::<StreamerElement>()?;
                trace!("elem = {:?}", elem);
            }
        }

        todo!()
    }
}

impl FactoryBuilder for StreamerInfo {
    fn register(factory: &mut Factory) {
        let f = || {
            let v: StreamerInfo = StreamerInfo::new();
            let b: Box<dyn FactoryItem> = Box::new(v);
            b
        };

        factory.add("TStreamerInfo", f);
    }
}

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

fn get_range(str: &str) -> (f64, f64, f64) {
    let (xmin, xmax, factor) = (0., 0., 0.);

    if str == "" {
        return (xmin, xmax, factor);
    }

    if str.rfind("[").is_none() {
        return (xmin, xmax, factor);
    }

    todo!()

    // return (xmin, xmax, factor);
}

impl Unmarshaler for StreamerElement {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("StreamerElement:unmarshal");
        let hdr = r.read_header(self.class())?;

        ensure!(
            hdr.vers <= rvers::StreamerElement,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::StreamerElement
        );

        r.read_object(&mut self.named)?;

        let etype = r.read_i32()?;

        trace!("StreamerElement:unmarshal etype = {}", etype);

        self.esize = r.read_i32()?;
        self.arr_len = r.read_i32()?;
        self.arr_dim = r.read_i32()?;

        if hdr.vers == 1 {
            todo!();
        } else {
            r.read_array_i32(&mut self.max_idx);
        }

        trace!(
            "StreamerElement:unmarshal self.max_idx = {:?}",
            self.max_idx
        );

        self.ename = r.read_string()?.to_string();
        trace!("StreamerElement:unmarshal self.ename = {:?}", self.ename);

        match self.etype {
            Enum::UChar => {
                if self.ename == "Bool_t" || self.ename == "bool" {
                    self.etype = rmeta::Enum::Bool;
                }
            }

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

impl StreamerBase {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl traits::Object for StreamerBase {
    fn class(&self) -> &'_ str {
        "TStreamerBase"
    }
}

impl root::traits::Named for StreamerBase {
    fn name(&self) -> &'_ str {
        unimplemented!()
    }

    fn title(&self) -> &'_ str {
        unimplemented!()
    }
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

impl FactoryBuilder for StreamerBase {
    fn register(factory: &mut Factory) {
        let f = || {
            let v: Self = Self::new();
            let b: Box<dyn FactoryItem> = Box::new(v);
            b
        };

        factory.add("TStreamerBase", f);
    }
}

#[derive(Default)]
pub struct StreamerString {
    element: StreamerElement,
}

impl StreamerString {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl traits::Object for StreamerString {
    fn class(&self) -> &'_ str {
        "TStreamerString"
    }
}

impl root::traits::Named for StreamerString {
    fn name(&self) -> &'_ str {
        unimplemented!()
    }

    fn title(&self) -> &'_ str {
        unimplemented!()
    }
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

impl FactoryBuilder for StreamerString {
    fn register(factory: &mut Factory) {
        let f = || {
            let v: Self = Self::new();
            let b: Box<dyn FactoryItem> = Box::new(v);
            b
        };

        factory.add("TStreamerString", f);
    }
}
