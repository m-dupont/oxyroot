use crate::rdict::StreamerInfo;
use crate::root;
use anyhow::Result;
use rbuffer::RBuffer;

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
        *self = r.read_string()?.to_string();
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
        // let mut b: Unmarshaler2
        Unmarshaler::unmarshal(&mut a, r)?;
        Ok(a)
    }
}
