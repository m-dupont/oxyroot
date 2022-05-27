use crate::file::RootFile;
use std::io::Read;

use crate::rbase::named::Named;
use crate::rbytes::rbuffer::{RBuffer, Rbuff};
use crate::rbytes::Unmarshaler;
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use log::trace;
use uuid::Uuid;

#[derive(Default)]
pub struct TDirectory {
    rvers: i16,
    uuid: Uuid,
    named: Named,
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
}

impl Default for TDirectoryFile {
    fn default() -> Self {
        TDirectoryFile {
            ctime: NaiveDateTime::from_timestamp(0, 0),
            mtime: NaiveDateTime::from_timestamp(0, 0),
            ..Default::default()
        }
    }
}

impl TDirectoryFile {
    pub fn read_dir_info(file: &mut RootFile) -> Result<()> {
        let header = file
            .header
            .as_ref()
            .ok_or(anyhow!("Header must exists here"))?;
        let nbytesname = header.n_bytes_name as i64;
        let nbytes = nbytesname as u64 + TDirectoryFile::record_size(header.version) as u64;
        let begin = header.begin as u64;

        trace!(
            "have to read nbytes = {} from {} to {}",
            nbytes,
            begin,
            begin + nbytes
        );

        if (nbytes + begin) > (header.end as u64) {
            return Err(anyhow!("file has an incorrect header length"));
        }

        let data = file.read_at(begin, nbytes)?;
        let nbytesname = nbytesname as usize;

        let mut r = RBuffer::new(&data[nbytesname..], 0);
        let mut dir = r.read_object::<TDirectoryFile>()?;

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
            return Err(anyhow!("riofs: can't read directory info"));
        }

        Ok(())
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
}

impl Unmarshaler for TDirectoryFile {
    type Item = TDirectoryFile;
    fn unmarshal_root(r: &mut RBuffer) -> Result<TDirectoryFile> {
        let version = r.read_i16()?;
        trace!("version: {}", version);

        let ctime = r.read_u32()?;
        let mtime = r.read_u32()?;

        let ctime = NaiveDateTime::from_timestamp(ctime as i64, 0);
        let mtime = NaiveDateTime::from_timestamp(mtime as i64, 0);
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
        r.read(&mut uuid);
        let uuid = Uuid::from_bytes(uuid);

        Ok(TDirectoryFile {
            ctime,
            mtime,
            n_bytes_keys,
            n_bytes_name,
            seek_dir,
            seek_parent,
            seek_keys,
            dir: TDirectory {
                uuid,
                rvers: version,
                ..Default::default()
            },
            ..Default::default()
        })
    }
}
