use anyhow::Result;
use rbuffer::RBuffer;

pub mod rbuffer;

pub trait Unmarshaler {
    type Item;
    fn unmarshal_root(r: &mut RBuffer) -> Result<Self::Item>;
}
