use crate::rbytes::{RVersioner, WBuffer};
use crate::rcont::objarray::WriterObjArray;
use crate::rdict::StreamerInfo;
use crate::riofs::file::RootFileStreamerInfoContext;
use crate::rtree::branch::wbranch::WBranch;
use crate::rtree::tree::tio_features::TioFeatures;
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

pub struct WriterTree {
    tree: Tree<WBranch<Box<dyn Marshaler>>>,
    callbacks: Vec<Box<dyn FnMut(StateCallBack)>>,
}

/// Argument for callbacks called before and during writing branches. Callbacks are mainly used to
/// monitor the writing process and by the derive [WriteToTree macro](../derive.WriteToTree.html).
#[derive(Debug)]
pub enum StateCallBack {
    Before,
    /// Before writing branches, called once per entry
    Branch(String),
    /// Before writing a branch, called once per entry with name of the branch
    After,
}

impl WriterTree {
    /// Create a new WriterTree with a name. The name is used to identify the TTree in the file.
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            tree: Tree {
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
            },
            callbacks: Vec::new(),
        }
    }

    pub(crate) fn iobits(&self) -> TioFeatures {
        self.tree.iobits
    }

    /// Register callback to be called before and after writing branches. See [StateCallBack](enum.StateCallBack.html) and
    /// [write](#method.write) method for more information.
    pub fn add_callback<F>(&mut self, f: Box<F>)
    where
        F: FnMut(StateCallBack) + 'static,
    {
        self.callbacks.push(f);
    }

    pub(crate) fn add_streamer(&mut self, si: StreamerInfo) {
        let sis = self.tree.sinfos.as_mut().unwrap();
        sis.push(si);
    }

    /// Add a new branch to the tree.
    ///
    /// At this point, the iterator is not consumed, it will be by calling the write method.
    /// The `T` type has to implement [Marshaler](crate::Marshaler) to be able to write to the file.
    /// Implementation for basic types are provided by oxyroot.
    ///
    /// In order to write a custom type, you have to implement the Marshaler trait or use the derive
    /// [WriteToTree macro](derive.WriteToTree.html).
    pub fn new_branch<T, S>(&mut self, name: S, provider: impl Iterator<Item = T> + 'static)
    where
        T: Marshaler + 'static,
        S: AsRef<str>,
    {
        let it = provider.map(|x| Box::new(x) as Box<dyn Marshaler>);
        let wbranchwb = WBranch::new::<T>(name.as_ref(), it, self);
        self.tree.branches.push(wbranchwb);
    }

    /// Effectively write the tree to the file.
    ///
    /// The branches are written until all provided iterator are exhausted. The branches are written all
    /// at the same time, the method jump from one iterator to another. During the writing, the callback
    /// registered with [add_callback](#method.add_callback) are called for each new entry:
    /// - Before writing branches with the argument [StateCallBack::Before](enum.StateCallBack.html)
    /// - Before writing each branch with the argument [StateCallBack::Branch](enum.StateCallBack.html)
    pub fn write(&mut self, file: &mut RootFile) -> crate::riofs::Result<()> {
        let mut branchs_done = self
            .tree
            .branches
            .iter()
            .map(|_b| false)
            .collect::<Vec<_>>();
        let mut branches = std::mem::take(&mut self.tree.branches);
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

            self.tree.tot_bytes += tot as i64;
            self.tree.zip_bytes += zip as i64;
            if branchs_done.iter().all(|d| *d) {
                break;
            }
            self.tree.entries += 1;
        }
        self.tree.branches = branches;

        trace!(";WriterTree.write_all.entries:{:?}", self.tree.entries);

        self.close(file)
    }

    fn flush(&mut self, file: &mut RootFile) -> crate::riofs::Result<()> {
        trace!(";WriterTree.flush:{:?}", true);
        for b in self.tree.branches.iter_mut() {
            b.flush(file)?;
        }
        Ok(())
    }

    /// Called by file close method.
    fn close(&mut self, file: &mut RootFile) -> crate::riofs::Result<()> {
        trace!(";WriterTree.close:{:?}", true);
        self.flush(file)?;

        // let t: ReaderTree = self.into();

        file.put(self.tree.named.name(), self)?;

        let sis = self.tree.sinfos.take().unwrap();

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
            self.tree.auto_flush
        );
        let hdr = w.write_header(self.class(), Self::rversion(self))?;

        trace!(";WriterTree.marshal.a{beg}.pos.before.named:{:?}", w.pos());
        w.write_object(&self.tree.named)?;
        trace!(
            ";WriterTree.marshal.a{beg}.pos.before.attline:{:?}",
            w.pos()
        );

        w.write_object(&self.tree.attline)?;
        trace!(
            ";WriterTree.marshal.a{beg}.pos.before.attfill:{:?}",
            w.pos()
        );

        // trace!(";WriterTree.marshal.buf.value:{:?}", w.p());
        w.write_object(&self.tree.attfill)?;
        trace!(
            ";WriterTree.marshal.a{beg}.pos.before.attmarker:{:?}",
            w.pos()
        );
        // trace!(";WriterTree.marshal.buf.value:{:?}", w.p());
        w.write_object(&self.tree.attmarker)?;

        w.write_i64(self.tree.entries)?;
        w.write_i64(self.tree.tot_bytes)?;
        w.write_i64(self.tree.zip_bytes)?;
        w.write_i64(self.tree.saved_bytes)?;
        w.write_i64(self.tree.flushed_bytes)?;
        w.write_f64(self.tree.weight)?;
        w.write_i32(self.tree.timer_interval)?;
        w.write_i32(self.tree.scan_field)?;
        w.write_i32(self.tree.update)?;
        w.write_i32(self.tree.default_entry_offset_len)?;

        w.write_i32(self.tree.clusters.ranges.len().try_into()?)?;

        w.write_i64(self.tree.max_entries)?;
        w.write_i64(self.tree.max_entry_loop)?;
        w.write_i64(self.tree.max_virtual_size)?;
        // trace!(";WriterTree.marshal.buf.value:{:?}", w.p());
        w.write_i64(self.tree.auto_save)?;
        // trace!(";WriterTree.marshal.buf.value:{:?}", w.p());
        trace!(";WriterTree.marshal.auto_flush:{:?}", self.tree.auto_flush);
        trace!(";WriterTree.marshal.auto_save:{:?}", self.tree.auto_save);
        trace!(";WriterTree.marshal.estimate:{:?}", self.tree.estimate);
        w.write_i64(self.tree.auto_flush)?;
        w.write_i64(self.tree.estimate)?;

        w.write_i8(0)?;
        w.write_array_i64(&self.tree.clusters.ranges)?;
        w.write_i8(0)?;
        w.write_array_i64(&self.tree.clusters.sizes)?;
        w.write_object(&self.tree.iobits)?;
        trace!(";WriterTree.marshal.buf.value:{:?}", w.p());

        {
            let mut branches = WriterObjArray::new();
            // let tbranches = std::mem::take()
            for b in self.tree.branches.iter() {
                branches.push(b, std::ptr::addr_of!(*b) as usize);
            }
            w.write_object(&branches)?;
        }

        trace!(";WriterTree.marshal.buf.value:{:?}", &w.p()[len..]);
        {
            let mut leaves = WriterObjArray::new();
            for b in self.tree.branches.iter() {
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
        self.tree.named.name()
    }

    fn title(&self) -> &'_ str {
        self.tree.named.title()
    }
}

impl RVersioner for WriterTree {
    fn rversion(&self) -> i16 {
        rvers::TREE
    }
}
