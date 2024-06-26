use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{RVersioner, Unmarshaler};
use crate::root::traits::Object;
use crate::rvers;
use crate::RBuffer;
use crate::{factory_all_for_register_impl, Marshaler};

use crate::rcolors::Color;

#[derive(Debug)]
pub(crate) struct AttFill {
    color: Color,
    style: i16,
}

impl Default for AttFill {
    fn default() -> Self {
        AttFill {
            color: Color::default(),
            style: 1001,
        }
    }
}

impl Unmarshaler for AttFill {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        if hdr.vers > rvers::ATT_FILL {
            return Err(crate::rbytes::Error::VersionTooHigh {
                class: self.class().into(),
                version_read: hdr.vers,
                max_expected: rvers::ATT_FILL,
            });
        }

        self.color = Color::from_i16(r.read_i16()?);

        self.style = r.read_i16()?;

        r.check_header(&hdr)?;

        Ok(())
    }
}

impl Marshaler for AttFill {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let hdr = w.write_header(self.class(), self.rversion())?;
        w.write_i16(self.color.to_i16())?;
        w.write_i16(self.style)?;
        w.set_header(hdr)
    }
}

impl RVersioner for AttFill {
    fn rversion(&self) -> i16 {
        rvers::ATT_FILL
    }
}

factory_all_for_register_impl!(AttFill, "TAttFill");
