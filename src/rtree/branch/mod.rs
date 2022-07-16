mod tbranch;
mod tbranch_element;
mod tbranch_props;

pub(crate) use crate::rtree::branch::tbranch::TBranch;
pub(crate) use crate::rtree::branch::tbranch_element::TBranchElement;
use std::fmt::{format, Debug, Formatter, Write};
use std::iter::{once, Once};

use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::UnmarshalerInto;
use crate::riofs::file::{RootFileReader, RootFileStreamerInfoContext};
use crate::root::traits::{Named, Object};

use crate::rtypes::FactoryItem;
use itertools::chain;
use log::{debug, trace};
use std::marker::PhantomData;

pub(crate) enum BranchChunks {
    RegularSized((u32, i32, Vec<u8>)),
    IrregularSized((u32, Vec<Vec<u8>>, i32)), // _,_, header_bytes
}

/// Rust equivalent of [`TBranch`](https://root.cern/doc/master/classTBranch.html)
/// or [`TBranchElement`](https://root.cern/doc/master/classTBranchElement.html) (ie column) of a TTree
///
/// Choice between `TBranch` or `TBranchElement` is done when Root file is read.
/// [Branch] should not be constructed by user but accessed via [crate::Tree::branch]
pub enum Branch {
    Base(TBranch),
    Element(TBranchElement),
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

impl Debug for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Branch::Base(bb) => f.write_str("TBranch{")?,
            Branch::Element(be) => f.write_str("TBranchElement{")?,
        };
        f.write_str(format!("name: {}", self.name()).as_str())?;
        f.write_str(format!("item_t: {}", self.item_type_name()).as_str())?;
        f.write_str("}")
    }
}

impl Branch {
    pub fn name(&self) -> &str {
        let b: &TBranch = self.into();
        b.name()
    }

    pub fn class(&self) -> &str {
        let b: &TBranch = self.into();
        b.class()
    }

    pub fn item_type_name(&self) -> String {
        match self {
            Branch::Base(bb) => bb.item_type_name(),
            Branch::Element(be) => be.item_type_name(),
        }
    }

    pub fn entries(&self) -> i64 {
        let b: &TBranch = self.into();
        b.entries()
    }

    pub fn branches(&self) -> impl Iterator<Item = &Branch> {
        match self {
            Branch::Base(bb) => bb.branches().iter(),
            Branch::Element(be) => be.branch.branches().iter(),
        }
    }

    pub fn branches_r(&self) -> Vec<&Branch> {
        let mut v = Vec::new();

        for b in self.branches() {
            v.push(b);
            for bb in b.branches_r() {
                v.push(bb);
            }
        }

        v
    }

    /// search in children branches
    pub fn branch(&self, name: &str) -> Option<&Branch> {
        match self {
            Branch::Base(bb) => bb.branch(name),
            Branch::Element(be) => be.branch.branch(name),
        }
    }

    // pub fn branches_recursively_apply<'a, B, F>(&'a self, f: &'a F) -> impl Iterator<Item = B> + 'a
    // where
    //     B: 'a,
    //     F: Fn(&Branch) -> B + 'a,
    // {
    //     // once(f(self))
    //     // chain(once(self), self.branches()).map(|b| b.branches_recursively_apply(f))
    //
    //     self.branches()
    //         .map(|b| b.branches_recursively_apply(f))
    //         .flatten()
    //
    // }

    pub(crate) fn set_top_level(&mut self, v: Option<bool>) {
        match self {
            Branch::Base(bb) => bb.props.is_top_level = v,
            Branch::Element(be) => be.set_is_top_level(v),
        }
    }

    // pub(crate) fn set_item_type_name(&mut self) {
    //     match self {
    //         Branch::Base(bb) => bb.set_item_type_name(),
    //         Branch::Element(be) => be.set_item_type_name(),
    //     }
    // }

    pub(crate) fn set_reader(&mut self, reader: Option<RootFileReader>) {
        match self {
            Branch::Base(bb) => bb.set_reader(Some(reader.unwrap())),
            Branch::Element(be) => be.branch.set_reader(Some(reader.unwrap())),
        }
    }

    pub(crate) fn set_streamer_info(&mut self, sinfos: RootFileStreamerInfoContext) {
        match self {
            Branch::Base(bb) => bb.set_streamer_info(sinfos),
            Branch::Element(be) => be.branch.set_streamer_info(sinfos),
        }
    }

    fn get_baskets_buffer(&self) -> Box<dyn Iterator<Item = BranchChunks> + '_> {
        match self {
            Branch::Base(bb) => bb.get_baskets_buffer(),
            Branch::Element(be) => be.get_baskets_buffer(),
        }
    }

    pub fn get_basket<'a, F, T>(&'a self, mut func: F) -> impl Iterator<Item = T> + 'a
    where
        T: 'a,
        F: FnMut(&mut RBuffer) -> T + 'a,
    {
        trace!("get_basket in BRANCH = {}", self.name());

        let tbranch = match self {
            Branch::Base(bb) => bb,
            Branch::Element(be) => &be.branch,
        };

        trace!(
            "get_basket in TBranch = {}, nb_branches = {} nb entries = {}",
            tbranch.name(),
            tbranch.branches().len(),
            tbranch.entries()
        );
        assert!(tbranch.reader().is_some());

        let it = if !tbranch.branches().is_empty() {
            let b: Box<dyn Iterator<Item = T>> = Box::new(
                ZiperBranches::<usize>::new(tbranch.branches(), tbranch.entries() as u32).map(
                    move |(_n, _chunk_size, buf)| {
                        let mut r = RBuffer::new(&buf, 0);
                        func(&mut r)
                    },
                ),
            );
            b
        } else {
            let b: Box<dyn Iterator<Item = T>> =
                Box::new(self.get_baskets_buffer().flat_map(move |chunk| {
                    match chunk {
                        BranchChunks::RegularSized((n, _chunk_size, buf)) => {
                            let mut r = RBuffer::new(&buf, 0);
                            let mut v = Vec::with_capacity(n as usize);

                            for _i in 0..n {
                                v.push(func(&mut r));
                            }
                            v
                        }
                        BranchChunks::IrregularSized((_n, data_chuncked, header_bytes)) => {
                            data_chuncked
                                .iter()
                                .map(|buf| {
                                    trace!("buf = {:?}", buf);
                                    let mut r = RBuffer::new(buf, 0);
                                    r.set_skip_header(Some(header_bytes));

                                    func(&mut r)
                                })
                                .collect::<Vec<_>>()
                        }
                    }
                }));
            b
        };

        it
    }

    /// Create an iterator over the data of a column (`TBranch`)
    pub fn as_iter<'a, T>(&'a self) -> impl Iterator<Item = T> + 'a
    where
        T: UnmarshalerInto<Item = T> + 'a,
    {
        self.get_basket(|r| r.read_object_into::<T>().unwrap())
    }

    pub fn streamer_type(&self) -> Option<i32> {
        match self {
            Branch::Base(_bb) => None,
            Branch::Element(be) => Some(be.streamer_type()),
        }
    }
}

pub struct ZiperBranchInnerO<'a, T> {
    pub num_entries: u32,
    pub chunk_size: i32,
    pub i: Vec<u8>,
    // pub o: &'a [u8],
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for ZiperBranchInnerO<'a, T> {
    type Item = (u32, i32, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        // let output_size =

        let o = self.i[0..self.chunk_size as usize].to_vec();

        trace!("o = {:?}", o);

        Some((self.num_entries, self.chunk_size, o))
    }
}

pub struct ZiperBranches<'a, T> {
    _branches: &'a Vec<Branch>,
    phantom: PhantomData<T>,
    iterators: Vec<Box<dyn Iterator<Item = BranchChunks> + 'a>>,
    // output_buffers: Option<ZiperBranchInnerO<'a, T>>,
    output_buffers: Vec<Option<BranchChunks>>,
    current_size: Vec<usize>,
    nb_entries: Vec<u32>,
}

impl<'a, T> ZiperBranches<'a, T> {
    pub fn new(branches: &'a Vec<Branch>, _nb_entries: u32) -> Self {
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
            _branches: branches,
            phantom: Default::default(),
            iterators: v,
            output_buffers: Vec::new(),
            current_size: Default::default(),
            nb_entries: Default::default(),
        }
    }
}

impl<'a, T> ZiperBranches<'a, T> {
    fn fill_output(&mut self) {
        for it_branch in &mut self.iterators {
            if let Some(chunk) = it_branch.next() {
                // let (n, chunk_size, buf) = data;
                // trace!("n = {}", n);

                let n = match chunk {
                    BranchChunks::RegularSized((n, _, _)) => n,
                    BranchChunks::IrregularSized((n, _, _)) => n,
                };

                self.output_buffers.push(Some(chunk));
                self.nb_entries.push(n);
                self.current_size.push(0);
            }
        }
    }

    fn fill_output_one_branch(&mut self, num_branch: usize) {
        trace!("self.fill_output_one_branch, num_branch = {}", num_branch);

        let it_branch = &mut self.iterators[num_branch];
        if let Some(chunk) = it_branch.next() {
            let n = match chunk {
                BranchChunks::RegularSized((n, _, _)) => n,
                BranchChunks::IrregularSized((n, _, _)) => n,
            };

            trace!("n = {}", n);

            self.output_buffers[num_branch] = Some(chunk);
            self.nb_entries[num_branch] = n;
            self.current_size[num_branch] = 0;
        }
    }
}

impl<'a, T> Iterator for ZiperBranches<'a, T> {
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

        let size = self.output_buffers.iter().fold(0_usize, |acc, par| {
            let s = match par.as_ref().unwrap() {
                BranchChunks::RegularSized((_, s, _)) => s,
                BranchChunks::IrregularSized(_) => {
                    todo!()
                }
            };
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
                Some(chunk) => {
                    let (chunk_size, buf) = match chunk {
                        BranchChunks::RegularSized((_, c, b)) => (c, b),
                        BranchChunks::IrregularSized(_) => {
                            todo!()
                        }
                    };

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

        Some((0, size as i32, outbuf))
    }
}
