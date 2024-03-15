use crate::rbytes::consts::kByteCountMask;
use crate::rbytes::{Header, Result};
use crate::rtypes::FactoryItem;
use log::trace;
use std::collections::HashMap;
use std::mem::size_of;

#[derive(Default, Debug)]
struct Wbuff {
    p: Vec<u8>,
    c: usize,
}

impl Wbuff {
    fn write_array_u8(&mut self, p0: &[u8]) -> Result<()> {
        self.p.extend_from_slice(p0);
        self.c += p0.len();
        Ok(())
    }

    fn write_in_place(&mut self, p0: &[u8], pos: usize) -> Result<()> {
        assert!(pos + p0.len() <= self.p.len());
        self.p[pos..pos + p0.len()].copy_from_slice(p0);
        Ok(())
    }
}

#[derive(Debug)]
pub struct WBuffer {
    w: Wbuff,
    offset: u32,
    // refs: HashMap<i64, crate::rbytes::rbuffer::RBufferRefsItem>,
}

impl WBuffer {
    pub(crate) fn write_array_u8(&mut self, p0: &[u8]) -> Result<()> {
        self.w.write_array_u8(p0)
    }

    pub(crate) fn offset(&self) -> u32 {
        self.offset
    }

    pub(crate) fn write_i32(&mut self, p0: i32) -> Result<()> {
        self.w.write_array_u8(&p0.to_be_bytes())
    }

    pub(crate) fn write_i64(&mut self, p0: i64) -> Result<()> {
        self.w.write_array_u8(&p0.to_be_bytes())
    }

    pub(crate) fn write_u32(&mut self, p0: u32) -> Result<()> {
        self.w.write_array_u8(&p0.to_be_bytes())
    }

    pub(crate) fn write_u64(&mut self, p0: u64) -> Result<()> {
        self.w.write_array_u8(&p0.to_be_bytes())
    }

    pub(crate) fn write_f32(&mut self, p0: f32) -> Result<()> {
        self.w.write_array_u8(&p0.to_be_bytes())
    }

    pub(crate) fn write_f64(&mut self, p0: f64) -> Result<()> {
        self.w.write_array_u8(&p0.to_be_bytes())
    }

    pub(crate) fn write_i16(&mut self, p0: i16) -> Result<()> {
        self.w.write_array_u8(&p0.to_be_bytes())
    }

    pub(crate) fn write_u16(&mut self, p0: u16) -> Result<()> {
        self.w.write_array_u8(&p0.to_be_bytes())
    }

    pub(crate) fn write_u8(&mut self, p0: u8) -> Result<()> {
        self.w.write_array_u8(&p0.to_be_bytes())
    }

    pub(crate) fn write_i8(&mut self, p0: i8) -> Result<()> {
        self.w.write_array_u8(&p0.to_be_bytes())
    }

    pub(crate) fn write_string(&mut self, p0: &str) -> Result<()> {
        let len = p0.len() as i32;
        let bytes = p0.as_bytes();
        if len < 255 {
            self.write_u8(len as u8)?;
            if len > 0 {
                self.write_array_u8(bytes)?;
            }
            return Ok(());
        }

        self.write_u8(255)?;
        self.write_u32(len as u32)?;
        self.write_array_u8(bytes)
    }

    pub(crate) fn write_header(&mut self, class: &str, vers: i16) -> crate::rbytes::Result<Header> {
        let hdr = Header {
            _name: String::from(class),
            pos: self.pos(),
            vers,
            ..Default::default()
        };
        self.write_u32(0)?;
        self.write_u16(vers as u16)?;
        Ok(hdr)
    }

    pub(crate) fn set_header(&mut self, hdr: Header) -> Result<()> {
        let cur = self.pos();
        trace!(";WBuffer.set_header.{cur}.cur:{:?}", cur);

        trace!(";WBuffer.set_header.{cur}.hdr.pos:{:?}", hdr.pos);
        let cnt = cur - hdr.pos - 4;
        let w = (cnt | kByteCountMask) as u32;
        let w = w.to_be_bytes();
        self.write_in_place(&w, hdr.pos as usize)?;
        trace!(";WBuffer.set_header.{cur}.buf:{:?}", self.p());
        Ok(())
    }

    pub(crate) fn write_in_place(&mut self, p0: &[u8], pos: usize) -> Result<()> {
        let pos = pos - self.offset as usize;
        self.w.write_in_place(p0, pos)
    }

    pub(crate) fn write_object_any(&mut self, obj: &Box<dyn FactoryItem>) -> Result<()> {
        todo!()
    }
}

impl WBuffer {
    pub fn new(offset: u32) -> Self {
        WBuffer {
            w: Wbuff::default(),
            offset,
            // refs: HashMap::new(),
        }
    }

    pub fn with_size(offset: u32, size: usize) -> Self {
        let mut w = WBuffer {
            w: Wbuff {
                p: Vec::with_capacity(size),
                c: 0,
            },
            offset,
            // refs: HashMap::new(),
        };
        w.w.p.resize(size, 0);
        w
    }

    pub fn buffer(self) -> Vec<u8> {
        self.w.p
    }
    pub(crate) fn p(&self) -> &Vec<u8> {
        &self.w.p
    }

    pub fn pos(&self) -> i64 {
        self.w.c as i64 + self.offset as i64
    }
}
