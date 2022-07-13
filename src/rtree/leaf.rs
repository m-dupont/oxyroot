use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::root::traits::Named;
use crate::root::traits::Object;
use crate::rtypes::FactoryItem;
use crate::{factory_all_for_register_impl, rbase};
use crate::{factory_fn_register_impl, rvers};
use anyhow::ensure;

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

impl Leaf {
    pub fn etype(&self) -> i32 {
        let l: &TLeaf = self.into();
        l.etype
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

impl From<Box<dyn FactoryItem>> for Leaf {
    fn from(obj: Box<dyn FactoryItem>) -> Self {
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
                    // count    leafCount // leaf count if variable length
                    // branch   BRANCH    // supporting branch of this leaf
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
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::Leaf,
            "rtree: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::Leaf
        );

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
            Some(_) => {
                todo!()
            }
        };

        if self.len == 0 {
            self.len = 1;
        }

        r.check_header(&hdr)?;

        Ok(())
    }
}

factory_fn_register_impl!(TLeaf, "TLeaf");

#[derive(Default, Debug)]
pub struct LeafI {
    rvers: i16,
    tleaf: TLeaf,
    min: i32,
    max: i32,
    // ptr: &i32;
}

impl Unmarshaler for LeafI {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::LEAF_I,
            "rtree: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::LEAF_I
        );

        self.rvers = hdr.vers;

        r.read_object(&mut self.tleaf)?;

        r.read_object(&mut self.min)?;
        r.read_object(&mut self.max)?;

        r.check_header(&hdr)?;

        Ok(())

        // todo!()
    }
}

factory_all_for_register_impl!(LeafI, "TLeafI");

#[derive(Default, Debug)]
pub struct LeafC {
    rvers: i16,
    tleaf: TLeaf,
    min: i32,
    max: i32,
    // ptr: &i32;
}

impl Unmarshaler for LeafC {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::LEAF_C,
            "rtree: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::LEAF_C
        );

        self.rvers = hdr.vers;

        r.read_object(&mut self.tleaf)?;

        r.read_object(&mut self.min)?;
        r.read_object(&mut self.max)?;

        r.check_header(&hdr)?;

        Ok(())

        // todo!()
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

        factory_all_for_register_impl!($struct_name, $root_name);

        impl Unmarshaler for $struct_name {
            fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
                let hdr = r.read_header(self.class())?;
                ensure!(
                    hdr.vers <= rvers::$struct_name,
                    "rtree: invalid {} version={} > {}",
                    self.class(),
                    hdr.vers,
                    rvers::$struct_name
                );

                self.rvers = hdr.vers;

                r.read_object(&mut self.tleaf)?;

                r.read_object(&mut self.min)?;
                r.read_object(&mut self.max)?;

                r.check_header(&hdr)?;

                Ok(())

                // todo!()
            }
        }
    };
}

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
    ltype: i32,
    // ptr: &i32;
}

impl Unmarshaler for LeafElement {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::LeafElement,
            "rtree: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::LeafElement
        );

        self.rvers = hdr.vers;

        r.read_object(&mut self.tleaf)?;

        r.read_object(&mut self.id)?;
        r.read_object(&mut self.ltype)?;

        r.check_header(&hdr)?;

        Ok(())

        // todo!()
    }
}

factory_all_for_register_impl!(LeafElement, "TLeafElement");
