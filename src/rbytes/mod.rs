use crate::rdict::StreamerInfo;
use crate::{root, Object};
pub use error::Error;
pub use error::Result;
use rbuffer::RBuffer;
use std::any::{type_name, TypeId};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

pub mod consts;
mod error;
pub mod rbuffer;
pub mod wbuffer;

pub(crate) use crate::rbytes::wbuffer::WBuffer;
use paste::paste;

/// Header represents a type header in a ROOT buffer.
///
#[derive(Default, Debug)]
pub(crate) struct Header {
    /// name of the type being guarded by this header.
    _name: String,
    /// version of the type being guarded by this header.
    pub(crate) vers: i16,
    /// position of the type in the ROOT buffer.
    pos: i64,
    /// length of the value in the ROOT buffer.
    len: u32,
}

/// RVersioner is the interface implemented by an object that
/// can tell the ROOT system what is its current version.
pub(crate) trait RVersioner {
    fn rversion(&self) -> i16;
}

/// STREAMER_ELEMENT describes a ROOT STREAMER_ELEMENT
pub(crate) trait StreamerElement: root::traits::Named {}

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
#[derive(Debug)]
pub(crate) enum MarshallerKindStd {
    Vector { class_name: String },
}

#[derive(Debug)]
pub(crate) enum MarshallerKind {
    Primitive,
    Array { shape: Vec<i32>, tys: String },
    Slice { std: MarshallerKindStd },
    String,
    Struct,
}

/// Trait that permits writing a type to an ROOT file.
///
/// Examples of types that implement this:
///
/// * Primitive integers, floats, etc
pub trait Marshaler {
    fn marshal(&self, w: &mut WBuffer) -> Result<i64>;
    fn kind() -> MarshallerKind
    where
        Self: Sized,
    {
        unimplemented!("Marshaler.rust_type_to_kind for {}", type_name::<Self>())
    }

    fn root_code() -> String
    where
        Self: Sized,
    {
        unimplemented!("Marshaler.root_code for {}", type_name::<Self>())
    }

    fn class_name() -> String
    where
        Self: Sized,
    {
        unimplemented!("Marshaler.class_name for {}", type_name::<Self>())
    }
}

/// Used by WBranch to marshal objects into a ROOT buffer.
impl Marshaler for Box<dyn Marshaler> {
    fn marshal(&self, w: &mut WBuffer) -> Result<i64> {
        self.as_ref().marshal(w)
    }
}

macro_rules! impl_marshalers_primitive {
    ($ftype:ty, $buffer_read_fn:ident, $buffer_write_fn:ident) => {
        impl Unmarshaler for $ftype {
            fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()> {
                *self = r.$buffer_read_fn()?;
                Ok(())
            }
        }

        impl Marshaler for $ftype {
            fn marshal(&self, w: &mut WBuffer) -> Result<i64> {
                let beg = w.pos();
                w.$buffer_write_fn(*self)?;
                Ok(w.pos() - beg)
            }

            fn kind() -> MarshallerKind
            where
                Self: Sized,
            {
                MarshallerKind::Primitive
            }

            fn class_name() -> String
            where
                Self: Sized,
            {
                let tys = type_name::<Self>();
                let ret = match tys {
                    "i32" => "int",
                    _ => unimplemented!("Marshaler.class_name for {}", type_name::<Self>()),
                };
                ret.to_string()
            }

            fn root_code() -> String {
                // TODO: use a macro to generate this
                let ty = TypeId::of::<Self>();
                if ty == TypeId::of::<i8>() {
                    return "B".to_string();
                }

                if ty == TypeId::of::<u8>() {
                    return "b".to_string();
                }

                if ty == TypeId::of::<i16>() {
                    return "S".to_string();
                }

                if ty == TypeId::of::<u16>() {
                    return "s".to_string();
                }

                if ty == TypeId::of::<i32>() {
                    return "I".to_string();
                }

                if ty == TypeId::of::<u32>() {
                    return "i".to_string();
                }

                if ty == TypeId::of::<i64>() {
                    return "L".to_string();
                }

                if ty == TypeId::of::<u64>() {
                    return "l".to_string();
                }

                if ty == TypeId::of::<f32>() {
                    return "F".to_string();
                }

                if ty == TypeId::of::<f64>() {
                    return "D".to_string();
                }

                if ty == TypeId::of::<bool>() {
                    return "B".to_string();
                }

                unimplemented!("Marshaler.root_code for {}", type_name::<Self>())
            }
        }
    };

    ($ftype:ty) => {
        paste! {
            impl_marshalers_primitive!($ftype, [<read_$ftype>], [<write_$ftype>]);
        }

        paste! {
                    impl $crate::root::traits::Object for $ftype {
                fn class(&self) -> &'_ str {
                "[<$ftype>]"
                }
            }
        }
    };
}

impl_marshalers_primitive!(i8);

impl_marshalers_primitive!(u8);
impl_marshalers_primitive!(i16);
impl_marshalers_primitive!(u16);
impl_marshalers_primitive!(i32);
impl_marshalers_primitive!(u32);
impl_marshalers_primitive!(i64);
impl_marshalers_primitive!(u64);

impl_marshalers_primitive!(f32);
impl_marshalers_primitive!(f64);
impl_marshalers_primitive!(bool);

impl Unmarshaler for String {
    fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()> {
        r.do_skip_header()?;
        *self = r.read_string()?.to_string();
        Ok(())
    }
}

impl Marshaler for String {
    fn marshal(&self, w: &mut WBuffer) -> Result<i64> {
        let beg = w.pos();
        w.write_string(self)?;
        Ok(w.pos() - beg)
    }

    fn kind() -> MarshallerKind {
        MarshallerKind::String
    }

    fn root_code() -> String {
        "string".to_string()
    }
}

impl<T> Unmarshaler for Vec<T>
where
    T: UnmarshalerInto<Item = T>,
{
    fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()> {
        r.do_skip_header()?;
        let size = r.read_i32()?;

        self.reserve(size as usize);
        r.set_skip_header(None);

        for _ in 0..size {
            let a = r.read_object_into::<T>()?;
            self.push(a);
        }

        Ok(())
    }
}

impl<T> Marshaler for Vec<T>
where
    T: Marshaler,
{
    fn marshal(&self, w: &mut WBuffer) -> Result<i64> {
        let beg = w.pos();
        w.write_i32(self.len() as i32)?;
        for item in self.iter() {
            item.marshal(w)?;
        }
        Ok(w.pos() - beg)
    }

    fn kind() -> MarshallerKind {
        MarshallerKind::Slice {
            std: MarshallerKindStd::Vector {
                class_name: T::class_name(),
            },
        }
    }

    fn root_code() -> String {
        format!("vector<{}>", T::root_code())
    }

    fn class_name() -> String
    where
        Self: Sized,
    {
        format!("vector<{}>", T::class_name())
    }
}

impl<T> Unmarshaler for HashSet<T>
where
    T: UnmarshalerInto<Item = T> + Eq + Hash,
{
    fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()> {
        r.do_skip_header()?;
        let size = r.read_i32()?;
        self.reserve(size as usize);
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
    V: UnmarshalerInto<Item = V>,
    K: UnmarshalerInto<Item = K> + Eq + Hash,
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

        self.reserve(size as usize);

        keys.into_iter().zip(values).for_each(|(k, v)| {
            self.insert(k, v);
        });

        Ok(())
    }
}

impl<T, const N: usize> Unmarshaler for [T; N]
where
    T: Unmarshaler,
{
    fn unmarshal(&mut self, r: &mut RBuffer) -> Result<()> {
        // for i in 0..N {
        //     self[i].unmarshal(r).unwrap();
        // }

        for item in self.iter_mut().take(N) {
            item.unmarshal(r).unwrap();
        }
        Ok(())
    }
}

impl<T, const N: usize> Marshaler for [T; N]
where
    T: Marshaler,
{
    fn marshal(&self, w: &mut WBuffer) -> Result<i64> {
        let beg = w.pos();
        for item in self.iter().take(N) {
            item.marshal(w)?;
        }
        Ok(w.pos() - beg)
    }

    fn kind() -> MarshallerKind {
        MarshallerKind::Array {
            shape: vec![N as i32],
            tys: type_name::<T>().to_string(),
        }
    }

    fn root_code() -> String {
        format!("[{}]/{}", N, T::root_code())
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

pub fn ensure_maximum_supported_version(
    read_version: i16,
    max_supported_version: i16,
    class_involved: &str,
) -> Result<()> {
    if read_version > max_supported_version {
        return Err(Error::VersionTooHigh {
            class: class_involved.into(),
            version_read: read_version,
            max_expected: max_supported_version,
        });
    }
    Ok(())
}

pub fn ensure_minimum_supported_version(
    read_version: i16,
    min_supported_version: i16,
    class_involved: &str,
) -> Result<()> {
    if read_version <= min_supported_version {
        return Err(Error::VersionTooLow {
            class: class_involved.into(),
            version_read: read_version,
            min_expected: min_supported_version,
        });
    }
    Ok(())
}
