use crate::rbase::consts::{K_IS_ON_HEAP, K_IS_REFERENCED};
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;

#[derive(Debug)]
pub(crate) struct Object {
    id: u32,
    bits: u32,
}

impl Object {
    fn test_bits(&self, bits: u32) -> bool {
        self.bits & bits != 0
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
