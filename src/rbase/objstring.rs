use crate::factory_all_for_register_impl;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::{RVersioner, Unmarshaler};
use crate::root::traits::Object;
use crate::{rbase, rvers};
use anyhow::ensure;

#[derive(Default)]
pub(crate) struct ObjString {
    obj: rbase::Object,
    str: String,
}

factory_all_for_register_impl!(ObjString, "TObjString");

impl RVersioner for ObjString {
    fn rversion() -> i16 {
        rvers::OBJ_STRING
    }
}

impl Unmarshaler for ObjString {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::OBJ_STRING,
            "rcont: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            Self::rversion()
        );

        r.read_object(&mut self.obj)?;
        self.str = r.read_string()?.to_string();

        r.check_header(&hdr)?;
        Ok(())
    }
}
