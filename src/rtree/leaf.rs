use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::root::traits::Named;
use crate::root::traits::Object;
use crate::{factotry_all_for_register_impl, rbase, root};
use crate::{factotry_fn_register_impl, rvers};
use anyhow::ensure;
use log::trace;
use regex::Regex;

#[derive(Default)]
pub struct TLeaf {
    named: rbase::Named,
    shape: Vec<i32>,

    len: i32,
    // number of fixed length elements in the leaf's data.
    etype: i32,
    // number of bytes for this data type
    offset: i32,
    // offset in ClonesArray object
    hasrange: bool,
    // whether the leaf has a range
    unsigned: bool, // whether the leaf holds unsigned data (uint8, uint16, uint32 or uint64)
                    // count    leafCount // leaf count if variable length
                    // branch   Branch    // supporting branch of this leaf
}

impl Named for TLeaf {
    fn title(&self) -> &'_ str {
        &self.named.title()
    }
}

pub fn leaf_dim(s: &str) -> Option<Vec<i32>> {
    return None;

    // let re = Regex::new(r"\w*?\[(\d*)\]+?").unwrap();
    //
    // if re.captures_iter(s).collect() == 0 {
    //     None
    // }
    //
    // todo!()
}

impl Unmarshaler for TLeaf {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("TLeaf:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::Leaf,
            "rtree: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::Leaf
        );

        r.read_object(&mut self.named)?;
        trace!("title = {}", self.title());

        self.shape = match leaf_dim(self.title()) {
            None => Vec::new(),
            Some(v) => v,
        };

        trace!("shape = {:?}", self.shape);
        self.len = r.read_i32()?;
        self.etype = r.read_i32()?;
        self.offset = r.read_i32()?;
        self.hasrange = r.read_bool()?;
        self.unsigned = r.read_bool()?;

        let ptr = r.read_object_any_into()?;

        match ptr {
            None => {}
            Some(o) => {
                todo!()
            }
        };

        if self.len == 0 {
            self.len = 1;
        }

        r.check_header(&hdr);

        Ok(())
    }
}

factotry_fn_register_impl!(TLeaf, "TLeaf");

#[derive(Default)]
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
            hdr.vers <= rvers::LeafI,
            "rtree: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::LeafI
        );

        self.rvers = hdr.vers;

        r.read_object(&mut self.tleaf)?;

        r.read_object(&mut self.min)?;
        r.read_object(&mut self.max)?;

        r.check_header(&hdr);

        Ok(())

        // todo!()
    }
}

factotry_all_for_register_impl!(LeafI, "TLeafI");
