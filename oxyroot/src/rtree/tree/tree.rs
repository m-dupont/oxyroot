use crate::rcont::list::ReaderList;
use crate::riofs::file::{RootFileReader, RootFileStreamerInfoContext};
use crate::rtree::tree::tio_features::TioFeatures;
use crate::{rbase, StateCallBack};

#[derive(Default)]
pub struct Clusters {
    pub(crate) ranges: Vec<i64>,
    pub(crate) sizes: Vec<i64>,
}

pub struct Tree<B> {
    pub(crate) rvers: i16,
    pub(crate) named: rbase::Named,
    pub(crate) attline: rbase::AttLine,
    pub(crate) attfill: rbase::AttFill,
    pub(crate) attmarker: rbase::AttMarker,

    /// Number of entries
    pub(crate) entries: i64,
    /// Total number of bytes in all branches before compression
    pub(crate) tot_bytes: i64,
    /// Total number of bytes in all branches after  compression
    pub(crate) zip_bytes: i64,
    /// number of autosaved bytes
    pub(crate) saved_bytes: i64,
    /// number of auto-flushed bytes
    pub(crate) flushed_bytes: i64,

    /// tree weight
    pub(crate) weight: f64,
    /// timer interval in milliseconds
    pub(crate) timer_interval: i32,
    /// number of runs before prompting in Scan
    pub(crate) scan_field: i32,
    /// update frequency for entry-loop
    pub(crate) update: i32,
    /// initial length of the entry offset table in the basket buffers
    pub(crate) default_entry_offset_len: i32,
    /// maximum number of entries in case of circular buffers
    pub(crate) max_entries: i64,
    /// maximum number of entries to process
    pub(crate) max_entry_loop: i64,
    /// maximum total size of buffers kept in memory
    pub(crate) max_virtual_size: i64,
    /// auto_save tree when auto_save entries written
    pub(crate) auto_save: i64,
    /// auto_flush tree when auto_flush entries written
    pub(crate) auto_flush: i64,
    /// number of entries to estimate histogram limits
    pub(crate) estimate: i64,

    pub(crate) clusters: Clusters,
    pub(crate) iobits: TioFeatures,

    pub(crate) branches: Vec<B>,

    pub(crate) sinfos: Option<RootFileStreamerInfoContext>,

    pub(crate) callbacks: Vec<Box<dyn FnMut(StateCallBack)>>,
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
            sinfos: None,
            callbacks: Vec::new(),
        }
    }
}
