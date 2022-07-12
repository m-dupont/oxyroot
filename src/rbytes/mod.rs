use crate::rdict::StreamerInfo;
use crate::root;
use anyhow::Result;
use rbuffer::RBuffer;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

pub mod consts;
pub mod rbuffer;

use paste::paste;

/// Header represents a type header in a ROOT buffer.
///
#[derive(Default, Debug)]
pub struct Header {
    /// name of the type being guarded by this header.
    name: String,
    /// version of the type being guarded by this header.
    pub(crate) vers: i16,
    /// position of the type in the ROOT buffer.
    pos: i64,
    /// length of the value in the ROOT buffer.
    len: u32,
}

/// RVersioner is the interface implemented by an object that
/// can tell the ROOT system what is its current version.
pub trait RVersioner {
    fn rversion() -> i16;
}

/// STREAMER_ELEMENT describes a ROOT STREAMER_ELEMENT
pub trait StreamerElement: root::traits::Named {}

/// Implement STREAMER_ELEMENT for ROOT objects

// impl<T> STREAMER_ELEMENT for T where T: FactoryItem {}
// impl<T> STREAMER_ELEMENT for T where T: root::traits::NAMED {}

/// StreamerInfoContext defines the protocol to retrieve a ROOT STREAMER_INFO
/// metadata type by name.
pub trait StreamerInfoContext {
    /// STREAMER_INFO returns the named STREAMER_INFO.
    /// If version is negative, the latest version should be returned.
    fn streamer_info(&self, name: &str, version: i32) -> Option<&StreamerInfo>;
}

/// Trait that permits reading a type from an ROOT file.
///
/// Examples of types that implement this:
///
/// * Primitive integers, floats, etc
/// * Owned byte containers (`Vec<T>`, `HashMap<K,V>`, HashSet<K> )
pub trait Unmarshaler {
    fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()>;
}

macro_rules! impl_unmarshaler_primitive {
    ($ftype:ty, $buffer_fn:ident) => {
        impl Unmarshaler for $ftype {
            fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()> {
                *self = r.$buffer_fn()?;
                Ok(())
            }
        }
    };

    ($ftype:ty) => {
        paste! {
            impl_unmarshaler_primitive!($ftype, [<read_$ftype>]);



        }
    };
}

impl_unmarshaler_primitive!(i8);
impl_unmarshaler_primitive!(u8);
impl_unmarshaler_primitive!(i16);
impl_unmarshaler_primitive!(u16);
impl_unmarshaler_primitive!(i32);
impl_unmarshaler_primitive!(u32);
impl_unmarshaler_primitive!(i64);
impl_unmarshaler_primitive!(u64);

impl_unmarshaler_primitive!(f32);
impl_unmarshaler_primitive!(f64);
impl_unmarshaler_primitive!(bool);

impl Unmarshaler for String {
    fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()> {
        r.do_skip_header()?;
        *self = r.read_string()?.to_string();
        Ok(())
    }
}

impl<T> Unmarshaler for Vec<T>
where
    T: UnmarshalerInto<Item = T>,
{
    fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()> {
        r.do_skip_header()?;
        let size = r.read_i32()?;

        r.set_skip_header(None);

        for _ in 0..size {
            let a = r.read_object_into::<T>()?;
            self.push(a);
        }

        Ok(())
    }
}

impl<T> Unmarshaler for HashSet<T>
where
    T: UnmarshalerInto<Item = T> + Eq + Hash + Debug,
{
    fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()> {
        r.do_skip_header()?;
        let size = r.read_i32()?;
        r.set_skip_header(None);
        for _ in 0..size {
            let a = r.read_object_into::<T>()?;
            self.insert(a);
        }
        Ok(())
    }
}

impl<K, V> Unmarshaler for HashMap<K, V>
where
    V: UnmarshalerInto<Item = V> + Debug,
    K: UnmarshalerInto<Item = K> + Eq + Hash + Debug,
{
    fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()> {
        r.do_skip_header()?;

        let size = r.read_i32()?;
        let mut keys = Vec::with_capacity(size as usize);
        let mut values = Vec::with_capacity(size as usize);

        r.set_skip_header(Some(6));

        for _i in 0..size {
            // r.set_skip_header(Some(0));
            let k = r.read_object_into::<K>()?;
            r.set_skip_header(Some(0));
            keys.push(k);
        }

        r.set_skip_header(Some(6));
        for _i in 0..size {
            let v = r.read_object_into::<V>()?;
            r.set_skip_header(Some(0));
            values.push(v);
        }

        keys.into_iter().zip(values.into_iter()).for_each(|(k, v)| {
            self.insert(k, v);
        });

        Ok(())
    }
}

/// Automatically implemented if [Unmarshaler] is implemented
pub trait UnmarshalerInto {
    type Item: Default + Unmarshaler;
    fn unmarshal_into(r: &mut RBuffer) -> Result<Self::Item>;
}

impl<T> UnmarshalerInto for T
where
    T: Default + Unmarshaler,
{
    type Item = T;

    fn unmarshal_into(r: &mut RBuffer) -> Result<Self::Item> {
        let mut a: Self::Item = Self::Item::default();
        Unmarshaler::unmarshal(&mut a, r)?;
        Ok(a)
    }
}
