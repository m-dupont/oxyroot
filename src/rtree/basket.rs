use crate::factory_fn_register_impl;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::riofs::file::RootFileReader;
use crate::root::traits::Named;

#[derive(Default)]
pub struct Basket {
    key: crate::riofs::Key,
    n_entry_buf: u32,
    last: i32,
    entry_size: i32,
}

impl Named for Basket {
    fn name(&self) -> &'_ str {
        self.key.name()
    }
}

/// Sometimes n_entry_buf in basket does not correspond to number of entries in Basket
/// The idea is to divide len of vec by the size chunck_size (in Tbranch)
pub(crate) enum BasketData {
    TrustNEntries((u32, Vec<u8>)),
    UnTrustNEntries((u32, Vec<u8>, Vec<i32>)),
}

impl Basket {
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
            return BasketData::UnTrustNEntries((self.n_entry_buf, data.to_vec(), byte_offsets));
        }

        BasketData::TrustNEntries((self.n_entry_buf, ret))
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
        self.entry_size
    }
}

impl Unmarshaler for Basket {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        r.read_object(&mut self.key)?;
        let _vers = r.read_i16()?;
        let _buf_size = r.read_u32()?;
        self.entry_size = r.read_i32()?;

        if self.entry_size < 0 {
            unimplemented!();
        }

        self.n_entry_buf = r.read_u32()?;
        self.last = r.read_i32()?;

        // todo!();
        Ok(())

        // todo!()
    }
}

factory_fn_register_impl!(Basket, "TBasket");
