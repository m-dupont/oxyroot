use crate::rdict::error::Result;
use crate::rdict::streamers::streamers_db_gen::populate_db;
use crate::rdict::StreamerInfo;
use crate::riofs::dir::TDirectoryFile;
use crate::rtypes::factory::FactoryItemWrite;
use crate::rtypes::FactoryItemRead;
use lazy_static::lazy_static;
use log::trace;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

#[derive(Debug)]
pub struct DbStreamer {
    map: HashMap<String, StreamerInfo>,
}

impl DbStreamer {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: StreamerInfo) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str, vers: i16) -> Option<StreamerInfo> {
        let ret = if vers > 0 {
            let key = format!("{}-{}", key, vers);
            self.map.get(&key).cloned()
        } else {
            let start = format!("{}-", key);
            let key = DBSTREAMER.keys().find(|k| k.starts_with(&start)).unwrap();
            self.map.get(key).cloned()
        };
        match ret {
            None => None,
            Some(mut si) => {
                si.id = ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Some(si)
            }
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.map.keys()
    }
}

static DUMP: &str = include_str!("dump_from_root.txt");

lazy_static! {
    pub static ref DBSTREAMER: DbStreamer = {
        let mut db = DbStreamer::new();

        populate_db(&mut db, DUMP).unwrap();

        db
    };
    pub static ref ID: AtomicUsize = AtomicUsize::new(0);
}

pub(crate) fn streamer_info_from<T>(obj: &T, dir: &mut TDirectoryFile) -> Result<StreamerInfo>
where
    T: FactoryItemWrite,
{
    let typename = obj.class();
    let cxxtype = obj.class();
    let vers = -1;

    let vers = T::rversion(obj);

    trace!(";streamer_info_from.typename: {}", typename);
    trace!(";streamer_info_from.vers: {}", vers);

    let si = DBSTREAMER.get(typename, vers).unwrap();

    trace!(";streamer_info_from.si: {:?}", si);

    Ok(si)
}

pub(crate) fn streamer_info(name: &str, vers: i16) -> Result<StreamerInfo> {
    // trace!(";streamer_info.name: {:?}", name);

    let si = DBSTREAMER.get(name, vers).unwrap();

    // trace!(";streamer_info.si: {:?}", si);

    Ok(si)
}
