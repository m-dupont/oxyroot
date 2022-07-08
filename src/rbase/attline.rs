use crate::factory_all_for_register_impl;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::root::traits::Object;
use crate::rvers;
use anyhow::ensure;
use log::trace;

use crate::rcolors::Color;

#[derive(Default)]
pub struct AttLine {
    color: Color,
    style: i16,
    width: i16, // rvers: i16,
                // named: rbase::NAMED,
}

impl Unmarshaler for AttLine {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("ATT_LINE:unmarshal");
        let hdr = r.read_header(self.class())?;

        ensure!(
            hdr.vers <= rvers::ATT_LINE,
            "rbase: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::ATT_LINE
        );

        self.color = Color::from_i16(r.read_i16()?);

        trace!("color = {:?}", self.color);
        self.style = r.read_i16()?;
        self.width = r.read_i16()?;

        r.check_header(&hdr)?;
        Ok(())
    }
}

factory_all_for_register_impl!(AttLine, "TAttLine");
