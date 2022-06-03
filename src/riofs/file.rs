use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

use crate::rbytes::rbuffer::RBuffer;
use crate::riofs::blocks::{FreeList, FreeSegments};
use crate::riofs::dir::TDirectoryFile;
use crate::riofs::key::Key;
use anyhow::{anyhow, Result};
use log::trace;
use uuid::Uuid;

const HEADER_LEN: u64 = 64;
const HEADER_EXTRA_LEN: u64 = 12;
// 64: small file + extra space for big file
const ROOT_MAGIC: &str = "root";

#[derive(Default)]
pub struct RootFile {
    reader: Option<BufReader<File>>,
    pub(crate) version: i32,
    pub(crate) begin: i64,
    pub(crate) end: i64,
    seek_free: i64,
    n_bytes_free: i32,
    n_free: i32,
    pub(crate) n_bytes_name: i32,
    units: u8,
    compression: i32,
    seek_info: i64,
    n_bytes_info: i32,
    uuid: Uuid,
    spans: FreeList,
}

impl RootFile {
    pub fn open(path: &str) -> Result<Self> {
        trace!("Open file, '{}'", path);
        let f = File::open(path)?;
        let buf_reader = BufReader::new(f);

        let mut f = RootFile {
            reader: Some(buf_reader),
            ..Default::default()
        };

        f.read_header()?;

        return Ok(f);
    }

    pub(crate) fn read_at(&mut self, start: u64, len: u64) -> Result<Vec<u8>> {
        self.reader
            .as_mut()
            .expect("ERROR")
            .seek(SeekFrom::Start(start))?;
        let mut buf = vec![0; len as usize];
        self.reader.as_mut().expect("ERROR").read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_header(&mut self) -> Result<()> {
        trace!("start to read header");
        let buf = self.read_at(0, HEADER_LEN + HEADER_EXTRA_LEN)?;
        let mut r = RBuffer::new(&buf, 0);
        let mut magic: [u8; 4] = [0; 4];
        r.read(&mut magic);

        trace!("magic = {:?}", magic);

        assert!(magic == "root".as_bytes());

        let version = r.read_i32()?;
        trace!("version = {}", version);

        self.begin = r.read_i32()? as i64;
        let is_64 = version > 1000000;

        self.end = if is_64 {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        self.seek_free = if is_64 {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        self.n_bytes_free = r.read_i32()?;
        self.n_free = r.read_i32()?;
        self.n_bytes_name = r.read_i32()?;
        self.units = r.read_u8()?;
        self.compression = r.read_i32()?;

        self.seek_info = if is_64 {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        self.n_bytes_info = r.read_i32()?;

        self.version = version % 1000000;
        trace!("version = {}", version);

        let _ = r.read_u16()?;
        let mut uuid: [u8; 16] = [0; 16];
        r.read(&mut uuid);
        self.uuid = Uuid::from_bytes(uuid);

        trace!("uuid = {}", self.uuid);

        let t = TDirectoryFile::read_dir_info(self);

        if self.seek_free > 0 {
            self.read_free_segments()?;
        }

        if self.seek_info > 0 {
            self.read_streamer_info()?;
        }

        // f.version %= 1000000

        Ok(())
    }
    fn read_free_segments(&mut self) -> Result<()> {
        trace!("read_free_segments");
        let buf = self.read_at(self.seek_free as u64, self.n_bytes_free as u64)?;
        if buf.len() != self.n_bytes_free as usize {
            return Err(anyhow!(
                "riofs: requested {} bytes, read {} bytes from file",
                self.n_bytes_free,
                buf.len()
            ));
        }

        let mut r = RBuffer::new(&buf, 0);
        let key = r.read_object_into::<Key>()?;
        println!("key = {:?}", key);

        let buf = key.bytes(self, None)?;
        trace!("buf = {:?}", buf);

        let mut rbuf = RBuffer::new(&buf, 0);
        trace!("rbuf len = {}", rbuf.len());

        while rbuf.len() > 0 {
            let span = rbuf.read_object_into::<FreeSegments>()?;

            self.spans.append(span);
        }

        trace!("spans = {:?}", self.spans);

        //
        Ok(())
    }

    fn read_streamer_info(&mut self) -> Result<()> {
        trace!("read_streamer_info");

        if self.seek_info <= 0 || self.seek_info >= self.end {
            return Err(anyhow!(
                "riofs: invalid pointer to StreamerInfo (pos={} end={})",
                self.seek_info,
                self.end
            ));
        }

        let buf = self.read_at(self.seek_info as u64, self.n_bytes_info as u64)?;

        if buf.len() != self.n_bytes_info as usize {
            return Err(anyhow!(
                "riofs: requested {} bytes, read {} bytes from file",
                self.n_bytes_info,
                buf.len()
            ));
        }

        let si_key = RBuffer::new(&buf, 0).read_object_into::<Key>()?;
        trace!("si_key = {:?}", si_key);

        let ogj = si_key.object(self)?;

        todo!()
    }
}
