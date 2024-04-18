use crate::rbytes::{RVersioner, WBuffer};
use crate::rcont::objarray::WriterObjArray;
use crate::rdict::StreamerInfo;
use crate::riofs::file::RootFileStreamerInfoContext;
use crate::rtree::branch::wbranch::WBranch;
use crate::rtree::tree::tree::Tree;
use crate::{rbase, rvers, Marshaler, Named, Object, RootFile};
use log::trace;

/// Write only Rust equivalent of [`TTree`](https://root.cern/doc/master/classTTree.html)
///
/// Mainly used to create [crate::Branch] with name with a provider of data.
///
/**
```
use oxyroot::RootFile;
use oxyroot::WriterTree;
let s = "/tmp/simple.root";
let mut file = RootFile::create(s).expect("Can not create file");
let mut tree = WriterTree::new("mytree");
let it = (0..15);
tree.new_branch("it", it);
tree.write(&mut file).expect("Can not write tree");
file.close().expect("Can not close file");
```
 */
pub type WriterTree = Tree<WBranch<Box<dyn Marshaler>>>;

#[derive(Debug)]
pub enum StateCallBack {
    Before,
    Branch(String),
    After,
}

impl WriterTree {
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            named: rbase::Named::default()
                .with_name(name.as_ref().to_string())
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
            sinfos: Some(RootFileStreamerInfoContext::new()),
            ..Default::default()
        }
    }

    pub fn add_callback<F>(&mut self, f: Box<F>)
    where
        F: FnMut(StateCallBack) + 'static,
    {
        self.callbacks.push(f);
    }

    pub fn add_streamer(&mut self, si: StreamerInfo) {
        let sis = self.sinfos.as_mut().unwrap();
        sis.push(si);
    }

    pub fn new_branch<T, S>(&mut self, name: S, provider: impl Iterator<Item = T> + 'static)
    where
        T: Marshaler + 'static,
        S: AsRef<str>,
    {
        // let b: Box<dyn Iterator<Item = dyn Marshaler>> =
        //     Box::new(provider.map(|x| Box::new(x) as Box<dyn Marshaler>));
        // let branch = WBranch::new(name, b);
        // self.branches.push(branch);
        let it = provider.map(|x| Box::new(x) as Box<dyn Marshaler>);
        let wbranchwb = WBranch::new::<T>(name.as_ref(), it, self);
        self.branches.push(wbranchwb);
    }
    pub fn write(&mut self, file: &mut RootFile) -> crate::riofs::Result<()> {
        let mut branchs_done = self.branches.iter().map(|_b| false).collect::<Vec<_>>();
        let mut branches = std::mem::take(&mut self.branches);
        loop {
            let mut tot = 0;
            let zip = 0;
            for f in self.callbacks.iter_mut() {
                f(StateCallBack::Before);
            }
            for (b, d) in branches.iter_mut().zip(branchs_done.iter_mut()) {
                for f in self.callbacks.iter_mut() {
                    f(StateCallBack::Branch(b.name().to_string()));
                }
                match b.write(self, file)? {
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

        let sis = self.sinfos.take().unwrap();

        for si in sis.list().iter() {
            file.add_streamer_info(si.clone());
        }

        // for sis in self.sinfos.take() {}

        Ok(())
    }
}

impl Marshaler for WriterTree {
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len();
        let beg = w.pos();
        trace!(
            ";WriterTree.marshal.a{beg}.auto_flush:{:?}",
            self.auto_flush
        );
        let hdr = w.write_header(self.class(), Self::rversion(self))?;

        trace!(";WriterTree.marshal.a{beg}.pos.before.named:{:?}", w.pos());
        w.write_object(&self.named)?;
        trace!(
            ";WriterTree.marshal.a{beg}.pos.before.attline:{:?}",
            w.pos()
        );

        w.write_object(&self.attline)?;
        trace!(
            ";WriterTree.marshal.a{beg}.pos.before.attfill:{:?}",
            w.pos()
        );

        // trace!(";WriterTree.marshal.buf.value:{:?}", w.p());
        w.write_object(&self.attfill)?;
        trace!(
            ";WriterTree.marshal.a{beg}.pos.before.attmarker:{:?}",
            w.pos()
        );
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
