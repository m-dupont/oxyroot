use crate::rbytes::consts::{kByteCountMask, kClassMask, kMapOffset, kNewClassTag, kNullTag};
use crate::rbytes::rbuffer::RBufferRefsItem::Func;
use crate::rbytes::{Header, StreamerInfoContext, Unmarshaler, UnmarshalerInto};
use crate::rtypes;
use crate::rtypes::factory::FactoryBuilderValue;
use crate::rtypes::FactoryItem;
use anyhow::{anyhow, bail, ensure, Result};
use log::{trace, warn};
use std::collections::HashMap;
use std::io::Read;
use std::mem::size_of;
use std::str::from_utf8;

#[derive(Default)]
pub struct Rbuff<'a> {
    p: &'a [u8],
    c: usize,
}

impl<'a> Rbuff<'a> {
    pub fn extract_as_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let buf: [u8; N] = self.p[self.c..(self.c + N)].as_ref().try_into()?;
        self.c += N;
        Ok(buf)
    }

    pub fn extract_n(&mut self, n: usize) -> Result<&[u8]> {
        let buf = &self.p[self.c..(self.c + n)];
        self.c += n;
        Ok(buf)
    }

    pub fn extract_until<F>(&mut self, n: usize, pred: F) -> Result<&'a [u8]>
    where
        F: FnMut(&u8) -> bool,
    {
        let buf = &self.p[self.c..(self.c + n)];
        let mut iter = buf.split(pred);
        if let Some(buf) = iter.next() {
            self.c += buf.len() + 1;
            return Ok(buf);
        }
        Ok(buf)
    }
}

impl<'a> Read for Rbuff<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if (self.c as usize) >= self.p.len() {
            return Ok(0);
        }

        fn copy_slice(dst: &mut [u8], src: &[u8]) -> usize {
            let mut c = 0;
            for (d, s) in dst.iter_mut().zip(src.iter()) {
                *d = *s;
                c += 1;
            }
            c
        }

        let n = copy_slice(buf, &self.p[self.c..]);
        self.c += n;
        Ok(n)
    }
}

#[derive(Debug)]
enum RBufferRefsItem<'a> {
    Func(&'a FactoryBuilderValue),
    // Obj(&'a Box<dyn FactoryItem>),
}

#[derive(Default)]
pub struct RBuffer<'a> {
    r: Rbuff<'a>,
    offset: u32,
    sictx: Option<&'a dyn StreamerInfoContext>,
    refs: HashMap<i64, RBufferRefsItem<'a>>,
}

impl<'a> RBuffer<'a> {
    pub fn new(data: &[u8], offset: u32) -> RBuffer {
        RBuffer {
            r: Rbuff { p: data, c: 0 },
            offset,
            ..Default::default()
        }
    }

    pub fn with_info_context(mut self, ctx: Option<&'a dyn StreamerInfoContext>) -> Self {
        self.sictx = ctx;
        self
    }

    pub fn len(&self) -> i64 {
        self.r.p.len() as i64 - self.r.c as i64
    }

    pub fn pos(&self) -> i64 {
        return self.r.c as i64 + self.offset as i64;
    }

    pub fn set_pos(&mut self, pos: i64) {
        let pos = pos - self.offset as i64;
        assert!(pos > 0);
        self.r.c = pos as usize;
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        const SIZE: usize = size_of::<u8>();
        let buf = self.r.extract_as_array::<SIZE>()?;
        Ok(u8::from_be_bytes(buf))
    }

    pub fn read_array_u8(&mut self, arr: &mut [u8]) -> Result<()> {
        for i in 0..arr.len() {
            arr[i] = self.read_u8()?;
        }

        Ok(())
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        const SIZE: usize = size_of::<i8>();
        let buf = self.r.extract_as_array::<SIZE>()?;
        Ok(i8::from_be_bytes(buf))
    }

    pub fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_i8()? != 0)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        const SIZE: usize = size_of::<u16>();
        let buf = self.r.extract_as_array::<SIZE>()?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        const SIZE: usize = size_of::<i16>();
        let buf = self.r.extract_as_array::<SIZE>()?;
        Ok(i16::from_be_bytes(buf))
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        const SIZE: usize = size_of::<i32>();
        let buf = self.r.extract_as_array::<SIZE>()?;
        Ok(i32::from_be_bytes(buf))
    }

    pub fn read_array_i32(&mut self, arr: &mut [i32]) -> Result<()> {
        for i in 0..arr.len() {
            arr[i] = self.read_i32()?;
        }
        Ok(())
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        const SIZE: usize = size_of::<u32>();
        let buf = self.r.extract_as_array::<SIZE>()?;
        Ok(u32::from_be_bytes(buf))
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        const SIZE: usize = size_of::<i64>();
        let buf = self.r.extract_as_array::<SIZE>()?;
        Ok(i64::from_be_bytes(buf))
    }

    pub fn read_array_i64(&mut self, arr: &mut [i64]) -> Result<()> {
        for i in 0..arr.len() {
            arr[i] = self.read_i64()?;
        }

        Ok(())
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        const SIZE: usize = size_of::<u64>();
        let buf = self.r.extract_as_array::<SIZE>()?;
        Ok(u64::from_be_bytes(buf))
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        const SIZE: usize = size_of::<f64>();
        let buf = self.r.extract_as_array::<SIZE>()?;
        Ok(f64::from_be_bytes(buf))
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        const SIZE: usize = size_of::<f32>();
        let buf = self.r.extract_as_array::<SIZE>()?;
        Ok(f32::from_be_bytes(buf))
    }

    pub fn read_object_into<T: UnmarshalerInto<Item = T>>(&mut self) -> Result<T> {
        T::unmarshal_into(self)
    }

    pub fn read_object<T: Unmarshaler>(&mut self, obj: &mut T) -> Result<()> {
        obj.unmarshal(self)
    }

    pub fn read_boxed_object(&mut self, obj: &mut Box<dyn rtypes::FactoryItem>) -> Result<()> {
        obj.unmarshal(self)
    }

    pub fn read_object_any_into(&mut self) -> Result<Option<Box<dyn FactoryItem>>> {
        let beg = self.pos();
        let mut bcnt = self.read_u32()?;
        let mut vers = 0;
        let tag: u32;
        let mut start = 0;

        if (bcnt as i64) & kByteCountMask == 0 || (bcnt as i64) == kNewClassTag {
            tag = bcnt;
            bcnt = 0;
        } else {
            vers = 1;
            start = self.pos();
            tag = self.read_u32()?;
        }

        trace!(
            "\t\t beg = {} bcnt = {} start = {} tag = {}",
            beg,
            bcnt,
            start,
            tag
        );

        let tag64 = tag as i64;

        trace!(
            "read_object_any_into: before case, pos = {}, tag64 = {tag64}",
            self.pos()
        );

        if tag64 & kClassMask == 0 {
            if tag64 == kNullTag {
                return Ok(None);
            }

            if tag == 1 {
                bail!("rbytes: tag == 1 means 'self'; not implemented yet");
            }

            warn!("Sadly return None");

            return Ok(None);

            // let o = &self.refs.get(&tag64);
            //
            // trace!("self.refs = {:?}", self.refs);
            // trace!("o = {:?}", o);
        } else if tag64 == kNewClassTag {
            let cname = self.read_cstring(80)?;

            // trace!("cname = {}", cname);
            let fct = rtypes::FACTORY
                .get(cname)
                .ok_or(anyhow!("rbufer: no registered factory for class {}", cname,))?;

            if vers > 0 {
                self.refs.insert(start + kMapOffset, Func(fct));
                // todo!()
            } else {
                todo!()
            }

            let mut obj: Box<dyn rtypes::FactoryItem> = fct();

            trace!("read_object_any_into: after cname pos = {}", self.pos());
            // obj.unmarshal(self);
            self.read_boxed_object(&mut obj)?;

            if vers > 0 {
                trace!(">store: have to store object at {}", beg + kMapOffset);

                // self.refs.insert(beg + kMapOffset, Obj(&obj));
                // todo!()
            } else {
                todo!()
            }

            return Ok(Some(obj));
        } else {
            let uref = tag64 & !kClassMask;
            trace!("read_object_any_into, uref = {}", uref);

            let fct = self.refs.get(&uref);
            assert!(fct.is_some());
            let fct = fct.unwrap();

            let fct = if let Func(fct) = fct {
                fct
            } else {
                unimplemented!()
            };
            let mut obj: Box<dyn rtypes::FactoryItem> = fct();
            self.read_boxed_object(&mut obj)?;
            return Ok(Some(obj));
        }
    }

    pub fn read_string(&mut self) -> Result<&str> {
        let u8 = self.read_u8()?;
        let n = if u8 == 255 {
            // large string
            self.read_u32()?
        } else {
            u8 as u32
        };

        if n == 0 {
            return Ok("");
        }

        // trace!("read_string, n = {}", n);

        let buf = self.r.extract_n(n as usize)?;

        // trace!("read_string: buf = {:?}", buf);

        if let Ok(s) = from_utf8(buf) {
            // trace!("read_string = {}", s);
            return Ok(s);
        }
        return Ok("");
    }

    pub fn read_cstring(&mut self, n: usize) -> Result<&'a str> {
        let buf = self.r.extract_until(n, |a| *a == 0)?;
        if let Ok(s) = from_utf8(buf) {
            return Ok(s);
        }

        Ok("")
    }

    pub fn read_header(&mut self, class: &str) -> Result<Header> {
        let mut hdr = Header {
            name: String::from(class),
            pos: self.pos(),
            ..Default::default()
        };

        trace!(">>read_header, pos = {}", self.pos());

        let bcnt = self.read_u32()? as i64;

        if bcnt & kByteCountMask != 0 {
            hdr.len = (bcnt & !kByteCountMask) as u32;
            hdr.vers = self.read_u16()? as i16;
        } else {
            self.set_pos(hdr.pos);
            hdr.vers = self.read_u16()? as i16;
        }

        if hdr.vers <= 0 {
            // todo!();

            self.sictx.unwrap().streamer_info(&hdr.name, -1);
            if hdr.name != "" && self.sictx.is_some() {
                if let Some(_) = self.sictx.unwrap().streamer_info(&hdr.name, -1) {
                    todo!()
                }
            }
        }

        trace!("hdr = {:?}", hdr);

        trace!("<<read_header, pos = {}", self.pos());

        Ok(hdr)
    }

    pub fn check_header(&self, _: &Header) -> Result<()> {
        Ok(())
    }

    pub fn skip_version(&mut self, class: &str) -> Result<()> {
        let version = self.read_i16()?;

        if ((version as i64) & kByteCountMask) != 0 {
            self.read_u16()?;
            self.read_u16()?;
        }

        ensure!(!(class != "" && version <= 1), "not implemented");

        Ok(())
    }
}

impl<'a> Read for RBuffer<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.r.read(buf)
    }
}

#[cfg(test)]
mod tests {
    use crate::rbytes::rbuffer::RBuffer;
    use anyhow::Result;

    #[test]
    fn rbuffer_i32() -> Result<()> {
        let refs: Vec<i32> = vec![0, 32];

        for r in refs.into_iter() {
            let a = r.to_be_bytes();
            let buf = a.as_slice();
            let i = RBuffer::new(buf, 0).read_i32()?;
            assert_eq!(i, r);
        }

        Ok(())
    }

    #[test]
    fn rbuffer_i64() -> Result<()> {
        let refs: Vec<i64> = vec![0, 32, 2i64.pow(31), -5, 2i64.pow(54)];

        for r in refs.into_iter() {
            let a = r.to_be_bytes();
            let buf = a.as_slice();
            let i = RBuffer::new(buf, 0).read_i64()?;
            assert_eq!(i, r);
        }

        Ok(())
    }

    #[test]
    fn rbuffer_i32_seq() -> Result<()> {
        let refs: Vec<i32> = vec![0, 32, -5, 2i32.pow(30)];
        let mut refs_u8 = Vec::new();
        for r in refs.iter() {
            let bytes = r.to_be_bytes();
            for u in bytes.into_iter() {
                refs_u8.push(u)
            }
        }

        let mut buffer = RBuffer::new(refs_u8.as_slice(), 0);

        for r in refs.into_iter() {
            let i = buffer.read_i32()?;
            assert_eq!(i, r);
        }

        Ok(())
    }
}
