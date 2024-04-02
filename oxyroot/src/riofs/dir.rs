use crate::riofs::file::{RootFile, RootFileReader};

use crate::rbase::named::Named as ObjNamed;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::{Marshaler, StreamerInfoContext, Unmarshaler};
use crate::riofs::key::Key;
use crate::riofs::utils::{datetime_to_u32, decode_name_cycle};
use crate::root::traits::Named;
use crate::root::traits::Object;

use crate::rbytes::wbuffer::WBuffer;
use crate::riofs::{utils, Error, Result};
use crate::root::traits;
use chrono::{DateTime, Utc};
use log::trace;
use uuid::Uuid;

use crate::rtypes::FactoryItemRead;
use crate::rvers;

#[derive(Clone)]
pub struct TDirectory {
    pub(crate) rvers: i16,
    _uuid: Uuid,
    named: ObjNamed,
}

impl Default for TDirectory {
    fn default() -> Self {
        TDirectory {
            rvers: rvers::DIRECTORY,
            _uuid: Uuid::default(),
            named: ObjNamed::default(),
        }
    }
}

impl TDirectory {
    pub fn named(&self) -> &ObjNamed {
        &self.named
    }
}

#[derive(Clone)]
pub struct TDirectoryFile {
    ctime: DateTime<Utc>,
    mtime: DateTime<Utc>, //
    n_bytes_keys: i32,
    pub(crate) n_bytes_name: i32,
    // seekdir: i64,
    // seekparent: i64,
    // seekkeys: i64,
    dir: TDirectory,
    pub seek_dir: i64,
    pub seek_parent: i64,
    pub(crate) seek_keys: i64,
    class_name: String,
    pub(crate) keys: Vec<Key>,
}

impl Default for TDirectoryFile {
    fn default() -> Self {
        TDirectoryFile {
            // FIXME: use now time
            ctime: utils::now(),
            mtime: utils::now(),
            n_bytes_keys: 0,
            n_bytes_name: 0,
            dir: TDirectory::default(),
            seek_dir: 0,
            seek_parent: 0,
            seek_keys: 0,
            class_name: String::new(),
            keys: Vec::new(),
        }
    }
}

impl traits::Object for TDirectoryFile {
    fn class(&self) -> &'_ str {
        "TDirectoryFile"
    }
}

impl TDirectoryFile {
    pub(crate) fn new(name: String) -> Self {
        let mut dir = TDirectoryFile::default();
        dir.dir.named.name = name;
        dir
    }

    pub fn read_keys(&mut self, file: &mut RootFile) -> Result<()> {
        if self.seek_keys <= 0 {
            return Err(Error::DirectoryNegativeSeekKeys(self.seek_keys));
        }

        let data = file.read_at(self.seek_keys as u64, self.n_bytes_keys as u64)?;
        trace!("read_keys data = {:?}", data);

        let mut r = RBuffer::new(&data, 0);

        let hdr = r.read_object_into::<Key>()?;

        trace!("read_keys: hdr = {:?}", hdr);

        let data = file.read_at(
            self.seek_keys as u64 + hdr.key_len() as u64,
            hdr.obj_len() as u64,
        )?;

        let mut r = RBuffer::new(&data, 0);
        let nkeys = r.read_i32()?;

        trace!("read_keys: nkeys = {:?}", nkeys);

        for i in 0..nkeys {
            trace!("read_keys: i = {:?}", i);
            let mut k = r.read_object_into::<Key>()?;
            trace!("read_keys: k = {:?}", k);
            if k.class() == "TDirectory" {
                k.set_class("TDirectoryFile");
            }

            self.keys.push(k);
        }

        // todo!()
        Ok(())
    }

    pub fn read_dir_info(file: &mut RootFile) -> Result<TDirectoryFile> {
        let nbytesname = file.n_bytes_name() as i64;
        let nbytes = nbytesname as u64 + TDirectoryFile::record_size(file.version()) as u64;
        let begin = file.begin() as u64;

        trace!(
            "have to read nbytes = {} from {} to {}",
            nbytes,
            begin,
            begin + nbytes
        );

        if (nbytes + begin) > (file.end() as u64) {
            return Err(crate::riofs::error::Error::FileHasAnIncorrectHeaderLength);
        }

        let data = file.read_at(begin, nbytes)?;
        let nbytesname = nbytesname as usize;

        let mut r = RBuffer::new(&data[nbytesname..], 0);
        let mut dir = r.read_object_into::<TDirectoryFile>()?;
        // r.read_object(&mut file.dir);

        let mut nk = 4; // KEY::fNumberOfBytes
        let mut r = RBuffer::new(&data[nk..], 0);
        let key_version = r.read_i16()?;

        if key_version > 1000 {
            // large files
            nk += 2; // KEY::fVersion
            nk += 2 * 4; // KEY::fObjectSize, Date
            nk += 2 * 2; // KEY::fKeyLength, fCycle
            nk += 2 * 8; // KEY::fSeekKey, fSeekParentDirectory
        } else {
            nk += 2; // KEY::fVersion
            nk += 2 * 4; // KEY::fObjectSize, Date
            nk += 2 * 2; // KEY::fKeyLength, fCycle
            nk += 2 * 4; // KEY::fSeekKey, fSeekParentDirectory
        }

        let mut r = RBuffer::new(&data[nk..], 0);
        dir.class_name = r.read_string()?.to_string();
        dir.dir.named.name = r.read_string()?.to_string();
        dir.dir.named.title = r.read_string()?.to_string();

        if dir.n_bytes_name < 10 || dir.n_bytes_name > 1000 {
            return Err(crate::riofs::error::Error::CantReadDirectoryInfo {
                n_bytes_name_read: dir.n_bytes_name,
                n_bytes_name_min_allowed: 10,
                n_bytes_name_max_allowed: 1000,
            });
        }

        Ok(dir)
    }

    pub(crate) fn close(&mut self, file: &mut RootFile) -> Result<()> {
        trace!(";TDirectoryFile.close:{:?}", true);
        self.save(file)?;
        Ok(())
    }

    fn save(&mut self, file: &mut RootFile) -> Result<()> {
        self.save_keys(file)?;
        self.write_header(file)?;

        // TODO: implement case where we have sub dirs

        Ok(())
    }

    fn save_keys(&mut self, file: &mut RootFile) -> Result<()> {
        trace!(";TDirectoryFile.save_keys:{:?}", true);
        trace!(
            ";TDirectoryFile.save_keys.n_bytes_keys:{:?}",
            self.n_bytes_keys
        );
        trace!(";TDirectoryFile.save_keys.keys.len:{:?}", self.keys.len());

        let mut n_bytes = 4;

        if file.is_big_file() {
            n_bytes += 8;
        }

        for (i, key) in self.keys.iter().enumerate() {
            trace!(
                ";TDirectoryFile.save_keys.for_loop.0.{i}.name:{:?}",
                key.name()
            );
            trace!(
                ";TDirectoryFile.save_keys.for_loop.0.{i}.key_len:{:?}",
                key.key_len()
            );
            n_bytes += key.key_len();
        }

        let mut hdr = Key::new(
            self.dir().named().name.clone(),
            self.dir().named().title.clone(),
            "TDirectory".to_string(),
            n_bytes,
            file,
        )?;

        let mut buf = WBuffer::new(0);
        buf.write_i32(self.keys.len() as i32)?;

        for (i, key) in self.keys.iter().enumerate() {
            trace!(
                ";TDirectoryFile.save_keys.for_loop.1.{i}.name:{:?}",
                key.name()
            );
            trace!(
                ";TDirectoryFile.save_keys.for_loop.1.{i}.pos_before_marshal:{:?}",
                buf.pos()
            );
            key.marshal(&mut buf)?;
            trace!(
                ";TDirectoryFile.save_keys.for_loop.1.{i}.pos_after_marshal:{:?}",
                buf.pos()
            );
        }
        hdr.set_buffer(buf.buffer(), false);

        self.seek_keys = hdr.seek_key();
        trace!(";TDirectoryFile.save_keys.seek_keys:{:?}", self.seek_keys);
        self.n_bytes_keys = hdr.n_bytes();
        trace!(
            ";TDirectoryFile.save_keys.n_bytes_keys:{:?}",
            self.n_bytes_keys
        );

        hdr.write_to_file(file.writer()?)?;
        trace!(";TDirectoryFile.save_keys.f.end:{:?}", file.end());

        Ok(())
    }

    fn write_header(&mut self, file: &mut RootFile) -> Result<()> {
        self.mtime = utils::now();

        let n_bytes = Self::record_size(file.version()) as i32;
        trace!(";TDirectoryFile.write_header.n_bytes:{:?}", n_bytes);
        trace!(
            ";TDirectoryFile.write_header.file.version:{:?}",
            file.version()
        );

        let mut buf = WBuffer::new(0);
        self.marshal(&mut buf)?;

        let buf = buf.buffer();
        trace!(";TDirectoryFile.write_header.buf.len:{:?}", buf.len());
        trace!(";TDirectoryFile.write_header.buf.value:{:?}", &buf);
        let wstart = self.seek_dir as u64 + self.n_bytes_name as u64;
        trace!(";TDirectoryFile.write_header.buf.wstart:{:?}", wstart);

        file.write_at(&buf, wstart)?;

        Ok(())
    }

    pub(crate) fn get_object(
        &self,
        namecycle: &str,
        file: &mut RootFileReader,
        ctx: Option<&dyn StreamerInfoContext>,
    ) -> Result<Box<dyn FactoryItemRead>> {
        trace!("get_object, namecycle = {}", namecycle);

        let (name, cycle) = decode_name_cycle(namecycle)?;
        trace!("get_object, name = {}", name);
        trace!("self.keys.len = {}", self.keys.len());

        let mut keys = self
            .keys
            .iter()
            .filter(|k| {
                if k.name() == name {
                    if cycle != 9999 {
                        todo!();
                    }
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        let key = match keys.len() {
            0 => {
                return Err(Error::KeyNotInFile {
                    key: name.to_string(),
                    file: file.to_string(),
                })
            }
            1 => keys.remove(0),
            _ => {
                keys.sort_by_key(|k| k.cycle());
                keys[keys.len() - 1]

                // unimplemented!()
            }
        };

        let obj = key.object(file, ctx)?;

        match obj {
            None => Err(Error::ObjectNotInDirectory(namecycle.to_string())),
            Some(o) => Ok(o),
        }
    }

    pub fn record_size(version: i32) -> i64 {
        let mut nbytes: i64 = 0;
        nbytes += 2; // fVersion
        nbytes += 4; // ctime
        nbytes += 4; // mtime
        nbytes += 4; // nbyteskeys
        nbytes += 4; // nbytesname
        if version >= 40000 {
            // assume that the file may be above 2 Gbytes if file version is > 4
            nbytes += 8; // seekdir
            nbytes += 8; // seekparent
            nbytes += 8; // seekkeys
        } else {
            nbytes += 4; // seekdir
            nbytes += 4; // seekparent
            nbytes += 4; // seekkeys
        }

        let uuid_size = 18;

        nbytes += uuid_size;

        nbytes
    }
    pub(crate) fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
    pub fn dir(&self) -> &TDirectory {
        &self.dir
    }
    pub(crate) fn is_big_file(&self) -> bool {
        self.dir.rvers > 1000
    }
}

impl Unmarshaler for TDirectoryFile {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let version = r.read_i16()?;
        trace!("version: {}", version);

        let ctime = r.read_u32()?;
        let mtime = r.read_u32()?;

        let ctime = DateTime::from_timestamp(ctime as i64, 0).unwrap();
        let mtime = DateTime::from_timestamp(mtime as i64, 0).unwrap();
        trace!("read ctime = {}", ctime);

        let n_bytes_keys = r.read_i32()?;
        let n_bytes_name = r.read_i32()?;

        let is_big_file = version > 1000;

        let seek_dir = if is_big_file {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };
        let seek_parent = if is_big_file {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };
        let seek_keys = if is_big_file {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        let _ = r.read_u16()?;
        let mut uuid: [u8; 16] = [0; 16];
        // r.read(&mut uuid)?;
        r.read_array_u8_into(&mut uuid)?;
        let uuid = Uuid::from_bytes(uuid);

        self.ctime = ctime;
        self.mtime = mtime;
        self.n_bytes_keys = n_bytes_keys;
        self.n_bytes_name = n_bytes_name;
        self.seek_dir = seek_dir;
        self.seek_parent = seek_parent;
        self.seek_keys = seek_keys;
        self.dir = TDirectory {
            _uuid: uuid,
            rvers: version,
            ..Default::default()
        };

        Ok(())
    }
}

impl Marshaler for TDirectoryFile {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let beg = w.pos();
        let version = self.dir().rvers;
        trace!(";TDirectoryFile.marshal.beg:{:?}", beg);
        trace!(";TDirectoryFile.marshal.version:{:?}", version);
        w.write_i16(version)?;

        trace!(";TDirectoryFile.marshal.ctime.date:{:?}", self.ctime);
        let t = datetime_to_u32(self.ctime);
        trace!(";TDirectoryFile.marshal.ctime.u32:{:?}", t);
        w.write_u32(t)?;
        w.write_u32(datetime_to_u32(self.mtime))?;
        trace!(
            ";TDirectoryFile.marshal.n_bytes_keys:{:?}",
            self.n_bytes_keys
        );
        w.write_i32(self.n_bytes_keys)?;
        trace!(
            ";TDirectoryFile.marshal.n_bytes_name:{:?}",
            self.n_bytes_name
        );
        w.write_i32(self.n_bytes_name)?;

        if self.is_big_file() {
            w.write_i64(self.seek_dir)?;
            w.write_i64(self.seek_parent)?;
            w.write_i64(self.seek_keys)?;
        } else {
            w.write_i32(self.seek_dir as i32)?;
            w.write_i32(self.seek_parent as i32)?;
            w.write_i32(self.seek_keys as i32)?;
        }

        trace!(";TDirectoryFile.marshal.seek_dir:{:?}", self.seek_dir);
        trace!(";TDirectoryFile.marshal.seek_parent:{:?}", self.seek_parent);
        trace!(";TDirectoryFile.marshal.seek_keys:{:?}", self.seek_keys);

        // TODO: marshal uuid version
        w.write_i16(rvers::UUID)?;

        w.write_array_u8(self.dir._uuid.as_ref())?;
        let end = w.pos();
        trace!(";TDirectoryFile.marshal.end:{:?}", end);
        Ok(end - beg)
    }
}
