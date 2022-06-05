use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::{RVersioner, Unmarshaler};
use crate::root::traits::Object;
use crate::{factory_register_impl, rbase, rvers};
use anyhow::ensure;
use log::{info, trace};

#[derive(Default)]
pub struct ObjString {
    obj: rbase::Object,
    str: String,
}

factory_register_impl!(ObjString, "TObjString");

impl RVersioner for ObjString {
    fn rversion() -> i16 {
        rvers::ObjString
    }
}

impl Unmarshaler for ObjString {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        info!("StreamerSTL:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::ObjString,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            Self::rversion()
        );

        r.read_object(&mut self.obj)?;
        self.str = r.read_string()?.to_string();

        trace!("str = {}", self.str);

        r.check_header(&hdr)?;
        Ok(())
    }
}
