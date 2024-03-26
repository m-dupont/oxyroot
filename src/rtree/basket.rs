use crate::rbase::AttFill;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{ensure_maximum_supported_version, Error, Unmarshaler};
use crate::riofs::file::RootFileReader;
use crate::root::traits::Named;
use crate::rtree::branch::wbranch::WBranch;
use crate::rtree::tree::{TioFeatures, WriterTree};
use crate::rtypes::FactoryItemRead;
use crate::{factory_fn_register_impl, rbytes, rvers, Branch, Marshaler, Object, RootFile};
use log::trace;

#[derive(Debug)]
pub struct Basket {
    key: crate::riofs::Key,
    pub(crate) nev_buf: i32,
    pub(crate) last: i32,
    pub(crate) nev_size: i32,
    pub(crate) buf_size: i32,
    pub(crate) offsets: Vec<i32>,
    header: bool,
    pub(crate) rvers: i16,
    pub(crate) iobits: TioFeatures,
}

impl Default for Basket {
    fn default() -> Self {
        Basket {
            key: crate::riofs::Key::default(),
            nev_buf: 0,
            last: 0,
            nev_size: 0,
            buf_size: 0,
            offsets: Vec::new(),
            header: false,
            rvers: rvers::BASKET,
            iobits: TioFeatures::default(),
        }
    }
}

impl Named for Basket {
    fn name(&self) -> &'_ str {
        self.key.name()
    }
}

impl From<Box<dyn FactoryItemRead>> for Basket {
    fn from(f: Box<dyn FactoryItemRead>) -> Self {
        if let Ok(b) = f.downcast::<Basket>() {
            return *b;
        }
        panic!("expecting Basket")
    }
}

/// Sometimes n_entry_buf in basket does not correspond to number of entries in Basket
/// The idea is to divide len of vec by the size chunck_size (in Tbranch)
pub(crate) enum BasketData {
    TrustNEntries((i32, Vec<u8>)),
    UnTrustNEntries((i32, Vec<u8>, Vec<i32>)),
}

impl Basket {
    pub(crate) fn new_from_branch(
        b: &Branch,
        cycle: i16,
        buf_size: i32,
        offset_len: i32,
        tree: &WriterTree,
        f: &RootFile,
    ) -> Self {
        let mut basket = Basket {
            key: crate::riofs::Key::new_key_for_basket_internal(
                b.name().to_string(),
                tree.name().to_string(),
                "TBasket".to_string(),
                cycle,
                f,
            ),
            nev_buf: 0,
            last: 0,
            nev_size: offset_len,
            buf_size,
            header: true,
            offsets: Vec::with_capacity(1000),
            ..Default::default()
        };
        // trace!(";Basket.new_from_branch.basket.value:{:?}", &basket);
        trace!(
            ";Basket.new_from_branch.basket.nev_size:{:?}",
            basket.nev_size
        );
        trace!(
            ";Basket.new_from_branch.basket.offsets:{:?}",
            basket.offsets
        );
        basket.offsets.resize(basket.nev_size as usize, 0);
        trace!(
            ";Basket.new_from_branch.basket.offsets:{:?}",
            basket.offsets
        );
        trace!(";Basket.new_from_branch.tree.title:{:?}", tree.title());
        basket
    }

    pub(crate) fn raw_data(&self, file: &mut RootFileReader) -> BasketData {
        let ret = self.key.bytes(file, None).unwrap();

        if self.border() != self.uncompressed_bytes() {
            let (data, byte_offsets) = ret.split_at(self.border() as usize);

            let mut bb: Vec<i32> = Vec::with_capacity(byte_offsets.len() / 4);

            byte_offsets
                .chunks(4)
                .map(|x| i32::from_be_bytes(x.try_into().unwrap()) - self.key.key_len())
                .skip(1)
                .for_each(|x| bb.push(x));

            let mut byte_offsets = bb;
            let last = byte_offsets.len() - 1;
            byte_offsets[last] = self.border();
            return BasketData::UnTrustNEntries((self.nev_buf, data.to_vec(), byte_offsets));
        }

        BasketData::TrustNEntries((self.nev_buf, ret))
    }

    ///         If this ``TBasket`` is embedded within its ``TBranch`` (i.e. must be
    //         deserialized as part of the ``TBranch``), then ``is_embedded`` is True.
    //
    //         If this ``TBasket`` is a free-standing object, then ``is_embedded`` is
    //         False.
    pub fn is_embedded(&self) -> bool {
        self.key.n_bytes() <= self.key.key_len()
    }

    pub fn uncompressed_bytes(&self) -> i32 {
        self.key.obj_len()
    }
    // pub fn compressed_bytes(&self) -> i32 {
    //     self.key.n_bytes() - self.key.key_len()
    // }

    pub fn border(&self) -> i32 {
        self.last - self.key.key_len()
    }
    pub fn entry_size(&self) -> i32 {
        self.nev_size
    }
    pub fn nev_buf(&self) -> i32 {
        self.nev_buf
    }

    pub fn key(&self) -> &crate::riofs::Key {
        &self.key
    }
    pub fn offsets(&self) -> &Vec<i32> {
        &self.offsets
    }
    pub fn set_key(&mut self, key: crate::riofs::Key) {
        self.key = key;
    }
}

impl Unmarshaler for Basket {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let _beg = r.pos();
        // if (_beg == 868) {
        //     panic!(";rbuffer.ReadObjectAny.beg: {}", _beg);
        // }
        trace!(";Basket.unmarshal.beg: {}", _beg);
        trace!(";Basket.unmarshal.{}.beg: {}", _beg, _beg);
        r.read_object(&mut self.key)?;

        if self.class() != "TBasket" {
            return Err(Error::WrongClass {
                expected: "TBasket".to_owned(),
                found: self.class().to_owned(),
            });
        }

        trace!(";Basket.unmarshal.{}.pos_after_header: {}", _beg, r.pos());

        let vers = r.read_i16()?;

        ensure_maximum_supported_version(vers, rvers::BASKET, self.class())?;

        self.buf_size = r.read_i32()?;
        self.nev_size = r.read_i32()?;

        if self.nev_size < 0 {
            unimplemented!();
        }

        self.nev_buf = r.read_i32()?;
        self.last = r.read_i32()?;

        let flag = r.read_u8()?;

        trace!(";Basket.unmarshal.{}.last: {}", _beg, self.last);
        trace!(";Basket.unmarshal.{}.buf_size: {}", _beg, self.buf_size);
        trace!(";Basket.unmarshal.{}.nev_size: {}", _beg, self.nev_size);
        trace!(";Basket.unmarshal.{}.nev_buf: {}", _beg, self.nev_buf);
        trace!(";Basket.unmarshal.{}.fNbytes: {}", _beg, self.key.n_bytes());
        trace!(";Basket.unmarshal.{}.flag: {}", _beg, flag);
        trace!(
            ";Basket.unmarshal.{}.is_embedded: {}",
            _beg,
            self.is_embedded()
        );

        if self.last > self.buf_size {
            self.buf_size = self.last;
        }

        let mut must_gen_offsets = false;
        let flag = if flag >= 80 {
            must_gen_offsets = true;
            flag - 80
        } else {
            flag
        };

        trace!(";Basket.unmarshal.{}.flag: {}", _beg, flag);
        trace!(
            ";Basket.unmarshal.{}.must_read_offset: {}",
            _beg,
            !must_gen_offsets && flag != 0 && (flag % 10 != 2)
        );

        if !must_gen_offsets && flag != 0 && (flag % 10 != 2) {
            if self.nev_buf > 0 {
                let n = r.read_i32()?;
                self.offsets.reserve(n as usize);
                for _ in 0..n {
                    self.offsets.push(r.read_i32()?);
                }
                if 20 < flag && flag < 40 {
                    unimplemented!();
                }
            }
            if flag > 40 {
                unimplemented!();
            }
        } else if must_gen_offsets {
            self.offsets.clear();
            if flag <= 40 {
                panic!(";Basket.unmarshal.{}.flag: {} <= 40", _beg, flag);
            }
        }

        trace!(";Basket.unmarshal.{}.offsets.vec: {:?}", _beg, self.offsets);
        trace!(
            ";Basket.unmarshal.{}.offsets.len: {:?}",
            _beg,
            self.offsets.len()
        );

        if flag == 1 || flag > 10 {
            let mut sz = self.last;
            if vers <= 1 {
                sz = r.read_i32()?;
            }
            let pos = r.pos();
            trace!(";Basket.unmarshal.{}.pos_before_buffer: {}", _beg, pos);
            trace!(";Basket.unmarshal.{}.sz: {}", _beg, sz);

            let buf = r.read_array_u8(sz as usize)?;
            trace!(";Basket.unmarshal.{}.buf.len: {}", _beg, buf.len());

            self.key.set_buffer(buf.to_vec(), true);
            let pos = r.pos();
            trace!(";Basket.unmarshal.{}.pos_after_buffer: {}", _beg, pos);
        }

        trace!(
            ";Basket.unmarshal.{}.key_lenght: {}",
            _beg,
            self.key.key_len()
        );
        trace!(";Basket.unmarshal.{}.border: {}", _beg, self.border());

        trace!(
            ";Basket.unmarshal.{}.key_obj_len: {}",
            _beg,
            self.key.obj_len()
        );

        // todo!();
        Ok(())

        // todo!()
    }
}

impl Marshaler for Basket {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        todo!()
    }
}
factory_fn_register_impl!(Basket, "TBasket");
