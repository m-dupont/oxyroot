use crate::rbytes::Error::Misc;
use crate::rbytes::{ensure_maximum_supported_version, RVersioner, WBuffer};
use crate::rdict::StreamerElement;
use crate::rmeta::{ESTLType, Enum, EnumNamed};
use crate::{
    factory_all_for_register_impl, rbytes, rmeta, rvers, Marshaler, Named, Object, RBuffer,
    Unmarshaler,
};
use log::trace;
use num_traits::ToPrimitive;

pub fn get_range(s: &str) -> (f64, f64, f64) {
    let (xmin, xmax, factor) = (0., 0., 0.);

    if s.is_empty() {
        return (xmin, xmax, factor);
    }

    let beg = s.rfind('[');

    if beg.is_none() {
        return (xmin, xmax, factor);
    }

    let beg = beg.unwrap();

    if beg > 0 {
        todo!()
    }

    let end = s.rfind(']');

    if end.is_none() {
        return (xmin, xmax, factor);
    }

    let end = end.unwrap();

    let s = &s[beg + 1..end];

    if s.rfind(',').is_none() {
        return (xmin, xmax, factor);
    }

    todo!()

    // return (xmin, xmax, factor);
}

#[derive(Default, Debug, Clone)]
pub struct StreamerBase {
    pub(crate) element: StreamerElement,
    pub(crate) vbase: i32,
}

impl StreamerBase {
    pub fn vbase(&self) -> i32 {
        self.vbase
    }
}

impl Unmarshaler for StreamerBase {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        if hdr.vers > rvers::STREAMER_BASE {
            return Err(crate::rbytes::Error::VersionTooHigh {
                class: self.class().into(),
                version_read: hdr.vers,
                max_expected: rvers::STREAMER_BASE,
            });
        }

        r.read_object(&mut self.element)?;

        if hdr.vers > 2 {
            self.vbase = r.read_i32()?;
        }

        r.check_header(&hdr)?;

        Ok(())
    }
}

impl RVersioner for StreamerBase {
    fn rversion(&self) -> i16 {
        rvers::STREAMER_BASE
    }
}

impl Marshaler for StreamerBase {
    fn marshal(&self, w: &mut WBuffer) -> rbytes::Result<i64> {
        let len = w.len() - 1;
        let beg = w.pos();
        trace!(";StreamerBase.marshal.a{}.beg:{}", beg, beg);
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        trace!(
            ";StreamerBase.marshal.a{beg}.buf.len_after_hdr:{:?}",
            &w.p()[len..].len()
        );
        trace!(";StreamerBase.marshal.a{beg}.buf.pos_element:{:?}", w.pos());
        w.write_object(&self.element)?;
        w.write_i32(self.vbase)?;
        trace!(";StreamerBase.marshal.a{beg}.buf.value:{:?}", &w.p()[len..]);
        trace!(
            ";StreamerBase.marshal.a{beg}.buf.len:{:?}",
            &w.p()[len..].len()
        );

        w.set_header(hdr)
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct StreamerString {
    pub(crate) element: StreamerElement,
}

impl StreamerString {
    pub fn element(&self) -> &StreamerElement {
        &self.element
    }
}

impl Unmarshaler for StreamerString {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, rvers::STREAMER_STRING, self.class())?;

        r.read_object(&mut self.element)?;
        r.check_header(&hdr)?;

        Ok(())
    }
}

impl Marshaler for StreamerString {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len() - 1;
        let beg = w.pos();
        trace!(";StreamerString.marshal.a{}.beg:{}", beg, beg);
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        trace!(
            ";StreamerString.marshal.a{beg}.buf.len_after_hdr:{:?}",
            &w.p()[len..].len()
        );
        trace!(
            ";StreamerString.marshal.a{beg}.buf.pos_element:{:?}",
            w.pos()
        );
        w.write_object(&self.element)?;

        w.set_header(hdr)
    }
}

impl RVersioner for StreamerString {
    fn rversion(&self) -> i16 {
        rvers::STREAMER_STRING
    }
}

#[derive(Default, Debug, Clone)]
pub struct StreamerBasicType {
    pub(crate) element: StreamerElement,
}

impl Unmarshaler for StreamerBasicType {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, rvers::STREAMER_BASIC_TYPE, self.class())?;

        r.read_object(&mut self.element)?;

        let mut etype = self.element.etype.to_i32();

        if EnumNamed::OffsetL.to_i32()? < etype && etype < EnumNamed::OffsetP.to_i32()? {
            etype -= EnumNamed::OffsetL.to_i32()?;
        }

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

        // todo!();

        r.check_header(&hdr)?;

        Ok(())
    }
}

impl Marshaler for StreamerBasicType {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len() - 1;
        let beg = w.pos();
        trace!(";StreamerBasicType.marshal.a{}.beg:{}", beg, beg);
        trace!(";StreamerBasicType.marshal.a{}.class:{}", beg, self.class());
        trace!(
            ";StreamerBasicType.marshal.a{}.vers:{}",
            beg,
            Self::rversion(self)
        );
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        trace!(
            ";StreamerBasicType.marshal.a{beg}.buf.len_after_hdr:{:?}",
            &w.p()[len..].len()
        );
        trace!(
            ";StreamerBasicType.marshal.a{beg}.buf.pos_element:{:?}",
            w.pos()
        );
        w.write_object(&self.element)?;
        trace!(
            ";StreamerBasicType.marshal.a{beg}.buf.value:{:?}",
            &w.p()[len..]
        );
        trace!(
            ";StreamerBasicType.marshal.a{beg}.buf.len:{:?}",
            &w.p()[len..].len()
        );

        w.set_header(hdr)
    }
}

impl RVersioner for StreamerBasicType {
    fn rversion(&self) -> i16 {
        rvers::STREAMER_BASIC_TYPE
    }
}

#[derive(Default, Debug, Clone)]
pub struct StreamerObject {
    pub(crate) element: StreamerElement,
}

impl Unmarshaler for StreamerObject {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, rvers::STREAMER_OBJECT, self.class())?;

        r.read_object(&mut self.element)?;
        r.check_header(&hdr)?;
        Ok(())
    }
}

impl Marshaler for StreamerObject {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len() - 1;
        let beg = w.pos();
        trace!(";StreamerObject.marshal.a{}.beg:{}", beg, beg);
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        trace!(
            ";StreamerObject.marshal.a{beg}.buf.len_after_hdr:{:?}",
            &w.p()[len..].len()
        );
        trace!(
            ";StreamerObject.marshal.a{beg}.buf.pos_element:{:?}",
            w.pos()
        );
        w.write_object(&self.element)?;

        w.set_header(hdr)
    }
}

impl RVersioner for StreamerObject {
    fn rversion(&self) -> i16 {
        rvers::STREAMER_OBJECT
    }
}

#[derive(Default, Debug, Clone)]
pub struct StreamerObjectPointer {
    pub(crate) element: StreamerElement,
}

impl Unmarshaler for StreamerObjectPointer {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, rvers::STREAMER_OBJECT_POINTER, self.class())?;

        r.read_object(&mut self.element)?;
        r.check_header(&hdr)?;
        Ok(())
    }
}

impl Marshaler for StreamerObjectPointer {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len() - 1;
        let beg = w.pos();
        trace!(";StreamerObjectPointer.marshal.a{}.beg:{}", beg, beg);
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        trace!(
            ";StreamerObjectPointer.marshal.a{beg}.buf.len_after_hdr:{:?}",
            &w.p()[len..].len()
        );
        trace!(
            ";StreamerObjectPointer.marshal.a{beg}.buf.pos_element:{:?}",
            w.pos()
        );
        w.write_object(&self.element)?;

        w.set_header(hdr)
    }
}

impl RVersioner for StreamerObjectPointer {
    fn rversion(&self) -> i16 {
        rvers::STREAMER_OBJECT_POINTER
    }
}

#[derive(Default, Debug, Clone)]
pub struct StreamerObjectAny {
    pub(crate) element: StreamerElement,
}

impl Unmarshaler for StreamerObjectAny {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, rvers::STREAMER_OBJECT_ANY, self.class())?;

        r.read_object(&mut self.element)?;
        r.check_header(&hdr)?;
        Ok(())
    }
}

impl Marshaler for StreamerObjectAny {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len() - 1;
        let beg = w.pos();
        trace!(";StreamerObjectAny.marshal.a{}.beg:{}", beg, beg);
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        trace!(
            ";StreamerObjectAny.marshal.a{beg}.buf.len_after_hdr:{:?}",
            &w.p()[len..].len()
        );
        trace!(
            ";StreamerObjectAny.marshal.a{beg}.buf.pos_element:{:?}",
            w.pos()
        );
        w.write_object(&self.element)?;

        trace!(
            ";StreamerObjectAny.marshal.a{beg}.buf.value:{:?}",
            &w.p()[len..]
        );
        trace!(
            ";StreamerObjectAny.marshal.a{beg}.buf.len:{:?}",
            &w.p()[len..].len()
        );

        w.set_header(hdr)
    }
}

impl RVersioner for StreamerObjectAny {
    fn rversion(&self) -> i16 {
        rvers::STREAMER_OBJECT_ANY
    }
}

#[derive(Default, Debug, Clone)]
pub struct StreamerBasicPointer {
    pub(crate) element: StreamerElement,
    /// version number of the class with the counter
    pub(crate) cvers: i32,
    /// name of data member holding the array count
    pub(crate) cname: String,
    /// name of the class with the counter
    pub(crate) ccls: String,
}

impl Marshaler for StreamerBasicPointer {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len() - 1;
        let beg = w.pos();
        trace!(";StreamerBasicPointer.marshal.a{}.beg:{}", beg, beg);
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        trace!(
            ";StreamerBasicPointer.marshal.a{beg}.buf.len_after_hdr:{:?}",
            &w.p()[len..].len()
        );
        trace!(
            ";StreamerBasicPointer.marshal.a{beg}.buf.pos_element:{:?}",
            w.pos()
        );
        w.write_object(&self.element)?;
        w.write_i32(self.cvers)?;
        w.write_string(&self.cname)?;
        w.write_string(&self.ccls)?;
        trace!(
            ";StreamerBasicPointer.marshal.a{beg}.buf.value:{:?}",
            &w.p()[len..]
        );
        trace!(
            ";StreamerBasicPointer.marshal.a{beg}.buf.len:{:?}",
            &w.p()[len..].len()
        );

        w.set_header(hdr)
    }
}

impl RVersioner for StreamerBasicPointer {
    fn rversion(&self) -> i16 {
        rvers::STREAMER_BASIC_POINTER
    }
}

impl Unmarshaler for StreamerBasicPointer {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, rvers::STREAMER_BASIC_POINTER, self.class())?;

        r.read_object(&mut self.element)?;

        self.cvers = r.read_i32()?;
        self.cname = r.read_string()?.to_string();
        self.ccls = r.read_string()?.to_string();

        r.check_header(&hdr)?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
pub struct StreamerSTL {
    pub(crate) element: StreamerElement,
    /// type of STL vector
    pub(crate) vtype: rmeta::ESTLType,
    /// STL contained type
    pub(crate) ctype: rmeta::Enum,
}

impl Marshaler for StreamerSTL {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len() - 1;
        let beg = w.pos();
        trace!(";StreamerSTL.marshal.a{}.beg:{}", beg, beg);
        trace!(";StreamerSTL.marshal.a{}.vtype:{:?}", beg, self.vtype);

        trace!(
            ";StreamerSTL.marshal.a{}.vtype_i32:{:?}",
            beg,
            self.vtype.to_i32()
        );
        trace!(";StreamerSTL.marshal.a{}.ctype:{:?}", beg, self.ctype);
        trace!(
            ";StreamerSTL.marshal.a{}.ctype_i32:{:?}",
            beg,
            self.ctype.to_i32()
        );
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        trace!(
            ";StreamerSTL.marshal.a{beg}.buf.len_after_hdr:{:?}",
            &w.p()[len..].len()
        );
        trace!(";StreamerSTL.marshal.a{beg}.buf.pos_element:{:?}", w.pos());
        w.write_object(&self.element)?;
        w.write_i32(self.vtype.to_i32().ok_or(Misc(format!(
            "cant convert vtype ({:?}) to i32 ",
            self.vtype
        )))?)?;
        w.write_i32(self.ctype.to_i32())?;
        trace!(";StreamerSTL.marshal.a{beg}.buf.value:{:?}", &w.p()[len..]);
        trace!(
            ";StreamerSTL.marshal.a{beg}.buf.len:{:?}",
            &w.p()[len..].len()
        );

        w.set_header(hdr)
    }
}

impl RVersioner for StreamerSTL {
    fn rversion(&self) -> i16 {
        rvers::STREAMER_STL
    }
}

impl Unmarshaler for StreamerSTL {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, rvers::STREAMER_STL, self.class())?;

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

        r.check_header(&hdr)?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
pub struct StreamerSTLstring {
    pub(crate) streamer_stl: StreamerSTL,
}

impl Marshaler for StreamerSTLstring {
    fn marshal(&self, _w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        todo!()
    }
}

impl Unmarshaler for StreamerSTLstring {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, rvers::STREAMER_STLSTRING, self.class())?;

        r.read_object(&mut self.streamer_stl)?;
        r.check_header(&hdr)?;
        Ok(())
    }
}

factory_all_for_register_impl!(StreamerBase, "TStreamerBase");
factory_all_for_register_impl!(StreamerString, "TStreamerString");
factory_all_for_register_impl!(StreamerBasicType, "TStreamerBasicType");
factory_all_for_register_impl!(StreamerObject, "TStreamerObject");
factory_all_for_register_impl!(StreamerObjectPointer, "TStreamerObjectPointer");
factory_all_for_register_impl!(StreamerObjectAny, "TStreamerObjectAny");
factory_all_for_register_impl!(StreamerBasicPointer, "TStreamerBasicPointer");
factory_all_for_register_impl!(StreamerSTL, "TStreamerSTL");
factory_all_for_register_impl!(StreamerSTLstring, "TStreamerSTLstring");
