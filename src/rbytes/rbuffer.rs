use crate::rbytes::Unmarshaler;
use anyhow::Result;
use log::trace;
use std::io::{Error, Read};
use std::mem::size_of;
use std::str::from_utf8;

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

pub struct RBuffer<'a> {
    r: Rbuff<'a>,
    offset: u32,
}

impl<'a> RBuffer<'a> {
    pub fn new(data: &[u8], offset: u32) -> RBuffer {
        RBuffer {
            r: Rbuff { p: data, c: 0 },
            offset,
        }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        const size: usize = size_of::<u8>();
        let buf = self.r.extract_as_array::<size>()?;
        Ok(u8::from_be_bytes(buf))
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        const size: usize = size_of::<u16>();
        let buf = self.r.extract_as_array::<size>()?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        const size: usize = size_of::<i16>();
        let buf = self.r.extract_as_array::<size>()?;
        Ok(i16::from_be_bytes(buf))
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        const size: usize = size_of::<i32>();
        let buf = self.r.extract_as_array::<size>()?;
        Ok(i32::from_be_bytes(buf))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        const size: usize = size_of::<u32>();
        let buf = self.r.extract_as_array::<size>()?;
        Ok(u32::from_be_bytes(buf))
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        const size: usize = size_of::<i64>();
        let buf = self.r.extract_as_array::<size>()?;
        Ok(i64::from_be_bytes(buf))
    }

    pub fn read_object<T: Unmarshaler<Item = T>>(&mut self) -> Result<T> {
        T::unmarshal_root(self)
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

        trace!("read_string, n = {}", n);

        let buf = self.r.extract_n(n as usize)?;
        if let Ok(s) = from_utf8(buf) {
            trace!("classname = {}", s);
            return Ok(s);
        }
        return Ok("");
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
    use num::pow;

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
