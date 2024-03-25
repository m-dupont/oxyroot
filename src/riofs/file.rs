use crate::rdict::streamers::db::streamer_info;
use crate::riofs::consts;
use crate::rtree::tree::{ReaderTree, WriterTree};
use crate::utils::is_cxx_builtin;
use itertools::cons_tuples;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::ptr::addr_of;
use std::rc::Rc;

use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{Marshaler, StreamerInfoContext};
use crate::rcont::list::{ReaderList, WriterList};
use crate::rdict::streamers::db::streamer_info_from;
use crate::rdict::{Streamer, StreamerInfo, Visitor};
use crate::riofs::blocks::{FreeList, FreeSegments};
use crate::riofs::consts::kStartBigFile;
use crate::riofs::dir::TDirectoryFile;
use crate::riofs::key::Key;
use crate::riofs::{Error, Result};
use crate::rmeta::ESTLType;
use crate::root::traits::Named;
use crate::rtree::tree::Tree;
use crate::rtypes::factory::FactoryItemWrite;
use crate::rtypes::FactoryItemRead;
use crate::utils::is_core_type;
use crate::{rcont, rvers, Object};
use log::{debug, trace};
use regex::Regex;
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

impl RootFileHeader {
    pub fn compression(&self) -> i32 {
        self.compression
    }
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

    pub(crate) fn write_at(&mut self, buf: &[u8], start: u64) -> Result<()> {
        self.writer
            .as_mut()
            .expect("ERROR")
            .seek(SeekFrom::Start(start))?;
        self.writer.as_mut().expect("ERROR").write_all(buf)?;
        Ok(())
    }
}

// impl Clone for RootFileWriter {
//     fn clone(&self) -> Self {
//         RootFileWriter::new(&self.path).unwrap()
//     }
// }

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
    id: String,
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
        let s = path.as_ref().to_str().unwrap().to_string();
        let writer = RootFileWriter::new(path)?;

        let inner = RootFileInner::Writer(writer);

        let mut header = RootFileHeader::default();
        header.version = rvers::ROOT;
        header.begin = consts::kBEGIN;
        header.end = consts::kBEGIN;
        header.units = 4;
        header.compression = 1;

        let mut spans = FreeList::default();
        spans.append(FreeSegments::new(header.begin, consts::kStartBigFile));

        let dir = TDirectoryFile::new(s.clone());

        let mut f = RootFile {
            inner,
            header,
            spans,
            dir,
            id: s.clone(),
            ..Default::default()
        };

        let name_len = f.dir.dir().named().size_of();
        trace!(";RootFile.create.f.dir.dir.named.Sizeof:{}", name_len);
        trace!(
            ";RootFile.create.f.dir.dir.named.name:{}",
            f.dir.dir().named().name
        );
        trace!(
            ";RootFile.create.f.dir.dir.named.title:{}",
            f.dir.dir().named().title
        );
        trace!(";RootFile.create.f.version:{}", f.version());
        trace!(
            ";RootFile.create.f.dir.recordSize:{}",
            TDirectoryFile::record_size(f.version())
        );

        let obj_len = name_len + TDirectoryFile::record_size(f.version()) as i32;
        trace!(";RootFile.create.f.objlen:{}", obj_len);

        let mut key = Key::new(
            f.dir.dir().named().name.clone(),
            f.dir.dir().named().title.clone(),
            "TFile".to_string(),
            obj_len,
            &mut f,
        )?;

        f.header.n_bytes_name = key.key_len() + name_len;
        trace!(
            ";RootFile.create.f.header.n_bytes_name:{}",
            f.header.n_bytes_name
        );
        f.dir.n_bytes_name = key.key_len() + name_len;

        f.dir.seek_dir = key.seek_key();
        trace!(";RootFile.create.f.dir.seek_dir:{}", f.dir.seek_dir);

        f.write_header()?;

        let mut buf = WBuffer::new(0);
        buf.write_string(&f.id)?;
        buf.write_string(f.title())?;

        f.dir().marshal(&mut buf)?;

        let mut buf = buf.buffer();
        trace!(";RootFile.create.buf_for_key.len:{:?}", buf.len());

        key.set_buffer(buf, false);

        key.write_to_file(f.writer()?)?;

        Ok(f)
    }

    pub(crate) fn put<T>(&mut self, name: &str, obj: &T) -> Result<()>
    where
        T: FactoryItemWrite,
    {
        trace!(";TDirectoryFile.put.name:{:?}", name);
        if name.contains('/') {
            return Err(Error::NameContainsSlash(name.to_string()));
        }

        let name = if name.is_empty() { obj.name() } else { name };
        let title = obj.title();

        trace!(";TDirectoryFile.put.name:{:?}", name);
        trace!(";TDirectoryFile.put.title:{:?}", title);

        let mut cycle = 0;

        for key in self.dir().keys().iter() {
            if key.name() != name {
                continue;
            }
            if key.class_name() != obj.class() {
                return Err(Error::KeyClassMismatch {
                    key: key.name().to_string(),
                    key_class: key.class_name().to_string(),
                    obj_class: obj.class().to_string(),
                });
            }

            if key.cycle() > cycle {
                cycle = key.cycle();
            }
        }

        cycle += 1;
        let typename = obj.class();
        trace!(";TDirectoryFile.put.typename:{:?}", typename);
        trace!(
            ";TDirectoryFile.put.is_core_type:{:?}",
            is_core_type(typename)
        );

        if !is_core_type(typename) {
            let cxx = typename;
            let streamer = streamer_info_from(obj, &mut self.dir)?;
            self.add_streamer_info(streamer);
        }

        // trace!(";TDirectoryFile.put.file.sinfos:{:?}", self.sinfos);

        let mut key = Key::new_from_object(name, title, obj.class(), obj, self)?;
        key.set_cycle(cycle as i16);
        key.write_to_file(self.writer()?)?;
        self.dir.keys.push(key);
        Ok(())
    }

    fn add_streamer_info(&mut self, si: StreamerInfo) {
        if self.sinfos.list().iter().any(|s| s.name() == si.name()) {
            return;
        }

        self.sinfos.push(si);
        trace!(
            ";TDirectoryFile.put.file.sinfos.len:{:?}",
            self.sinfos.list.len()
        );
    }

    pub fn close(&mut self) -> Result<()> {
        let mut dir = self.dir().clone();
        dir.close(self)?;
        let _ = std::mem::replace(&mut self.dir, dir);
        self.write_streamer_info()?;
        self.write_free_segments()?;
        self.write_header()?;

        Ok(())
    }

    fn reader(&self) -> Result<&RootFileReader> {
        match &self.inner {
            RootFileInner::Reader(r) => Ok(r),
            RootFileInner::Writer(_) => Err(Error::FileIsOpenedWriteOnly),
            _ => Err(Error::FileIsNotOpened),
        }
    }

    pub(crate) fn writer(&mut self) -> Result<&mut RootFileWriter> {
        match &mut self.inner {
            RootFileInner::Writer(w) => Ok(w),
            RootFileInner::Reader(_) => Err(Error::FileIsOpenedReadOnly),
            _ => Err(Error::FileIsNotOpened),
        }
    }

    pub(crate) fn read_at(&self, start: u64, len: u64) -> Result<Vec<u8>> {
        let mut reader = self.reader()?.clone();
        reader.read_at(start, len)
    }

    pub(crate) fn write_at(&mut self, buf: &[u8], start: u64) -> Result<()> {
        let mut writer = self.writer()?;
        writer.write_at(buf, start)
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

    pub(crate) fn set_end(&mut self, pos: i64) -> Result<()> {
        trace!(";RootFile.set_end.pos:{:?}", pos);
        self.header.end = pos;
        if self.spans.len() == 0 {
            panic!("self.spans.len()")
        }
        let blk = self.spans.vec().last_mut().unwrap();
        if blk.last != kStartBigFile {
            panic!("blk.last")
        }
        blk.first = pos;
        Ok(())
    }

    fn write_header(&mut self) -> Result<()> {
        self.header.n_free = self.spans.len() as i32;

        let mut w = WBuffer::new(self.begin() as u32);

        let mut version = self.version();
        if self.is_big_file()
            || self.header.seek_free > kStartBigFile
            || self.header.seek_info > kStartBigFile
        {
            if version < 1000000 {
                version += 1000000;
            }
            self.header.units = 8
        }

        w.write_array_u8(ROOT_MAGIC.as_bytes())?;
        w.write_i32(version as i32)?;
        w.write_i32(self.header.begin as i32)?;

        if version < 1000000 {
            w.write_i32(self.header.end as i32)?;
            w.write_i32(self.header.seek_free as i32)?;
            w.write_i32(self.header.n_bytes_free as i32)?;
            w.write_i32(self.header.n_free as i32)?;
            w.write_i32(self.header.n_bytes_name as i32)?;
            w.write_u8(self.header.units)?;
            w.write_i32(self.header.compression as i32)?;
            w.write_i32(self.header.seek_info as i32)?;
            w.write_i32(self.header.n_bytes_info as i32)?;
        } else {
            w.write_i64(self.header.end)?;
            w.write_i64(self.header.seek_free)?;
            w.write_i32(self.header.n_bytes_free as i32)?;
            w.write_i32(self.header.n_free as i32)?;
            w.write_i32(self.header.n_bytes_name as i32)?;
            w.write_u8(self.header.units)?;
            w.write_i32(self.header.compression as i32)?;
            w.write_i64(self.header.seek_info)?;
            w.write_i32(self.header.n_bytes_info as i32)?;
        }

        // TODO: marshal uuid version
        w.write_i16(rvers::UUID)?;
        w.write_array_u8(self.header.uuid.as_ref())?;

        let mut buf = Vec::new();
        buf.resize(self.header.begin as usize, 0u8);
        self.write_at(&buf, 0)?;

        let buf = w.buffer();
        trace!(";RootFile.write_header.buf.len:{:?}", buf.len());
        self.write_at(&buf, 0)?;

        //

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

        let mut objs: Box<ReaderList> = ogj.downcast::<ReaderList>().unwrap();

        for i in (0..objs.len()).rev() {
            let obj = objs.at(i);

            if obj.class() == "TStreamerInfo" {
                let obj: Box<StreamerInfo> = obj.downcast::<StreamerInfo>().unwrap();
                self.sinfos.push(*obj);
            } else {
                let mut list: Box<ReaderList> = obj.downcast::<ReaderList>().unwrap();
                for j in (0..list.len()).rev() {
                    let _jobj = list.at(j);
                    // let obj: Box<StreamerInfo> = jobj.downcast::<StreamerInfo>().unwrap();
                    // trace!("\tobj.name = {}", obj.name());
                }
            }
        }

        Ok(())
    }

    fn write_streamer_info(&mut self) -> Result<()> {
        trace!(
            ";RootFile.write_streamer_info.f.sinfos.init.len:{}",
            self.sinfos.list().len()
        );

        self.find_deep_streamer()?;

        trace!(
            ";RootFile.write_streamer_info.f.sinfos.after_find_deep_streamer.len:{}",
            self.sinfos.list().len()
        );

        let mut sinfos = WriterList::new();
        let binding = self.sinfos.list.clone();
        for si in binding.as_ref() {
            sinfos.push(si, addr_of!(*si) as usize);
        }

        if self.header.seek_info != 0 {
            unimplemented!("f.seek_info != 0")
        }

        trace!(
            ";RootFile.write_streamer_info.sinfos.title:{}",
            sinfos.title()
        );
        trace!(
            ";RootFile.write_streamer_info.sinfos.class:{}",
            sinfos.class()
        );

        let mut key = Key::new(
            "StreamerInfo".to_string(),
            sinfos.title().to_string(),
            sinfos.class().to_string(),
            0,
            self,
        )?;
        let offset = key.key_len();
        trace!(";RootFile.write_streamer_info.offset:{}", offset);

        let mut buf = WBuffer::new(offset as u32);

        sinfos.marshal(&mut buf)?;

        trace!(
            ";RootFile.write_streamer_info.buf.after_sinfos.len:{:?}",
            buf.p().len()
        );

        let key = Key::new_from_buffer(
            "StreamerInfo".to_string(),
            sinfos.title().to_string(),
            sinfos.class().to_string(),
            1,
            buf.buffer(),
            self,
        )?;

        self.header.seek_info = key.seek_key();
        self.header.n_bytes_info = key.n_bytes();

        trace!(
            ";RootFile.write_streamer_info.f.seek_info:{}",
            self.header.seek_info
        );
        trace!(
            ";RootFile.write_streamer_info.f.n_bytes_info:{}",
            self.header.n_bytes_info
        );

        key.write_to_file(self.writer()?)?;

        Ok(())
    }

    fn write_free_segments(&mut self) -> Result<()> {
        trace!(";RootFile.write_free_segments.call:{}", true);
        if self.header.seek_free != 0 {
            unimplemented!("self.header.seek_free != 0")
        }

        let mut nbytes = 0;
        for span in self.spans.vec() {
            nbytes += span.size_of();
        }
        trace!(";RootFile.write_free_segments.nbytes:{}", nbytes);

        let mut key = Key::new(
            self.dir.dir().named().name.clone(),
            self.dir.dir().named().title.clone(),
            "TFile".to_string(),
            nbytes,
            self,
        )?;

        assert_ne!(key.seek_key(), 0);

        if !self.is_big_file() && self.end() > kStartBigFile {
            unimplemented!("!self.is_big_file() && self.end() > kStartBigFile")
        }

        let nbytes = key.obj_len();
        let mut buf = WBuffer::new(0);

        for span in self.spans.vec() {
            span.marshal(&mut buf)?;
        }

        if buf.pos() != nbytes.into() {
            unimplemented!("buf.pos() != nbytes as usize")
        }

        self.header.n_bytes_free = key.n_bytes();
        self.header.seek_free = key.seek_key();
        key.set_buffer(buf.buffer(), false);
        key.write_to_file(self.writer()?)?;

        Ok(())
    }

    fn find_deep_streamer(&mut self) -> Result<()> {
        trace!(";Rootfile.find_deep_streamer.call:true");
        trace!(
            ";Rootfile.find_deep_streamer.f.sinfos.len:{}",
            self.sinfos.list().len()
        );

        // let mut visited = Vec::new();
        let mut v = Vec::new();

        #[derive(Debug)]
        struct DepsType {
            name: String,
            vers: i16,
        }

        for (i, si) in self.sinfos.list().iter().enumerate() {
            trace!(
                ";Rootfile.find_deep_streamer.for_loop.{i}.si.name:{:?}",
                si.name()
            );

            let mut visitor = Visitor::new(|depth, se| {
                trace!(";Rootfile.find_deep_streamer.fnmut.se.name:{}", se.name());
                let name = se.name().to_string();
                match se {
                    Streamer::String(o) => {
                        let d = DepsType {
                            name: o.element.ename.to_string(),
                            vers: -1,
                        };
                        v.push(d)
                    }
                    Streamer::STLstring(o) => {
                        let d = DepsType {
                            name: o.streamer_stl.element.ename.to_string(),
                            vers: -1,
                        };
                        v.push(d)
                    }
                    Streamer::BasicType(_) => {}
                    Streamer::BasicPointer(_) => {}
                    Streamer::Stl(stl) => {
                        match &stl.vtype {
                            ESTLType::STLvector => {
                                let etn = se.item_type_name();
                                let reg = Regex::new(r"vector<([A-Za-z]+)>").unwrap();
                                let cap = reg.captures(etn).unwrap();
                                let etn = &cap[1];

                                // trace!(";StreamerInfo.visit.se.etn:{} {}", etn, depth);
                                trace!(";Rootfile.find_deep_streamer.fnmut.etn:{}", etn);

                                let d = DepsType {
                                    name: etn.to_string(),
                                    vers: -1,
                                };
                                v.push(d)

                                // itt.push(Box::new(empty::<StreamerInfo>()));
                                // todo!("Streamer::Stl");
                            }
                            _ => {
                                todo!("Streamer::Stl, vtype = {:?}", &stl.vtype);
                            }
                        }
                    }
                    Streamer::Base(o) => {
                        let d = DepsType {
                            name: name,
                            vers: o.vbase() as i16,
                        };

                        v.push(d);
                    }
                    Streamer::Object(o) => {
                        let d = DepsType {
                            name: o.element.ename.to_string(),
                            vers: -1,
                        };
                        v.push(d)
                    }
                    Streamer::ObjectAny(o) => {
                        let d = DepsType {
                            name: o.element.ename.to_string(),
                            vers: -1,
                        };
                        v.push(d)
                    }
                    Streamer::ObjectPointer(o) => {
                        let tname = o.element.ename.trim_end_matches('*').to_string();
                        let d = DepsType {
                            name: tname,
                            vers: -1,
                        };
                        v.push(d)
                    }
                }
            });
            visitor.run(0, si)?;
        }

        trace!(";Rootfile.find_deep_streamer.v.len:{}", v.len());
        v.iter().for_each(|d| {
            trace!(
                ";Rootfile.find_deep_streamer.v.{}.name:{:?} {}",
                d.name,
                d.name,
                d.vers
            );
            trace!(
                ";Rootfile.find_deep_streamer.v.{}.is_core_type:{:?}",
                d.name,
                is_core_type(&d.name),
            );
            trace!(
                ";Rootfile.find_deep_streamer.v.{}.is_cxx_builtin:{:?}",
                d.name,
                is_cxx_builtin(&d.name),
            );

            if is_core_type(&d.name) || is_cxx_builtin(&d.name) {
                return;
            }

            let si = streamer_info(&d.name, d.vers).unwrap();
            self.add_streamer_info(si);
        });

        // self.sinfos.extend(v);
        //
        // self.sinfos.list().iter().for_each(|si| {
        //     trace!(";Rootfile.find_deep_streamer.f.sinfos.name:{:?}", si.name());
        // });

        trace!(
            ";Rootfile.find_deep_streamer.f.sinfos.len:{}",
            self.sinfos.list().len()
        );

        Ok(())
    }

    fn get_object(&mut self, name: &str) -> Result<Box<dyn FactoryItemRead>> {
        let mut reader = self.reader()?.clone();
        self.dir.get_object(name, &mut reader, Some(&self.sinfos))

        // let a = &(&*self).dir;
        // let b = self.reader()?;
        //
        // a.get_object(name, b, Some(&self.sinfos))

        // Ok(obj)
    }

    pub fn get_tree(&mut self, name: &str) -> Result<ReaderTree> {
        let objet = self.get_object(name)?;
        let mut objet: ReaderTree = *objet.downcast::<ReaderTree>().expect("");

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
    pub fn dir(&self) -> &TDirectoryFile {
        &self.dir
    }
    pub(crate) fn mut_dir(&mut self) -> &mut TDirectoryFile {
        &mut self.dir
    }

    pub(crate) fn is_big_file(&self) -> bool {
        self.end() > consts::kStartBigFile
    }

    fn title(&self) -> &str {
        self.dir().dir().named().title()
    }
    pub fn compression(&self) -> i32 {
        self.header.compression()
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

    fn extend(&mut self, infos: Vec<StreamerInfo>) {
        let v = Rc::get_mut(&mut self.list).expect("Do not panic ! ");
        v.extend(infos);
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
