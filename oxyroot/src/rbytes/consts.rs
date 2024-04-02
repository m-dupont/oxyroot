#[allow(non_upper_case_globals)]
pub const kNullTag: i64 = 0;
// on tag :
#[allow(non_upper_case_globals)]
pub const kNewClassTag: i64 = 0xFFFFFFFF;
#[allow(non_upper_case_globals)]
pub const kClassMask: i64 = 0x80000000;
#[allow(non_upper_case_globals)]
pub const kMapOffset: i64 = 2;
#[allow(non_upper_case_globals, dead_code)]
pub const kByteCountVMask: i64 = 0x4000;
#[allow(non_upper_case_globals)]
pub const kByteCountMask: i64 = 0x40000000;
#[allow(non_upper_case_globals, dead_code)]
pub const kIsOnHeap: i64 = 0x01000000;
//kNotDeleted   = 0x02000000
//kZombie       = 0x04000000
//K_BIT_MASK      = 0x00ffffff
#[allow(non_upper_case_globals, dead_code)]
pub const kIsReferenced: i64 = 1 << 4;

//baskets
#[allow(non_upper_case_globals, dead_code)]
pub const DisplacementMask: i64 = 0xFF000000;
