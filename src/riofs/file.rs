use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

use crate::rbytes::rbuffer::RBuffer;
use crate::riofs::dir::TDirectoryFile;
use anyhow::Result;
use log::trace;
use uuid::Uuid;

const HEADER_LEN: u64 = 64;
const HEADER_EXTRA_LEN: u64 = 12;
// 64: small file + extra space for big file
const ROOT_MAGIC: &str = "root";

pub struct RootFile {
    reader: BufReader<File>,
    pub(crate) header: Option<RootFileHeader>,
}

pub struct RootFileHeader {
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
}

impl RootFile {
    pub fn open(path: &str) -> Result<Self> {
        trace!("Open file, '{}'", path);
        let f = File::open(path)?;
        let buf_reader = BufReader::new(f);

        let mut f = RootFile {
            reader: buf_reader,
            header: None,
            // dir: None,
        };

        f.header = Some(f.read_header()?);

        let t = TDirectoryFile::read_dir_info(&mut f);

        return Ok(f);
    }

    pub(crate) fn read_at(&mut self, start: u64, len: u64) -> Result<Vec<u8>> {
        self.reader.seek(SeekFrom::Start(start))?;
        let mut buf = vec![0; len as usize];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_header(&mut self) -> Result<RootFileHeader> {
        trace!("start to read header");
        let buf = self.read_at(0, HEADER_LEN + HEADER_EXTRA_LEN)?;
        let mut r = RBuffer::new(&buf, 0);
        let mut magic: [u8; 4] = [0; 4];
        r.read(&mut magic);

        trace!("magic = {:?}", magic);

        assert!(magic == "root".as_bytes());

        let version = r.read_i32()?;
        trace!("version = {}", version);

        let begin = r.read_i32()? as i64;
        let is_64 = version > 1000000;

        let end = if is_64 {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        let seek_free = if is_64 {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        let n_bytes_free = r.read_i32()?;
        let n_free = r.read_i32()?;
        let n_bytes_name = r.read_i32()?;
        let units = r.read_u8()?;
        let compression = r.read_i32()?;

        let seek_info = if is_64 {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        let n_bytes_info = r.read_i32()?;

        let version = version % 1000000;
        trace!("version = {}", version);

        let _ = r.read_u16()?;
        let mut uuid: [u8; 16] = [0; 16];
        r.read(&mut uuid);
        let uuid = Uuid::from_bytes(uuid);

        trace!("uuid = {}", uuid);

        // f.version %= 1000000

        Ok(RootFileHeader {
            version,
            begin,
            end,
            seek_free,
            n_bytes_free,
            n_free,
            n_bytes_name,
            units,
            compression,
            seek_info,
            n_bytes_info,
            uuid,
        })
    }
}
