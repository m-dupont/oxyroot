use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::StreamerInfoContext;
use crate::rcont::list::List;
use crate::rdict::StreamerInfo;
use crate::riofs::blocks::{FreeList, FreeSegments};
use crate::riofs::dir::TDirectoryFile;
use crate::riofs::key::Key;
use crate::riofs::{Error, Result};
use crate::root::traits::Named;
use crate::rtree::tree::Tree;
use crate::rtypes::FactoryItem;
use log::{debug, trace};
use uuid::Uuid;

const HEADER_LEN: u64 = 64;
const HEADER_EXTRA_LEN: u64 = 12;
// 64: small file + extra space for big file
const ROOT_MAGIC: &str = "root";

#[derive(Default, Debug)]
struct RootFileHeader {
    version: i32,
    begin: i64,
    end: i64,
    seek_free: i64,
    n_bytes_free: i32,
    n_free: i32,
    n_bytes_name: i32,
    units: u8,
    compression: i32,
    seek_info: i64,
    n_bytes_info: i32,
    uuid: Uuid,
}

#[derive(Default)]
pub(crate) struct RootFileReader {
    path: PathBuf,
    reader: Option<BufReader<File>>,
}

impl Display for RootFileReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.path.to_str().unwrap())?;
        Ok(())
    }
}

impl RootFileReader {
    pub(crate) fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let f = File::open(path.as_ref())?;
        let buf_reader = BufReader::new(f);
        let reader = Self {
            path: path.as_ref().to_path_buf(),
            reader: Some(buf_reader),
        };
        Ok(reader)
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
}

impl Clone for RootFileReader {
    fn clone(&self) -> Self {
        RootFileReader::new(self.path.clone()).unwrap()
    }
}

#[derive(Default)]
struct RootFileInner {
    reader: RootFileReader,
    header: RootFileHeader,
}

/// Rust equivalent of [`TFile`](https://root.cern/doc/master/classTFile.html).
///
/// Can only read for now. Aims to be constructed with [open](crate::RootFile::open) method.
#[derive(Default)]
pub struct RootFile {
    inner: RootFileInner,
    spans: FreeList,
    sinfos: RootFileStreamerInfoContext,
    dir: Option<TDirectoryFile>,
}

impl RootFile {
    pub(crate) fn n_bytes_name(&self) -> i32 {
        self.inner.header.n_bytes_name
    }

    pub(crate) fn version(&self) -> i32 {
        self.inner.header.version
    }

    pub(crate) fn begin(&self) -> i64 {
        self.inner.header.begin
    }

    pub(crate) fn end(&self) -> i64 {
        self.inner.header.end
    }

    /// Open file, use [std::io::BufReader] for reading, so it can only handle local files for now.
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let reader = RootFileReader::new(path)?;

        let inner = RootFileInner {
            reader,
            ..Default::default()
        };

        let mut f = RootFile {
            inner,
            ..Default::default()
        };

        f.read_header()?;

        Ok(f)
    }

    pub(crate) fn read_at(&mut self, start: u64, len: u64) -> Result<Vec<u8>> {
        self.inner.reader.read_at(start, len)
    }

    fn read_header(&mut self) -> Result<()> {
        trace!("start to read header");
        let buf = self.read_at(0, HEADER_LEN + HEADER_EXTRA_LEN)?;
        let mut r = RBuffer::new(&buf, 0);
        let mut magic: [u8; 4] = [0; 4];
        r.read_array_u8(&mut magic)?;

        trace!("magic = {:?}", magic);

        assert_eq!(magic, ROOT_MAGIC.as_bytes());

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
        r.read_array_u8(&mut uuid)?;
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
            return Err(Error::CantReadAmountOfBytesFromFile {
                requested: self.inner.header.n_bytes_free as usize,
                read: buf.len(),
            });
        }

        let mut r = RBuffer::new(&buf, 0);
        let key = r.read_object_into::<Key>()?;
        debug!("key = {:?}", key);

        let buf = key.bytes(&mut self.inner.reader, None)?;
        trace!("buf = {:?}", buf);

        let mut rbuf = RBuffer::new(&buf, 0);
        trace!("rbuf len = {}", rbuf.len());

        while !rbuf.is_empty() {
            let span = rbuf.read_object_into::<FreeSegments>()?;

            self.spans.append(span);
        }

        Ok(())
    }

    fn read_streamer_info(&mut self) -> Result<()> {
        if self.inner.header.seek_info <= 0 || self.inner.header.seek_info >= self.end() {
            return Err(Error::InvalidPointerToStreamerInfo {
                seek: self.inner.header.seek_info,
                min_allowed: 0,
                max_allowed: self.end(),
            });
        }

        let buf = self.read_at(
            self.inner.header.seek_info as u64,
            self.inner.header.n_bytes_info as u64,
        )?;

        if buf.len() != self.inner.header.n_bytes_info as usize {
            return Err(Error::CantReadAmountOfBytesFromFile {
                requested: self.inner.header.n_bytes_info as usize,
                read: buf.len(),
            });
        }

        let si_key = RBuffer::new(&buf, 0).read_object_into::<Key>()?;

        let ogj = si_key.object(&mut self.inner.reader, None)?.unwrap();

        let mut objs: Box<List> = ogj.downcast::<List>().unwrap();

        for i in (0..objs.len()).rev() {
            let obj = objs.at(i);

            if obj.class() == "TStreamerInfo" {
                let obj: Box<StreamerInfo> = obj.downcast::<StreamerInfo>().unwrap();
                self.sinfos.push(*obj);
            } else {
                let mut list: Box<List> = obj.downcast::<List>().unwrap();
                for j in (0..list.len()).rev() {
                    let _jobj = list.at(j);
                    // let obj: Box<StreamerInfo> = jobj.downcast::<StreamerInfo>().unwrap();
                    // trace!("\tobj.name = {}", obj.name());
                }
            }
        }

        Ok(())
    }

    fn get_object(&mut self, name: &str) -> Result<Box<dyn FactoryItem>> {
        self.dir
            .as_mut()
            .unwrap()
            .get_object(name, &mut self.inner.reader, Some(&self.sinfos))

        // Ok(obj)
    }

    pub fn get_tree(&mut self, name: &str) -> Result<Tree> {
        let objet = self.get_object(name)?;
        let mut objet: Tree = *objet.downcast::<Tree>().expect("");
        objet.set_reader(Some(self.inner.reader.clone()));
        objet.set_streamer_info(self.sinfos.clone());
        Ok(objet)
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.dir.as_ref().unwrap().keys().iter().map(|e| e.name())
    }
}

#[derive(Default, Clone)]
pub(crate) struct RootFileStreamerInfoContext {
    list: Rc<Vec<StreamerInfo>>,
}

impl RootFileStreamerInfoContext {
    fn push(&mut self, info: StreamerInfo) {
        let v = Rc::get_mut(&mut self.list).expect("Do not panic ! ");
        v.push(info);
    }
    fn list(&self) -> &Rc<Vec<StreamerInfo>> {
        &self.list
    }

    pub(crate) fn get(&self, name: &str) -> Option<&StreamerInfo> {
        for streamer in self.list().iter().rev() {
            if streamer.name() == name {
                return Some(streamer);
            }
        }
        None
    }
}

impl StreamerInfoContext for RootFileStreamerInfoContext {
    fn streamer_info(&self, name: &str, _version: i32) -> Option<&StreamerInfo> {
        if self.list.len() == 0 {
            return None;
        }

        for si in self.list.iter() {
            if si.name() == name {
                return Some(si);
            }
        }

        if name.find('<').is_some() {
            todo!()
        }

        None
    }
}
