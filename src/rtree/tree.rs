use crate::rbase::{AttFill, AttLine};
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{
    ensure_maximum_supported_version, ensure_minimum_supported_version, Marshaler, RVersioner,
    Unmarshaler,
};
use crate::rcont::objarray::{ReaderObjArray, WriterObjArray};
use crate::riofs::file::{RootFileReader, RootFileStreamerInfoContext};
use crate::root::traits::Object;
use crate::rtree::branch::wbranch::WBranch;
use crate::rtree::branch::Branch;
use crate::{factory_all_for_register_impl, rvers, RootFile};
use crate::{rbase, Named};
use log::trace;
use std::fmt::Debug;

#[derive(Default)]
pub struct Clusters {
    ranges: Vec<i64>,
    sizes: Vec<i64>,
}

// pub struct TioFeatures {
//     val:
// }

#[derive(Default, Debug, Copy, Clone)]
pub struct TioFeatures(pub(crate) u8);

impl Unmarshaler for TioFeatures {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, crate::rvers::ROOT_IOFEATURES, self.class())?;

        let mut buf = [0_u8; 4];
        r.read_array_u8_into(&mut buf[..1])?;

        self.0 = if buf[0] != 0 {
            r.read_array_u8_into(&mut buf[1..])?;
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

impl Marshaler for TioFeatures {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let hdr = w.write_header(self.class(), Self::rversion(self))?;

        if self.0 != 0 {
            let buf = [0x1a, 0xa1, 0x2f, 0x10];
            w.write_array_u8(&buf)?;
        }
        w.write_u8(self.0)?;

        w.set_header(hdr)
    }
}

impl RVersioner for TioFeatures {
    fn rversion(&self) -> i16 {
        rvers::ROOT_IOFEATURES
    }
}

factory_all_for_register_impl!(TioFeatures, "TIOFeatures");
/// Rust equivalent of [`TTree`](https://root.cern/doc/master/classTTree.html)
///
/// Mainly used to retrieve [crate::Branch] ant iterate over them
pub struct Tree<B> {
    rvers: i16,
    named: rbase::Named,
    attline: rbase::AttLine,
    attfill: rbase::AttFill,
    attmarker: rbase::AttMarker,

    /// Number of entries
    entries: i64,
    /// Total number of bytes in all branches before compression
    tot_bytes: i64,
    /// Total number of bytes in all branches after  compression
    zip_bytes: i64,
    /// number of autosaved bytes
    saved_bytes: i64,
    /// number of auto-flushed bytes
    flushed_bytes: i64,

    /// tree weight
    weight: f64,
    /// timer interval in milliseconds
    timer_interval: i32,
    /// number of runs before prompting in Scan
    scan_field: i32,
    /// update frequency for entry-loop
    update: i32,
    /// initial length of the entry offset table in the basket buffers
    default_entry_offset_len: i32,
    /// maximum number of entries in case of circular buffers
    max_entries: i64,
    /// maximum number of entries to process
    max_entry_loop: i64,
    /// maximum total size of buffers kept in memory
    max_virtual_size: i64,
    /// auto_save tree when auto_save entries written
    auto_save: i64,
    /// auto_flush tree when auto_flush entries written
    auto_flush: i64,
    /// number of entries to estimate histogram limits
    estimate: i64,

    clusters: Clusters,
    pub(crate) iobits: TioFeatures,

    branches: Vec<B>,

    reader: Option<RootFileReader>,
    sinfos: Option<RootFileStreamerInfoContext>,
}

impl<B> Tree<B> {
    pub fn tot_bytes(&self) -> i64 {
        self.tot_bytes
    }
    pub fn title(&self) -> &str {
        self.named.title.as_str()
    }
}

impl<B> Default for Tree<B> {
    fn default() -> Self {
        Self {
            rvers: 0,
            named: rbase::Named::default(),
            attline: rbase::AttLine::default(),
            attfill: rbase::AttFill::default(),
            attmarker: rbase::AttMarker::default(),
            entries: 0,
            tot_bytes: 0,
            zip_bytes: 0,
            saved_bytes: 0,
            flushed_bytes: 0,
            weight: 1.0,
            timer_interval: 0,
            scan_field: 0,
            update: 0,
            default_entry_offset_len: 0,
            max_entries: 0,
            max_entry_loop: 0,
            max_virtual_size: 0,
            auto_save: 0,
            auto_flush: 0,
            estimate: 0,
            clusters: Clusters::default(),
            iobits: TioFeatures::default(),
            branches: Vec::new(),
            reader: None,
            sinfos: None,
        }
    }
}

pub type ReaderTree = Tree<Branch>;
// pub type WriterTree<T> = Tree<WBranch<T>>;
pub type WriterTree = Tree<WBranch<Box<dyn Marshaler>>>;

trait NewTrait: Marshaler + std::fmt::Debug {}

impl WriterTree {
    pub fn new(name: String) -> Self {
        Self {
            named: rbase::Named::default()
                .with_name(name.clone())
                .with_title(String::new()),
            weight: 1.0,
            scan_field: 25,
            default_entry_offset_len: 1000,
            max_entries: 1000000000000,
            max_entry_loop: 1000000000000,
            auto_save: -300000000,
            auto_flush: -30000000,
            estimate: 1000000,
            branches: Vec::new(),
            ..Default::default()
        }
    }

    // TODO: ckeck if f is mandatory, now used in new_key_for_basket_internal to check is_big_file
    pub fn new_branch<T>(
        &mut self,
        name: String,
        provider: impl Iterator<Item = T> + 'static,
        f: &RootFile,
    ) where
        T: Marshaler + 'static,
    {
        // let b: Box<dyn Iterator<Item = dyn Marshaler>> =
        //     Box::new(provider.map(|x| Box::new(x) as Box<dyn Marshaler>));
        // let branch = WBranch::new(name, b);
        // self.branches.push(branch);
        let it = provider.map(|x| Box::new(x) as Box<dyn Marshaler>);
        let WBranchwb = WBranch::new::<T>(name, it, self, f);
        self.branches.push(WBranchwb);
    }
    pub fn write_all(&mut self, file: &mut RootFile) -> crate::riofs::Result<()> {
        let mut branchs_done = self.branches.iter().map(|b| false).collect::<Vec<_>>();
        let mut branches = std::mem::take(&mut self.branches);
        loop {
            let mut tot = 0;
            let mut zip = 0;
            for (b, d) in branches.iter_mut().zip(branchs_done.iter_mut()) {
                match b.write(self, file) {
                    None => *d = true,
                    Some(nbytes) => {
                        tot += nbytes;
                    }
                }
            }

            self.tot_bytes += tot as i64;
            self.zip_bytes += zip as i64;
            if branchs_done.iter().all(|d| *d) {
                break;
            }
            self.entries += 1;
        }
        self.branches = branches;

        trace!(";WriterTree.write_all.entries:{:?}", self.entries);

        self.close(file)
    }

    fn flush(&mut self, file: &mut RootFile) -> crate::riofs::Result<()> {
        trace!(";WriterTree.flush:{:?}", true);
        for b in self.branches.iter_mut() {
            b.flush(file)?;
        }
        Ok(())
    }

    fn close(&mut self, file: &mut RootFile) -> crate::riofs::Result<()> {
        trace!(";WriterTree.close:{:?}", true);
        self.flush(file)?;

        // let t: ReaderTree = self.into();

        file.put(self.named.name(), self)?;

        Ok(())
    }
}

impl ReaderTree {
    pub(crate) fn set_reader(&mut self, reader: Option<RootFileReader>) {
        if let Some(r) = &reader {
            for b in self.branches.iter_mut() {
                b.set_reader(Some(r.clone()));
            }
            self.reader = reader;
        }
    }

    pub(crate) fn set_streamer_info(&mut self, sinfos: RootFileStreamerInfoContext) {
        for b in self.branches.iter_mut() {
            b.set_streamer_info(sinfos.clone());
        }
        self.sinfos = Some(sinfos);
    }

    /// Get a branch from this tree
    pub fn branch(&self, name: &str) -> Option<&Branch> {
        for b in self.branches.iter() {
            if b.name() == name {
                return Some(b);
            }

            if let Some(bb) = b.branch(name) {
                return Some(bb);
            }
        }
        None
    }

    /// Get iterator over top-level branches
    pub fn branches(&self) -> impl Iterator<Item = &Branch> {
        self.branches.iter()
    }

    /// Number of entries in the TTree, as reported by fEntries.
    pub fn entries(&self) -> i64 {
        self.entries
    }

    /// Get all (recursively) branches in this tree
    pub fn branches_r(&self) -> Vec<&Branch> {
        let mut v = Vec::new();

        for b in self.branches() {
            trace!("ADD {:?}", b);
            v.push(b);
            for bb in b.branches_r() {
                v.push(bb);
            }
        }

        v
    }

    /// Display branches in this tree
    ///
    /// Provide name, C++ type and a possible Rust interpretation.
    ///
    /// Example:
    /// ```ignore
    /// name                           | typename                       | interpretation                
    /// -------------------------------+-------------------------------+-------------------------------
    /// string                         | string                         | String                        
    /// vector_vector_int32            | vector<vector<int32_t>>        | Vec<Vec<i32>>                      
    /// vector_int32                   | vector<int32_t>                | Vec<i32>                      
    /// vector_string                  | vector<string>                 | Vec<String>                   
    /// three                          | char*                          | String
    ///```
    ///
    /// In this example, last branch can be read with:
    /// ```ignore    
    /// let three = tree
    ///         .branch("three")
    ///         .unwrap()
    ///         .as_iter::<String>()
    ///         .collect::<Vec<_>>();
    /// ```

    pub fn show(&self) {
        // const TYPE_NAME_SIZE:usize = 30;
        println!(
            "{:<30} | {:<30} | {:<30}",
            "name", "typename", "interpretation"
        );
        let s: String = ['-'; 31].iter().collect();
        println!("{}+{}+{}", s, s, s);
        fn show_one_branch(b: &&Branch) {
            let mut item_type_name = b.item_type_name();
            item_type_name.truncate(30);
            println!(
                "{:<30} | {:<30} | {:<30}",
                b.name(),
                item_type_name,
                b.interpretation()
            );
        }

        self.branches_r().iter().for_each(show_one_branch);
    }
}

impl Marshaler for WriterTree {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len();
        let beg = w.pos();
        let hdr = w.write_header(self.class(), Self::rversion(self))?;

        w.write_object(&self.named)?;

        w.write_object(&self.attline)?;
        // trace!(";WriterTree.marshal.buf.value:{:?}", w.p());
        w.write_object(&self.attfill)?;
        // trace!(";WriterTree.marshal.buf.value:{:?}", w.p());
        w.write_object(&self.attmarker)?;

        w.write_i64(self.entries)?;
        w.write_i64(self.tot_bytes)?;
        w.write_i64(self.zip_bytes)?;
        w.write_i64(self.saved_bytes)?;
        w.write_i64(self.flushed_bytes)?;
        w.write_f64(self.weight)?;
        w.write_i32(self.timer_interval)?;
        w.write_i32(self.scan_field)?;
        w.write_i32(self.update)?;
        w.write_i32(self.default_entry_offset_len)?;

        w.write_i32(self.clusters.ranges.len().try_into()?)?;

        w.write_i64(self.max_entries)?;
        w.write_i64(self.max_entry_loop)?;
        w.write_i64(self.max_virtual_size)?;
        // trace!(";WriterTree.marshal.buf.value:{:?}", w.p());
        w.write_i64(self.auto_save)?;
        // trace!(";WriterTree.marshal.buf.value:{:?}", w.p());
        trace!(";WriterTree.marshal.auto_flush:{:?}", self.auto_flush);
        trace!(";WriterTree.marshal.auto_save:{:?}", self.auto_save);
        trace!(";WriterTree.marshal.estimate:{:?}", self.estimate);
        w.write_i64(self.auto_flush)?;
        w.write_i64(self.estimate)?;

        w.write_i8(0)?;
        w.write_array_i64(&self.clusters.ranges)?;
        w.write_i8(0)?;
        w.write_array_i64(&self.clusters.sizes)?;
        w.write_object(&self.iobits)?;
        trace!(";WriterTree.marshal.buf.value:{:?}", w.p());

        {
            let mut branches = WriterObjArray::new();
            // let tbranches = std::mem::take()
            for b in self.branches.iter() {
                branches.push(b, std::ptr::addr_of!(*b) as usize);
            }
            w.write_object(&branches)?;
        }

        trace!(";WriterTree.marshal.buf.value:{:?}", &w.p()[len..]);
        {
            let mut leaves = WriterObjArray::new();
            for b in self.branches.iter() {
                for leaf in b.branch().tbranch().leaves.iter() {
                    leaves.push(leaf, std::ptr::addr_of!(*leaf) as usize);
                }
            }

            w.write_object(&leaves)?;
        }
        {
            w.write_object_nil()?;
            w.write_object_nil()?;
            w.write_object_nil()?;
            w.write_object_nil()?;
            w.write_object_nil()?;
            w.write_object_nil()?;
            w.write_object_nil()?;
        }

        let ret = w.set_header(hdr)?;
        trace!(";WriterTree.marshal.buf.value:{:?}", &w.p()[len..]);
        Ok(ret)

        // trace!(";WriterTree.marshal.buf.value:{:?}", w.p());
    }
}

impl Unmarshaler for WriterTree {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        todo!()
    }
}

impl Unmarshaler for ReaderTree {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let _beg = r.pos();
        // if (_beg == 868) {
        //     panic!(";rbuffer.ReadObjectAny.beg: {}", _beg);
        // }
        trace!(";Tree.unmarshal.beg: {}", _beg);
        trace!(";Tree.unmarshal.{}.beg: {}", _beg, _beg);

        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, crate::rvers::TREE, self.class())?;

        self.rvers = hdr.vers;
        r.read_object(&mut self.named)?;
        r.read_object(&mut self.attline)?;
        r.read_object(&mut self.attfill)?;
        r.read_object(&mut self.attmarker)?;

        ensure_minimum_supported_version(hdr.vers, 4, self.class())?;

        if hdr.vers > 5 {
            self.entries = r.read_i64()?;
            self.tot_bytes = r.read_i64()?;
            self.zip_bytes = r.read_i64()?;
            self.saved_bytes = r.read_i64()?;
        } else {
            self.entries = r.read_f64()? as i64;
            self.tot_bytes = r.read_f64()? as i64;
            self.zip_bytes = r.read_f64()? as i64;
            self.saved_bytes = r.read_f64()? as i64;
        }

        if hdr.vers >= 18 {
            self.flushed_bytes = r.read_i64()?;
        }

        if hdr.vers >= 16 {
            self.weight = r.read_f64()?;
        }

        self.timer_interval = r.read_i32()?;
        self.scan_field = r.read_i32()?;
        self.update = r.read_i32()?;

        if hdr.vers >= 17 {
            self.default_entry_offset_len = r.read_i32()?;
        }

        let mut nclus = 0;

        if hdr.vers >= 19 {
            nclus = r.read_i32()?;
        }

        if hdr.vers > 5 {
            self.max_entries = r.read_i64()?;
        }

        if hdr.vers > 5 {
            self.max_entry_loop = r.read_i64()?;
            self.max_virtual_size = r.read_i64()?;
            self.auto_save = r.read_i64()?;
        } else {
            self.max_entry_loop = r.read_i32()? as i64;
            self.max_virtual_size = r.read_i32()? as i64;
            self.auto_save = r.read_i32()? as i64;
        }

        if hdr.vers >= 18 {
            self.auto_flush = r.read_i64()?;
        }

        if hdr.vers > 5 {
            self.estimate = r.read_i64()?;
        } else {
            self.estimate = r.read_i32()? as i64;
        }

        if hdr.vers >= 19 {
            self.clusters.ranges = vec![0; nclus as usize];
            self.clusters.sizes = vec![0; nclus as usize];
            let _ = r.read_i8();
            r.read_array_i64(&mut self.clusters.ranges)?;

            let _ = r.read_i8();
            r.read_array_i64(&mut self.clusters.sizes)?;
        }

        if hdr.vers >= 20 {
            r.read_object(&mut self.iobits)?;
        }

        trace!(";Tree.unmarshal.{}.pos_before_branch: {}", _beg, r.pos());

        {
            let mut branches = r.read_object_into::<ReaderObjArray>()?;

            self.branches = branches
                .take_objs()
                .into_iter()
                .map(|obj| obj.into())
                .collect();

            self.branches.iter_mut().for_each(|b| {
                b.set_top_level(Some(true));
                // b.set_item_type_name();
            });
        }
        trace!(
            ";Tree.unmarshal.{}.pos_before_index_leaves: {}",
            _beg,
            r.pos()
        );
        {
            let mut _leaves = r.read_object_into::<ReaderObjArray>()?;
        }

        trace!(
            ";Tree.unmarshal.{}.pos_before_index_values: {}",
            _beg,
            r.pos()
        );

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

impl Marshaler for ReaderTree {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        todo!()
    }
}

factory_all_for_register_impl!(ReaderTree, "TTree", rvers::TREE);

impl Object for WriterTree {
    fn class(&self) -> &'_ str {
        "TTree"
    }
}

impl Named for WriterTree {
    fn name(&self) -> &'_ str {
        self.named.name()
    }

    fn title(&self) -> &'_ str {
        self.named.title()
    }
}

impl RVersioner for WriterTree {
    fn rversion(&self) -> i16 {
        rvers::TREE
    }
}
