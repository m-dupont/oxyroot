use num;
use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum CantMakeError {
    EnumNamedFromInteger(i32),
    IntegerFromEnumNamed(EnumNamed),
    ESTLTypeFromInteger(i32),
    EReadWriteFromInteger(i32),
}

pub type Result<T> = std::result::Result<T, CantMakeError>;

#[derive(Debug, Clone, PartialEq)]
pub enum Enum {
    Named(EnumNamed),
    Int(i32),
}

impl Default for Enum {
    fn default() -> Self {
        Self::Named(EnumNamed::default())
    }
}

impl From<EnumNamed> for Enum {
    fn from(e: EnumNamed) -> Self {
        Self::Named(e)
    }
}

impl Enum {
    pub fn from_i32(i: i32) -> Self {
        match EnumNamed::from_i32(i) {
            Ok(ii) => Self::Named(ii),
            Err(_) => Enum::Int(i),
        }
    }

    pub fn to_i32(&self) -> i32 {
        // match  { }

        match self {
            Self::Named(o) => num::ToPrimitive::to_i32(o).unwrap(),
            Enum::Int(i) => *i,
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Default, Debug, Copy, Clone, PartialEq)]
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
    OffsetP16 = 56,
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

    Stl = 300,
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
        num::FromPrimitive::from_i32(i).ok_or(CantMakeError::EnumNamedFromInteger(i))
    }

    pub fn to_i32(self) -> Result<i32> {
        num::ToPrimitive::to_i32(&self).ok_or(CantMakeError::IntegerFromEnumNamed(self))
    }

    pub fn from_string(s: &str) -> Result<Self> {
        match s {
            "int" => Ok(EnumNamed::Int),
            "Int_t" => Ok(EnumNamed::Int),
            "int32_t" => Ok(EnumNamed::Int),
            "uint32_t" => Ok(EnumNamed::UInt),
            "int8_t" => Ok(EnumNamed::Char),
            "uint8_t" => Ok(EnumNamed::UChar),
            "unsigned int" => Ok(EnumNamed::UInt),
            "UInt_t" => Ok(EnumNamed::UInt),
            "short" => Ok(EnumNamed::Short),
            "Short_t" => Ok(EnumNamed::Short),
            "int16_t" => Ok(EnumNamed::Short),
            "unsigned short" => Ok(EnumNamed::UShort),
            "UShort_t" => Ok(EnumNamed::UShort),
            "uint16_t" => Ok(EnumNamed::UShort),
            "int64_t" => Ok(EnumNamed::Long64),
            "uint64_t" => Ok(EnumNamed::ULong64),
            "float" => Ok(EnumNamed::Float),
            "double" => Ok(EnumNamed::Double),
            _ => unimplemented!("{} not implemented", s),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Default, Debug, Clone)]
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
        num::FromPrimitive::from_i32(i).ok_or(CantMakeError::ESTLTypeFromInteger(i))
    }

    // pub fn to_i32(&self) -> Result<i32> {
    //     num::ToPrimitive::to_i32(self).ok_or_else(|| anyhow!("Cant make a i32 from {:?}", self))
    // }
}

#[derive(FromPrimitive, ToPrimitive, Default, Debug)]
pub enum EReadWrite {
    #[default]
    Base = 0,
    OffsetL = 20,
    OffsetP = 40,
    Counter = 6,
    CharStar = 7,
    Char = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 8,
    Double32 = 9,
    UChar = 11,
    UShort = 12,
    UInt = 13,
    ULong = 14,
    Bits = 15,
    Long64 = 16,
    ULong64 = 17,
    Bool = 18,
    Float16 = 19,
    Object = 61,
    Any = 62,
    Objectp = 63,
    ObjectP = 64,
    TString = 65,
    TObject = 66,
    TNamed = 67,
    Anyp = 68,
    AnyP = 69,
    AnyPnoVT = 70,
    STLp = 71,
    Skip = 100,
    SkipL = 120,
    SkipP = 140,
    Conv = 200,
    ConvL = 220,
    ConvP = 240,
    Stl = 300,
    //ROOT::kSTLany /* 300 */,
    STLstring = 365,
    //ROOT::kSTLstring /* 365 */,
    Streamer = 500,
    StreamLoop = 501,
    Cache = 600,
    // Cache the value in memory than is not part of the object but is accessible via a SchemaRule
    Artificial = 1000,
    CacheNew = 1001,
    CacheDelete = 1002,
    NeedObjectForVirtualBaseClass = 99997,
    Missing = 99999,
}

impl EReadWrite {
    pub fn from_i32(i: i32) -> Result<Self> {
        num::FromPrimitive::from_i32(i).ok_or(CantMakeError::EReadWriteFromInteger(i))
    }

    pub fn to_i32(&self) -> i32 {
        num::ToPrimitive::to_i32(self).expect("Can not go wrong")
    }
}

impl From<EReadWrite> for i32 {
    fn from(e: EReadWrite) -> Self {
        e.to_i32()
    }
}

impl PartialEq<EReadWrite> for i32 {
    fn eq(&self, other: &EReadWrite) -> bool {
        *self == other.to_i32()
    }
}

impl PartialOrd<EReadWrite> for i32 {
    fn partial_cmp(&self, other: &EReadWrite) -> Option<Ordering> {
        Some(self.cmp(&other.to_i32()))
    }
}

impl PartialEq<i32> for EReadWrite {
    fn eq(&self, other: &i32) -> bool {
        self.to_i32() == *other
    }
}

impl PartialOrd<i32> for EReadWrite {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        Some(self.to_i32().cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::rmeta::EReadWrite;

    #[test]
    fn compare_eread_write() {
        let offset_p_p1 = EReadWrite::OffsetP.to_i32();
        assert_eq!(offset_p_p1, EReadWrite::OffsetP);
        let offset_p_p1 = offset_p_p1 + 1;
        assert!(offset_p_p1 > EReadWrite::OffsetP);
        assert!(EReadWrite::OffsetP < offset_p_p1);
    }
}
