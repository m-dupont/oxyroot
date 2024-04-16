use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{RVersioner, Unmarshaler};
use crate::root::traits::Object;
use crate::{factory_all_for_register_impl, Marshaler};
use crate::{rbase, rvers};

#[derive(Default)]
pub struct TObjString {
    obj: rbase::Object,
    str: String,
}

impl ToString for TObjString {
    fn to_string(&self) -> String {
        self.str.clone()
    }
}

impl Marshaler for TObjString {
    fn marshal(&self, _w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        todo!()
    }
}
factory_all_for_register_impl!(TObjString, "TObjString", rvers::OBJ_STRING);
//
// impl RVersioner for ObjString {
//     fn rversion() -> i16 {
//         rvers::OBJ_STRING
//     }
// }

impl Unmarshaler for TObjString {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        if hdr.vers > rvers::OBJ_STRING {
            return Err(crate::rbytes::Error::VersionTooHigh {
                class: self.class().into(),
                version_read: hdr.vers,
                max_expected: Self::rversion(self),
            });
        }

        r.read_object(&mut self.obj)?;
        self.str = r.read_string()?.to_string();

        r.check_header(&hdr)?;
        Ok(())
    }
}
