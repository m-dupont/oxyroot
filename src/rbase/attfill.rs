use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::Unmarshaler;
use crate::root::traits::Object;
use crate::rvers;
use crate::RBuffer;
use crate::{factory_all_for_register_impl, Marshaler};

use crate::rcolors::Color;

#[derive(Debug)]
pub(crate) struct AttFill {
    color: Color,
    style: i16,
    _width: i16,
}

impl Default for AttFill {
    fn default() -> Self {
        AttFill {
            color: Color::default(),
            style: 1001,
            _width: 0,
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
        todo!()
    }
}

factory_all_for_register_impl!(AttFill, "TAttFill");
