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
                    EnumNamed::Stl => {}
                    EnumNamed::STLstring => {}
                    EnumNamed::Streamer => {}
                    EnumNamed::StreamLoop => {}
                    EnumNamed::Cache => {}
                    EnumNamed::Artificial => {}
                    EnumNamed::CacheNew => {}
                    EnumNamed::CacheDelete => {}
                    EnumNamed::NeedObjectForVirtualBaseClass => {}
                    EnumNamed::Missing => {}
                    EnumNamed::OffsetP16 => {
                        todo!()
                    }
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
    } else if ty == EReadWrite::Stl {
        return 6;
    }

    todo!();
}

pub(crate) fn _from_leaftype_to_str(leaftype: i32) -> Option<&'static str> {
    // trace!("leaftype = {}", leaftype);

    trace!(";_from_leaftype_to_str.leaftype:{}", leaftype);

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

    // assert!(leaftype > 0);

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

pub fn replace_string(s: &str, from: &str, to: &str) -> String {
    let mut ret = String::new();
    let indices: Vec<_> = s.match_indices(from).collect();
    let mut g_indices = Vec::new();
    let chars = s.chars().collect::<Vec<_>>();
    for (i, _) in indices.iter() {
        let start = {
            if *i == 0 {
                None
            } else {
                chars.get(*i - 1)
            }
        };

        let end = chars.get(*i + from.len());

        match (start, end) {
            (Some(start), Some(end)) => {
                if start.is_alphabetic() || end.is_alphabetic() {
                    continue;
                } else {
                    g_indices.push(*i);
                }
            }
            (Some(start), None) => {
                if start.is_alphabetic() {
                    continue;
                } else {
                    g_indices.push(*i);
                }
            }
            (None, Some(end)) => {
                if end.is_alphabetic() {
                    continue;
                } else {
                    g_indices.push(*i);
                }
            }
            (None, None) => {
                g_indices.push(*i);
            }
        }
    }

    let mut last_end = 0;

    for start in g_indices.iter() {
        let start = *start;
        ret.push_str(&s[last_end..start]);
        ret.push_str(to);
        last_end = start + from.len();
    }
    ret.push_str(&s[last_end..s.len()]);

    ret
}

/// change c++ typename from "int" to int32_t and so on
pub(crate) fn clean_type_name<T: AsRef<str>>(ty: T) -> String {
    let ret = ty.as_ref().to_string().replace("unsigned int", "uint32_t");
    let ret = replace_string(&ret, "int", "int32_t");
    // undo change (uint32_t -> uint32_t32_t)
    let ret = ret.replace("uint32_t32_t", "uint32_t");
    // undo change (int32_t -> int32_t32_t)
    let ret = ret.replace("int32_t32_t", "int32_t");
    let ret = ret.replace("uint32_t16_t", "uint16_t");
    let ret = ret.replace("uint32_t8_t", "uint8_t");
    let ret = ret.replace("int32_t16_t", "int16_t");
    let ret = ret.replace("int32_t8_t", "int8_t");
    let ret = ret.replace("int32_t64_t", "int64_t");

    let ret = ret.replace("unsigned short", "uint16_t");
    let ret = ret.replace("short", "int16_t");

    let ret = ret.replace("unsigned long", "uint64_t");
    let ret = ret.replace("long", "int64_t");

    ret.replace(' ', "")
}

/// Convert C++ templated name to a rust one
/// Example vector<int> -> Vec<i32>
pub(crate) fn type_name_cpp_to_rust(ty: &str) -> String {
    let ty = clean_type_name(ty);
    let ret = ty.replace("string", "String");
    let ret = ret.replace("vector", "Vec");
    let ret = ret.replace("map", "HashMap");
    let mut ret = ret.replace("set", "HashSet");

    let replaces = [
        ("uint64_t", "u64"),
        ("int64_t", "i64"),
        ("uint32_t", "u32"),
        ("int32_t", "i32"),
        ("uint16_t", "u16"),
        ("int16_t", "i16"),
        ("uint8_t", "u8"),
        ("int8_t", "i8"),
        ("float", "f32"),
        ("double", "f64"),
        ("bool", "bool"),
    ];

    for (cpp, rust) in replaces.iter() {
        ret = ret.replace(cpp, rust);

        //i32[] -> Slice<i32>
        let cpp_s = format!("{}[]", rust);
        let rust_s = format!("Slice<{}>", rust);
        ret = ret.replace(cpp_s.as_str(), rust_s.as_str());

        let start = &format!("{}[", rust);
        let end = "]";
        // trace!("ty = {} -> ret = {}", ty, ret);
        if let Some(start) = ret.find(start) {
            let start = start + rust.len() + 1;
            // trace!("\tstart = {}", start);

            let right = ret.get(start..).unwrap();
            // trace!("\tright = {}", right);
            if !right.starts_with(']') {
                let end = right.find(end).unwrap();
                // trace!("\t\tend = {}", end);
                let inner = ret.get(start..(end + start)).unwrap();
                // trace!("\t\tinner = {}", inner);
                let rust_s = format!("{}[{}]", rust, inner);
                let rust_o = format!("[{};{}]", rust, inner);
                // trace!("\t\trust_s = {}", rust_s);
                ret = ret.replace(rust_s.as_str(), rust_o.as_str());
            }
        }
    }

    ret.replace("char*", "String")
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_replace_string() -> Result<()> {
        let s = "int";
        let from = "int";
        let to = "int32_t";
        assert_eq!(replace_string(s, from, to), "int32_t");

        let s = "unsigned int";
        let from = "unsigned int";
        let to = "uint32_t";
        assert_eq!(replace_string(s, from, to), "uint32_t");

        let s = "Point";
        let from = "int";
        let to = "uint32_t";
        assert_eq!(replace_string(s, from, to), "Point");

        let s = "vector<int>";
        let from = "int";
        let to = "int32_t";
        assert_eq!(replace_string(s, from, to), "vector<int32_t>");

        Ok(())
    }

    #[test]
    fn test_clean_type_name() -> Result<()> {
        let dirties_clean = [
            ("vector<long>", "vector<int64_t>"),
            ("vector<long>", "vector<int64_t>"),
            ("vector<int>", "vector<int32_t>"),
            ("vector<set<int>>", "vector<set<int32_t>>"),
            ("vector<unsigned int>", "vector<uint32_t>"),
            ("vector<short>", "vector<int16_t>"),
            ("vector<unsigned short>", "vector<uint16_t>"),
            ("int[10]", "int32_t[10]"),
            ("unsigned int[10]", "uint32_t[10]"),
            ("Point", "Point"),
        ];

        dirties_clean.iter().for_each(|(dirty, clean)| {
            assert_eq!(clean_type_name(dirty), *clean);
            assert_eq!(clean_type_name(dirty), clean_type_name(clean));
            assert_eq!(clean_type_name(clean_type_name(dirty).as_str()), *clean);
        });

        Ok(())
    }

    #[test]
    fn test_clean_type_to_rust() -> Result<()> {
        let names = [
            ("int8_t", "i8"),
            ("vector<int>", "Vec<i32>"),
            ("vector<set<int>>", "Vec<HashSet<i32>>"),
            ("vector<unsigned int>", "Vec<u32>"),
            ("vector<short>", "Vec<i16>"),
            ("vector<unsigned short>", "Vec<u16>"),
            ("int[10]", "[i32;10]"),
            ("vector<int[10]>", "Vec<[i32;10]>"),
            ("int[]", "Slice<i32>"),
            ("unsigned int[10]", "[u32;10]"),
        ];

        names.iter().for_each(|(cpp, rust)| {
            assert_eq!(type_name_cpp_to_rust(cpp), *rust);
            // assert_eq!(clean_type_name(cpp), clean_type_name(rust));
            // assert_eq!(clean_type_name(clean_type_name(cpp).as_str()), *rust);
        });

        Ok(())
    }
}
