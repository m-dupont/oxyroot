use crate::rbytes::consts::{kByteCountMask, kClassMask, kMapOffset, kNewClassTag};
use crate::rbytes::{Header, Result};
use crate::rtypes::factory::FactoryItemWrite;
use crate::Marshaler;
use log::trace;
use std::collections::HashMap;

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
    refs_p: HashMap<usize, i64>, // refs: HashMap<i64, crate::rbytes::rbuffer::RBufferRefsItem>,
    refs_s: HashMap<String, i64>, // refs: HashMap<i64, crate::rbytes::rbuffer::RBufferRefsItem>,
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
        let p0 = p0 as u64;
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

    pub(crate) fn write_bool(&mut self, p0: bool) -> Result<()> {
        self.w.write_array_u8(&(p0 as u8).to_be_bytes())
    }

    pub(crate) fn write_array_i64(&mut self, p0: &[i64]) -> Result<()> {
        for v in p0 {
            self.write_i64(*v)?;
        }
        Ok(())
    }

    pub(crate) fn write_array_i32(&mut self, p0: &[i32]) -> Result<()> {
        for v in p0 {
            self.write_i32(*v)?;
        }
        Ok(())
    }

    pub(crate) fn write_array_i16(&mut self, p0: &[i16]) -> Result<()> {
        for v in p0 {
            self.write_i16(*v)?;
        }
        Ok(())
    }

    pub(crate) fn write_array_i8(&mut self, p0: &[i8]) -> Result<()> {
        for v in p0 {
            self.write_i8(*v)?;
        }
        Ok(())
    }

    pub(crate) fn write_array_u64(&mut self, p0: &[u64]) -> Result<()> {
        for v in p0 {
            self.write_u64(*v)?;
        }
        Ok(())
    }

    pub(crate) fn write_array_u32(&mut self, p0: &[u32]) -> Result<()> {
        for v in p0 {
            self.write_u32(*v)?;
        }
        Ok(())
    }

    pub(crate) fn write_array_u16(&mut self, p0: &[u16]) -> Result<()> {
        for v in p0 {
            self.write_u16(*v)?;
        }
        Ok(())
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

    pub(crate) fn set_header(&mut self, hdr: Header) -> Result<i64> {
        let cur = self.pos();
        // trace!(";WBuffer.set_header.{cur}.cur:{:?}", cur);

        // trace!(";WBuffer.set_header.{cur}.hdr.pos:{:?}", hdr.pos);
        let cnt = cur - hdr.pos - 4;
        let w = (cnt | kByteCountMask) as u32;
        let w = w.to_be_bytes();
        self.write_in_place(&w, hdr.pos as usize)?;
        // trace!(";WBuffer.set_header.{cur}.buf:{:?}", self.p());
        Ok(cnt + 4)
    }

    pub(crate) fn write_in_place(&mut self, p0: &[u8], pos: usize) -> Result<()> {
        let pos = pos - self.offset as usize;
        self.w.write_in_place(p0, pos)
    }

    pub(crate) fn write_object_nil(&mut self) -> Result<i64> {
        self.write_u32(0)?;
        Ok(0)
    }

    pub(crate) fn write_object_any(
        &mut self,
        obj: &(dyn FactoryItemWrite + '_),
        addr: usize,
    ) -> Result<i64> {
        let beg = self.pos();
        let len = self.w.p.len() - 1;
        trace!(
            ";WBuffer.write_object_any.a{beg}.buf.value:{:?}",
            &self.p()[len..]
        );
        let pos = self.pos();
        self.write_i32(0)?;
        trace!(";WBuffer.write_object_any.a{beg}.addr:{:?}", addr);
        let bcnt = self.write_class(pos, obj, addr)?;
        trace!(";WBuffer.write_object_any.a{beg}.bcnt:{:?}", bcnt);
        trace!(
            ";WBuffer.write_object_any.a{beg}.buf.value:{:?}",
            &self.p()[len..]
        );
        let end = self.pos();
        trace!(";WBuffer.write_object_any.a{beg}.end:{:?}", end);
        trace!(";WBuffer.write_object_any.a{beg}.wpos:{:?}", pos);
        self.write_in_place(&(bcnt as u32).to_be_bytes(), pos as usize)?;
        trace!(
            ";WBuffer.write_object_any.a{beg}.buf.value:{:?}",
            &self.p()[len..]
        );
        trace!(
            ";WBuffer.write_object_any.a{beg}.buf.len:{:?}",
            &self.p()[len..].len()
        );
        trace!(
            ";WBuffer.write_object_any.a{beg}.buf.total_len:{:?}",
            &self.p().len()
        );
        Ok(self.pos() - beg)
    }

    pub(crate) fn len(&self) -> usize {
        self.w.p.len()
    }

    pub(crate) fn write_class(
        &mut self,
        beg: i64,
        obj: &dyn FactoryItemWrite,
        addr: usize,
    ) -> Result<i64> {
        let len = self.w.p.len() - 1;
        let class = obj.class();
        let start = self.pos();
        trace!(";WBuffer.write_class.begs:{:?}", beg);
        trace!(";WBuffer.write_class.a{beg}.class:{:?}", class);

        trace!(";WBuffer.write_class.a{beg}.addr:{:?}", addr);

        if let Some(ref64) = self.refs_p.get(&addr) {
            trace!(";WBuffer.write_class.a{beg}.ref64:{:?}", ref64);
            trace!(";WBuffer.write_class.a{beg}.ref64_by_ptr:{:?}", true);
            self.write_u32(*ref64 as u32)?;
            let bcnt = self.pos() - start;
            trace!(";WBuffer.write_class.a{beg}.bcnt:{:?}", bcnt);
            Ok(bcnt | kByteCountMask)
        } else if let Some(ref64) = self.refs_s.get(class) {
            trace!(";WBuffer.write_class.a{beg}.ref64_by_class:{:?}", true);
            self.write_u32(*ref64 as u32 | kClassMask as u32)?;
            trace!(
                ";WBuffer.write_class.a{beg}.pos.before_marshall:{:?}",
                self.pos()
            );
            obj.marshal(self)?;
            trace!(
                ";WBuffer.write_class.a{beg}.pos.after_marshall:{:?}",
                self.pos()
            );
            self.refs_p.insert(addr, beg + kMapOffset);
            let bcnt = self.pos() - start;
            let bcnt = bcnt | kByteCountMask;
            trace!(";WBuffer.write_class.a{beg}.bcnt:{:?}", bcnt);
            Ok(bcnt)
        } else {
            self.write_u32(kNewClassTag.try_into()?)?;
            trace!(
                ";WBuffer.write_class.a{beg}.buf.value:{:?}",
                &self.p()[len..]
            );
            self.write_cstring(class)?;
            trace!(
                ";WBuffer.write_class.a{beg}.buf.value:{:?}",
                &self.p()[len..]
            );
            let val = (start + kMapOffset) | kClassMask;
            trace!(";WBuffer.write_class.a{beg}.class_value:{:?}", val);

            self.refs_s.insert(class.to_string(), val);
            trace!(
                ";WBuffer.write_class.a{beg}.obj_value:{:?}",
                beg + kMapOffset
            );
            self.refs_p.insert(addr, beg + kMapOffset);
            obj.marshal(self)?;
            let bcnt = self.pos() - start;
            let bcnt = bcnt | kByteCountMask;
            trace!(";WBuffer.write_class.a{beg}.bcnt:{:?}", bcnt);
            Ok(bcnt)
        }
    }

    pub(crate) fn write_cstring(&mut self, p0: &str) -> Result<()> {
        self.write_array_u8(p0.as_bytes())?;
        self.write_u8(0)
    }

    pub(crate) fn write_object(&mut self, obj: &impl Marshaler) -> Result<i64> {
        let beg = self.pos();
        let len = self.w.p.len();
        trace!(
            ";WBuffer.write_object.a{beg}.buf.value:{:?}",
            &self.p()[len..]
        );
        obj.marshal(self)
    }
    pub(crate) fn write_object_box(&mut self, obj: impl Marshaler) -> Result<i64> {
        obj.marshal(self)
    }

    pub(crate) fn clear(&mut self) {
        self.w.p.clear();
        self.w.c = 0;
    }
}

impl WBuffer {
    pub fn new(offset: u32) -> Self {
        WBuffer {
            w: Wbuff::default(),
            offset,
            refs_p: HashMap::new(),
            refs_s: HashMap::new(),
        }
    }

    pub fn with_size(offset: u32, size: usize) -> Self {
        let mut w = WBuffer {
            w: Wbuff {
                p: Vec::with_capacity(size),
                c: 0,
            },
            offset,
            refs_p: HashMap::new(),
            refs_s: HashMap::new(),
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
