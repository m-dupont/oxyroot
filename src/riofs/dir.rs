use crate::riofs::file::{RootFile, RootFileReader};

use crate::rbase::named::Named as ObjNamed;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::{StreamerInfoContext, Unmarshaler};
use crate::riofs::key::Key;
use crate::riofs::utils::decode_name_cycle;
use crate::root::traits::Named;
use crate::root::traits::Object;

use crate::riofs::{Error, Result};
use chrono::NaiveDateTime;
use log::trace;
use uuid::Uuid;

use crate::rtypes::FactoryItem;

#[derive(Default)]
pub struct TDirectory {
    _rvers: i16,
    _uuid: Uuid,
    named: ObjNamed,
}

pub struct TDirectoryFile {
    ctime: NaiveDateTime,
    mtime: NaiveDateTime, //
    n_bytes_keys: i32,
    n_bytes_name: i32,
    // seekdir: i64,
    // seekparent: i64,
    // seekkeys: i64,
    dir: TDirectory,
    pub seek_dir: i64,
    pub seek_parent: i64,
    pub seek_keys: i64,
    class_name: String,
    keys: Vec<Key>,
}

impl Default for TDirectoryFile {
    fn default() -> Self {
        TDirectoryFile {
            ctime: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
            mtime: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
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

impl TDirectoryFile {
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

        let mut nk = 4; // Key::fNumberOfBytes
        let mut r = RBuffer::new(&data[nk..], 0);
        let key_version = r.read_i16()?;

        if key_version > 1000 {
            // large files
            nk += 2; // Key::fVersion
            nk += 2 * 4; // Key::fObjectSize, Date
            nk += 2 * 2; // Key::fKeyLength, fCycle
            nk += 2 * 8; // Key::fSeekKey, fSeekParentDirectory
        } else {
            nk += 2; // Key::fVersion
            nk += 2 * 4; // Key::fObjectSize, Date
            nk += 2 * 2; // Key::fKeyLength, fCycle
            nk += 2 * 4; // Key::fSeekKey, fSeekParentDirectory
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

    pub(crate) fn get_object(
        &self,
        namecycle: &str,
        file: &mut RootFileReader,
        ctx: Option<&dyn StreamerInfoContext>,
    ) -> Result<Box<dyn FactoryItem>> {
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

        trace!("record size = {}", nbytes);

        nbytes
    }
    pub(crate) fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

impl Unmarshaler for TDirectoryFile {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let version = r.read_i16()?;
        trace!("version: {}", version);

        let ctime = r.read_u32()?;
        let mtime = r.read_u32()?;

        let ctime = NaiveDateTime::from_timestamp_opt(ctime as i64, 0).unwrap();
        let mtime = NaiveDateTime::from_timestamp_opt(mtime as i64, 0).unwrap();
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
            _rvers: version,
            ..Default::default()
        };

        Ok(())
    }
}
