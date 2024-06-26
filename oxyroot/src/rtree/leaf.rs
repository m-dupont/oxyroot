use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{ensure_maximum_supported_version, MarshallerKind, RVersioner, Unmarshaler};
use crate::root::traits::Named;
use crate::root::traits::Object;
use crate::rtree::branch::TBranch;
use crate::rtypes::FactoryItemRead;
use crate::{factory_all_for_register_impl, rbase, Marshaler};
use crate::{factory_fn_register_impl, rvers};
use log::trace;
use std::any::type_name;

#[derive(Debug)]
pub enum Leaf {
    Base(TLeaf),
    Element(LeafElement),
    I(LeafI),
    S(LeafS),
    D(LeafD),
    F(LeafF),
    B(LeafB),
    L(LeafL),
    O(LeafO),
    C(LeafC),
}

impl Marshaler for Leaf {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        match &self {
            // Leaf::Base(_) => {}
            Leaf::Element(i) => i.marshal(w),
            Leaf::I(i) => i.marshal(w),
            Leaf::S(i) => i.marshal(w),
            Leaf::L(i) => i.marshal(w),
            Leaf::B(i) => i.marshal(w),
            Leaf::D(i) => i.marshal(w),
            Leaf::F(i) => i.marshal(w),
            // Leaf::B(_) => {}
            // Leaf::L(_) => {}
            Leaf::O(i) => i.marshal(w),
            Leaf::C(i) => i.marshal(w),
            _ => {
                todo!("Implement Leaf::marshal for {:?}", self)
            }
        }
    }
}

impl Object for Leaf {
    fn class(&self) -> &'_ str {
        match &self {
            Leaf::Base(_) => "TLeaf",
            Leaf::Element(_) => "TLeafElement",
            Leaf::I(_) => "TLeafI",
            Leaf::S(_) => "TLeafS",
            Leaf::D(_) => "TLeafD",
            Leaf::F(_) => "TLeafF",
            Leaf::B(_) => "TLeafB",
            Leaf::L(_) => "TLeafL",
            Leaf::O(_) => "TLeafO",
            Leaf::C(_) => "TLeafC",
        }
    }
}

impl Named for Leaf {}

impl RVersioner for Leaf {
    fn rversion(&self) -> i16 {
        todo!()
    }
}

impl Leaf {
    pub(crate) fn new<T: 'static + Marshaler>(b: &TBranch) -> Self {
        let tys = type_name::<T>();

        trace!(";Leaf.new.typename:{:?}", tys);

        let tleaf = TLeaf::default()
            .with_etype(std::mem::size_of::<T>() as i32)
            .with_name(b.named.name.clone())
            .with_title(b.named.name.clone())
            .with_len(1);

        fn make_leaf(tleaf: TLeaf, tys: &str) -> Leaf {
            match tys {
                "i32" => Leaf::I(LeafI::new(tleaf)),
                "u32" => Leaf::I(LeafI::new(tleaf.with_unsigned(true))),
                "i16" => Leaf::S(LeafS::new(tleaf)),
                "u16" => Leaf::S(LeafS::new(tleaf.with_unsigned(true))),
                "i8" => Leaf::B(LeafB::new(tleaf)),
                "u8" => Leaf::B(LeafB::new(tleaf.with_unsigned(true))),
                "i64" => Leaf::L(LeafL::new(tleaf)),
                "u64" => Leaf::L(LeafL::new(tleaf.with_unsigned(true))),
                "f32" => Leaf::F(LeafF::new(tleaf)),
                "f64" => Leaf::D(LeafD::new(tleaf)),
                "bool" => Leaf::O(LeafO::new(tleaf)),
                "String" => Leaf::C(LeafC::new(tleaf)),
                _ => unimplemented!("ty = {}", tys),
            }
        }

        let leaf = match T::kind() {
            MarshallerKind::Primitive => make_leaf(tleaf, tys),
            MarshallerKind::Array { shape, tys: t } => {
                let mut nelems = 1;
                for s in &shape {
                    nelems *= s;
                }
                let etype = std::mem::size_of::<T>() as i32 / nelems;
                trace!(";Leaf.new.nelems:{:?}", nelems);
                let tleaf = tleaf
                    .with_len(nelems)
                    .with_etype(etype)
                    .with_title(format!("{}[{}]", &b.named.name, nelems));
                make_leaf(tleaf, &t)
            }
            MarshallerKind::Slice { .. } => {
                let tleaf = tleaf.with_etype(0);
                Leaf::Element(LeafElement::new(tleaf))
            }
            MarshallerKind::String => make_leaf(tleaf, "String"),
            MarshallerKind::Struct => {
                unimplemented!("ty = {}", tys)
            }
        };

        trace!(";Leaf.new.leaf:{:?}", leaf);

        leaf
    }

    fn tleaf(&self) -> &TLeaf {
        let l: &TLeaf = self.into();
        l
    }

    pub fn etype(&self) -> i32 {
        self.tleaf().etype
    }

    pub fn title(&self) -> &str {
        self.tleaf().title()
    }

    pub fn unsigned(&self) -> Option<bool> {
        match self {
            Leaf::Base(_) => None,
            Leaf::Element(_) => None,
            _ => Some(self.tleaf().unsigned),
        }
    }

    pub fn type_name(&self) -> Option<&str> {
        match self {
            Leaf::Base(_) => None,
            Leaf::Element(_) => None,

            Leaf::I(_) => Some(match self.unsigned().unwrap() {
                true => "uint32_t",
                false => "int32_t",
            }),
            Leaf::S(_) => Some(match self.unsigned().unwrap() {
                true => "uint16_t",
                false => "int16_t",
            }),
            Leaf::D(_) => Some("double"),
            Leaf::F(_) => Some("float"),
            Leaf::B(_) => Some(match self.unsigned().unwrap() {
                true => "uint8_t",
                false => "int8_t",
            }),
            Leaf::L(_) => Some(match self.unsigned().unwrap() {
                true => "uint64_t",
                false => "int64_t",
            }),
            Leaf::O(_) => Some("bool"),
            Leaf::C(_) => Some("char*"),
        }
    }

    pub(crate) fn write_to_buffer(
        &mut self,
        w: &mut WBuffer,
        value: &impl Marshaler,
    ) -> crate::rbytes::Result<i64> {
        match self {
            Leaf::Base(_) => {
                todo!()
            }
            Leaf::Element(l) => l.write_to_buffer(w, value),
            Leaf::I(_) => w.write_object(value),
            Leaf::S(_) => w.write_object(value),
            Leaf::D(_) => w.write_object(value),
            Leaf::F(_) => w.write_object(value),
            Leaf::B(_) => w.write_object(value),
            Leaf::L(_) => w.write_object(value),
            Leaf::O(_) => w.write_object(value),
            Leaf::C(l) => l.write_to_buffer(w, value),
        }
    }
}

impl<'a> From<&'a Leaf> for &'a TLeaf {
    fn from(l: &'a Leaf) -> Self {
        match l {
            Leaf::Base(ll) => ll,
            Leaf::Element(le) => &le.tleaf,
            Leaf::I(li) => &li.tleaf,
            Leaf::S(li) => &li.tleaf,
            Leaf::D(li) => &li.tleaf,
            Leaf::F(li) => &li.tleaf,
            Leaf::B(li) => &li.tleaf,
            Leaf::L(li) => &li.tleaf,
            Leaf::O(li) => &li.tleaf,
            Leaf::C(li) => &li.tleaf,
        }
    }
}

impl From<Leaf> for TLeaf {
    fn from(l: Leaf) -> Self {
        match l {
            Leaf::Base(ll) => ll,
            Leaf::Element(le) => le.tleaf,
            Leaf::I(li) => li.tleaf,
            Leaf::S(li) => li.tleaf,
            Leaf::D(li) => li.tleaf,
            Leaf::F(li) => li.tleaf,
            Leaf::B(li) => li.tleaf,
            Leaf::L(li) => li.tleaf,
            Leaf::O(li) => li.tleaf,
            Leaf::C(li) => li.tleaf,
        }
    }
}

impl From<Box<dyn FactoryItemRead>> for Leaf {
    fn from(obj: Box<dyn FactoryItemRead>) -> Self {
        match obj.class() {
            "TLeaf" => Leaf::Base(*obj.downcast::<TLeaf>().unwrap()),
            "TLeafI" => Leaf::I(*obj.downcast::<LeafI>().unwrap()),
            "TLeafS" => Leaf::S(*obj.downcast::<LeafS>().unwrap()),
            "TLeafF" => Leaf::F(*obj.downcast::<LeafF>().unwrap()),
            "TLeafD" => Leaf::D(*obj.downcast::<LeafD>().unwrap()),
            "TLeafB" => Leaf::B(*obj.downcast::<LeafB>().unwrap()),
            "TLeafL" => Leaf::L(*obj.downcast::<LeafL>().unwrap()),
            "TLeafO" => Leaf::O(*obj.downcast::<LeafO>().unwrap()),
            "TLeafC" => Leaf::C(*obj.downcast::<LeafC>().unwrap()),
            "TLeafElement" => Leaf::Element(*obj.downcast::<LeafElement>().unwrap()),
            &_ => todo!("Implement {}", obj.class()),
        }
    }
}
#[derive(Debug)]
struct LeafCount {}

#[derive(Default, Debug)]
pub struct TLeaf {
    named: rbase::Named,
    shape: Vec<i32>,

    len: i32,
    // number of fixed length elements in the leaf's data.
    etype: i32,
    // number of bytes for this data type
    offset: i32,
    // offset in CLONES_ARRAY object
    hasrange: bool,
    // whether the leaf has a range
    unsigned: bool, // whether the leaf holds unsigned data (uint8, uint16, uint32 or uint64)
    // leaf count if variable length
    count: Option<LeafCount>,
    // branch   BRANCH    // supporting branch of this leaf
}

impl TLeaf {
    pub(crate) fn with_name(mut self, name: String) -> Self {
        self.named.name = name;
        self
    }

    pub(crate) fn with_title(mut self, title: String) -> Self {
        self.named.title = title;
        self
    }

    pub(crate) fn with_len(mut self, len: i32) -> Self {
        self.len = len;
        self
    }

    pub(crate) fn with_etype(mut self, etype: i32) -> Self {
        self.etype = etype;
        self
    }

    pub(crate) fn with_unsigned(mut self, unsigned: bool) -> Self {
        self.unsigned = unsigned;
        self
    }
}

impl Named for TLeaf {
    fn title(&self) -> &'_ str {
        self.named.title()
    }
}

pub fn leaf_dim(_s: &str) -> Option<Vec<i32>> {
    None
}

impl Unmarshaler for TLeaf {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, crate::rvers::Leaf, self.class())?;

        r.read_object(&mut self.named)?;

        self.shape = match leaf_dim(self.title()) {
            None => Vec::new(),
            Some(v) => v,
        };

        self.len = r.read_i32()?;
        self.etype = r.read_i32()?;
        self.offset = r.read_i32()?;
        self.hasrange = r.read_bool()?;
        self.unsigned = r.read_bool()?;

        let ptr = r.read_object_any_into()?;

        match ptr {
            None => {}
            Some(p) => {
                if p.class() == "TLeafElement" {
                    let _p = p.downcast::<LeafElement>().unwrap();

                    //  todo!("");

                    self.count = Some(LeafCount {});
                } else {
                    todo!("can not read ptr:{}", p.class())
                }
            }
        };

        if self.len == 0 {
            self.len = 1;
        }

        trace!(";TLeaf.unmarshal.buf.pos:{:?}", r.pos());
        trace!(";TLeaf.unmarshal.etype:{:?}", self.etype);
        trace!(";TLeaf.unmarshal.offset:{:?}", self.offset);
        trace!(";TLeaf.unmarshal.len:{:?}", self.len);
        trace!(";TLeaf.unmarshal.count:{:?}", self.count);
        trace!(";TLeaf.unmarshal.name:{:?}", self.named.name());
        trace!(";TLeaf.unmarshal.title:{:?}", self.named.title());
        trace!(";TLeaf.unmarshal.shape:{:?}", self.shape);
        // trace!(";TLeaf.marshal.buf.value:{:?}", &w.p()[len..]);

        r.check_header(&hdr)?;

        Ok(())
    }
}

impl Marshaler for TLeaf {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len() - 1;
        trace!(";TLeaf.marshal.buf.pos:{:?}", w.pos());
        trace!(";TLeaf.marshal.etype:{:?}", self.etype);
        trace!(";TLeaf.marshal.offset:{:?}", self.offset);
        trace!(";TLeaf.marshal.len:{:?}", self.len);
        trace!(";TLeaf.marshal.count:{:?}", self.count);
        trace!(";TLeaf.marshal.name:{:?}", self.named.name());
        trace!(";TLeaf.marshal.title:{:?}", self.named.title());
        trace!(";TLeaf.marshal.buf.value:{:?}", &w.p()[len..]);
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        w.write_object(&self.named)?;
        w.write_i32(self.len)?;
        w.write_i32(self.etype)?;
        w.write_i32(self.offset)?;
        w.write_bool(self.hasrange)?;
        w.write_bool(self.unsigned)?;
        match &self.count {
            None => {
                w.write_object_nil()?;
            }
            Some(c) => {
                todo!(";TLeaf.marshal.count:{:?}", c);
            }
        }
        trace!(";TLeaf.marshal.buf.value:{:?}", &w.p()[len..]);
        w.set_header(hdr)
    }
}

impl RVersioner for TLeaf {
    fn rversion(&self) -> i16 {
        rvers::Leaf
    }
}

factory_fn_register_impl!(TLeaf, "TLeaf");

#[derive(Default, Debug)]
pub struct LeafC {
    rvers: i16,
    tleaf: TLeaf,
    min: i32,
    max: i32,
    // ptr: &i32;
}

impl LeafC {
    pub fn new(tleaf: TLeaf) -> Self {
        Self {
            tleaf,
            rvers: rvers::LEAF_C,
            ..Default::default()
        }
    }

    pub(crate) fn write_to_buffer(
        &mut self,
        w: &mut WBuffer,
        value: &impl Marshaler,
    ) -> crate::rbytes::Result<i64> {
        // TODO: implement LeafC::write_to_buffer from groot  ? needed ?
        let sz = w.write_object(value)?;
        if sz >= self.max as i64 {
            self.max = sz as i32;
        }
        if sz >= self.tleaf.len as i64 {
            self.tleaf.len = sz as i32;
        }
        trace!(";TLeafC.write_to_buffer.sz:{:?}", sz);
        Ok(sz)
    }
}

impl Unmarshaler for LeafC {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, crate::rvers::LEAF_C, self.class())?;

        self.rvers = hdr.vers;

        r.read_object(&mut self.tleaf)?;

        r.read_object(&mut self.min)?;
        r.read_object(&mut self.max)?;

        r.check_header(&hdr)?;

        Ok(())

        // todo!()
    }
}

impl Marshaler for LeafC {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let _len = w.len() - 1;
        trace!(";T{}.marshal.buf.pos:{:?}", "LeafC", w.pos());
        trace!(";T{}.marshal.min:{:?}", "LeafC", self.min);
        trace!(";T{}.marshal.max:{:?}", "LeafC", self.max);
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        w.write_object(&self.tleaf)?;
        w.write_object(&self.min)?;
        w.write_object(&self.max)?;
        w.set_header(hdr)
    }
}

impl RVersioner for LeafC {
    fn rversion(&self) -> i16 {
        rvers::LEAF_C
    }
}

factory_all_for_register_impl!(LeafC, "TLeafC");

macro_rules! make_tleaf_variant {
    ($struct_name:ident, $root_name:literal, $field_type:ty) => {
        #[derive(Default, Debug)]
        pub struct $struct_name {
            rvers: i16,
            tleaf: TLeaf,
            min: $field_type,
            max: $field_type,
            // ptr: &i32;
        }

        impl $struct_name {
            pub fn new(tleaf: TLeaf) -> Self {
                // let tleaf = TLeaf::default()
                //     .with_etype(std::mem::size_of::<$field_type>() as i32)
                //     .with_name(name);
                Self {
                    tleaf,
                    rvers: rvers::$struct_name,
                    ..Default::default()
                }
            }
        }

        factory_all_for_register_impl!($struct_name, $root_name);

        impl RVersioner for $struct_name {
            fn rversion(&self) -> i16 {
                rvers::$struct_name
            }
        }

        // pub(crate) fn write_to_buffer(
        //     &mut self,
        //     w: &mut WBuffer,
        //     value: &impl Marshaler,
        // ) -> crate::rbytes::Result<i64> {
        //     let sz = w.write_object(value)?;
        //     if sz >= self.max as i64 {
        //         self.max = sz as i32;
        //     }
        //     if sz >= self.tleaf.len as i64 {
        //         self.tleaf.len = sz as i32;
        //     }
        //     trace!(";$struct_name.write_to_buffer.sz:{:?}", sz);
        //     Ok(sz)
        // }

        impl Unmarshaler for $struct_name {
            fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
                trace!(";{}.unmarshal.buf.pos:{:?}", $root_name, r.pos());
                let hdr = r.read_header(self.class())?;

                ensure_maximum_supported_version(hdr.vers, rvers::$struct_name, self.class())?;

                self.rvers = hdr.vers;

                r.read_object(&mut self.tleaf)?;

                r.read_object(&mut self.min)?;
                r.read_object(&mut self.max)?;

                r.check_header(&hdr)?;

                Ok(())

                // todo!()
            }
        }

        impl Marshaler for $struct_name {
            fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
                let _len = w.len() - 1;
                trace!(";{}.marshal.buf.pos:{:?}", $root_name, w.pos());
                trace!(";{}.marshal.min:{:?}", $root_name, self.min);
                trace!(";{}.marshal.max:{:?}", $root_name, self.max);
                let hdr = w.write_header(self.class(), Self::rversion(self))?;
                w.write_object(&self.tleaf)?;
                w.write_object(&self.min)?;
                w.write_object(&self.max)?;
                w.set_header(hdr)
            }
        }
    };
}

make_tleaf_variant!(LeafI, "TLeafI", i32);
make_tleaf_variant!(LeafB, "TLeafB", i8);
make_tleaf_variant!(LeafS, "TLeafS", i16);
make_tleaf_variant!(LeafL, "TLeafL", i64);
make_tleaf_variant!(LeafF, "TLeafF", f32);
make_tleaf_variant!(LeafD, "TLeafD", f64);
make_tleaf_variant!(LeafO, "TLeafO", bool);

/// LeafElement is a Leaf for a general object derived from OBJECT.
#[derive(Default, Debug)]
pub struct LeafElement {
    rvers: i16,
    tleaf: TLeaf,

    /// element serial number in fInfo
    id: i32,
    /// leaf type
    pub(crate) ltype: i32,
    // ptr: &i32;
}

impl LeafElement {
    pub fn new(tleaf: TLeaf) -> Self {
        Self {
            tleaf,
            rvers: rvers::LeafElement,
            id: -1,
            ltype: -1,
        }
    }

    pub(crate) fn write_to_buffer(
        &mut self,
        w: &mut WBuffer,
        value: &impl Marshaler,
    ) -> crate::rbytes::Result<i64> {
        let _beg = w.pos();

        let hdr = w.write_header(self.class(), rvers::STREAMER_INFO)?;
        // w.write_array_u8(&a)?;
        w.write_object(value)?;
        w.set_header(hdr)
    }
}

impl Unmarshaler for LeafElement {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;
        trace!(";LeafElement.unmarshal.buf.pos:{:?}", r.pos());

        ensure_maximum_supported_version(hdr.vers, crate::rvers::LeafElement, self.class())?;

        self.rvers = hdr.vers;

        r.read_object(&mut self.tleaf)?;

        r.read_object(&mut self.id)?;
        r.read_object(&mut self.ltype)?;

        trace!(";LeafElement.unmarshal.self.id:{:?}", self.id);
        trace!(";LeafElement.unmarshal.self.ltype:{:?}", self.ltype);
        r.check_header(&hdr)?;

        Ok(())

        // todo!()
    }
}

impl Marshaler for LeafElement {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let _len = w.len() - 1;
        trace!(";LeafElement.marshal.buf.pos:{:?}", w.pos());
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        w.write_object(&self.tleaf)?;
        w.write_i32(self.id)?;
        w.write_i32(self.ltype)?;
        w.set_header(hdr)
    }
}

impl RVersioner for LeafElement {
    fn rversion(&self) -> i16 {
        rvers::LeafElement
    }
}

factory_all_for_register_impl!(LeafElement, "TLeafElement");
