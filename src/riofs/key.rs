use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::{StreamerInfoContext, Unmarshaler};
use crate::rcompress;
use crate::riofs::dir::TDirectoryFile;
use crate::riofs::file::RootFileReader;
use crate::root::traits::Named;
use crate::root::{objects, traits};
use crate::rtypes;
use crate::rtypes::FactoryItem;
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use std::fmt;
use std::fmt::Debug;

// pub struct KeyObject(Option<Box<dyn OBJECT>>);

impl fmt::Debug for objects::Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OBJECT")
            .field("class", &self.class())
            .finish()
        // Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct Key {
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

        // todo!();

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

impl traits::Object for Key {
    fn class(&self) -> &'_ str {
        &self.class
    }
}

impl Named for Key {
    fn name(&self) -> &'_ str {
        &self.name
    }

    fn title(&self) -> &'_ str {
        &self.title
    }
}

impl Key {
    pub(crate) fn obj_len(&self) -> i32 {
        self.obj_len
    }

    fn is_compressed(&self) -> bool {
        self.obj_len != self.n_bytes - self.key_len
    }

    pub(crate) fn bytes(&self, file: &mut RootFileReader, _: Option<&[u8]>) -> Result<Vec<u8>> {
        self.load(file)
    }

    fn load(&self, file: &mut RootFileReader) -> Result<Vec<u8>> {
        if self.is_compressed() {
            let mut buf = vec![0; self.obj_len as usize];
            let start = self.seek_key as u64 + self.key_len as u64;
            let sr = file.read_at(start, (self.n_bytes as u64) - (self.key_len as u64))?;

            rcompress::decompress(&mut buf, &sr)?;

            return Ok(buf);
        }

        let start = self.seek_key as u64 + self.key_len as u64;

        let buf = file.read_at(start, self.obj_len as u64)?;
        Ok(buf)
    }

    pub(crate) fn object(
        &self,
        file: &mut RootFileReader,
        ctx: Option<&dyn StreamerInfoContext>,
    ) -> Result<Option<Box<dyn FactoryItem>>> {
        // return &self.obj;

        // if let Some(ref obj) = self.obj {
        //     return Ok(Some(obj));
        // }

        let buf = self
            .bytes(file, None)
            .map_err(|e| anyhow!("riofs: could not load key payload: {}", e))?;

        let fct = rtypes::FACTORY.get(&self.class).ok_or_else(|| {
            anyhow!(
                "riofs: no registered factory for class {} (key={})",
                self.class,
                self.name
            )
        })?;

        let v = fct();
        //obj, ok := v.Interface().(root.OBJECT)
        let obj: Box<dyn rtypes::FactoryItem> = v;

        // vv, ok := obj.(rbytes.Unmarshaler)
        let mut vv: Box<dyn rtypes::FactoryItem> = obj;

        // vv.unmarshal(&mut RBuffer::new(&buf, self.key_len as u32))?;
        vv.unmarshal(&mut RBuffer::new(&buf, self.key_len as u32).with_info_context(ctx))?;

        // self.objarr = *objs.downcast::<rcont::objarray::OBJ_ARRAY>().unwrap();

        let obj = Some(vv);

        if obj
            .as_ref()
            .unwrap()
            .downcast_ref::<TDirectoryFile>()
            .is_ok()
        {
            todo!();
        }

        // todo!();

        // let vv: Box<dyn Unmarshaler> = v.downcast();

        // if let Some(fct) = rtypes::FACTORY.get(&self.class);

        Ok(obj)

        // Err(anyhow!("euh"))
    }
    pub(crate) fn key_len(&self) -> i32 {
        self.key_len
    }
    pub(crate) fn set_class(&mut self, class: &str) {
        self.class = class.to_string();
    }
    // pub(crate) fn n_bytes(&self) -> i32 {
    //     self.n_bytes
    // }
    pub(crate) fn cycle(&self) -> i32 {
        self.cycle as i32
    }
}
