use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{Marshaler, Unmarshaler};
use crate::riofs::consts::kStartBigFile;
use log::trace;

#[derive(Default)]
pub struct FreeSegments {
    pub(crate) first: i64,
    // first free word of segment
    pub(crate) last: i64, // last free word of segment
}

impl FreeSegments {
    pub(crate) fn new(first: i64, last: i64) -> Self {
        FreeSegments { first, last }
    }
    pub(crate) fn size_of(&self) -> i32 {
        if self.last > kStartBigFile {
            return 18;
        }
        10
    }
}

impl Unmarshaler for FreeSegments {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let vers = r.read_i16()?;

        let is_big_file = vers > 1000;
        let first = if is_big_file {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };
        let last = if is_big_file {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        self.first = first;
        self.last = last;
        Ok(())
    }
}

impl Marshaler for FreeSegments {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let beg = w.pos();
        trace!(";FreeSegments.marshal.beg:{:?}", beg);
        let mut vers = 1 as i16;
        if self.last > kStartBigFile {
            vers += 1000;
        }
        w.write_i16(vers)?;
        if vers > 1000 {
            w.write_i64(self.first)?;
            w.write_i64(self.last)?;
        } else {
            w.write_i32(self.first as i32)?;
            w.write_i32(self.last as i32)?;
        }

        let end = w.pos();

        trace!(";FreeSegments.marshal.buf.end:{:?}", w.p());
        Ok((end - beg) as i64)
    }
}

#[derive(Default)]
pub struct FreeList(Vec<FreeSegments>);

impl FreeList {
    pub(crate) fn vec(&mut self) -> &mut Vec<FreeSegments> {
        &mut self.0
    }
    pub fn append(&mut self, seg: FreeSegments) {
        self.0.push(seg)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
