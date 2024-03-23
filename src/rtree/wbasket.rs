use crate::rbytes::wbuffer::WBuffer;
use crate::riofs::Key;
use crate::rtree::basket::Basket;
use crate::{riofs, Branch, Marshaler, Named, Object, RootFile};
use log::trace;
use riofs::Result;
use std::default;

#[derive(Debug)]
pub(crate) struct BasketBytesWritten {
    pub(crate) tot_bytes: i64,
    pub(crate) zip_bytes: i64,
}

#[derive(Debug)]
pub struct WBasket<T>
where
    T: Marshaler,
{
    pub(crate) basket: Basket,
    phantom: std::marker::PhantomData<T>,
    // wbuf: WBuffer,
}

impl<T> WBasket<T>
where
    T: Marshaler,
{
    pub(crate) fn new(b: Basket) -> Self {
        WBasket {
            basket: b,
            phantom: std::marker::PhantomData,
            // wbuf: WBuffer::new(0),
        }
    }

    pub(crate) fn write_to_file(&mut self, file: &mut RootFile) -> Result<BasketBytesWritten> {
        trace!(";WBasket.write_to_file.call:{:?}", true);
        trace!(
            ";WBasket.write_to_file.basket.key.rvers:{:?}",
            self.basket.key().rvers()
        );
        trace!(
            ";WBasket.write_to_file.file.is_big_fil:{:?}",
            file.is_big_file()
        );

        let adjust = !(self.basket.key().rvers() > 1000) && file.is_big_file();
        trace!(";WBasket.write_to_file.adjust:{:?}", adjust);
        trace!(
            ";WBasket.write_to_file.basket.key().key_len():{:?}",
            self.basket.key().key_len()
        );
        self.basket.last = self.basket.key().key_len() as i32;
        trace!(";WBasket.write_to_file.b.last:{:?}", adjust);

        let key = self.basket.key();

        trace!(";WBasket.write_to_file.key.name:{:?}", key.name());
        trace!(";WBasket.write_to_file.key.title:{:?}", key.title());

        let key = Key::new_from_buffer(
            key.name().to_string(),
            key.title().to_string(),
            self.basket.class().to_string(),
            key.cycle() as i16,
            Vec::new(),
            file,
        )?;

        let n_bytes = key.key_len() + key.obj_len();
        trace!(";WBasket.write_to_file.n_bytes:{:?}", n_bytes);

        self.basket.set_key(key);

        if adjust {
            self.basket.last += 8;
        }

        let n_bytes = self.basket.key().key_len() + self.basket.key().obj_len();
        trace!(";WBasket.write_to_file.n_bytes:{:?}", n_bytes);
        let mut buf = WBuffer::new(self.basket.key().key_len() as u32);
        self.marshal(&mut buf)?;

        trace!(";WBasket.write_to_file.buf.value:{:?}", buf.p());
        trace!(
            ";WBasket.write_to_file.b.key.SeekKey:{:?}",
            self.basket.key().seek_key()
        );
        trace!(
            ";WBasket.write_to_file.b.key.buf.wpos:{:?}",
            self.basket.key().seek_key() as u64 + self.basket.key().key_len() as u64
        );

        file.write_at(buf.p(), self.basket.key().seek_key() as u64)?;
        file.write_at(
            self.basket.key().buffer(),
            self.basket.key().seek_key() as u64 + self.basket.key().key_len() as u64,
        )?;

        let ret = BasketBytesWritten {
            tot_bytes: n_bytes as i64,
            zip_bytes: self.basket.key().n_bytes() as i64,
        };

        Ok(ret)
    }
}

impl<T> Marshaler for WBasket<T>
where
    T: Marshaler,
{
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let beg = w.pos();

        w.write_object(self.basket.key())?;

        w.write_i16(self.basket.rvers)?;
        w.write_i32(self.basket.buf_size as i32)?;

        if self.basket.iobits.0 != 0 {
            unimplemented!("WBasket.marshal.iobits");
        } else {
            w.write_i32(self.basket.nev_size)?;
        }

        w.write_i32(self.basket.nev_buf())?;
        w.write_i32(self.basket.last)?;

        trace!(";WBasket.marshal.buf.value:{:?}", w.p());

        const kGenerateOffsetMap: u8 = 0;

        let must_gen_offset = self.basket.offsets().len() > 0
            && self.basket.nev_buf() > 0
            && (self.basket.iobits.0 & kGenerateOffsetMap) != 0
            && true;

        trace!(";WBasket.marshal.must_gen_offset:{:?}", must_gen_offset);
        let mut flag = 0;
        if must_gen_offset {
            flag = 80;
        }
        w.write_u8(flag)?;
        trace!(";WBasket.marshal.buf.value:{:?}", w.p());

        let n = w.pos() - beg;
        trace!(";WBasket.marshal.n:{:?}", n);

        Ok(n)

        //
        // self.basket.key().marshal(w)?;
    }
}
