use crate::factory_all_for_register_impl;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::root::traits::Object;
use crate::rvers;

use crate::rcolors::Color;

#[derive(Default)]
pub(crate) struct AttLine {
    color: Color,
    style: i16,
    width: i16,
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

factory_all_for_register_impl!(AttLine, "TAttLine");
