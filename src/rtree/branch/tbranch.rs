use crate::rbase::AttFill;
use crate::rbytes::ensure_maximum_supported_version;
use crate::rbytes::wbuffer::WBuffer;
use crate::rcont::objarray::ObjArray;
use crate::riofs::file::{RootFileReader, RootFileStreamerInfoContext};
use crate::root::traits::Named;
use crate::root::traits::Object;
use crate::rtree::basket::{Basket, BasketData};
use crate::rtree::branch::tbranch_props::TBranchProps;
use crate::rtree::branch::BranchChunks;
use crate::rtree::leaf::Leaf;
use crate::rtree::streamer_type;
use crate::rtree::tree::TioFeatures;
use crate::{factory_fn_register_impl, rbase, Branch, Marshaler, RBuffer, Unmarshaler};
use itertools::izip;
use lazy_static::lazy_static;
use log::trace;
use regex::Regex;

pub(crate) const DEFAULT_BASKET_SIZE: i32 = 32 * 1024;
pub(crate) const DEFAULT_SPLIT_LEVEL: i32 = 99;
pub(crate) const DEFAULT_MAX_BASKETS: i32 = 10;

#[derive(Default, Debug)]
pub struct TBranch {
    pub(crate) named: rbase::Named,
    attfill: rbase::AttFill,

    /// compression level and algorithm
    compress: i32,
    /// initial size of BASKET buffer
    pub(crate) basket_size: i32,
    /// initial length of entryOffset table in the basket buffers
    entry_offset_len: i32,
    /// last basket number written
    pub(crate) write_basket: i32,
    /// current entry number (last one filled in this branch)
    entry_number: i64,
    /// IO features for newly-created baskets
    pub(crate) iobits: TioFeatures,
    /// offset of this branch
    offset: i32,
    /// maximum number of baskets so far
    pub(crate) max_baskets: i32,
    /// branch split level
    pub(crate) split_level: i32,
    /// number of entries
    entries: i64,
    /// number of the first entry in this branch
    first_entry: i64,
    /// total number of bytes in all leaves before compression
    pub(crate) tot_bytes: i64,
    /// total number of bytes in all leaves after compression
    pub(crate) zip_bytes: i64,

    branches: Vec<Branch>,
    pub(crate) leaves: Vec<Leaf>,
    pub(crate) baskets: Vec<Basket>,

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

    pub fn new(name: String) -> Self {
        TBranch {
            named: rbase::Named::default().with_name(name),
            ..Default::default()
        }
    }

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

        let leaves = if self.leaves.len() == 1 {
            let mut v = Vec::with_capacity(self.basket_seek.len());
            for _ in 0..self.basket_seek.len() {
                v.push(&self.leaves[0]);
            }
            v
        } else if self.leaves.len() == self.basket_seek.len() {
            let mut v = Vec::with_capacity(self.basket_seek.len());
            for l in self.leaves.iter() {
                v.push(l);
            }
            v
        } else {
            unimplemented!();
        };

        trace!(
            "{} {} {} {}",
            self.basket_seek.len(),
            self.basket_bytes.len(),
            size_leaves.len(),
            self.leaves.len()
        );

        let embedded_basket = if !self.baskets.is_empty() {
            assert_eq!(self.baskets.len(), 1);

            Some(self.baskets.iter().map(|b| {
                let key_lenght = b.key().key_len() as usize;
                let buf = b
                    .key()
                    .buffer()
                    .iter()
                    .skip(key_lenght)
                    .cloned()
                    .collect::<Vec<_>>();
                let n = b.nev_buf();
                let chunk_size = 1;

                BranchChunks::RegularSized((n, chunk_size, buf))
            }))
        } else {
            None
        };

        let ret = izip!(&self.basket_seek, &self.basket_bytes, size_leaves, leaves)
            .filter(|(_start, len, _chunk_size, _leave)| **len > 0)
            .map(|(start, len, mut chunk_size, leave)| {
                assert_ne!(*len, 0);
                let mut reader = self.reader.as_ref().unwrap().clone();
                let buf = reader.read_at(*start as u64, *len as u64).unwrap();
                let mut r = RBuffer::new(&buf, 0);
                let b = r.read_object_into::<Basket>().unwrap();

                trace!(
                    "chunk_size = {}, b.entry_size() = {}",
                    chunk_size,
                    b.entry_size()
                );

                match leave {
                    // In case of string, we have to use n
                    Leaf::C(_) | Leaf::Element(_) => {
                        chunk_size = b.entry_size();
                    }
                    _ => {}
                }

                match b.raw_data(&mut reader) {
                    BasketData::TrustNEntries((n, buf)) => {
                        trace!("send ({n},{chunk_size},{:?})", buf);
                        BranchChunks::RegularSized((n, chunk_size, buf))
                    }
                    BasketData::UnTrustNEntries((n, buf, byte_offsets)) => match leave {
                        Leaf::C(_) => {
                            // In case of string, we have to use n
                            trace!("send ({n},{chunk_size},{:?})", buf);
                            BranchChunks::RegularSized((n, chunk_size, buf))
                        }
                        Leaf::Element(_) => {
                            panic!("I dont want to be here (Element should be in TBranchElement)");
                        }
                        _ => {
                            let n_elements_in_buffer = buf.len() / chunk_size as usize;
                            // trial and error...
                            if n_elements_in_buffer == self.entries as usize {
                                // assert_eq!(n, self.entries as usize);
                                trace!("send ({n},{chunk_size},{:?})", buf);
                                BranchChunks::RegularSized((
                                    n_elements_in_buffer as i32,
                                    chunk_size,
                                    buf,
                                ))
                            } else {
                                let byte_offsets =
                                    byte_offsets.iter().zip(byte_offsets.iter().skip(1));
                                let data: Vec<_> = byte_offsets
                                    .map(|(start, stop)| {
                                        let b = &buf[*start as usize..*stop as usize];
                                        b.to_vec()
                                    })
                                    .collect();
                                BranchChunks::IrregularSized((n, data, 0))
                            }
                        } // _ => {
                          //     trace!("leave = {:?}", leave);
                          //     let n = buf.len() / chunk_size as usize;
                          //     trace!("send ({n},{chunk_size},{:?})", buf);
                          //     BranchChunks::RegularSized((n as i32, chunk_size, buf))
                          // }
                    },
                }
            });
        match embedded_basket {
            None => Box::new(ret),
            Some(before) => Box::new(before.chain(ret)),
        }
    }

    pub fn entries(&self) -> i64 {
        self.entries
    }

    pub fn item_type_name(&self) -> String {
        let unknown = "unknown";

        // trace!("len = {} leaves = {:?}", self.leaves.len(), self.leaves);

        if self.leaves.len() == 1 {
            let leave = self.leaves.first().unwrap();
            trace!("leave = {:?}", leave);

            lazy_static! {
                static ref RE_TITLE_HAS_DIMS: Regex =
                    Regex::new(r"^([^\[\]]*)(\[[^\[\]]+\])+").unwrap();
                static ref RE_ITEM_DIM_PATTERN: Regex = Regex::new(r"\[([1-9][0-9]*)\]").unwrap();
            }

            let m = RE_TITLE_HAS_DIMS.captures(leave.title());
            trace!("RE_TITLE_HAS_DIMS = {:?}", m);

            let dim = if m.is_some() {
                if let Some(m) = RE_ITEM_DIM_PATTERN.captures(leave.title()) {
                    trace!("m = {:?}", m);
                    let dim: &str = m.get(1).unwrap().as_str();
                    Some(dim.parse::<i32>().unwrap())
                } else {
                    Some(0)
                }
            } else {
                None
            };

            match leave.type_name() {
                Some(s) => match dim {
                    None => {
                        return s.to_string();
                    }
                    Some(dim) => {
                        if dim > 0 {
                            return format!("{}[{}]", s, dim);
                        } else {
                            return format!("{}[]", s);
                        }
                    }
                },
                None => panic!("can not be here"),
            };
        }

        unknown.to_string()
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
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let _beg = r.pos();
        trace!(";tBranch.unmarshal.{}.beg: {}", _beg, _beg);
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, crate::rvers::BRANCH, self.class())?;

        if hdr.vers >= 10 {
            r.read_object(&mut self.named)?;
            r.read_object(&mut self.attfill)?;
            self.compress = r.read_i32()?;
            self.basket_size = r.read_i32()?;
            self.entry_offset_len = r.read_i32()?;
            trace!(";tBranch.unmarshal.{}.beg: {}", _beg, _beg);
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
                    self.baskets = baskets
                        .take_objs()
                        .into_iter()
                        .map(|obj| obj.into())
                        .collect();
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
            r.read_object(&mut self.named)?;
            if hdr.vers > 7 {
                r.read_object(&mut self.attfill)?;
            }

            self.compress = r.read_i32()?;
            self.basket_size = r.read_i32()?;
            self.entry_offset_len = r.read_i32()?;
            self.write_basket = r.read_i32()?;
            self.entry_number = r.read_i32()? as i64;
            self.offset = r.read_i32()?;
            self.max_baskets = r.read_i32()?;

            if hdr.vers > 6 {
                self.split_level = r.read_i32()?;
            }

            self.entries = r.read_f64()? as i64;
            self.tot_bytes = r.read_f64()? as i64;
            self.zip_bytes = r.read_f64()? as i64;

            trace!(
                ";tBranch.unmarshal.{}..vers>6.pos_read_branches: {}",
                _beg,
                r.pos()
            );

            {
                let mut branches = r.read_object_into::<ObjArray>()?;
                self.branches = branches
                    .take_objs()
                    .into_iter()
                    .map(|obj| obj.into())
                    .collect();
            }

            trace!(
                ";tBranch.unmarshal.{}..vers>6.pos_read_leaves: {}",
                _beg,
                r.pos()
            );

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

            trace!(
                ";tBranch.unmarshal.{}..vers>6.pos.before_read_baskets: {}",
                _beg,
                r.pos()
            );

            {
                let mut baskets = r.read_object_into::<ObjArray>()?;
                if !baskets.objs.is_empty() {
                    self.baskets = baskets
                        .take_objs()
                        .into_iter()
                        .map(|obj| obj.into())
                        .collect();
                }
            }

            trace!(
                ";tBranch.unmarshal.{}..vers>6.pos.after_read_baskets: {}",
                _beg,
                r.pos()
            );

            trace!(
                ";tBranch.unmarshal.{}..vers>6.baskets.len: {}",
                _beg,
                self.baskets.len()
            );

            trace!(
                ";tBranch.unmarshal.{}..vers>6.pos.basket_bytes: {}",
                _beg,
                r.pos()
            );
            {
                let _ = r.read_i8()?;
                let mut b = vec![0; self.max_baskets as usize];
                r.read_array_i32(b.as_mut_slice())?;

                self.basket_bytes.extend_from_slice(b.as_slice());

                trace!(
                    ";tBranch.unmarshal.{}..vers>6.basket_bytes.max_baskets: {}",
                    _beg,
                    self.max_baskets
                );
                trace!(
                    ";tBranch.unmarshal.{}..vers>6.basket_bytes.write_basket: {}",
                    _beg,
                    self.write_basket
                );
                trace!(
                    ";tBranch.unmarshal.{}..vers>6.basket_bytes.size: {}",
                    _beg,
                    self.basket_bytes.len()
                );
            }
            trace!(
                ";tBranch.unmarshal.{}..vers>6.pos.basket_entry: {}",
                _beg,
                r.pos()
            );
            {
                let _ = r.read_i8()?;
                let mut b = vec![0_i32; self.max_baskets as usize];
                r.read_array_i32(b.as_mut_slice())?;
                self.basket_entry.reserve(b.len());

                for v in b {
                    self.basket_entry.push(v as i64);
                }

                // self.basket_entry
                //     .extend_from_slice(&b.as_slice()[..(self.write_basket + 1) as usize]);
            }
            trace!(
                ";tBranch.unmarshal.{}..vers>6.pos.basket_seek: {}",
                _beg,
                r.pos()
            );
            {
                match r.read_i8()? {
                    2 => {
                        let mut b = vec![0_i64; self.max_baskets as usize];
                        r.read_array_i64(b.as_mut_slice())?;

                        self.basket_seek
                            .extend_from_slice(&b.as_slice()[..self.write_basket as usize]);
                    }
                    _ => {
                        let mut b = vec![0_i32; self.max_baskets as usize];
                        r.read_array_i32(b.as_mut_slice())?;
                        self.basket_seek.reserve(b.len());

                        for v in b {
                            self.basket_seek.push(v as i64);
                        }
                    }
                }
            }

            trace!(
                ";tBranch.unmarshal.{}..vers>6.pos.after_basket_seek: {}",
                _beg,
                r.pos()
            );

            self.fname = r.read_string()?.to_string();

            trace!(";tBranch.unmarshal.{}..vers>6.fname: {}", _beg, self.fname);

            trace!("self = {:?}", self);

            // todo!();
            // r.read_object(&mut self.named)?;
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

impl Marshaler for TBranch {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        todo!()
    }
}

factory_fn_register_impl!(TBranch, "TBranch");
