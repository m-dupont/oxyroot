use crate::file::RootFileReader;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::rcont::objarray::ObjArray;
use crate::root::traits::{Named, Object};
use crate::rtree::basket::Basket;
use crate::rtree::leaf::Leaf;
use crate::rtree::tree::TioFeatures;
use crate::rtypes::FactoryItem;
use crate::{factotry_all_for_register_impl, rbase};
use crate::{factotry_fn_register_impl, rvers};
use anyhow::ensure;
use chrono::format::Item;
use itertools::{enumerate, izip, IntoChunks, Itertools};
use log::{debug, trace};
use num::range;
use std::cmp;
use std::fmt::Debug;
use std::iter::{repeat, Map};
use std::marker::PhantomData;
use std::vec::IntoIter;

pub enum Branch {
    Base(TBranch),
    Element(TBranchElement),
}

impl From<Branch> for TBranch {
    fn from(b: Branch) -> Self {
        match b {
            Branch::Base(bb) => bb,
            Branch::Element(be) => be.branch,
        }
    }
}

impl From<Box<dyn FactoryItem>> for Branch {
    fn from(obj: Box<dyn FactoryItem>) -> Self {
        match obj.class() {
            "TBranch" => Branch::Base(*obj.downcast::<TBranch>().unwrap()),
            "TBranchElement" => Branch::Element(*obj.downcast::<TBranchElement>().unwrap()),
            &_ => todo!(),
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

impl Branch {
    pub fn name(&self) -> &str {
        let b: &TBranch = self.into();
        b.name()
    }

    pub fn set_reader(&mut self, reader: Option<RootFileReader>) {
        match self {
            Branch::Base(bb) => bb.set_reader(Some(reader.unwrap().clone())),
            Branch::Element(be) => be.branch.set_reader(Some(reader.unwrap().clone())),
        }
    }

    pub fn get_basket<'a, F, T>(&'a self, func: F) -> impl Iterator<Item = T> + 'a
    where
        T: Debug + 'a,
        F: Fn(&mut RBuffer) -> T + 'a,
    {
        trace!("get_basket in Branch = {}", self.name());
        match self {
            Branch::Base(bb) => bb.get_basket(func),
            Branch::Element(be) => be.branch.get_basket(func),
        }
    }
}

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

    branches: Vec<Branch>,
    leaves: Vec<Leaf>,
    baskets: Vec<Box<dyn FactoryItem>>,

    /// length of baskets on file
    basketBytes: Vec<i32>,
    /// table of first entry in each basket
    basketEntry: Vec<i64>,
    /// addresses of baskets on file
    basketSeek: Vec<i64>,
    /// named of file where buffers are stored (empty if in same file as Tree header)
    fname: String,

    reader: Option<RootFileReader>,
}

#[derive(Debug)]
pub struct ZiperBranchInnerO<'a, T> {
    pub num_entries: u32,
    pub chunk_size: i32,
    pub i: Vec<u8>,
    // pub o: &'a [u8],
    phantom: PhantomData<&'a T>,
}

impl<'a, T> ZiperBranchInnerO<'a, T> {
    pub fn new(num_entries: u32, chunk_size: i32, buf: Vec<u8>) -> Self {
        ZiperBranchInnerO {
            num_entries,
            chunk_size,
            i: buf,
            // o: &[],
            phantom: Default::default(),
        }
    }
}

impl<'a, T> Iterator for ZiperBranchInnerO<'a, T> {
    type Item = (u32, i32, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        // let output_size =

        let o = self.i[0..self.chunk_size as usize].to_vec();

        trace!("o = {:?}", o);

        return Some((self.num_entries, self.chunk_size, o));

        // if self.o.is_empty() {
        //     None
        // } else {
        //     let chunksz = cmp::min(self.o.len(), self.chunk_size as usize);
        //     let (fst, snd) = self.o.split_at(chunksz);
        //
        //     Some((self.num_entries, self.chunk_size, fst))
        // }

        todo!()
    }
}

pub struct ZiperBranches<'a, T>
where
    T: Debug,
{
    reader: RootFileReader,
    branches: &'a Vec<Branch>,
    phantom: PhantomData<T>,
    iterators: Vec<Box<dyn Iterator<Item = (u32, i32, Vec<u8>)> + 'a>>,
    // output_buffers: Option<ZiperBranchInnerO<'a, T>>,
    output_buffers: Vec<Option<(u32, i32, Vec<u8>)>>,
    current_size: Vec<usize>,
    nb_entries: Vec<u32>,
}

impl<'a, T> ZiperBranches<'a, T>
where
    T: Debug,
{
    pub fn new(
        reader: Option<&RootFileReader>,
        branches: &'a Vec<Branch>,
        nb_entries: u32,
    ) -> Self {
        let mut v = Vec::new();
        // let mut v: Vec<dyn Iterator<Item = (u32, i32, Vec<u8>)>> = Vec::new();
        for branch in branches {
            let tbranch: &TBranch = branch.into();
            let data = tbranch.get_baskets_buffer();

            // let b = data.collect::<Vec<_>>();
            //
            // trace!("b = {:?}", b);

            // todo!();

            v.push(data);

            // let d = data as &dyn Iterator<Item = (u32, i32, Vec<u8>)>;
            // v.push(Box::new(data));
        }

        // todo!();

        ZiperBranches {
            reader: reader.unwrap().clone(),
            branches,
            phantom: Default::default(),
            iterators: v,
            output_buffers: Vec::new(),
            current_size: Default::default(),
            nb_entries: Default::default(),
        }
    }
}

impl<'a, T> ZiperBranches<'a, T>
where
    T: Debug,
{
    fn fill_output(&mut self) {
        for it_branch in &mut self.iterators {
            if let Some(data) = it_branch.next() {
                let (n, chunk_size, buf) = data;
                trace!("n = {}", n);

                self.output_buffers.push(Some((n, chunk_size, buf)));
                self.nb_entries.push(n);
                self.current_size.push(0);
            }
        }
    }

    fn fill_output_one_branch(&mut self, num_branch: usize) {
        trace!("self.fill_output_one_branch, num_branch = {}", num_branch);

        let it_branch = &mut self.iterators[num_branch];
        if let Some(data) = it_branch.next() {
            let (n, chunk_size, buf) = data;
            trace!("n = {}", n);

            self.output_buffers[num_branch] = (Some((n, chunk_size, buf)));
            self.nb_entries[num_branch] = n;
            self.current_size[num_branch] = 0;

            // for it_branch in &mut self.iterators {
            //     if let Some(data) = it_branch.next() {
            //         let (n, chunk_size, buf) = data;
            //         trace!("n = {}", n);
            //
            //         self.output_buffers.push(Some((n, chunk_size, buf)));
            //         self.nb_entries.push(n);
            //         self.current_size.push(0);
            //     }
            // }
        }
    }
}

impl<'a, T> Iterator for ZiperBranches<'a, T>
where
    T: Debug,
{
    type Item = (u32, i32, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        // let mut ret = Vec::new();

        if self.output_buffers.is_empty() {
            trace!("self.output_buffers.is_empty()");
            self.fill_output();
        }

        if self.output_buffers.is_empty() {
            return None;
        }

        for b in &self.output_buffers {
            if b.is_none() {
                return None;
            }
        }

        let size = self.output_buffers.iter().fold(0 as usize, |acc, par| {
            let (n, s, v) = par.as_ref().unwrap();
            acc + *s as usize
        });
        // let mut outbuf = vec![0 as u8; size];

        let mut outbuf: Vec<u8> = Vec::with_capacity(size);

        // for (ib, bbuffer) in enumerate(&self.output_buffers) {
        for ib in 0..self.current_size.len() {
            match &self.output_buffers[ib] {
                None => {
                    panic!("faut remplit");
                }
                Some((n, chunk_size, buf)) => {
                    let csize = *chunk_size as usize;
                    let begin = self.current_size[ib] * csize;
                    let end = (self.current_size[ib] + 1) * csize;

                    let mut ibuffer = buf[begin..end].to_vec();
                    outbuf.append(&mut ibuffer);

                    self.current_size[ib] += 1;

                    // trace!(
                    //     "self.current_size = {:?}, self.nb_entries = {:?}",
                    //     self.current_size,
                    //     self.nb_entries
                    // );

                    if self.current_size[ib] == self.nb_entries[ib] as usize {
                        self.output_buffers[ib] = None;

                        self.fill_output_one_branch(ib);
                    }
                }
            }
        }

        // for ib in 0..self.current_size.len() {
        //     if self.current_size[ib] == self.nb_entries[ib] as usize {
        //         self.output_buffers[ib] = None;
        //
        //         self.fill_output_one_branch(ib);
        //     }
        // }

        trace!(
            "self.current_size = {:?}, self.nb_entries = {:?}",
            self.current_size,
            self.nb_entries
        );

        return Some((0, size as i32, outbuf));

        // let next = self.output_buffers.as_mut().unwrap().next();

        // self.output_buffers.unwrap().next();

        // let v = self.output_buffers.as_ref().unwrap();
        //
        // return Some((v.num_entries, v.chunk_size, v.o.ne()));

        // let v = self.output_buffers.unwrap();

        // let v = self.output_buffers[0];

        // println!("inner = {:?}", self.output_buffers);

        // for branch in self.branches {
        //     let tbranch: &TBranch = branch.into();
        //     let mut data = tbranch.get_baskets_buffer();
        //     let d = data.next();
        //     println!("d = {:?}", d);
        // }

        todo!()
    }
}

impl TBranch {
    pub fn set_reader(&mut self, reader: Option<RootFileReader>) {
        for branch in self.branches.iter_mut() {
            branch.set_reader(Some(reader.as_ref().unwrap().clone()));
        }

        self.reader = reader;
    }

    pub fn get_baskets_buffer(&self) -> Box<dyn Iterator<Item = (u32, i32, Vec<u8>)> + '_> {
        let mut size_leaves = self.leaves.iter().map(|e| e.etype()).collect::<Vec<_>>();
        trace!("leaves = {:?}", self.leaves.len());

        trace!(
            "get_baskets_buffer: (start = {:?}, len = {:?}, chunk_size) = {:?}",
            &self.basketSeek,
            &self.basketBytes,
            size_leaves
        );

        if size_leaves.len() != self.basketSeek.len() {
            for i in (1..self.basketSeek.len()) {
                size_leaves.push(size_leaves[0]);
            }
        }

        Box::new(izip!(&self.basketSeek, &self.basketBytes, size_leaves).map(
            |(start, len, chunk_size)| {
                trace!(
                    "get_baskets_buffer: (start = {}, len = {}, chunk_size) = {}",
                    start,
                    len,
                    chunk_size
                );
                let mut reader = self.reader.as_ref().unwrap().clone();
                let buf = reader.read_at(*start as u64, *len as u64).unwrap();
                let mut r = RBuffer::new(&buf, 0);
                let b = r.read_object_into::<Basket>().unwrap();
                let (n, buf) = b.raw_data(&mut reader);
                (n, chunk_size, buf)
            },
        ))
    }

    pub fn get_basket<'a, F, T>(&'a self, func: F) -> impl Iterator<Item = T> + 'a
    where
        T: Debug + 'a,
        F: Fn(&mut RBuffer) -> T + 'a,
    {
        trace!(
            "get_basket in TBranch = {}, nb_branches = {} nb entries = {}",
            self.name(),
            self.branches.len(),
            self.entries
        );

        assert!(self.reader.is_some());

        let mut reader = self.reader.as_ref().unwrap().clone();
        // let mut reader = self.reader.unwrap();

        let it = if self.branches.len() > 0 {
            let b: Box<dyn Iterator<Item = T>> = Box::new(
                ZiperBranches::<usize>::new(
                    self.reader.as_ref(),
                    &self.branches,
                    self.entries as u32,
                )
                .map(move |(n, chunk_size, buf)| {
                    let mut r = RBuffer::new(&buf, 0);
                    func(&mut r)
                    // trace!("buf = {:?}", buf);
                    // trace!("buf.len = {} n = {}", buf.len(), n);
                    // let size = buf.len() / n as usize;
                    // let mut v = Vec::new();
                    // for i in 0..n {
                    //     v.push(func(&mut r));
                    // }
                    // v
                }),
            );
            b
        } else {
            let b: Box<dyn Iterator<Item = T>> = Box::new(
                self.get_baskets_buffer()
                    .map(move |(n, chunk_size, buf)| {
                        let mut r = RBuffer::new(&buf, 0);
                        // trace!("buf = {:?}", buf);
                        // trace!("buf.len = {} n = {}", buf.len(), n);
                        let size = buf.len() / n as usize;
                        let mut v = Vec::new();
                        for i in 0..n {
                            v.push(func(&mut r));
                        }
                        v
                    })
                    .flatten(),
            );
            b
        };

        // return it;

        return it;
        // .map(|a| a as dyn Iterator<Item = (u32, i32, Vec<u8>)>)
        // .map(move |(n, chunk_size, buf)| {
        //     let mut r = RBuffer::new(&buf, 0);
        //     trace!("buf = {:?}", buf);
        //     trace!("buf.len = {} n = {}", buf.len(), n);
        //     let size = buf.len() / n as usize;
        //     let mut v = Vec::new();
        //     for i in 0..n {
        //         v.push(func(&mut r));
        //     }
        //     v
        // })
        // .flatten();
        // .for_each(drop);

        // self.get_baskets_buffer()
        //     .map(move |(n, chunk_size, buf)| {
        //         let mut r = RBuffer::new(&buf, 0);
        //         trace!("buf = {:?}", buf);
        //         trace!("buf.len = {} n = {}", buf.len(), n);
        //         let size = buf.len() / n as usize;
        //         let mut v = Vec::new();
        //         for i in 0..n {
        //             v.push(func(&mut r));
        //         }
        //         v
        //     })
        //     .flatten()

        // self.basketSeek
        //     .iter()
        //     .zip(&self.basketBytes)
        //     .map(move |(start, len)| {
        //         let buf = reader.read_at(*start as u64, *len as u64).unwrap();
        //         let mut r = RBuffer::new(&buf, 0);
        //         let b = r.read_object_into::<Basket>().unwrap();
        //         let (n, buf) = b.raw_data(&mut reader);
        //         let mut r = RBuffer::new(&buf, 0);
        //
        //         trace!("buf = {:?}", buf);
        //
        //         trace!("buf.len = {} n = {}", buf.len(), n);
        //
        //         // let buf = buf.take(..n)
        //
        //         let size = buf.len() / n as usize;
        //         // buf.chunks(size)
        //         //     .map(|b| {
        //         //         let mut r = RBuffer::new(b, 0);
        //         //         func(&mut r)
        //         //     })
        //         //     .collect::<Vec<_>>()
        //         let mut v = Vec::new();
        //         for i in 0..n {
        //             v.push(func(&mut r));
        //         }
        //
        //         v
        //     })
        //     .flatten()

        // self.basketSeek
        //     .iter()
        //     .zip(&self.basketBytes)
        //     .map(move |(start, len)| {
        //         let buf = reader.read_at(*start as u64, *len as u64).unwrap();
        //         let mut r = RBuffer::new(&buf, 0);
        //         let b = r.read_object_into::<Basket>().unwrap();
        //         let (n, buf) = b.raw_data(&mut reader);
        //         let mut r = RBuffer::new(&buf, 0);
        //
        //         trace!("buf = {:?}", buf);
        //
        //         trace!("buf.len = {} n = {}", buf.len(), n);
        //
        //         // let buf = buf.take(..n)
        //
        //         let size = buf.len() / n as usize;
        //         // buf.chunks(size)
        //         //     .map(|b| {
        //         //         let mut r = RBuffer::new(b, 0);
        //         //         func(&mut r)
        //         //     })
        //         //     .collect::<Vec<_>>()
        //         let mut v = Vec::new();
        //         for i in 0..n {
        //             v.push(func(&mut r));
        //         }
        //
        //         v
        //     })
        //     .flatten()
    }
}

impl Named for TBranch {
    fn name(&self) -> &'_ str {
        &self.named.name()
    }
}

impl Unmarshaler for TBranch {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("TBranch:unmarshal, name = {}", self.name());
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
                let mut branches = r.read_object_into::<ObjArray>()?;
                trace!("branches for {} = {:?}", self.name(), branches);
                self.branches = branches
                    .take_objs()
                    .into_iter()
                    .map(|obj| obj.into())
                    .collect();
            }

            {
                let mut leaves = r.read_object_into::<ObjArray>()?;
                trace!("leaves = {:?}", leaves);
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

        r.check_header(&hdr)?;

        Ok(())

        // todo!()
    }
}

factotry_fn_register_impl!(TBranch, "TBranch");

#[derive(Default)]
pub struct TBranchElement {
    branch: TBranch,

    class: String,
    // class name of referenced object
    parent: String,
    // name of parent class
    clones: String,
    // named of class in TClonesArray (if any)
    chksum: i32,
    // checksum of class
    clsver: i16,
    // version number of class
    id: i32,
    // element serial number in fInfo
    btype: i32,
    // branch type
    stype: i32,
    // branch streamer type
    max: i32,
    // maximum entries for a TClonesArray or variable array
    stltyp: i32, // STL container type
                 // bcount1: *tbranchElement // pointer to primary branchcount branch
                 // bcount2: *tbranchElement // pointer to secondary branchcount branch
}

impl Named for TBranchElement {
    fn name(&self) -> &'_ str {
        &self.branch.name()
    }
}

impl TBranchElement {
    pub fn branch(self) -> TBranch {
        self.branch
    }
}

impl Unmarshaler for TBranchElement {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("TBranchElement:unmarshal, name = {}", self.name());
        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::BranchElement,
            "rtree: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::BranchElement
        );

        r.read_object(&mut self.branch)?;

        self.class = r.read_string()?.to_string();

        // trace!("class = {}", self.class);

        if hdr.vers > 1 {
            self.parent = r.read_string()?.to_string();
            self.clones = r.read_string()?.to_string();
            self.chksum = r.read_i32()?;
        }
        if hdr.vers >= 10 {
            self.clsver = r.read_i16()?;
        } else {
            self.clsver = r.read_i32()? as i16;
        }

        self.id = r.read_i32()?;
        self.btype = r.read_i32()?;
        self.stype = r.read_i32()?;

        if hdr.vers > 1 {
            self.max = r.read_i32()?;

            let bcount1 = r.read_object_any_into()?;
            let bcount2 = r.read_object_any_into()?;

            // bcount1 := r.ReadObjectAny()
            // if bcount1 != nil {
            //     b.bcount1 = bcount1.(*tbranchElement)
            // }

            // bcount2 := r.ReadObjectAny()
            // if bcount2 != nil {
            //     b.bcount2 = bcount2.(*tbranchElement)
            // }
        }

        // todo!();
        Ok(())
    }
}

factotry_fn_register_impl!(TBranchElement, "TBranchElement");
