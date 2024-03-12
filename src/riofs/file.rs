use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom};
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

#[derive(Default, Debug)]
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
        RootFileReader::new(&self.path).unwrap()
    }
}

#[derive(Default, Debug)]
pub(crate) struct RootFileWriter {
    path: PathBuf,
    writer: Option<BufWriter<File>>,
}

impl RootFileWriter {
    pub(crate) fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let f = File::create(path.as_ref())?;
        let buf_writer = BufWriter::new(f);
        let reader = Self {
            path: path.as_ref().to_path_buf(),
            writer: Some(buf_writer),
        };
        Ok(reader)
    }
}
#[derive(Default)]
enum RootFileInner {
    Reader(RootFileReader),
    Writer(RootFileWriter),
    #[default]
    None,
}

/// Rust equivalent of [`TFile`](https://root.cern/doc/master/classTFile.html).
///
/// Can only read for now. Aims to be constructed with [open](crate::RootFile::open) method.
#[derive(Default)]
pub struct RootFile {
    inner: RootFileInner,
    header: RootFileHeader,
    spans: FreeList,
    sinfos: RootFileStreamerInfoContext,
    dir: TDirectoryFile,
}

impl RootFile {
    pub(crate) fn n_bytes_name(&self) -> i32 {
        self.header.n_bytes_name
    }

    pub(crate) fn version(&self) -> i32 {
        self.header.version
    }

    pub(crate) fn begin(&self) -> i64 {
        self.header.begin
    }

    pub(crate) fn end(&self) -> i64 {
        self.header.end
    }

    /// Open file, use [std::io::BufReader] for reading, so it can only handle local files for now.
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let reader = RootFileReader::new(path)?;

        let inner = RootFileInner::Reader(reader);

        let mut f = RootFile {
            inner,
            ..Default::default()
        };

        f.read_header()?;

        Ok(f)
    }

    pub fn create<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let writer = RootFileWriter::new(path)?;

        let inner = RootFileInner::Writer(writer);

        let mut f = RootFile {
            inner,
            ..Default::default()
        };

        f.read_header()?;

        Ok(f)
    }

    fn reader(&self) -> Result<&RootFileReader> {
        match &self.inner {
            RootFileInner::Reader(r) => Ok(r),
            RootFileInner::Writer(_) => Err(Error::FileIsOpenedWriteOnly),
            _ => Err(Error::FileIsNotOpened),
        }
    }

    pub(crate) fn read_at(&self, start: u64, len: u64) -> Result<Vec<u8>> {
        let mut reader = self.reader()?.clone();
        reader.read_at(start, len)
    }

    fn read_header(&mut self) -> Result<()> {
        trace!("start to read header");
        let buf = self.read_at(0, HEADER_LEN + HEADER_EXTRA_LEN)?;
        let mut r = RBuffer::new(&buf, 0);
        // let mut magic: [u8; 4] = [0; 4];
        let magic = r.read_array_u8(4)?;

        trace!("magic = {:?}", magic);

        assert_eq!(magic, ROOT_MAGIC.as_bytes());

        let version = r.read_i32()?;
        trace!("version = {}", version);

        self.header.begin = r.read_i32()? as i64;
        let is_64 = version > 1000000;

        self.header.end = if is_64 {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        self.header.seek_free = if is_64 {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        self.header.n_bytes_free = r.read_i32()?;
        self.header.n_free = r.read_i32()?;
        self.header.n_bytes_name = r.read_i32()?;
        self.header.units = r.read_u8()?;
        self.header.compression = r.read_i32()?;

        self.header.seek_info = if is_64 {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        self.header.n_bytes_info = r.read_i32()?;

        self.header.version = version % 1000000;
        trace!("version = {}", version);

        let _ = r.read_u16()?;
        // let mut uuid: [u8; 16] = [0; 16];
        let uuid = r.read_array_u8(16)?;
        let uuid = <&[u8; 16]>::try_from(uuid)?;
        self.header.uuid = Uuid::from_bytes(*uuid);

        trace!("uuid = {}", self.header.uuid);

        let mut dir = TDirectoryFile::read_dir_info(self)?;

        if self.header.seek_free > 0 {
            self.read_free_segments()?;
        }

        if self.header.seek_info > 0 {
            self.read_streamer_info()?;
        }

        dir.read_keys(self)?;

        self.dir = dir;

        // f.version %= 1000000

        Ok(())
    }
    fn read_free_segments(&mut self) -> Result<()> {
        trace!("read_free_segments");
        let buf = self.read_at(
            self.header.seek_free as u64,
            self.header.n_bytes_free as u64,
        )?;
        if buf.len() != self.header.n_bytes_free as usize {
            return Err(Error::CantReadAmountOfBytesFromFile {
                requested: self.header.n_bytes_free as usize,
                read: buf.len(),
            });
        }

        let mut r = RBuffer::new(&buf, 0);
        let key = r.read_object_into::<Key>()?;
        debug!("key = {:?}", key);

        let mut reader = self.reader()?.clone();

        let buf = key.bytes(&mut reader, None)?;
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
        if self.header.seek_info <= 0 || self.header.seek_info >= self.end() {
            return Err(Error::InvalidPointerToStreamerInfo {
                seek: self.header.seek_info,
                min_allowed: 0,
                max_allowed: self.end(),
            });
        }

        let buf = self.read_at(
            self.header.seek_info as u64,
            self.header.n_bytes_info as u64,
        )?;

        if buf.len() != self.header.n_bytes_info as usize {
            return Err(Error::CantReadAmountOfBytesFromFile {
                requested: self.header.n_bytes_info as usize,
                read: buf.len(),
            });
        }

        let si_key = RBuffer::new(&buf, 0).read_object_into::<Key>()?;
        let mut reader = self.reader()?.clone();

        let ogj = si_key.object(&mut reader, None)?.unwrap();

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
        let mut reader = self.reader()?.clone();
        self.dir.get_object(name, &mut reader, Some(&self.sinfos))

        // let a = &(&*self).dir;
        // let b = self.reader()?;
        //
        // a.get_object(name, b, Some(&self.sinfos))

        // Ok(obj)
    }

    pub fn get_tree(&mut self, name: &str) -> Result<Tree> {
        let objet = self.get_object(name)?;
        let mut objet: Tree = *objet.downcast::<Tree>().expect("");

        objet.set_reader(Some(self.reader()?.clone()));
        objet.set_streamer_info(self.sinfos.clone());
        Ok(objet)
    }

    pub fn keys_name(&self) -> impl Iterator<Item = &str> {
        self.dir.keys().iter().map(|e| e.name())
    }

    pub fn keys(&self) -> Vec<Key> {
        self.dir.keys().clone()
    }
}

#[derive(Default, Clone, Debug)]
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
        self.list()
            .iter()
            .rev()
            .find(|&streamer| streamer.name() == name)
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
