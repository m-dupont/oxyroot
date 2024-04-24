use crate::rbytes::{ensure_maximum_supported_version, ensure_minimum_supported_version};
use crate::rcont::list::ReaderList;
use crate::rcont::objarray::ReaderObjArray;
use crate::riofs::file::{RootFileReader, RootFileStreamerInfoContext};
use crate::rtree::tree::base::Tree;
use crate::rvers;
use crate::{factory_all_for_register_impl, Branch, Object, RBuffer, Unmarshaler};
use log::trace;

/// Read only Rust equivalent of [`TTree`](https://root.cern/doc/master/classTTree.html)
///
/// Mainly used to retrieve [`Branch`](crate::Branch) and iterate on data.
#[derive(Default)]
pub struct ReaderTree {
    tree: Tree<Branch>,
    reader: Option<RootFileReader>,
    user_infos: Option<ReaderList>,
}

impl ReaderTree {
    pub(crate) fn set_reader(&mut self, reader: Option<RootFileReader>) {
        if let Some(r) = &reader {
            for b in self.tree.branches.iter_mut() {
                b.set_reader(Some(r.clone()));
            }
            self.reader = reader;
        }
    }

    pub(crate) fn set_streamer_info(&mut self, sinfos: RootFileStreamerInfoContext) {
        for b in self.tree.branches.iter_mut() {
            b.set_streamer_info(sinfos.clone());
        }
        self.tree.sinfos = Some(sinfos);
    }

    /// Get a branch from this tree
    pub fn branch(&self, name: &str) -> Option<&Branch> {
        for b in self.tree.branches.iter() {
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
        self.tree.branches.iter()
    }

    /// Number of entries in the TTree, as reported by fEntries.
    pub fn entries(&self) -> i64 {
        self.tree.entries
    }

    /// Get all (recursively) branches in this tree
    pub fn branches_r(&self) -> Vec<&Branch> {
        let mut v = Vec::new();

        for b in self.branches() {
            v.push(b);
            for bb in b.branches_r() {
                v.push(bb);
            }
        }

        v
    }

    pub fn user_info(&self) -> Option<&ReaderList> {
        self.user_infos.as_ref()
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

    // pub fn as_iter<'a, T>(&'a self) -> impl Iterator<Item = T> + 'a
    // where
    //     T: FromTree + 'a,
    // {
    //     T::from_tree(self)
    // }
}

impl Unmarshaler for ReaderTree {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let _beg = r.pos();
        // if (_beg == 868) {
        //     panic!(";rbuffer.ReadObjectAny.beg: {}", _beg);
        // }
        trace!(";Tree.unmarshal.beg: {}", _beg);

        let hdr = r.read_header(self.class())?;

        ensure_maximum_supported_version(hdr.vers, crate::rvers::TREE, self.class())?;

        self.tree.rvers = hdr.vers;
        r.read_object(&mut self.tree.named)?;
        trace!(";Tree.unmarshal.{_beg}.pos.before.attline: {}", r.pos());
        r.read_object(&mut self.tree.attline)?;
        trace!(";Tree.unmarshal.{_beg}.pos.before.attfill: {}", r.pos());

        r.read_object(&mut self.tree.attfill)?;
        trace!(";Tree.unmarshal.{_beg}.pos.before.attmarker: {}", r.pos());
        r.read_object(&mut self.tree.attmarker)?;

        ensure_minimum_supported_version(hdr.vers, 4, self.class())?;

        if hdr.vers > 5 {
            self.tree.entries = r.read_i64()?;
            self.tree.tot_bytes = r.read_i64()?;
            self.tree.zip_bytes = r.read_i64()?;
            self.tree.saved_bytes = r.read_i64()?;
        } else {
            self.tree.entries = r.read_f64()? as i64;
            self.tree.tot_bytes = r.read_f64()? as i64;
            self.tree.zip_bytes = r.read_f64()? as i64;
            self.tree.saved_bytes = r.read_f64()? as i64;
        }

        if hdr.vers >= 18 {
            self.tree.flushed_bytes = r.read_i64()?;
        }

        if hdr.vers >= 16 {
            self.tree.weight = r.read_f64()?;
        }

        self.tree.timer_interval = r.read_i32()?;
        self.tree.scan_field = r.read_i32()?;
        self.tree.update = r.read_i32()?;

        if hdr.vers >= 17 {
            self.tree.default_entry_offset_len = r.read_i32()?;
        }

        let mut nclus = 0;

        if hdr.vers >= 19 {
            nclus = r.read_i32()?;
        }

        if hdr.vers > 5 {
            self.tree.max_entries = r.read_i64()?;
        }

        if hdr.vers > 5 {
            self.tree.max_entry_loop = r.read_i64()?;
            self.tree.max_virtual_size = r.read_i64()?;
            self.tree.auto_save = r.read_i64()?;
        } else {
            self.tree.max_entry_loop = r.read_i32()? as i64;
            self.tree.max_virtual_size = r.read_i32()? as i64;
            self.tree.auto_save = r.read_i32()? as i64;
        }

        if hdr.vers >= 18 {
            self.tree.auto_flush = r.read_i64()?;
        }

        if hdr.vers > 5 {
            self.tree.estimate = r.read_i64()?;
        } else {
            self.tree.estimate = r.read_i32()? as i64;
        }

        if hdr.vers >= 19 {
            self.tree.clusters.ranges = vec![0; nclus as usize];
            self.tree.clusters.sizes = vec![0; nclus as usize];
            let _ = r.read_i8();
            r.read_array_i64(&mut self.tree.clusters.ranges)?;

            let _ = r.read_i8();
            r.read_array_i64(&mut self.tree.clusters.sizes)?;
        }

        if hdr.vers >= 20 {
            r.read_object(&mut self.tree.iobits)?;
        }

        trace!(";Tree.unmarshal.{}.pos_before_branch: {}", _beg, r.pos());

        {
            let mut branches = r.read_object_into::<ReaderObjArray>()?;

            self.tree.branches = branches
                .take_objs()
                .into_iter()
                .map(|obj| obj.into())
                .collect();

            self.tree.branches.iter_mut().for_each(|b| {
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
            // tree.aliases
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
            //tree.treeindex
            let v = r.read_object_any_into()?;
            if v.is_some() {
                todo!()
            }

            //tree.friends
            let v = r.read_object_any_into()?;
            if v.is_some() {
                todo!()
            }

            trace!(";Tree.unmarshal.{}.pos_before_user_info: {}", _beg, r.pos());

            //tree.userInfo
            let v = r.read_object_any_into()?;
            if let Some(v) = v {
                self.user_infos = Some(*v.downcast::<ReaderList>().unwrap());
                trace!(
                    ";Tree.unmarshal.a{_beg}.userInfo.len: {}",
                    self.user_infos.as_ref().unwrap().len()
                );
            }

            trace!(";Tree.unmarshal.{}.pos_after_user_info: {}", _beg, r.pos());

            // let user_info = r.read_object_into::<ReaderList>()?;

            // todo!();

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

factory_all_for_register_impl!(ReaderTree, "TTree", rvers::TREE);
