use crate::rbytes::{Unmarshaler, UnmarshalerInto};
use crate::RBuffer;
use log::trace;
use std::fmt::Debug;

/// Represent a array of `T*` in C++
///
/// To read branch with `int` (aka `i32` rust) in it
/// ```rust
/// use oxyroot::RootFile;
///
/// let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";///
/// let mut f = RootFile::open(s).unwrap();
/// let tree = f.get_tree("tree").unwrap();
///
/// tree.branch("SliceI16")
///         .unwrap()
///         .as_iter::<oxyroot::Slice<i16>>()
///         .map(|a| a.into_vec())
///         .enumerate()
///         .for_each(|(i, val)| {
///             assert_eq!(val.len(), i % 10);
///
///             val.into_iter()
///                 .map(|v| {
///                     assert_eq!(v, i as i16);
///                 })
///                 .for_each(drop) // Consume iterator
///         });
/// ```
///
#[derive(Default, Debug)]
pub struct Slice<T> {
    inner: Vec<T>,
}

impl<T> Slice<T> {
    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }
    pub fn inner(&self) -> &Vec<T> {
        &self.inner
    }
}

impl<T> From<Slice<T>> for Vec<T> {
    fn from(slice: Slice<T>) -> Self {
        slice.into_vec()
    }
}

impl<T> Unmarshaler for Slice<T>
where
    T: UnmarshalerInto<Item = T> + Debug,
{
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let beg = r.pos();
        trace!("Slice<T>.unmarshal.{beg}.pos.beg.:{beg}");
        r.do_skip_header()?;
        trace!("Slice<T>.unmarshal.{beg}.pos.after_header.:{}", r.pos());
        // r.skip(1)?;

        let mut len = r.len() as usize;
        while len > 0 {
            let before = r.pos();
            self.inner.push(r.read_object_into::<T>().unwrap());
            let after = r.pos();
            len -= (after - before) as usize;
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct SizedSlice<T> {
    inner: Vec<T>,
    n: usize,
}

impl<T> From<SizedSlice<T>> for Vec<T> {
    fn from(value: SizedSlice<T>) -> Self {
        value.inner
    }
}

impl<T> SizedSlice<T> {
    pub fn new(n: usize) -> Self {
        SizedSlice {
            inner: Vec::with_capacity(n),
            n,
        }
    }
    pub fn inner(&self) -> &Vec<T> {
        &self.inner
    }
}

impl<T> Unmarshaler for SizedSlice<T>
where
    T: UnmarshalerInto<Item = T>,
{
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        for i in 0..self.n {
            self.inner.push(r.read_object_into::<T>().unwrap());
        }

        Ok(())
    }
}
