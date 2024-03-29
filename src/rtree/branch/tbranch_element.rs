use crate::rbase::{AttFill, Object};
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{ensure_maximum_supported_version, RVersioner};
use crate::rdict::Streamer;
use crate::rmeta::EReadWrite;
use crate::root::traits::Named;
use crate::root::traits::Object as TraitObject;
use crate::rtree::basket::{Basket, BasketData};
use crate::rtree::branch::tbranch_props::TBranchProps;
use crate::rtree::branch::{BranchChunks, TBranch};
use crate::rtree::leaf::Leaf;
use crate::rtree::streamer_type;
use crate::rtree::streamer_type::{_from_leaftype_to_str, clean_type_name};
use crate::{factory_fn_register_impl, Marshaler, RBuffer, Unmarshaler};
use itertools::izip;
use lazy_static::lazy_static;
use log::trace;
use regex::Regex;
use std::cell::RefCell;
use std::iter::once;

#[derive(Default, Debug)]
pub struct TBranchElement {
    pub(crate) branch: TBranch,

    class_name: String,
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
    /// branch streamer type
    stype: i32,

    max: i32,
    // maximum entries for a TClonesArray or variable array
    // bcount1: *tbranchElement // pointer to primary branchcount branch
    // bcount2: *tbranchElement // pointer to secondary branchcount branch
    pub(crate) props: TBranchProps,
}

impl Named for TBranchElement {
    fn name(&self) -> &'_ str {
        self.branch.name()
    }
}

impl TBranchElement {
    pub fn new(class_name: String, tbranch: TBranch) -> Self {
        TBranchElement {
            class_name,
            branch: tbranch,
            stype: -1,
            id: -1,
            ..Default::default()
        }
    }

    pub fn streamer_type(&self) -> i32 {
        self.stype
    }

    pub fn class_name(&self) -> &str {
        &self.class_name
    }

    pub fn is_top_level(&self) -> Option<bool> {
        self.props.is_top_level
        // let props = self.props.take();
        // let ret = props.is_top_level;
        // self.props.set(props);
        // ret
    }

    pub fn set_is_top_level(&mut self, v: Option<bool>) {
        self.props.is_top_level = v;

        // let mut props = self.props.take();
        // props.is_top_level = v;
        // self.props.set(props);
    }

    pub fn streamer(&self) -> Option<&Streamer> {
        let streamer = self.branch.sinfos.as_ref().unwrap().get(self.class_name());

        let element = match streamer {
            None => None,
            Some(streamer) => streamer.get(self.clean_name()),
        };

        element
    }

    pub fn item_type_name(&self) -> String {
        self._item_type_name()
        // if self.props.borrow().item_type_name.is_none() {
        //     self.props.borrow_mut().item_type_name = Some(self._item_type_name());
        // }
        //
        // self.props.borrow().item_type_name.unwrap()
    }

    fn _item_type_name(&self) -> String {
        if !self.branch.branches().is_empty() {
            if let Some(true) = self.is_top_level() {
                if !self.class_name.is_empty() {
                    return self.class_name.to_string();
                } else {
                    todo!()
                }
            } else {
                match self.streamer() {
                    None => {
                        todo!()
                    }
                    Some(streamer) => {
                        return streamer.name().into();
                        // trace!("current streamer = {:?}", streamer);
                    }
                }
            }
        } else if self.branch.leaves.len() == 1 {
            let leave = self.branch.leaves.first().unwrap();
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

            return match leave.type_name() {
                Some(t) => t.into(),
                None => match leave {
                    Leaf::Base(_) => {
                        todo!()
                    }
                    Leaf::Element(leave) => {
                        let leaftype = leave.ltype;
                        if let Some(s) = _from_leaftype_to_str(leaftype) {
                            trace!("dim = {:?}", dim);
                            match dim {
                                None => {}
                                Some(dim) => {
                                    if dim > 0 {
                                        return format!("{}[{}]", s, dim);
                                    } else {
                                        return format!("{}[]", s);
                                    }
                                }
                            }
                            return s.into();
                        }

                        if self.streamer_type() == EReadWrite::TString.to_i32() {
                            return "TString".to_string();
                        }

                        if self.streamer_type() == EReadWrite::STL || self.streamer_type() == -1 {
                            match self.streamer() {
                                None => {
                                    return clean_type_name(self.class_name());
                                }
                                Some(s) => {
                                    return clean_type_name(s.item_type_name());
                                }
                            }
                        }

                        todo!()
                    }
                    _ => {
                        panic!("Impossible to leaf like that");
                    }
                },
            };
        }

        self.branch.item_type_name()
    }

    fn clean_name(&self) -> &str {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.*\.)*([^\.\[\]]*)(\[.*\])*").unwrap();
        }

        RE.captures(self.name()).unwrap().get(2).unwrap().as_str()
    }

    pub(crate) fn get_baskets_buffer(&self) -> Box<dyn Iterator<Item = BranchChunks> + '_> {
        trace!(";TBranchElement.get_baskets_buffer.call:{:?}", true);
        let mut size_leaves = self
            .branch
            .leaves
            .iter()
            .map(|e| e.etype())
            .collect::<Vec<_>>();

        trace!(
            ";TBranchElement.get_baskets_buffer.size_leaves:{:?}",
            size_leaves
        );

        if size_leaves.len() != self.branch.basket_seek.len() {
            for _i in 1..self.branch.basket_seek.len() {
                size_leaves.push(size_leaves[0]);
            }
        }

        trace!(
            ";TBranchElement.get_baskets_buffer.size_leaves:{:?}",
            size_leaves
        );

        let leaves = if self.branch.leaves.len() == 1 {
            let mut v = Vec::new();
            for _ in 0..self.branch.basket_seek.len() {
                v.push(&self.branch.leaves[0]);
            }
            v
        } else if self.branch.leaves.len() == self.branch.basket_seek.len() {
            let mut v = Vec::new();
            for l in self.branch.leaves.iter() {
                v.push(l);
            }
            v
        } else {
            unimplemented!();
        };

        trace!(
            ";TBranchElement.get_baskets_buffer.size_leaves:{:?}",
            size_leaves
        );

        let embedded_basket = if !self.branch.baskets.is_empty() {
            assert_eq!(self.branch.baskets.len(), 1);

            let element = self.streamer();

            let header_bytes = streamer_type::header_bytes_from_type(
                self.streamer_type(),
                element,
                self.class_name(),
            );

            trace!("header_bytes = {}", header_bytes);

            Some(self.branch.baskets.iter().map(move |b| {
                let key_lenght = b.key().key_len();
                let buf = b.key().buffer();

                let n = buf.len() as i32;

                let offsets = b.offsets().iter().chain(once(&n));
                // let offsets = std::iter::once(&key_lenght).chain(offsets);

                // let byte_offsets = offsets.iter().zip(offsets.iter().skip(1));
                let byte_offsets = offsets.clone().zip(offsets.skip(1));
                // .collect();
                // trace!("byte_offsets = {:?}", byte_offsets);
                // trace!("buf = {:?}", buf);

                trace!(
                    ";TBranchElement.get_baskets_buffer.embedded_basket.byte_offsets: {:?}",
                    byte_offsets.clone().collect::<Vec<_>>()
                );

                let data: Vec<_> = byte_offsets
                    .map(|(start, stop)| {
                        let b = &buf[*start as usize..*stop as usize];
                        b.to_vec()
                    })
                    .collect();

                trace!(
                    ";TBranchElement.get_baskets_buffer.embedded_basket.buf.len: {}",
                    buf.len()
                );
                trace!(
                    ";TBranchElement.get_baskets_buffer.embedded_basket.data.len: {}",
                    data.len()
                );

                trace!(
                    ";TBranchElement.get_baskets_buffer.embedded_basket.data.value: {:?}",
                    data
                );

                trace!(
                    ";TBranchElement.get_baskets_buffer.embedded_basket.buf.key_lenght: {}",
                    key_lenght
                );

                BranchChunks::IrregularSized((0, data, header_bytes))
            }))
        } else {
            None
        };

        trace!(
            ";TBranchElement.get_baskets_buffer.before_zip.basket_seek:{:?}",
            self.branch.basket_seek
        );
        trace!(
            ";TBranchElement.get_baskets_buffer.before_zip.basket_bytes:{:?}",
            self.branch.basket_bytes
        );
        trace!(
            ";TBranchElement.get_baskets_buffer.before_zip.leaves:{:?}",
            leaves
        );

        let ret = izip!(
            &self.branch.basket_seek,
            &self.branch.basket_bytes,
            size_leaves,
            leaves
        )
        .filter(|(_start, len, _chunk_size, _leave)| **len > 0)
        .map(|(start, len, mut chunk_size, leave)| {
            let mut reader = self.branch.reader().as_ref().unwrap().clone();
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
            trace!(
                "chunk_size = {}, b.entry_size() = {}",
                chunk_size,
                b.entry_size()
            );
            trace!(
                "classname = {} streamer_type = {}",
                self.class_name(),
                self.streamer_type(),
            );

            match b.raw_data(&mut reader) {
                BasketData::TrustNEntries((n, buf)) => {
                    trace!("send ({n},{chunk_size},{:?})", buf);
                    BranchChunks::RegularSized((n, chunk_size, buf))
                }
                BasketData::UnTrustNEntries((n, buf, byte_offsets)) => match leave {
                    // In case of string, we have to use n
                    Leaf::C(_) => {
                        trace!("send ({n},{chunk_size},{:?})", buf);
                        BranchChunks::RegularSized((n, chunk_size, buf))
                    }
                    Leaf::Element(_) => {
                        let element = self.streamer();

                        trace!(
                            ";TBranchElement.get_baskets_buffer.UnTrustNEntries.element:{:?}",
                            element
                        );
                        trace!(
                            ";TBranchElement.get_baskets_buffer.UnTrustNEntries.streamer_type:{:?}",
                            self.streamer_type()
                        );

                        let header_bytes = streamer_type::header_bytes_from_type(
                            self.streamer_type(),
                            element,
                            self.class_name(),
                        );

                        trace!(";TBranchElement.get_baskets_buffer.UnTrustNEntries.header_bytes:{}", header_bytes);
                        // trace!("buf = {:?}", buf);

                        let byte_offsets = byte_offsets.iter().zip(byte_offsets.iter().skip(1));
                        // .collect();
                        // trace!("byte_offsets = {:?}", byte_offsets);
                        // trace!("buf = {:?}", buf);

                        let data: Vec<_> = byte_offsets
                            .map(|(start, stop)| {
                                let b = &buf[*start as usize..*stop as usize];
                                b.to_vec()
                            })
                            .collect();

                        trace!(
                            ";TBranchElement.get_baskets_buffer.UnTrustNEntries.data.value:{:?}",
                            data
                        );                        trace!(
                            ";TBranchElement.get_baskets_buffer.UnTrustNEntries.data.len:{:?}",
                            data.len()
                        );



                        // trace!("data = {:?}", data);

                        trace!("send ({n},{chunk_size},{:?})", data);
                        BranchChunks::IrregularSized((n, data, header_bytes))
                    }
                    _ => {
                        let n = buf.len() / chunk_size as usize;
                        trace!("send ({n},{chunk_size},{:?})", buf);
                        BranchChunks::RegularSized((n as i32, chunk_size, buf))
                    }
                },
            }
        });
        match embedded_basket {
            None => Box::new(ret),
            Some(before) => Box::new(before.chain(ret)),
        }
    }
}

impl Unmarshaler for TBranchElement {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let beg = r.pos();
        trace!(";TBranchElement.unmarshal.{beg}.call:{:?}", true);
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, crate::rvers::BRANCH_ELEMENT, self.class())?;

        trace!(
            ";TBranchElement.unmarshal.{beg}.pos.before.branch:{:?}",
            r.pos()
        );
        r.read_object(&mut self.branch)?;

        self.class_name = r.read_string()?.to_string();
        trace!(";TBranchElement.unmarshal.class_name:{:?}", self.class_name);

        // trace!("class = {}", self.class);

        if hdr.vers > 1 {
            self.parent = r.read_string()?.to_string();
            self.clones = r.read_string()?.to_string();
            self.chksum = r.read_i32()?;
        }
        if hdr.vers >= 10 {
            self.clsver = r.read_i16()?;
        } else {
            self.clsver = r.read_u32()? as i16;
        }

        trace!(";TBranchElement.unmarshal.hdr.vers:{:?}", hdr.vers);

        self.id = r.read_i32()?;
        self.btype = r.read_i32()?;
        self.stype = r.read_i32()?;

        trace!(";TBranchElement.unmarshal.id:{:?}", self.id);
        trace!(";TBranchElement.unmarshal.btype:{:?}", self.btype);
        trace!(";TBranchElement.unmarshal.stype:{:?}", self.stype);

        if hdr.vers > 1 {
            self.max = r.read_i32()?;

            let _bcount1 = r.read_object_any_into()?;
            assert!(_bcount1.is_none());
            let _bcount2 = r.read_object_any_into()?;
            assert!(_bcount2.is_none());

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

impl Marshaler for TBranchElement {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let beg = w.pos();
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        trace!(
            ";TBranchElement.marshal.{beg}.pos.before.branch:{:?}",
            w.pos()
        );
        w.write_object(&self.branch)?;
        w.write_string(&self.class_name)?;
        trace!(";TBranchElement.marshal.{beg}.class:{:?}", self.class());
        trace!(";TBranchElement.marshal.class_name:{:?}", self.class_name);
        trace!(";TBranchElement.marshal.chksum:{:?}", self.chksum);
        trace!(";TBranchElement.marshal.id:{:?}", self.id);
        trace!(";TBranchElement.marshal.btype:{:?}", self.btype);
        trace!(";TBranchElement.marshal.stype:{:?}", self.stype);
        w.write_string(&self.parent)?;
        w.write_string(&self.clones)?;
        w.write_i32(self.chksum)?;
        w.write_u16(self.clsver as u16)?;
        w.write_i32(self.id)?;
        w.write_i32(self.btype)?;
        w.write_i32(self.stype)?;
        w.write_i32(self.max)?;
        let obj = Object::default();
        // w.write_object_any(&obj)?;
        // w.write_object_any(&obj)?;
        w.write_object_nil()?;
        w.write_object_nil()?;
        w.set_header(hdr)?;
        Ok((w.pos() - beg))
    }
}

impl RVersioner for TBranchElement {
    fn rversion(&self) -> i16 {
        crate::rvers::BRANCH_ELEMENT
    }
}

factory_fn_register_impl!(TBranchElement, "TBranchElement");
