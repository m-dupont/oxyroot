use crate::rcont::objarray::ObjArray;
use crate::riofs::file::{RootFileReader, RootFileStreamerInfoContext};
use crate::root::traits::Named;
use crate::root::traits::Object;
use crate::rtree::basket::{Basket, BasketData};
use crate::rtree::branch::tbranch_props::TBranchProps;
use crate::rtree::branch::BranchChunks;
use crate::rtree::leaf::Leaf;
use crate::rtree::tree::TioFeatures;
use crate::rtypes::FactoryItem;
use crate::{factory_fn_register_impl, rbase, Branch, RBuffer, Unmarshaler};
use anyhow::ensure;
use itertools::izip;
use log::trace;

#[derive(Default)]
pub struct TBranch {
    named: rbase::Named,
    attfill: rbase::AttFill,

    /// compression level and algorithm
    compress: i32,
    /// initial size of BASKET buffer
    basket_size: i32,
    /// initial length of entryOffset table in the basket buffers
    entry_offset_len: i32,
    /// last basket number written
    write_basket: i32,
    /// current entry number (last one filled in this branch)
    entry_number: i64,
    /// IO features for newly-created baskets
    iobits: TioFeatures,
    /// offset of this branch
    offset: i32,
    /// maximum number of baskets so far
    max_baskets: i32,
    /// branch split level
    split_level: i32,
    /// number of entries
    entries: i64,
    /// number of the first entry in this branch
    first_entry: i64,
    /// total number of bytes in all leaves before compression
    tot_bytes: i64,
    /// total number of bytes in all leaves after compression
    zip_bytes: i64,

    branches: Vec<Branch>,
    pub(crate) leaves: Vec<Leaf>,
    pub(crate) baskets: Vec<Box<dyn FactoryItem>>,

    /// length of baskets on file
    pub(crate) basket_bytes: Vec<i32>,
    /// table of first entry in each basket
    pub(crate) basket_entry: Vec<i64>,
    /// addresses of baskets on file
    pub(crate) basket_seek: Vec<i64>,
    /// named of file where buffers are stored (empty if in same file as TREE header)
    fname: String,

    reader: Option<RootFileReader>,
    pub(crate) sinfos: Option<RootFileStreamerInfoContext>,

    pub(crate) props: TBranchProps,
}

impl From<Branch> for TBranch {
    fn from(b: Branch) -> Self {
        match b {
            Branch::Base(bb) => bb,
            Branch::Element(be) => be.branch,
        }
    }
}

impl<'a> From<&'a Branch> for &'a TBranch {
    fn from(b: &'a Branch) -> Self {
        match b {
            Branch::Base(bb) => bb,
            Branch::Element(be) => &be.branch,
        }
    }
}

impl TBranch {
    // pub fn branches(&self) -> impl Iterator<Item = &Branch> {
    //     self.branches.iter() //.map(|b| b.into())
    // }

    pub fn branches(&self) -> &Vec<Branch> {
        &self.branches //.map(|b| b.into())
    }

    pub fn branch(&self, name: &str) -> Option<&Branch> {
        for b in self.branches.iter() {
            if b.name() == name {
                return Some(b);
            }

            if let Some(bb) = b.branch(name) {
                return Some(bb);
            }
        }

        None
    }

    pub(crate) fn set_reader(&mut self, reader: Option<RootFileReader>) {
        for branch in self.branches.iter_mut() {
            branch.set_reader(Some(reader.as_ref().unwrap().clone()));
        }

        self.reader = reader;
    }

    pub(crate) fn set_streamer_info(&mut self, sinfos: RootFileStreamerInfoContext) {
        for branch in self.branches.iter_mut() {
            branch.set_streamer_info(sinfos.clone());
        }

        self.sinfos = Some(sinfos);
    }

    pub(crate) fn get_baskets_buffer(&self) -> Box<dyn Iterator<Item = BranchChunks> + '_> {
        trace!("We are in branch = {}", self.name());
        let mut size_leaves = self.leaves.iter().map(|e| e.etype()).collect::<Vec<_>>();

        trace!("leaves = {:?}", self.leaves.len());

        trace!(
            "get_baskets_buffer: (start = {:?}, len = {:?}, chunk_size = {:?})",
            &self.basket_seek,
            &self.basket_bytes,
            size_leaves
        );

        if size_leaves.len() != self.basket_seek.len() {
            for _i in 1..self.basket_seek.len() {
                size_leaves.push(size_leaves[0]);
            }
        }

        Box::new(
            izip!(
                &self.basket_seek,
                &self.basket_bytes,
                size_leaves,
                &self.leaves
            )
                .map(|(start, len, mut chunk_size, leave)| {
                    trace!(
                    "get_baskets_buffer: (start = {start}, len = {len} (-> {}), chunk_size = {}, leave = {:?})",
                    *start as i64 + *len as i64,
                    chunk_size, leave
                );
                    let mut reader = self.reader.as_ref().unwrap().clone();
                    let buf = reader.read_at(*start as u64, *len as u64).unwrap();
                    let mut r = RBuffer::new(&buf, 0);
                    let b = r.read_object_into::<Basket>().unwrap();


                    trace!("chunk_size = {}, b.entry_size() = {}", chunk_size, b.entry_size());

                    match leave {
                        // In case of string, we have to use n
                        Leaf::C(_) | Leaf::Element(_) => {
                            chunk_size = b.entry_size();
                        },
                        _ => {}
                    }


                    match b.raw_data(&mut reader) {
                        BasketData::TrustNEntries((n, buf)) => {
                            trace!("send ({n},{chunk_size},{:?})", buf);
                            return BranchChunks::RegularSized((n, chunk_size, buf));
                        }
                        BasketData::UnTrustNEntries((n, buf, _byte_offsets)) => match leave {
                            Leaf::C(_) => {
                                // In case of string, we have to use n
                                trace!("send ({n},{chunk_size},{:?})", buf);
                                return BranchChunks::RegularSized((n, chunk_size, buf));
                            },
                            Leaf::Element(_) => {
                                panic!("I dont want to be here (Element should be in TBranchElement)");
                            },
                            _ => {
                                let n = buf.len() / chunk_size as usize;
                                trace!("send ({n},{chunk_size},{:?})", buf);
                                return BranchChunks::RegularSized((n as u32, chunk_size, buf));
                            }
                        },
                    };
                }),
        )
    }

    pub fn entries(&self) -> i64 {
        self.entries
    }

    pub fn item_type_name(&self) -> String {
        let unknown = "unknown";

        trace!("len = {} leaves = {:?}", self.leaves.len(), self.leaves);

        if self.leaves.len() == 1 {
            let leave = self.leaves.get(0).unwrap();
            return match leave.type_name() {
                Some(t) => t.to_string(),
                None => panic!("can not be here"),
            };
        }

        return unknown.to_string();
    }
    pub(crate) fn reader(&self) -> &Option<RootFileReader> {
        &self.reader
    }
}

impl Named for TBranch {
    fn name(&self) -> &'_ str {
        self.named.name()
    }
}

impl Unmarshaler for TBranch {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= crate::rvers::BRANCH,
            "rtree: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            crate::rvers::BRANCH
        );

        if hdr.vers >= 10 {
            r.read_object(&mut self.named)?;
            r.read_object(&mut self.attfill)?;
            self.compress = r.read_i32()?;
            self.basket_size = r.read_i32()?;
            self.entry_offset_len = r.read_i32()?;
            self.write_basket = r.read_i32()?;
            self.entry_number = r.read_i64()?;

            if hdr.vers >= 13 {
                r.read_object(&mut self.iobits)?;
            }

            self.offset = r.read_i32()?;
            self.max_baskets = r.read_i32()?;
            self.split_level = r.read_i32()?;
            self.entries = r.read_i64()?;

            if hdr.vers >= 11 {
                self.first_entry = r.read_i64()?;
            }

            self.tot_bytes = r.read_i64()?;
            self.zip_bytes = r.read_i64()?;

            {
                let mut branches = r.read_object_into::<ObjArray>()?;
                self.branches = branches
                    .take_objs()
                    .into_iter()
                    .map(|obj| obj.into())
                    .collect();
            }

            {
                let mut leaves = r.read_object_into::<ObjArray>()?;
                if !leaves.objs.is_empty() {
                    self.leaves = leaves
                        .take_objs()
                        .into_iter()
                        .map(|obj| obj.into())
                        .collect();
                }
            }

            {
                let mut baskets = r.read_object_into::<ObjArray>()?;
                if !baskets.objs.is_empty() {
                    self.baskets = baskets.take_objs();
                }
            }

            {
                let _ = r.read_i8()?;
                let mut b = vec![0; self.max_baskets as usize];
                r.read_array_i32(b.as_mut_slice())?;

                self.basket_bytes
                    .extend_from_slice(&b.as_slice()[..self.write_basket as usize]);
            }

            {
                let _ = r.read_i8()?;
                let mut b = vec![0_i64; self.max_baskets as usize];
                r.read_array_i64(b.as_mut_slice())?;

                self.basket_entry
                    .extend_from_slice(&b.as_slice()[..(self.write_basket + 1) as usize]);
            }

            {
                let _ = r.read_i8()?;
                let mut b = vec![0_i64; self.max_baskets as usize];
                r.read_array_i64(b.as_mut_slice())?;

                self.basket_seek
                    .extend_from_slice(&b.as_slice()[..self.write_basket as usize]);
            }

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

        if self.split_level == 0 && !self.branches.is_empty() {
            self.split_level = 1;
        }

        r.check_header(&hdr)?;

        Ok(())

        // todo!()
    }
}

factory_fn_register_impl!(TBranch, "TBranch");
