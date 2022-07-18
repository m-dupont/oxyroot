use crate::rdict::Streamer;
use crate::rmeta::{EReadWrite, Enum, EnumNamed};
use lazy_static::lazy_static;
use log::trace;
use regex::Regex;

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
            Streamer::Stl(_) => {}
            Streamer::Base(_) => {}
            Streamer::Object(_) => {}
            Streamer::ObjectPointer(_) => {}
        },
    }

    if ty == -1 {
        return parse_typename(class_name);
    } else if ty < EReadWrite::Object {
        if ty > EReadWrite::OffsetP {
            return 1;
        } else {
            return 0;
        }
    } else if ty == EReadWrite::TString {
        return 0;
    } else if ty == EReadWrite::STL {
        return 6;
    }

    todo!();
}

pub(crate) fn _from_leaftype_to_str(leaftype: i32) -> Option<&'static str> {
    // trace!("leaftype = {}", leaftype);

    if leaftype < 0 {
        return None;
    }

    let leaftype = if EReadWrite::OffsetL < leaftype && leaftype < EReadWrite::OffsetP {
        leaftype - EReadWrite::OffsetL.to_i32()
    } else {
        leaftype
    };

    // trace!("leaftype = {}", leaftype);
    let leaftype = if leaftype > EReadWrite::OffsetP
        && (leaftype - EReadWrite::OffsetP.to_i32()) < EReadWrite::OffsetP
    {
        leaftype - EReadWrite::OffsetP.to_i32()
    } else {
        leaftype
    };

    assert!(leaftype > 0);

    // trace!("leaftype = {}", leaftype);

    match EReadWrite::from_i32(leaftype) {
        Ok(leaftype) => match leaftype {
            EReadWrite::Char => Some("int8_t"),
            EReadWrite::UChar => Some("uint8_t"),

            EReadWrite::Short => Some("int16_t"),
            EReadWrite::UShort => Some("uint16_t"),

            EReadWrite::Int => Some("int32_t"),
            EReadWrite::UInt | EReadWrite::Bits | EReadWrite::Counter => Some("uint32_t"),

            EReadWrite::Long => Some("int64_t"),
            EReadWrite::ULong => Some("uint64_t"),

            EReadWrite::Float => Some("float"),
            EReadWrite::Double => Some("double"),

            EReadWrite::TString => Some("TString"),

            _ => {
                // todo!()
                None
            }
        },
        _ => None,
    }
}

pub(crate) fn clean_type_name(ty: &str) -> String {
    let ret = ty.replace("unsigned int", "uint32_t");
    let ret = ret.replace("int", "int32_t");
    let ret = ret.replace("uint32_t32_t", "uint32_t");

    let ret = ret.replace("unsigned short", "uint16_t");
    let ret = ret.replace("short", "int16_t");

    let ret = ret.replace("unsigned long", "uint64_t");
    let ret = ret.replace("long", "int64_t");

    ret.replace(" ", "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_clean_type_name() -> Result<()> {
        assert_eq!(clean_type_name("vector<int>"), "vector<int32_t>");
        assert_eq!(clean_type_name("vector<unsigned int>"), "vector<uint32_t>");

        assert_eq!(clean_type_name("vector<short>"), "vector<int16_t>");
        assert_eq!(
            clean_type_name("vector<unsigned short>"),
            "vector<uint16_t>"
        );

        assert_eq!(
            clean_type_name("vector<unsigned short >"),
            "vector<uint16_t>"
        );

        Ok(())
    }
}
