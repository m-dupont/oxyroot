use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::rcont::objarray::ObjArray;
use crate::root::traits::Object;
use crate::rtree::tree::TioFeatures;
use crate::rtypes::FactoryItem;
use crate::rvers;
use crate::{factotry_all_for_register_impl, rbase};
use anyhow::ensure;
use log::trace;

#[derive(Default)]
pub struct TBranch {
    named: rbase::Named,
    attfill: rbase::AttFill,

    /// compression level and algorithm
    compress: i32,
    /// initial size of Basket buffer
    basketSize: i32,
    entryOffsetLen: i32,
    // initial length of entryOffset table in the basket buffers
    writeBasket: i32,
    // last basket number written
    entryNumber: i64,
    // current entry number (last one filled in this branch)
    iobits: TioFeatures,
    // IO features for newly-created baskets
    offset: i32,
    // offset of this branch
    maxBaskets: i32,
    // maximum number of baskets so far
    splitLevel: i32,
    // branch split level
    entries: i64,
    // number of entries
    firstEntry: i64,
    // number of the first entry in this branch
    totBytes: i64,
    // total number of bytes in all leaves before compression
    zipBytes: i64, // total number of bytes in all leaves after compression

    branches: Vec<Box<dyn FactoryItem>>,
    leaves: Vec<Box<dyn FactoryItem>>,
    baskets: Vec<Box<dyn FactoryItem>>,

    /// length of baskets on file
    basketBytes: Vec<i32>,
    /// table of first entry in each basket
    basketEntry: Vec<i64>,
    /// addresses of baskets on file
    basketSeek: Vec<i64>,
    /// named of file where buffers are stored (empty if in same file as Tree header)
    fname: String,
}

impl Unmarshaler for TBranch {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("TBranch:unmarshal");
        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::Branch,
            "rtree: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::Branch
        );

        if hdr.vers >= 10 {
            r.read_object(&mut self.named)?;
            r.read_object(&mut self.attfill)?;
            self.compress = r.read_i32()?;
            self.basketSize = r.read_i32()?;
            self.entryOffsetLen = r.read_i32()?;
            self.writeBasket = r.read_i32()?;
            self.entryNumber = r.read_i64()?;

            if hdr.vers >= 13 {
                r.read_object(&mut self.iobits)?;
            }

            self.offset = r.read_i32()?;
            self.maxBaskets = r.read_i32()?;
            self.splitLevel = r.read_i32()?;
            self.entries = r.read_i64()?;

            if hdr.vers >= 11 {
                self.firstEntry = r.read_i64()?;
            }

            self.totBytes = r.read_i64()?;
            self.zipBytes = r.read_i64()?;

            {
                let branches = r.read_object_into::<ObjArray>()?;
                trace!("branches = {:?}", branches);
                if !branches.objs.is_empty() {
                    todo!()
                }
            }

            {
                let mut leaves = r.read_object_into::<ObjArray>()?;
                trace!("leaves = {:?}", leaves);
                if !leaves.objs.is_empty() {
                    self.leaves = leaves.take_objs();
                }
            }

            {
                let mut baskets = r.read_object_into::<ObjArray>()?;
                trace!("leaves = {:?}", baskets);
                if !baskets.objs.is_empty() {
                    self.baskets = baskets.take_objs();
                }

                trace!("self.baskets = {:?}", self.baskets);
            }

            {
                let _ = r.read_i8()?;
                let mut b = vec![0; self.maxBaskets as usize];
                r.read_array_i32(b.as_mut_slice())?;
                trace!("b = {:?}", b);

                self.basketBytes
                    .extend_from_slice(&b.as_slice()[..self.writeBasket as usize]);
            }

            {
                let _ = r.read_i8()?;
                let mut b = vec![0 as i64; self.maxBaskets as usize];
                r.read_array_i64(b.as_mut_slice())?;
                trace!("b = {:?}", b);

                self.basketEntry
                    .extend_from_slice(&b.as_slice()[..(self.writeBasket + 1) as usize]);
            }

            {
                let _ = r.read_i8()?;
                let mut b = vec![0 as i64; self.maxBaskets as usize];
                r.read_array_i64(b.as_mut_slice())?;
                trace!("b = {:?}", b);

                self.basketSeek
                    .extend_from_slice(&b.as_slice()[..self.writeBasket as usize]);
            }

            trace!("self.basketBytes = {:?}", self.basketBytes);
            trace!("self.basketEntry = {:?}", self.basketEntry);
            trace!("self.basketSeek = {:?}", self.basketSeek);

            self.fname = r.read_string()?.to_string();
        } else if hdr.vers >= 6 {
            todo!();
            // r.read_object(&mut self.named)?;
            // if hdr.vers > 7 {
            //     r.read_object(&mut self.attfill)?;
            // }
        } else {
            unimplemented!()
        }

        if self.splitLevel == 0 && self.branches.len() > 0 {
            self.splitLevel = 1;
        }

        r.check_header(&hdr);

        Ok(())

        // todo!()
    }
}

factotry_all_for_register_impl!(TBranch, "TBranch");
