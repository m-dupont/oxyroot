use crate::rbytes::{ensure_maximum_supported_version, RVersioner, WBuffer};
use crate::{factory_all_for_register_impl, rvers, Marshaler, Object, RBuffer, Unmarshaler};

#[derive(Default, Debug, Copy, Clone)]
pub struct TioFeatures(pub(crate) u8);

impl Unmarshaler for TioFeatures {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, crate::rvers::ROOT_IOFEATURES, self.class())?;

        let mut buf = [0_u8; 4];
        r.read_array_u8_into(&mut buf[..1])?;

        self.0 = if buf[0] != 0 {
            r.read_array_u8_into(&mut buf[1..])?;
            r.read_u8()?
        } else {
            0
        };

        r.check_header(&hdr)?;

        Ok(())

        // trace!("buf = {:?}", buf);
        //
        // todo!()
    }
}

impl Marshaler for TioFeatures {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let hdr = w.write_header(self.class(), Self::rversion(self))?;

        if self.0 != 0 {
            let buf = [0x1a, 0xa1, 0x2f, 0x10];
            w.write_array_u8(&buf)?;
        }
        w.write_u8(self.0)?;

        w.set_header(hdr)
    }
}

impl RVersioner for TioFeatures {
    fn rversion(&self) -> i16 {
        rvers::ROOT_IOFEATURES
    }
}

factory_all_for_register_impl!(TioFeatures, "TIOFeatures");
