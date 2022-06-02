use anyhow::Result;
use rbuffer::RBuffer;

pub mod rbuffer;

pub trait Unmarshaler {
    type Item;
    fn unmarshal_root(r: &mut RBuffer) -> Result<Self::Item>;
}

pub trait Unmarshaler2 {
    fn unmarshal_root2(&mut self, r: &mut RBuffer) -> Result<()>;
    // where
    //     Self: Sized;
}

pub trait UnmarshalerInto {
    type Item: Default + Unmarshaler2;
    fn unmarshal_root(r: &mut RBuffer) -> Result<Self::Item> {
        let mut a: Self::Item = Self::Item::default();
        // let mut b: Unmarshaler2
        Unmarshaler2::unmarshal_root2(&mut a, r)?;
        Ok(a)
    }
}
