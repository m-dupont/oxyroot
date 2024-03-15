use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{Marshaler, StreamerInfoContext, Unmarshaler};
use crate::riofs::consts::kStartBigFile;
use crate::riofs::dir::TDirectoryFile;
use crate::riofs::file::{RootFileReader, RootFileWriter};
use crate::riofs::utils::datetime_to_u32;
use crate::riofs::{utils, Result};
use crate::root::traits::{datimeSizeof, tstringSizeof, Named};
use crate::root::{objects, traits};
use crate::rtypes::FactoryItem;
use crate::{rcompress, riofs, rvers};
use crate::{rtypes, RootFile};
use chrono::{DateTime, NaiveDateTime, Utc};
use log::trace;
use std::fmt;
use std::fmt::Debug;
use utils::now;

// pub struct KeyObject(Option<Box<dyn OBJECT>>);

impl fmt::Debug for objects::Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OBJECT")
            .field("class", &self.class())
            .finish()
        // Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Key {
    rvers: i16,
    // version of the Key struct
    n_bytes: i32,
    // number of bytes for the compressed object+key
    obj_len: i32,
    // length of uncompressed object
    datetime: DateTime<Utc>,
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

    // number of bytes left in current segment
    left: i32,

    buffer: Vec<u8>, // buffer of the Key's value
}

impl Default for Key {
    fn default() -> Self {
        Key {
            rvers: rvers::Key,
            n_bytes: 0,
            obj_len: 0,
            datetime: utils::now(),
            key_len: 0,
            cycle: 1,
            seek_key: 100,
            seek_pdir: 0,
            left: 0,
            class: String::new(),
            name: String::new(),
            title: String::new(),
            buffer: Vec::new(),
        }
    }
}

impl Unmarshaler for Key {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let n_bytes = r.read_i32()?;

        if n_bytes < 0 {
            self.n_bytes = n_bytes;
            self.class = String::from("[GAP]");
            return Ok(());
        }

        let rvers = r.read_i16()?;
        let obj_len = r.read_i32()?;
        let datetime = DateTime::from_timestamp(r.read_u32()? as i64, 0).unwrap();
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

impl Marshaler for Key {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let beg = w.pos();
        trace!(";Key.marshal.{beg}.beg:{:?}", beg);
        trace!(";Key.marshal.{beg}.n_bytes:{:?}", self.n_bytes);
        w.write_i32(self.n_bytes)?;

        if self.n_bytes < 0 {
            panic!("n_bytes < 0");
            return Ok(w.pos() - beg);
        }

        let rvers = if self.seek_key > kStartBigFile && self.rvers < 1000 {
            self.rvers + 1000
        } else {
            self.rvers
        };

        trace!(";Key.marshal.{beg}.rvers:{:?}", rvers);
        w.write_i16(rvers)?;
        trace!(";Key.marshal.{beg}.obj_len:{:?}", self.obj_len);
        w.write_i32(self.obj_len)?;
        w.write_u32(datetime_to_u32(self.datetime))?;
        trace!(";Key.marshal.{beg}.key_len:{:?}", self.key_len);
        w.write_i16(self.key_len as i16)?;
        trace!(";Key.marshal.{beg}.cycle:{:?}", self.cycle);
        w.write_i16(self.cycle)?;

        if self.rvers > 1000 {
            w.write_i64(self.seek_key)?;
            w.write_i64(self.seek_pdir)?;
        } else {
            w.write_i32(self.seek_key as i32)?;
            w.write_i32(self.seek_pdir as i32)?;
        }

        trace!(";Key.marshal.{beg}.seek_key:{:?}", self.seek_key);
        trace!(";Key.marshal.{beg}.seek_pdir:{:?}", self.seek_pdir);
        trace!(";Key.marshal.{beg}.class:{:?}", self.class);
        w.write_string(&self.class)?;
        trace!(";Key.marshal.{beg}.name:{:?}", self.name);
        w.write_string(&self.name)?;
        trace!(";Key.marshal.{beg}.title:{:?}", self.title);
        w.write_string(&self.title)?;

        trace!(";Key.marshal.{beg}.buf:{:?}", w);

        let end = w.pos();
        trace!(";Key.marshal.{beg}.end:{:?}", end);

        Ok(end - beg)
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
    pub(crate) fn new(
        name: String,
        title: String,
        class: String,
        obj_len: i32,
        f: &mut RootFile,
    ) -> Result<Self> {
        let mut key = Key {
            key_len: key_len_for(&name, &title, &class, f),
            name,
            title,
            class,
            obj_len,

            ..Default::default()
        };
        key.n_bytes = key.obj_len + key.key_len;
        let eof = f.end();
        if obj_len > 0 {
            key.seek_key = eof;
            f.set_end(key.seek_key + key.n_bytes as i64)?;
        }
        if f.end() > kStartBigFile {
            key.rvers += 1000
        }

        key.seek_pdir = f.dir().seek_dir;

        Ok(key)
    }

    pub(crate) fn new_from_buffer(
        name: String,
        title: String,
        class: String,
        cycle: i16,
        buf: Vec<u8>,
        f: &mut RootFile,
    ) -> Result<Self> {
        let indent = name.clone() + "-" + &title;
        trace!(";Key.new_from_buffer.buf.value:{:?}", &buf);
        let key_len = key_len_for(&name, &title, &class, f);
        let obj_len = buf.len() as i32;
        let mut key = Key {
            key_len,
            name,
            title,
            class,
            obj_len,
            n_bytes: key_len + obj_len,
            seek_key: f.end(),
            seek_pdir: f.dir().seek_dir,
            ..Default::default()
        };

        if f.is_big_file() {
            key.rvers += 1000;
        }

        key.buffer = rcompress::compress(buf, f.compression())?;
        trace!(
            ";Key.new_from_buffer.buf.after_compression:{:?}",
            key.buffer
        );
        key.n_bytes = key.key_len + key.buffer.len() as i32;
        f.set_end(key.seek_key + key.n_bytes as i64)?;

        Ok(key)
    }

    pub(crate) fn set_buffer(&mut self, buffer: Vec<u8>, update_obj_len: bool) {
        self.buffer = buffer;
        if update_obj_len {
            self.obj_len = self.buffer.len() as i32;
        }
    }

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
    ) -> riofs::Result<Option<Box<dyn FactoryItem>>> {
        // return &self.obj;

        // if let Some(ref obj) = self.obj {
        //     return Ok(Some(obj));
        // }

        let buf = self.bytes(file, None)?;

        let fct = rtypes::FACTORY.get(&self.class)?;

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

        Ok(obj)
    }
    pub(crate) fn key_len(&self) -> i32 {
        self.key_len
    }
    pub(crate) fn set_class(&mut self, class: &str) {
        self.class = class.to_string();
    }

    pub(crate) fn cycle(&self) -> i32 {
        self.cycle as i32
    }
    pub fn n_bytes(&self) -> i32 {
        self.n_bytes
    }
    pub fn seek_key(&self) -> i64 {
        self.seek_key
    }
    pub fn buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    pub(crate) fn write_to_file(&self, w: &mut RootFileWriter) -> Result<()> {
        if self.left > 0 {
            panic!("left > 0")
        }

        let mut buf = WBuffer::new(0);
        self.marshal(&mut buf)?;

        let buf = buf.buffer();
        trace!(";Key.write_to_file.buf.value:{:?}", buf);
        trace!(";Key.write_to_file.buf.value:{:x?}", buf);
        trace!(";Key.write_to_file.buf.wstart:{:?}", self.seek_key);

        w.write_at(&buf, self.seek_key as u64)?;
        let buf = self.buffer();
        trace!(";Key.write_to_file.k.buf.value:{:?}", buf);
        trace!(";Key.write_to_file.k.buf.value:{:x?}", buf);
        trace!(
            ";Key.write_to_file.k.buf.wstart:{:?}",
            self.seek_key as u64 + self.key_len as u64
        );
        w.write_at(buf, self.seek_key as u64 + self.key_len as u64)?;

        Ok(())
    }
}

fn key_len_for(name: &str, title: &str, class: &str, f: &RootFile) -> i32 {
    // 	nbytes := int32(22)
    // 	if dir.isBigFile() || eof > kStartBigFile {
    // 		nbytes += 8
    // 	}
    // 	nbytes += datimeSizeof()
    // 	nbytes += tstringSizeof(class)
    // 	nbytes += tstringSizeof(name)
    // 	nbytes += tstringSizeof(title)
    // 	if class == "TBasket" {
    // 		nbytes += 2 // version
    // 		nbytes += 4 // bufsize
    // 		nbytes += 4 // nevsize
    // 		nbytes += 4 // nevbuf
    // 		nbytes += 4 // last
    // 		nbytes += 1 // flag
    // 	}
    // 	return nbytes
    // }

    let mut nbytes = 22;

    if f.dir().is_big_file() || f.end() > kStartBigFile {
        nbytes += 8;
    }

    nbytes += datimeSizeof();

    nbytes += tstringSizeof(class);
    nbytes += tstringSizeof(name);
    nbytes += tstringSizeof(title);

    if class == "TBasket" {
        nbytes += 2; // version
        nbytes += 4; // bufsize
        nbytes += 4; // nevsize
        nbytes += 4; // nevbuf
        nbytes += 4; // last
        nbytes += 1; // flag
    }

    nbytes
}
