use crate::rdict::StreamerInfo;
use crate::root;
use anyhow::Result;
use rbuffer::RBuffer;

pub mod consts;
pub mod rbuffer;

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

/// StreamerElement describes a ROOT StreamerElement
pub trait StreamerElement: root::traits::Named {}

/// Implement StreamerElement for ROOT objects

// impl<T> StreamerElement for T where T: FactoryItem {}
// impl<T> StreamerElement for T where T: root::traits::Named {}

/// StreamerInfoContext defines the protocol to retrieve a ROOT StreamerInfo
/// metadata type by name.
pub trait StreamerInfoContext {
    /// StreamerInfo returns the named StreamerInfo.
    /// If version is negative, the latest version should be returned.
    fn streamer_info(&self, name: &str, version: i32) -> Option<&StreamerInfo>;
}

pub trait Unmarshaler {
    fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()>;
}

impl Unmarshaler for i32 {
    fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()> {
        *self = r.read_i32()?;
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
