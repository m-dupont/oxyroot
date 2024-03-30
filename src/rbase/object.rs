use crate::rbase::consts::{K_IS_ON_HEAP, K_IS_REFERENCED};
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{Marshaler, Unmarshaler};
use crate::root::traits;
use crate::rvers;

#[derive(Debug, Clone)]
pub(crate) struct Object {
    id: u32,
    bits: u32,
}

impl Object {
    fn test_bits(&self, bits: u32) -> bool {
        self.bits & bits != 0
    }
}
impl traits::Object for Object {
    fn class(&self) -> &'_ str {
        "TObject"
    }
}

impl Default for Object {
    fn default() -> Self {
        Object {
            id: 0x0,
            bits: 0x3000000,
        }
    }
}

impl Unmarshaler for Object {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        r.skip_version("")?;
        self.id = r.read_u32()?;
        self.bits = r.read_u32()?;

        self.bits |= K_IS_ON_HEAP;

        if self.test_bits(K_IS_REFERENCED) {
            r.read_u16()?;
        }

        Ok(())
    }
}

impl Marshaler for Object {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let n = w.pos();
        w.write_u16(rvers::OBJECT as u16)?;

        if self.test_bits(K_IS_REFERENCED) {
            let uid = self.id & 0xffffff;
            w.write_u32(uid)?;
            w.write_u32(self.bits)?;
            w.write_u16(0x0)?;
        } else {
            w.write_u32(self.id)?;
            w.write_u32(self.bits)?;
        }

        // trace!(";Object.marshal.buf:{:?}", w.p());
        Ok(w.pos() - n)
    }
}
