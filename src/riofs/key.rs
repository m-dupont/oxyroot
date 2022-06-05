use crate::file::RootFile;
use crate::rbytes::rbuffer::{RBuffer, Rbuff};
use crate::rbytes::{Unmarshaler, UnmarshalerInto};
use crate::rcompress;
use crate::root::{objects, traits};
use crate::rtypes;
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use log::trace;
use std::fmt;

// pub struct KeyObject(Option<Box<dyn Object>>);

impl fmt::Debug for objects::Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Object")
            .field("class", &self.class())
            .finish()
        // Ok(())
    }
}
// impl fmt::Debug for KeyObject {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let op = &self.0;
//         match op {
//             None => f
//                 .debug_struct("KeyObject")
//                 .field("Object", &String::from("None"))
//                 .finish(),
//             Some(obj) => f
//                 .debug_struct("KeyObject")
//                 .field("Object", &obj.class())
//                 .finish(),
//         }
//     }
// }

#[derive(Debug)]
pub struct Key {
    rvers: i16,
    // version of the Key struct
    n_bytes: i32,
    // number of bytes for the compressed object+key
    obj_len: i32,
    // length of uncompressed object
    datetime: NaiveDateTime,
    // Date/Time when the object was written
    key_len: i32,
    // number of bytes for the Key struct
    cycle: i16, // cycle number of the object

    // address of the object on file (points to Key.bytes)
    // this is a redundant information used to cross-check
    // the data base integrity
    seek_key: i64,
    seek_pdir: i64, // pointer to the directory supporting this object

    class: String,
    // object class name
    name: String,
    // name of the object
    title: String, // title of the object

    obj: Option<objects::Object>,
}

impl Default for Key {
    fn default() -> Self {
        Key {
            rvers: 0,
            n_bytes: 0,
            obj_len: 0,
            datetime: NaiveDateTime::from_timestamp(0, 0),
            key_len: 0,
            cycle: 0,
            seek_key: 0,
            seek_pdir: 0,
            class: String::new(),
            name: String::new(),
            title: String::new(),
            obj: None,
        }
    }
}

impl Unmarshaler for Key {
    fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()> {
        let n_bytes = r.read_i32()?;

        if n_bytes < 0 {
            self.n_bytes = n_bytes;
            self.class = String::from("[GAP]");
            return Ok(());
        }

        let rvers = r.read_i16()?;
        let obj_len = r.read_i32()?;
        let datetime = NaiveDateTime::from_timestamp(r.read_u32()? as i64, 0);
        let key_len = r.read_i16()? as i32;
        let cycle = r.read_i16()?;

        let is_big_file = rvers > 1000;
        let seek_key = if is_big_file {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };
        let seek_pdir = if is_big_file {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        let class = r.read_string()?.to_string();
        let name = r.read_string()?.to_string();
        let title = r.read_string()?.to_string();

        self.rvers = rvers;
        self.n_bytes = n_bytes;
        self.obj_len = obj_len;
        self.datetime = datetime;
        self.key_len = key_len;
        self.cycle = cycle;
        self.seek_key = seek_key;
        self.seek_pdir = seek_pdir;
        self.class = class;
        self.name = name;
        self.title = title;
        Ok(())
    }
}

// impl UnmarshalerInto for Key {
//     type Item = Self;
//
//     fn unmarshal_into(r: &mut RBuffer) -> anyhow::Result<Self::Item> {
//         let n_bytes = r.read_i32()?;
//
//         if n_bytes < 0 {
//             return Ok(Key {
//                 n_bytes,
//                 class: String::from("[GAP]"),
//                 ..Default::default()
//             });
//         }
//
//         let rvers = r.read_i16()?;
//         let obj_len = r.read_i32()?;
//         let datetime = NaiveDateTime::from_timestamp(r.read_u32()? as i64, 0);
//         let key_len = r.read_i16()? as i32;
//         let cycle = r.read_i16()?;
//
//         let is_big_file = rvers > 1000;
//         let seek_key = if is_big_file {
//             r.read_i64()?
//         } else {
//             r.read_i32()? as i64
//         };
//         let seek_pdir = if is_big_file {
//             r.read_i64()?
//         } else {
//             r.read_i32()? as i64
//         };
//
//         let class = r.read_string()?.to_string();
//         let name = r.read_string()?.to_string();
//         let title = r.read_string()?.to_string();
//
//         Ok(Key {
//             rvers,
//             n_bytes,
//             obj_len,
//             datetime,
//             key_len,
//             cycle,
//             seek_key,
//             seek_pdir,
//             class,
//             name,
//             title,
//             obj: None,
//         })
//
//         // todo!()
//     }
// }

impl Key {
    fn is_compressed(&self) -> bool {
        self.obj_len != self.n_bytes - self.key_len
    }

    pub fn bytes(&self, file: &mut RootFile, buf: Option<&[u8]>) -> Result<Vec<u8>> {
        self.load(file)
    }

    fn load(&self, file: &mut RootFile) -> Result<Vec<u8>> {
        if self.is_compressed() {
            let mut buf = vec![0 as u8; self.obj_len as usize];
            trace!("load, is_compressed");
            let start = self.seek_key as u64 + self.key_len as u64;
            let mut sr = file.read_at(start, (self.n_bytes as u64) - (self.key_len as u64))?;
            trace!("sr = {:?}", sr);

            if let Ok(a) = rcompress::decompress(&mut buf, &sr) {
                trace!("buf = {:?}..", &buf);
                return Ok(buf);
            } else {
                return Err(anyhow!("riofs: could not decompress key payload"));
            }

            // trace!("buf = ..{:?}", buf.iter().rev().collect::<u8>()[0..100]);
        }

        let start = self.seek_key as u64 + self.key_len as u64;

        trace!("read from {} for {}", start, self.obj_len);

        let buf = file.read_at(start, self.obj_len as u64)?;
        trace!("buf = {:?}", buf);
        Ok(buf)
    }

    pub fn object(&self, file: &mut RootFile) -> Result<Option<&objects::Object>> {
        // return &self.obj;

        if let Some(ref obj) = self.obj {
            return Ok(Some(&obj));
        }

        let buf = self
            .bytes(file, None)
            .map_err(|e| anyhow!("riofs: could not load key payload: {}", e))?;

        trace!("k.class = {}", self.class);

        let fct = rtypes::FACTORY.get(&self.class).ok_or(anyhow!(
            "riofs: no registered factory for class {} (key={})",
            self.class,
            self.name
        ))?;

        let v = fct();
        //obj, ok := v.Interface().(root.Object)
        let obj: Box<dyn rtypes::FactoryItem> = v;

        // vv, ok := obj.(rbytes.Unmarshaler)
        let mut vv: Box<dyn rtypes::FactoryItem> = obj;

        vv.unmarshal(&mut RBuffer::new(&buf, self.key_len as u32))?;

        todo!();

        // let vv: Box<dyn Unmarshaler> = v.downcast();

        // if let Some(fct) = rtypes::FACTORY.get(&self.class);

        Ok(None)

        // Err(anyhow!("euh"))
    }
}
