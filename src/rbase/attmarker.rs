use crate::rbase::{AttFill, AttLine};
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{RVersioner, Unmarshaler};
use crate::root::traits::Object;
use crate::rvers;
use crate::{factory_all_for_register_impl, Marshaler};

use crate::rcolors::Color;

pub(crate) struct AttMarker {
    color: Color,
    style: i16,
    width: f32,
}

impl Default for AttMarker {
    fn default() -> Self {
        AttMarker {
            color: Color::Int(1),
            style: 1,
            width: 1.0,
        }
    }
}

impl Unmarshaler for AttMarker {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        if hdr.vers > rvers::ATT_MARKER {
            return Err(crate::rbytes::Error::VersionTooHigh {
                class: self.class().into(),
                version_read: hdr.vers,
                max_expected: rvers::ATT_MARKER,
            });
        }

        self.color = Color::from_i16(r.read_i16()?);

        self.style = r.read_i16()?;
        self.width = r.read_f32()?;
        r.check_header(&hdr)?;
        Ok(())
    }
}

impl Marshaler for AttMarker {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let hdr = w.write_header(self.class(), Self::rversion(self))?;
        w.write_i16(self.color.to_i16())?;
        w.write_i16(self.style)?;
        w.write_f32(self.width)?;

        w.set_header(hdr)
    }
}

impl RVersioner for AttMarker {
    fn rversion(&self) -> i16 {
        rvers::ATT_MARKER
    }
}

factory_all_for_register_impl!(AttMarker, "TAttMarker");
