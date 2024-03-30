use crate::factory_all_for_register_impl;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{RVersioner, Unmarshaler};
use crate::root::traits::Object;
use crate::{rvers, Marshaler};

use crate::rcolors::{Color, ColorNamed};

pub(crate) struct AttLine {
    color: Color,
    style: i16,
    width: i16,
}

impl Default for AttLine {
    fn default() -> Self {
        AttLine {
            color: Color::Int(ColorNamed::Blue as i16 + 2),
            style: 1,
            width: 1,
        }
    }
}

impl Unmarshaler for AttLine {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        if hdr.vers > rvers::ATT_LINE {
            return Err(crate::rbytes::Error::VersionTooHigh {
                class: self.class().into(),
                version_read: hdr.vers,
                max_expected: rvers::ATT_LINE,
            });
        }

        self.color = Color::from_i16(r.read_i16()?);

        self.style = r.read_i16()?;
        self.width = r.read_i16()?;

        r.check_header(&hdr)?;
        Ok(())
    }
}

impl Marshaler for AttLine {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        w.write_i16(self.color.to_i16())?;
        w.write_i16(self.style)?;
        w.write_i16(self.width)?;

        w.set_header(hdr)
    }
}

impl RVersioner for AttLine {
    fn rversion(&self) -> i16 {
        rvers::ATT_LINE
    }
}

factory_all_for_register_impl!(AttLine, "TAttLine");
