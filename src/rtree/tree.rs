use crate::factotry_all_for_register_impl;
use crate::file::RootFileReader;
use crate::rbase;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::rcont::objarray::ObjArray;
use crate::root::traits::Named;
use crate::root::traits::Object;
use crate::rtree::branch::{Branch, TBranch};
use crate::rvers;
use anyhow::{bail, ensure};
use log::{debug, trace};
use std::io::Read;

#[derive(Default)]
pub struct Clusters {
    ranges: Vec<i64>,
    sizes: Vec<i64>,
}

// pub struct TioFeatures {
//     val:
// }

#[derive(Default, Debug)]
pub struct TioFeatures(u8);

impl Unmarshaler for TioFeatures {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("TioFeatures:unmarshal");

        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::ROOT_IOFeatures,
            "rtree: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::ROOT_IOFeatures
        );

        let mut buf = [0 as u8; 4];
        r.read(&mut buf[..1])?;

        trace!("buf = {:?}", buf);

        self.0 = if buf[0] != 0 {
            trace!("buf[0] = {:?}", buf[0]);
            r.read(&mut buf[1..])?;
            r.read_u8()?
        } else {
            0
        };

        r.check_header(&hdr)?;

        Ok(())

        // trace!("buf = {:?}", buf);
        //
        // todo!()
    }
}

factotry_all_for_register_impl!(TioFeatures, "TIOFeatures");

#[derive(Default)]
pub struct Tree {
    rvers: i16,
    named: rbase::Named,
    attline: rbase::AttLine,
    attfill: rbase::AttFill,
    attmarker: rbase::AttMarker,

    /// Number of entries
    entries: i64,
    /// Total number of bytes in all branches before compression
    totBytes: i64,
    /// Total number of bytes in all branches after  compression
    zipBytes: i64,
    /// number of autosaved bytes
    savedBytes: i64,
    /// number of auto-flushed bytes
    flushedBytes: i64,

    /// tree weight
    weight: f64,
    /// timer interval in milliseconds
    timerInterval: i32,
    /// number of runs before prompting in Scan
    scanField: i32,
    /// update frequency for entry-loop
    update: i32,
    /// initial length of the entry offset table in the basket buffers
    defaultEntryOffsetLen: i32,
    /// maximum number of entries in case of circular buffers
    maxEntries: i64,
    /// maximum number of entries to process
    maxEntryLoop: i64,
    /// maximum total size of buffers kept in memory
    maxVirtualSize: i64,
    /// autosave tree when autoSave entries written
    autoSave: i64,
    /// autoflush tree when autoFlush entries written
    autoFlush: i64,
    /// number of entries to estimate histogram limits
    estimate: i64,

    clusters: Clusters,
    iobits: TioFeatures,

    branches: Vec<Branch>,

    reader: Option<RootFileReader>,
}

impl Tree {
    pub fn set_reader(&mut self, reader: Option<RootFileReader>) {
        if let Some(ref r) = reader {
            for b in self.branches.iter_mut() {
                b.set_reader(Some(r.clone()));
            }
            self.reader = reader;
        }
    }

    pub fn get_branch(&self, name: &str) -> Option<&Branch> {
        for b in self.branches.iter() {
            if b.name() == name {
                return Some(b.into());
            }
        }
        None
    }

    pub fn branches(&self) -> impl Iterator<Item = &TBranch> {
        // self.branches.iter().map()

        self.branches.iter().map(|b| b.into())
    }
}

impl Unmarshaler for Tree {
    fn unmarshal(&mut self, r: &mut RBuffer) -> anyhow::Result<()> {
        trace!("Tree:unmarshal");
        let hdr = r.read_header(self.class())?;
        ensure!(
            hdr.vers <= rvers::Tree,
            "rtree: invalid {} version={} > {}",
            self.class(),
            hdr.vers,
            rvers::Tree
        );

        self.rvers = hdr.vers;
        r.read_object(&mut self.named)?;
        r.read_object(&mut self.attline)?;
        r.read_object(&mut self.attfill)?;
        r.read_object(&mut self.attmarker)?;

        if hdr.vers <= 4 {
            bail!(
                "rtree: tree {} with version {} is not supported (too old)",
                self.name(),
                hdr.vers,
            )
        }

        if hdr.vers > 5 {
            self.entries = r.read_i64()?;
            self.totBytes = r.read_i64()?;
            self.zipBytes = r.read_i64()?;
            self.savedBytes = r.read_i64()?;
        } else {
            self.entries = r.read_f64()? as i64;
            self.totBytes = r.read_f64()? as i64;
            self.zipBytes = r.read_f64()? as i64;
            self.savedBytes = r.read_f64()? as i64;
        }

        trace!("nentries = {}", self.entries);

        if hdr.vers >= 18 {
            self.flushedBytes = r.read_i64()?;
        }

        if hdr.vers >= 16 {
            self.weight = r.read_f64()?;
        }

        self.timerInterval = r.read_i32()?;
        self.scanField = r.read_i32()?;
        self.update = r.read_i32()?;

        if hdr.vers >= 17 {
            self.defaultEntryOffsetLen = r.read_i32()?;
        }

        let mut nclus = 0;

        if hdr.vers >= 19 {
            nclus = r.read_i32()?;
        }

        if hdr.vers > 5 {
            self.maxEntries = r.read_i64()?;
        }

        if hdr.vers > 5 {
            self.maxEntryLoop = r.read_i64()?;
            self.maxVirtualSize = r.read_i64()?;
            self.autoSave = r.read_i64()?;
        } else {
            self.maxEntryLoop = r.read_i32()? as i64;
            self.maxVirtualSize = r.read_i32()? as i64;
            self.autoSave = r.read_i32()? as i64;
        }

        if hdr.vers >= 18 {
            self.autoFlush = r.read_i64()?;
        }

        if hdr.vers > 5 {
            self.estimate = r.read_i64()?;
        } else {
            self.estimate = r.read_i32()? as i64;
        }

        if hdr.vers >= 19 {
            self.clusters.ranges = vec![0 as i64; nclus as usize];
            self.clusters.sizes = vec![0 as i64; nclus as usize];
            let _ = r.read_i8();
            r.read_array_i64(&mut self.clusters.ranges)?;

            let _ = r.read_i8();
            r.read_array_i64(&mut self.clusters.sizes)?;

            trace!("ranges = {:?}", self.clusters.ranges);
        }

        if hdr.vers >= 20 {
            r.read_object(&mut self.iobits)?;
        }

        trace!("iobits = {:?}", self.iobits);

        {
            let mut branches = r.read_object_into::<ObjArray>()?;

            // for obj in branches.take_objs().into_iter() {
            //     trace!("convert branch");
            //     obj.downcast::<TBranch>().unwrap();
            // }

            self.branches = branches
                .take_objs()
                .into_iter()
                .map(|obj| obj.into())
                .collect();
        }

        {
            let mut leaves = r.read_object_into::<ObjArray>()?;
        }

        debug!("read new element");
        if hdr.vers > 5 {
            let v = r.read_object_any_into()?;
            if v.is_some() {
                todo!()
            }
        }

        {
            //tree.indexValues
            let v = r.read_object_any_into()?;
            if v.is_some() {
                todo!()
            }
        }

        {
            //tree.index
            let v = r.read_object_any_into()?;
            if v.is_some() {
                todo!()
            }
        }

        if hdr.vers > 5 {
            //tree.index
            let v = r.read_object_any_into()?;
            if v.is_some() {
                todo!()
            }

            //tree.friends
            let v = r.read_object_any_into()?;
            if v.is_some() {
                todo!()
            }

            //tree.userInfo
            let v = r.read_object_any_into()?;
            if v.is_some() {
                todo!()
            }

            //tree.branchRef
            let v = r.read_object_any_into()?;
            if v.is_some() {
                todo!()
            }
        }

        Ok(())
        // todo!()
    }
}

factotry_all_for_register_impl!(Tree, "TTree");
