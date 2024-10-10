use crate::rdict::streamers::streamer_types::{
    StreamerBase, StreamerBasicPointer, StreamerBasicType, StreamerObject, StreamerObjectAny,
    StreamerObjectPointer, StreamerSTL, StreamerString,
};
use crate::rdict::{Streamer, StreamerElement};
use crate::rmeta::{ESTLType, Enum, EnumNamed};

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub(crate) struct ElementStrings {
    pub(crate) class: &'static str,
    pub(crate) f_name: &'static str,
    pub(crate) fTitle: &'static str,
    pub(crate) fSize: i32,
    pub(crate) fType: i32,
    pub(crate) fTypeName: &'static str,
    pub(crate) fBaseVersion: Option<i32>,
    pub(crate) fCountName: Option<&'static str>,
    pub(crate) fCountClass: Option<&'static str>,
    pub(crate) fCountVersion: Option<i32>,
    pub(crate) fSTLtype: Option<i32>,
}

impl ElementStrings {
    pub(crate) fn etype(&self) -> Enum {
        Enum::from_i32(self.fType)
    }

    pub(crate) fn esize(&self) -> i32 {
        self.fSize
    }

    pub(crate) fn build_streamer(&self, mut streamer_element: StreamerElement) -> Streamer {
        match self.etype() {
            Enum::Named(named) => match named {
                EnumNamed::Long64
                | EnumNamed::Int
                | EnumNamed::Long
                | EnumNamed::Double
                | EnumNamed::Counter
                | EnumNamed::Short
                | EnumNamed::Float
                | EnumNamed::Float16
                | EnumNamed::Double32
                | EnumNamed::UInt
                | EnumNamed::Bits
                | EnumNamed::UShort
                | EnumNamed::UChar
                | EnumNamed::Char
                | EnumNamed::CharStar
                | EnumNamed::Bool => {
                    let basic = StreamerBasicType {
                        element: streamer_element,
                    };
                    Streamer::BasicType(basic)
                }
                EnumNamed::TNamed | EnumNamed::Base | EnumNamed::TObject => {
                    let vbase = self.fBaseVersion.unwrap();

                    let base = StreamerBase {
                        element: streamer_element,
                        vbase,
                    };
                    Streamer::Base(base)
                }
                EnumNamed::OffsetP16 => {
                    let cvers = self.fCountVersion.unwrap();
                    let cname = self.fCountName.as_ref().unwrap();
                    let ccls = self.fCountClass.as_ref().unwrap();

                    let s = StreamerBasicPointer {
                        element: streamer_element,
                        cvers,
                        cname: cname.to_string(),
                        ccls: ccls.to_string(),
                    };
                    Streamer::BasicPointer(s)
                }
                EnumNamed::Any => {
                    let s = StreamerObjectAny {
                        element: streamer_element,
                    };
                    Streamer::ObjectAny(s)
                }
                EnumNamed::Object => {
                    let s = StreamerObject {
                        element: streamer_element,
                    };
                    Streamer::Object(s)
                }
                EnumNamed::ObjectP | EnumNamed::Objectp => {
                    let s = StreamerObjectPointer {
                        element: streamer_element,
                    };
                    Streamer::ObjectPointer(s)
                }
                EnumNamed::TString => {
                    let s = StreamerString {
                        element: streamer_element,
                    };
                    Streamer::String(s)
                }
                EnumNamed::Stl => {
                    let vtype = ESTLType::from_i32(self.fSTLtype.unwrap()).unwrap();
                    let _ctype = Enum::from_i32(self.fSTLtype.unwrap());
                    let ctype = Enum::Named(EnumNamed::Object);
                    streamer_element.etype = Enum::Named(EnumNamed::Streamer);
                    let s = StreamerSTL {
                        element: streamer_element,
                        vtype,
                        ctype,
                    };
                    Streamer::Stl(s)
                }
                _ => {
                    unimplemented!(
                        "populate_db: named type: {:?} -- class = {}",
                        named,
                        self.f_name
                    );
                }
            },
            Enum::Int(i) => match i {
                23 | 31 => {
                    let basic = StreamerBasicType {
                        element: streamer_element,
                    };
                    Streamer::BasicType(basic)
                }
                41..=48 | 51 | 53 => {
                    let cvers = self.fCountVersion.unwrap();
                    let cname = self.fCountName.as_ref().unwrap();
                    let ccls = self.fCountClass.as_ref().unwrap();

                    let s = StreamerBasicPointer {
                        element: streamer_element,
                        cvers,
                        cname: cname.to_string(),
                        ccls: ccls.to_string(),
                    };
                    Streamer::BasicPointer(s)
                }
                _ => {
                    unimplemented!("populate_db: int type: {:?} -- class = {}", i, self.f_name);
                }
            },
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.f_name
    }
}

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub(crate) struct ClassStrings {
    pub(crate) class: &'static str,
    pub(crate) title: &'static str,
    pub(crate) fName: &'static str,
    pub(crate) fCheckSum: u32,
    pub(crate) fClassVersion: i32,
}

impl ClassStrings {
    pub(crate) fn rvers(&self) -> i32 {
        let rvers = self.fClassVersion;
        let rvers = if rvers < 0 { 1 } else { rvers };
        rvers
    }

    pub fn name(&self) -> &str {
        &self.fName
    }
}

#[derive(Debug, Default)]
struct ClassStreamerStrings {
    class: ClassStrings,
    elements: Vec<ElementStrings>,
}
