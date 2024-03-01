use crate::factory_all_for_register_impl;
use crate::rbytes::Unmarshaler;
use crate::root::traits::Object;
use crate::rvers;
use crate::RBuffer;
use anyhow::ensure;

use crate::rcolors::Color;

#[derive(Default)]
pub(crate) struct AttFill {
    color: Color,
    style: i16,
    _width: i16,
}

impl Unmarshaler for AttFill {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure!(
            hdr.vers <= rvers::ATT_FILL,
            "rbase: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::ATT_FILL
        );

        self.color = Color::from_i16(r.read_i16()?);

        self.style = r.read_i16()?;

        r.check_header(&hdr)?;

        Ok(())
    }
}

factory_all_for_register_impl!(AttFill, "TAttFill");
