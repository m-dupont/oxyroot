use log::trace;

#[allow(non_upper_case_globals)]
pub const kBase: i32 = 0;
#[allow(non_upper_case_globals)]
pub const kOffsetL: i32 = 20;
#[allow(non_upper_case_globals)]
pub const kOffsetP: i32 = 40;
#[allow(non_upper_case_globals)]
pub const kCounter: i32 = 6;
#[allow(non_upper_case_globals)]
pub const kCharStar: i32 = 7;
#[allow(non_upper_case_globals)]
pub const kChar: i32 = 1;
#[allow(non_upper_case_globals)]
pub const kShort: i32 = 2;
#[allow(non_upper_case_globals)]
pub const kInt: i32 = 3;
#[allow(non_upper_case_globals)]
pub const kLong: i32 = 4;
#[allow(non_upper_case_globals)]
pub const kFloat: i32 = 5;
#[allow(non_upper_case_globals)]
pub const kDouble: i32 = 8;
#[allow(non_upper_case_globals)]
pub const kDouble32: i32 = 9;
#[allow(non_upper_case_globals)]
pub const kUChar: i32 = 11;
#[allow(non_upper_case_globals)]
pub const kUShort: i32 = 12;
#[allow(non_upper_case_globals)]
pub const kUInt: i32 = 13;
#[allow(non_upper_case_globals)]
pub const kULong: i32 = 14;
#[allow(non_upper_case_globals)]
pub const kBits: i32 = 15;
#[allow(non_upper_case_globals)]
pub const kLong64: i32 = 16;
#[allow(non_upper_case_globals)]
pub const kULong64: i32 = 17;
#[allow(non_upper_case_globals)]
pub const kBool: i32 = 18;
#[allow(non_upper_case_globals)]
pub const kFloat16: i32 = 19;
#[allow(non_upper_case_globals)]
pub const kObject: i32 = 61;
#[allow(non_upper_case_globals)]
pub const kAny: i32 = 62;
#[allow(non_upper_case_globals)]
pub const kObjectp: i32 = 63;
#[allow(non_upper_case_globals)]
pub const kObjectP: i32 = 64;
#[allow(non_upper_case_globals)]
pub const kTString: i32 = 65;
#[allow(non_upper_case_globals)]
pub const kTObject: i32 = 66;
#[allow(non_upper_case_globals)]
pub const kTNamed: i32 = 67;
#[allow(non_upper_case_globals)]
pub const kAnyp: i32 = 68;
#[allow(non_upper_case_globals)]
pub const kAnyP: i32 = 69;
#[allow(non_upper_case_globals)]
pub const kAnyPnoVT: i32 = 70;
#[allow(non_upper_case_globals)]
pub const kSTLp: i32 = 71;
#[allow(non_upper_case_globals)]
pub const kSkip: i32 = 100;
#[allow(non_upper_case_globals)]
pub const kSkipL: i32 = 120;
#[allow(non_upper_case_globals)]
pub const kSkipP: i32 = 140;
#[allow(non_upper_case_globals)]
pub const kConv: i32 = 200;
#[allow(non_upper_case_globals)]
pub const kConvL: i32 = 220;
#[allow(non_upper_case_globals)]
pub const kConvP: i32 = 240;
#[allow(non_upper_case_globals)]
pub const kSTL: i32 = 300 /* 300 */;
#[allow(non_upper_case_globals)]
pub const kSTLstring: i32 = 365 /* 365 */;
#[allow(non_upper_case_globals)]
pub const kStreamer: i32 = 500;
#[allow(non_upper_case_globals)]
pub const kStreamLoop: i32 = 501;
#[allow(non_upper_case_globals)]
pub const kCache: i32 = 600;
#[allow(non_upper_case_globals)]
pub const // Cache the value in memory than is not part of the object but is accessible via a SchemaRule
kArtificial: i32 = 1000;
#[allow(non_upper_case_globals)]
pub const kCacheNew: i32 = 1001;
#[allow(non_upper_case_globals)]
pub const kCacheDelete: i32 = 1002;
#[allow(non_upper_case_globals)]
pub const kNeedObjectForVirtualBaseClass: i32 = 99997;
#[allow(non_upper_case_globals)]
pub const kMissing: i32 = 99999;

pub fn header_bytes_from_type(ty: i32) -> i32 {
    let header_bytes = match ty {
        -1 => 10, // array
        i if i < kObject => {
            trace!("ty = {}, i = {}", ty, i);
            if i > kOffsetP {
                1
            } else {
                0
            }
        }
        kTString => 0,
        kSTL => 10,

        _ => todo!(),
    };

    header_bytes
}
