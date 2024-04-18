pub mod reader;
pub mod tio_features;
pub mod writer;

pub mod traits;

pub mod tree;

use crate::rbytes::{Marshaler, RVersioner, Unmarshaler};
use crate::root::traits::Object;
use crate::Named;
use crate::UnmarshalerInto;
use std::fmt::Debug;
// pub struct TioFeatures {
//     val:
// }
// pub type WriterTree<T> = Tree<WBranch<T>>;

pub use reader::ReaderTree;
pub use tree::Tree;
pub use writer::StateCallBack;
pub use writer::WriterTree;
