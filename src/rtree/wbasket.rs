use crate::rbytes::wbuffer::WBuffer;
use crate::riofs::Key;
use crate::rtree::basket::Basket;
use crate::{riofs, Marshaler, Named, Object, RootFile};
use log::trace;
use riofs::Result;

#[derive(Debug)]
pub(crate) struct BasketBytesWritten {
    pub(crate) tot_bytes: i64,
    pub(crate) zip_bytes: i64,
}

#[derive(Debug)]
pub struct WBasket {
    pub(crate) basket: Basket,
    pub(crate) wbuf: WBuffer,
}

impl WBasket {
    pub(crate) fn new(b: Basket) -> Self {
        WBasket {
            basket: b,
            wbuf: WBuffer::new(0),
        }
    }

    pub(crate) fn update(&mut self, offset: i64) -> Result<()> {
        let beg = offset;
        trace!(";WBranch.write.wbasket.update.{beg}.call:{:?}", true);
        let offset = offset + self.basket.key().key_len() as i64;
        trace!(";WBranch.write.wbasket.update.{beg}.offset:{:?}", offset);
        let b = &mut self.basket;
        if !b.offsets().is_empty() {
            if b.nev_buf + 1 >= b.nev_size {
                let mut nev_size = 10;
                if nev_size < 2 * b.nev_size {
                    nev_size = 2 * b.nev_size;
                }
                b.nev_size = nev_size;
                let mut delta = b.offsets().len() as i32 - nev_size;
                if delta < 0 {
                    delta = -delta;
                }
                for _ in 0..delta {
                    b.offsets.push(0);
                }
            }
            b.offsets[b.nev_buf as usize] = offset as i32;
        }
        self.basket.nev_buf += 1;
        trace!(
            ";WBranch.write.wbasket.update.{beg}.nev_buf:{:?}",
            self.basket.nev_buf
        );
        trace!(
            ";WBranch.write.wbasket.update.{beg}.offsets:{:?}",
            self.basket.offsets
        );
        Ok(())
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

        let adjust = self.basket.key().rvers() <= 1000 && file.is_big_file();
        trace!(";WBasket.write_to_file.adjust:{:?}", adjust);
        trace!(
            ";WBasket.write_to_file.key.rvers:{:?}",
            self.basket.key().rvers()
        );
        trace!(
            ";WBasket.write_to_file.file.is_big_file:{:?}",
            file.is_big_file()
        );
        trace!(
            ";WBasket.write_to_file.basket.key().key_len():{:?}",
            self.basket.key().key_len()
        );
        self.basket.last = self.basket.key().key_len() + self.wbuf.len() as i32;
        trace!(";WBasket.write_to_file.b.last:{:?}", self.basket.last);

        if !self.basket.offsets.is_empty() {
            if adjust {
                for v in self.basket.offsets.iter_mut() {
                    *v += 8;
                }
            }
            self.wbuf.write_i32(self.basket.nev_buf + 1)?;
            self.wbuf
                .write_array_i32(&self.basket.offsets[0..self.basket.nev_buf as usize])?;
            self.wbuf.write_i32(0)?;
        }

        let key = self.basket.key();
        trace!(";WBasket.write_to_file.key.name:{:?}", key.name());
        trace!(";WBasket.write_to_file.key.title:{:?}", key.title());

        let key = Key::new_from_buffer(
            key.name().to_string(),
            key.title().to_string(),
            self.basket.class().to_string(),
            key.cycle() as i16,
            self.wbuf.p().clone(),
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

        self.wbuf.clear();

        Ok(ret)
    }
}

impl Marshaler for WBasket {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let beg = w.pos();

        w.write_object(self.basket.key())?;

        w.write_i16(self.basket.rvers)?;
        w.write_i32(self.basket.buf_size)?;

        if self.basket.iobits.0 != 0 {
            unimplemented!("WBasket.marshal.iobits");
        } else {
            w.write_i32(self.basket.nev_size)?;
        }

        w.write_i32(self.basket.nev_buf())?;
        w.write_i32(self.basket.last)?;

        trace!(";WBasket.marshal.buf.value:{:?}", w.p());

        // let must_gen_offset = self.basket.offsets().len() > 0
        //     && self.basket.nev_buf() > 0
        //     && (self.basket.iobits.0 & kGenerateOffsetMap) != 0
        //     && true;

        let must_gen_offset = false;

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
