use crate::Branch;
use std::marker::PhantomData;

pub struct WBranch<T> {
    branch: Branch,
    data: Option<T>,
}
