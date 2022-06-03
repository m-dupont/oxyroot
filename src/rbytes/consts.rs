pub const kNullTag: i64 = 0;
// on tag :
pub const kNewClassTag: i64 = 0xFFFFFFFF;
pub const kClassMask: i64 = 0x80000000;
pub const kMapOffset: i64 = 2;
pub const kByteCountVMask: i64 = 0x4000;
pub const kByteCountMask: i64 = 0x40000000;

pub const kIsOnHeap: i64 = 0x01000000;
//kNotDeleted   = 0x02000000
//kZombie       = 0x04000000
//kBitMask      = 0x00ffffff
pub const kIsReferenced: i64 = 1 << 4;

//baskets
pub const DisplacementMask: i64 = 0xFF000000;
