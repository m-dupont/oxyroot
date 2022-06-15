use crate::factotry_all_for_register_impl;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::root::traits::Object;
use crate::rvers;
use anyhow::ensure;
use log::trace;

use crate::rcolors::Color;

#[derive(Default)]
pub struct AttMarker {
    color: Color,
    style: i16,
    width: f32,
}

impl Unmarshaler for AttMarker {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("AttMarker:unmarshal");
        let hdr = r.read_header(self.class())?;

        ensure!(
            hdr.vers <= rvers::AttMarker,
            "rbase: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::AttMarker
        );

        self.color = Color::from_i16(r.read_i16()?);

        self.style = r.read_i16()?;
        self.width = r.read_f32()?;
        trace!("width = {:?}", self.width);
        r.check_header(&hdr)?;
        Ok(())
    }
}

factotry_all_for_register_impl!(AttMarker, "TAttMarker");
