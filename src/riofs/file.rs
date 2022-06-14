use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::StreamerInfoContext;
use crate::rcont::list::List;
use crate::rdict::StreamerInfo;
use crate::riofs::blocks::{FreeList, FreeSegments};
use crate::riofs::dir::TDirectoryFile;
use crate::riofs::key::Key;
use crate::root;
use crate::root::traits::Named;
use crate::rtree::tree::Tree;
use crate::rtypes::FactoryItem;
use crate::rvers::String;
use anyhow::{anyhow, bail, Result};
use log::{debug, trace};
use uuid::Uuid;

const HEADER_LEN: u64 = 64;
const HEADER_EXTRA_LEN: u64 = 12;
// 64: small file + extra space for big file
const ROOT_MAGIC: &str = "root";

#[derive(Default)]
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

impl RootFileHeader {
    pub fn version(&self) -> i32 {
        self.version
    }
    pub fn begin(&self) -> i64 {
        self.begin
    }
    pub fn end(&self) -> i64 {
        self.end
    }
    pub fn seek_free(&self) -> i64 {
        self.seek_free
    }
    pub fn n_bytes_free(&self) -> i32 {
        self.n_bytes_free
    }
    pub fn n_free(&self) -> i32 {
        self.n_free
    }
    pub fn n_bytes_name(&self) -> i32 {
        self.n_bytes_name
    }
    pub fn units(&self) -> u8 {
        self.units
    }
    pub fn compression(&self) -> i32 {
        self.compression
    }
    pub fn seek_info(&self) -> i64 {
        self.seek_info
    }
    pub fn n_bytes_info(&self) -> i32 {
        self.n_bytes_info
    }
}

#[derive(Default)]
pub struct RootFileReader {
    path: String,
    reader: Option<BufReader<File>>,
}

impl Drop for RootFileReader {
    fn drop(&mut self) {
        println!("Drop RootFileReader")
    }
}

impl RootFileReader {
    pub fn new(path: &str) -> Result<Self> {
        let f = File::open(path)?;

        let buf_reader = BufReader::new(f);

        let reader = Self {
            path: path.to_string(),
            reader: Some(buf_reader),
        };

        Ok(reader)
    }

    // pub fn copy(&self) -> Self {
    //     RootFileReader::new(self.path.as_str()).unwrap()
    // }

    pub(crate) fn read_at(&mut self, start: u64, len: u64) -> Result<Vec<u8>> {
        self.reader
            .as_mut()
            .expect("ERROR")
            .seek(SeekFrom::Start(start))?;
        let mut buf = vec![0; len as usize];
        self.reader.as_mut().expect("ERROR").read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl Clone for RootFileReader {
    fn clone(&self) -> Self {
        debug!("create new RootFileReader");
        RootFileReader::new(self.path.as_str()).unwrap()
    }
}

#[derive(Default)]
pub struct RootFileInner {
    reader: RootFileReader,
    header: RootFileHeader,
}

#[derive(Default)]
pub struct RootFile {
    inner: RootFileInner,
    spans: FreeList,
    sinfos: RootFileStreamerInfoContext,
    pub(crate) dir: Option<TDirectoryFile>,
}

impl RootFile {
    pub fn n_bytes_name(&self) -> i32 {
        self.inner.header.n_bytes_name
    }

    pub fn version(&self) -> i32 {
        self.inner.header.version
    }

    pub fn begin(&self) -> i64 {
        self.inner.header.begin
    }

    pub fn end(&self) -> i64 {
        self.inner.header.end
    }

    pub fn open(path: &str) -> Result<Self> {
        trace!("Open file, '{}'", path);
        // let f = File::open(path)?;

        // let buf_reader = BufReader::new(f);

        // let reader = RootFileReader {
        //     reader: Some(buf_reader),
        // };

        let reader = RootFileReader::new(path)?;

        let inner = RootFileInner {
            reader,
            ..Default::default()
        };

        let mut f = RootFile {
            inner: inner,
            ..Default::default()
        };

        f.read_header()?;

        return Ok(f);
    }

    pub(crate) fn read_at(&mut self, start: u64, len: u64) -> Result<Vec<u8>> {
        self.inner.reader.read_at(start, len)
    }

    fn read_header(&mut self) -> Result<()> {
        trace!("start to read header");
        let buf = self.read_at(0, HEADER_LEN + HEADER_EXTRA_LEN)?;
        let mut r = RBuffer::new(&buf, 0);
        let mut magic: [u8; 4] = [0; 4];
        r.read(&mut magic)?;

        trace!("magic = {:?}", magic);

        assert!(magic == "root".as_bytes());

        let version = r.read_i32()?;
        trace!("version = {}", version);

        self.inner.header.begin = r.read_i32()? as i64;
        let is_64 = version > 1000000;

        self.inner.header.end = if is_64 {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        self.inner.header.seek_free = if is_64 {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        self.inner.header.n_bytes_free = r.read_i32()?;
        self.inner.header.n_free = r.read_i32()?;
        self.inner.header.n_bytes_name = r.read_i32()?;
        self.inner.header.units = r.read_u8()?;
        self.inner.header.compression = r.read_i32()?;

        self.inner.header.seek_info = if is_64 {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        self.inner.header.n_bytes_info = r.read_i32()?;

        self.inner.header.version = version % 1000000;
        trace!("version = {}", version);

        let _ = r.read_u16()?;
        let mut uuid: [u8; 16] = [0; 16];
        r.read(&mut uuid);
        self.inner.header.uuid = Uuid::from_bytes(uuid);

        trace!("uuid = {}", self.inner.header.uuid);

        let mut dir = TDirectoryFile::read_dir_info(self)?;

        if self.inner.header.seek_free > 0 {
            self.read_free_segments()?;
        }

        if self.inner.header.seek_info > 0 {
            self.read_streamer_info()?;
        }

        dir.read_keys(self)?;

        self.dir = Some(dir);

        // f.version %= 1000000

        Ok(())
    }
    fn read_free_segments(&mut self) -> Result<()> {
        trace!("read_free_segments");
        let buf = self.read_at(
            self.inner.header.seek_free as u64,
            self.inner.header.n_bytes_free as u64,
        )?;
        if buf.len() != self.inner.header.n_bytes_free as usize {
            return Err(anyhow!(
                "riofs: requested {} bytes, read {} bytes from file",
                self.inner.header.n_bytes_free,
                buf.len()
            ));
        }

        let mut r = RBuffer::new(&buf, 0);
        let key = r.read_object_into::<Key>()?;
        println!("key = {:?}", key);

        let buf = key.bytes(&mut self.inner.reader, None)?;
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

        if self.inner.header.seek_info <= 0 || self.inner.header.seek_info >= self.end() {
            return Err(anyhow!(
                "riofs: invalid pointer to StreamerInfo (pos={} end={})",
                self.inner.header.seek_info,
                self.end()
            ));
        }

        let buf = self.read_at(
            self.inner.header.seek_info as u64,
            self.inner.header.n_bytes_info as u64,
        )?;

        if buf.len() != self.inner.header.n_bytes_info as usize {
            return Err(anyhow!(
                "riofs: requested {} bytes, read {} bytes from file",
                self.inner.header.n_bytes_info,
                buf.len()
            ));
        }

        let mut si_key = RBuffer::new(&buf, 0).read_object_into::<Key>()?;
        trace!("si_key = {:?}", si_key);

        let mut ogj = si_key.object(&mut self.inner.reader, None)?.unwrap();

        let mut objs = ogj.downcast::<List>().unwrap();

        for i in objs.len()..0 {
            debug!(" i = {i}");
            let obj = objs.at(i);

            let obj = obj.downcast::<StreamerInfo>().unwrap();

            self.sinfos.push(*obj);

            // todo!()
        }

        Ok(())

        // todo!()
    }

    pub fn get_object(&mut self, name: &str) -> Result<Box<dyn FactoryItem>> {
        trace!("get_object, name = {}", name);

        // let mut dir = self.dir.as_mut().unwrap();
        // self.dir = None;

        let obj = self.dir.as_mut().unwrap().get_object(
            name,
            &mut self.inner.reader,
            Some(&self.sinfos),
        )?;

        Ok(obj)
    }

    pub fn get_tree(&mut self, name: &str) -> Result<Option<Tree>> {
        let objet = self.get_object(name)?;
        let mut objet: Tree = *objet.downcast::<Tree>().unwrap();
        objet.set_reader(Some(self.inner.reader.clone()));
        Ok(Some(objet))

        // match self.get_object(name) {
        //     Ok(obj) => match obj.downcast::<Tree>() {
        //         Ok(mut o) => {
        //             (*o).set_reader(Some(self.inner.reader.clone()));
        //             Ok(Some(*o))
        //         }
        //         Err(e) => bail!(" Can not retreive TTree because : {e} "),
        //     },
        //
        //     Err(e) => e,
        // }
    }
}

#[derive(Default)]
pub struct RootFileStreamerInfoContext(Vec<StreamerInfo>);

impl RootFileStreamerInfoContext {
    pub fn push(&mut self, info: StreamerInfo) {
        self.0.push(info);
    }
}

impl StreamerInfoContext for RootFileStreamerInfoContext {
    fn streamer_info(&self, name: &str, version: i32) -> Option<&StreamerInfo> {
        if self.0.len() == 0 {
            return None;
        }

        for si in self.0.iter() {
            if si.name() == name {
                return Some(si);
            }
        }

        if name.find("<").is_some() {
            todo!()
        }

        return None;
    }
}
