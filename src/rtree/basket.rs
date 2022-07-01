use crate::factotry_fn_register_impl;
use crate::file::RootFileReader;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::root::traits::Named;
use log::trace;

#[derive(Default)]
pub struct Basket {
    key: crate::riofs::Key,
    n_entry_buf: u32,
    last: i32,
}

impl Named for Basket {
    fn name(&self) -> &'_ str {
        &self.key.name()
    }
}

/// Sometimes n_entry_buf in basket does not correspond to number of entries in Basket
/// The idea is to divide len of vec by the size chunck_size (in Tbranch)
pub(crate) enum BasketData {
    TrustNEntries((u32, Vec<u8>)),
    UnTrustNEntries((u32, Vec<u8>)),
}

impl Basket {
    pub(crate) fn raw_data(&self, file: &mut RootFileReader) -> BasketData {
        trace!("basket:  = {}", self.name());
        trace!(
            "basket: objlen = {} border = {}",
            self.key.obj_len(),
            self.border()
        );

        let mut ret = self.key.bytes(file, None).unwrap();

        if self.border() != self.uncompressed_bytes() {
            ret = ret[0..self.border() as usize].to_vec();
            trace!("new len buf = {}", ret.len());
            return BasketData::UnTrustNEntries((self.n_entry_buf, ret));
        }

        BasketData::TrustNEntries((self.n_entry_buf, ret))
    }

    pub fn uncompressed_bytes(&self) -> i32 {
        self.key.obj_len()
    }

    pub fn border(&self) -> i32 {
        self.last - self.key.key_len()
    }
}

impl Unmarshaler for Basket {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("BASKET:unmarshal, name = {}", self.name());

        r.read_object(&mut self.key)?;
        let _vers = r.read_i16()?;
        let _buf_size = r.read_u32()?;
        let _entry_size = r.read_i32()?;

        if _entry_size < 0 {
            unimplemented!();
        }

        trace!("_buf_size = {} _entry_size = {}", _buf_size, _entry_size);

        self.n_entry_buf = r.read_u32()?;
        self.last = r.read_i32()?;

        trace!("n_entry_buf = {} ", self.n_entry_buf);
        // trace!("_last = {} ", _last);
        // trace!(" = {} ", _last);
        trace!("border = {} ", self.border());

        // todo!();
        Ok(())

        // todo!()
    }
}

factotry_fn_register_impl!(Basket, "TBasket");
