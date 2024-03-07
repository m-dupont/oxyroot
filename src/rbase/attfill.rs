use crate::factory_all_for_register_impl;
use crate::rbytes::Unmarshaler;
use crate::root::traits::Object;
use crate::rvers;
use crate::RBuffer;

use crate::rcolors::Color;

#[derive(Default, Debug)]
pub(crate) struct AttFill {
    color: Color,
    style: i16,
    _width: i16,
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

factory_all_for_register_impl!(AttFill, "TAttFill");
