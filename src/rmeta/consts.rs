use crate::rmeta::consts::Enum::Named;
use anyhow::{anyhow, Result};
use num;
use num_derive::FromPrimitive;
use num_derive::ToPrimitive;

#[derive(Debug)]
pub enum Enum {
    Named(EnumNamed),
    Int(i32),
}

impl Default for Enum {
    fn default() -> Self {
        Self::Named(EnumNamed::default())
    }
}

impl Enum {
    pub fn from_i32(i: i32) -> Self {
        match EnumNamed::from_i32(i) {
            Ok(ii) => Named(ii),
            Err(_) => Enum::Int(i),
        }

        // .ok_or(anyhow!("Cant make enum from {i}"))
    }

    pub fn to_i32(&self) -> i32 {
        // match  { }

        match self {
            Named(o) => num::ToPrimitive::to_i32(o).unwrap(),
            Enum::Int(i) => *i,
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Default, Debug)]
pub enum EnumNamed {
    #[default]
    Base = 0,
    // Base class
    Char = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Counter = 6,
    // Counter for array size
    CharStar = 7,
    // Pointer to array of char
    Double = 8,
    Double32 = 9,
    LegacyChar = 10,
    // Equal to TDataType's kchar
    UChar = 11,
    UShort = 12,
    UInt = 13,
    ULong = 14,
    Bits = 15,
    Long64 = 16,
    ULong64 = 17,
    Bool = 18,
    Float16 = 19,
    OffsetL = 20,
    /// Fixed size array
    OffsetP = 40,
    // OffsetP_3 = 43,
    // OffsetP_16 = 56,
    /// Pointer to object
    Object = 61,
    // Class  derived from TObject, or for TStreamerSTL::fCtype non-pointer elements
    Any = 62,
    // Class  not derived from TObject
    Objectp = 63,
    // Class* derived from TObject and with    comment field //->Class, or for TStreamerSTL::fCtype: pointer elements
    ObjectP = 64,
    // Class* derived from TObject and with NO comment field //->Class
    TString = 65,
    // TString, special case
    TObject = 66,
    // TObject, special case
    TNamed = 67,
    // TNamed,  special case
    Anyp = 68,
    // Class* not derived from TObject with    comment field //->Class
    AnyP = 69,
    // Class* not derived from TObject with NO comment field //->Class
    AnyPnoVT = 70,
    // Class* not derived from TObject with NO comment field //->Class and Class has NO virtual table
    STLp = 71, // Pointer to STL container

    Skip = 100,
    SkipL = 120,
    SkipP = 140,

    Conv = 200,
    ConvL = 220,
    ConvP = 240,

    STL = 300,
    STLstring = 365,

    Streamer = 500,
    StreamLoop = 501,
    /// Cache the value in memory than is not part of the object but is accessible via a SchemaRule
    Cache = 600,
    Artificial = 1000,
    CacheNew = 1001,
    CacheDelete = 1002,

    NeedObjectForVirtualBaseClass = 99997,
    Missing = 99999,
}

impl EnumNamed {
    pub fn from_i32(i: i32) -> Result<Self> {
        num::FromPrimitive::from_i32(i).ok_or(anyhow!("Cant make enum from {i}"))
    }

    pub fn to_i32(&self) -> Result<i32> {
        num::ToPrimitive::to_i32(self).ok_or(anyhow!("Cant make a i32 from {:?}", self))
    }
}

#[derive(FromPrimitive, ToPrimitive, Default, Debug)]
pub enum ESTLType {
    #[default]
    NotSTL = 0,
    STLvector = 1,
    STLlist = 2,
    STLdeque = 3,
    STLmap = 4,
    STLmultimap = 5,
    STLset = 6,
    STLmultiset = 7,
    STLbitset = 8,
    // Here the c++11 containers start. Order counts. For example,
    // tstreamerelements in written rootfiles carry a value and we cannot
    // introduce shifts.
    STLforwardlist = 9,
    STLunorderedset = 10,
    STLunorderedmultiset = 11,
    STLunorderedmap = 12,
    STLunorderedmultimap = 13,
    STLend = 14,
    STLany = 300,
    /* TVirtualStreamerInfo::kSTL */
    STLstdstring = 365,
    /* TVirtualStreamerInfo::kSTLstring */
}

impl ESTLType {
    pub fn from_i32(i: i32) -> Result<Self> {
        num::FromPrimitive::from_i32(i).ok_or(anyhow!("Cant make enum from {i}"))
    }

    pub fn to_i32(&self) -> Result<i32> {
        num::ToPrimitive::to_i32(self).ok_or(anyhow!("Cant make a i32 from {:?}", self))
    }
}
