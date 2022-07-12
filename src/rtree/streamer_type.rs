use crate::rdict::Streamer;
use crate::rmeta::{Enum, EnumNamed};
use lazy_static::lazy_static;
use log::trace;
use regex::Regex;

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
pub const kArtificial: i32 = 1000; // Cache the value in memory than is not part of the object but is accessible via a SchemaRule

#[allow(non_upper_case_globals)]
pub const kCacheNew: i32 = 1001;
#[allow(non_upper_case_globals)]
pub const kCacheDelete: i32 = 1002;
#[allow(non_upper_case_globals)]
pub const kNeedObjectForVirtualBaseClass: i32 = 99997;
#[allow(non_upper_case_globals)]
pub const kMissing: i32 = 99999;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(\b([A-Za-z_0-9]+)(\s*::\s*[A-Za-z_][A-Za-z_0-9]*)*\b(\s*\*)*|<|>|,)")
            .unwrap();
}

pub fn parse_typename(typename: &str) -> i32 {
    let tokens = RE.captures(typename).unwrap();

    trace!("tokens = {:?}", tokens);

    if tokens.get(0).unwrap().as_str() == "vector" {
        return 6;
    }

    if tokens.get(0).unwrap().as_str() == "set" {
        return 6;
    }

    if tokens.get(0).unwrap().as_str() == "map" {
        return 12;
    }

    0
}

pub fn header_bytes_from_type(ty: i32, streamer: Option<&Streamer>, class_name: &str) -> i32 {
    trace!(
        "ty = {} streamer = {} class_name = {}",
        ty,
        match streamer {
            None => {
                "None"
            }
            Some(s) => {
                s.name()
            }
        },
        class_name
    );
    match streamer {
        None => {}
        Some(streamer) => match streamer {
            Streamer::String(s) => match s.element().etype() {
                Enum::Named(a) => match a {
                    EnumNamed::Base => {}
                    EnumNamed::Char => {}
                    EnumNamed::Short => {}
                    EnumNamed::Int => {}
                    EnumNamed::Long => {}
                    EnumNamed::Float => {}
                    EnumNamed::Counter => {}
                    EnumNamed::CharStar => {}
                    EnumNamed::Double => {}
                    EnumNamed::Double32 => {}
                    EnumNamed::LegacyChar => {}
                    EnumNamed::UChar => {}
                    EnumNamed::UShort => {}
                    EnumNamed::UInt => {}
                    EnumNamed::ULong => {}
                    EnumNamed::Bits => {}
                    EnumNamed::Long64 => {}
                    EnumNamed::ULong64 => {}
                    EnumNamed::Bool => {}
                    EnumNamed::Float16 => {}
                    EnumNamed::OffsetL => {}
                    EnumNamed::OffsetP => {}
                    EnumNamed::Object => {}
                    EnumNamed::Any => {}
                    EnumNamed::Objectp => {}
                    EnumNamed::ObjectP => {}
                    EnumNamed::TString => {
                        return 0;
                    }
                    EnumNamed::TObject => {}
                    EnumNamed::TNamed => {}
                    EnumNamed::Anyp => {}
                    EnumNamed::AnyP => {}
                    EnumNamed::AnyPnoVT => {}
                    EnumNamed::STLp => {}
                    EnumNamed::Skip => {}
                    EnumNamed::SkipL => {}
                    EnumNamed::SkipP => {}
                    EnumNamed::Conv => {}
                    EnumNamed::ConvL => {}
                    EnumNamed::ConvP => {}
                    EnumNamed::STL => {}
                    EnumNamed::STLstring => {}
                    EnumNamed::Streamer => {}
                    EnumNamed::StreamLoop => {}
                    EnumNamed::Cache => {}
                    EnumNamed::Artificial => {}
                    EnumNamed::CacheNew => {}
                    EnumNamed::CacheDelete => {}
                    EnumNamed::NeedObjectForVirtualBaseClass => {}
                    EnumNamed::Missing => {}
                },
                Enum::Int(_a) => {}
            },
            Streamer::STLstring(_) => {
                return 6;
            }
            Streamer::BasicType(_) => {}
            Streamer::BasicPointer(_) => {}
            Streamer::ObjectAny(_) => {}
            Streamer::STL(_) => {}
            Streamer::Base(_) => {}
            Streamer::Object(_) => {}
            Streamer::ObjectPointer(_) => {}
        },
    }

    let header_bytes = match ty {
        -1 => parse_typename(class_name), // array
        i if i < kObject => {
            trace!("ty = {}, i = {}", ty, i);
            if i > kOffsetP {
                1
            } else {
                0
            }
        }
        kTString => 0,
        kSTL => 6,

        _ => todo!(),
    };

    header_bytes
}
