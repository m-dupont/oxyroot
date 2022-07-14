use std::cell::{Cell, RefCell};

#[derive(Default)]
pub(crate) struct TBranchProps {
    /// this property is set by [crate::Tree] when branch is read
    pub(crate) is_top_level: Option<bool>,
    pub(crate) item_type_name: Option<String>,
}
