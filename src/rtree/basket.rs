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

impl Basket {
    pub fn raw_data(&self, file: &mut RootFileReader) -> (u32, Vec<u8>) {
        (self.n_entry_buf, self.key.bytes(file, None).unwrap())
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
