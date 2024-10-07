use crate::rdict::error::Error;
use crate::rdict::streamers::db::DbStreamer;
use crate::rdict::{Streamer, StreamerElement, StreamerInfo};
use crate::Named;
use lazy_static::lazy_static;
use log::trace;
use regex::Regex;
use std::collections::HashMap;

use crate::rdict::error::Result;
use crate::rdict::streamers::streamer_types::{
    StreamerBase, StreamerBasicPointer, StreamerBasicType, StreamerObject, StreamerObjectAny,
    StreamerObjectPointer, StreamerSTL, StreamerString,
};
use crate::rmeta::{ESTLType, Enum, EnumNamed};

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"==> Dumping object at: 0x[a-z0-9]+, name=(?<name>[A-Za-z0-9_:]+), class=(?<class>[A-Za-z0-9]+)"
    )
    .unwrap();

}

#[derive(Debug, Default)]
pub(crate) struct ElementStrings {
    pub(crate) class: String,
    pub(crate) f_name: String,
    pub(crate) fTitle: String,
    pub(crate) fSize: i32,
    pub(crate) fType: i32,
    pub(crate) fTypeName: String,
    pub(crate) fBaseVersion: Option<i32>,
    pub(crate) fCountName: Option<String>,
    pub(crate) fCountClass: Option<String>,
    pub(crate) fCountVersion: Option<i32>,
    pub(crate) fSTLtype: Option<i32>,
}

impl ElementStrings {
    fn etype(&self) -> Enum {
        Enum::from_i32(self.fType)
    }

    fn esize(&self) -> i32 {
        self.fSize
    }

    fn build_streamer(&self, mut streamer_element: StreamerElement) -> Streamer {
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

    pub fn name(&self) -> &str {
        &self.f_name
    }
}

fn generate_elements_strings(lines: Vec<Vec<&str>>) -> Vec<ElementStrings> {
    let mut ret = Vec::new();
    for elements_lines in lines {
        let mut element = ElementStrings::default();
        let header = elements_lines[0];
        // println!("header = {:?}", header);

        let r = RE.captures(header).unwrap();
        assert_eq!(r.len(), 3);
        element.class = r["class"].to_string();
        element.f_name = r["name"].to_string();

        for line in elements_lines.iter().skip(1) {
            //let line = line.split_ascii_whitespace().collect::<Vec<_>>();
            let key = &line[0..30].trim();
            let value = &line[30..50].trim();

            match *key {
                "fSize" => {
                    element.fSize = value.parse::<_>().unwrap();
                }
                "fType" => {
                    element.fType = value.parse::<_>().unwrap();
                }
                "fTypeName" => {
                    element.fTypeName = value.to_string();
                }
                "fCountVersion" => {
                    element.fCountVersion = Some(value.parse::<_>().unwrap());
                }
                "fCountName" => {
                    element.fCountName = Some(value.to_string());
                }
                "fCountClass" => {
                    element.fCountClass = Some(value.to_string());
                }

                "fBaseVersion" => {
                    element.fBaseVersion = Some(value.parse::<_>().unwrap());
                }
                "fSTLtype" => {
                    element.fSTLtype = Some(value.parse::<_>().unwrap());
                }
                _ => {}
            }
        }
        // println!("element = {:?}", element);
        ret.push(element);
    }
    ret
}

#[derive(Debug, Default)]
pub(crate) struct ClassStrings {
    pub(crate) class: String,
    pub(crate) fName: String,
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

fn generate_class_strings(elements_lines: Vec<&str>) -> ClassStrings {
    let mut element = ClassStrings::default();
    let header = elements_lines[0];
    // println!("header = {:?}", header);

    let r = RE.captures(header).unwrap();
    assert_eq!(r.len(), 3);
    element.class = r["class"].to_string();
    element.fName = r["name"].to_string();

    for line in elements_lines.iter().skip(1) {
        let line = line.split_ascii_whitespace().collect::<Vec<_>>();
        let key = line[0];
        let value = line[1];
        if key == "fCheckSum" {
            let value = value.parse::<u32>().unwrap();
            element.fCheckSum = value;
        }
        if key == "fClassVersion" {
            let value = value.parse::<i32>().unwrap();
            element.fClassVersion = value;
        }
    }
    // println!("element = {:?}", element);
    element
}

enum WhatIAmReading {
    Header,
    StreamerInfo,
    Elements,
    Trailer,
}

#[derive(Debug, Default)]
struct ClassStreamerStrings {
    class: ClassStrings,
    elements: Vec<ElementStrings>,
}

// static DUMP: &str = include_str!("/tmp/rust/gen_streamers/gen_all.txt");

fn generate_class(dump: &str) -> Result<Vec<ClassStreamerStrings>> {
    // let dump = gen_cat_streamers_with_root(class)?; // TODO: implement gen_cat_streamers_with_root
    // println!("dump = {}", dump);

    let mut get_streamer_infos = Vec::new();
    let mut element = Vec::new();
    let mut what = WhatIAmReading::Header;

    #[derive(Debug, Default)]
    struct CurrentClass<'a> {
        get_streamer_info: Vec<&'a str>,
        get_elements: Vec<Vec<&'a str>>,
        current_class_name_point_virgule: &'a str,
    }

    let mut current_class = CurrentClass::default();
    let mut current_class_name_point_virgule = "";

    for line in dump.split('\n') {
        if line.is_empty() {
            continue;
        }
        if line.starts_with(';') {
            let l = line.split('=').collect::<Vec<_>>()[1].trim();
            what = match what {
                WhatIAmReading::Header => {
                    assert!(line.starts_with(";get_streamer_info"));
                    current_class_name_point_virgule = l;
                    WhatIAmReading::StreamerInfo
                }
                WhatIAmReading::StreamerInfo => {
                    assert!(line.starts_with(";get_elements"));
                    assert_eq!(current_class_name_point_virgule, l);
                    WhatIAmReading::Elements
                }
                WhatIAmReading::Elements => {
                    assert!(line.starts_with(";End"));
                    assert_eq!(current_class_name_point_virgule, l);
                    WhatIAmReading::Header
                }
                WhatIAmReading::Trailer => {
                    return Err(Error::StreamerReadDumpError(format!(
                        "unexpected line: {}",
                        line
                    )))
                }
            };
            continue;
        }

        match what {
            WhatIAmReading::Header => {
                if !current_class.get_streamer_info.is_empty() {
                    // current_class.current_class_name_point_virgule =
                    //     current_class_name_point_virgule;
                    if !element.is_empty() {
                        current_class.get_elements.push(element);
                        element = Vec::new();
                    }
                    get_streamer_infos.push(current_class);
                    current_class = CurrentClass::default();
                }
            }
            WhatIAmReading::StreamerInfo => {
                if line.starts_with("==>") {

                    //element = Vec::new();
                }
                current_class.get_streamer_info.push(line);
            }
            WhatIAmReading::Elements => {
                if line.starts_with("==>") && !element.is_empty() {
                    {
                        current_class.get_elements.push(element);
                        element = Vec::new();
                    }
                }
                if !line.is_empty() {
                    element.push(line);
                }

                // get_elements.push(line);
            }
            WhatIAmReading::Trailer => {}
        }
    }

    let mut ret = Vec::new();

    for current_class in get_streamer_infos.into_iter() {
        trace!(
            ";generate_class.current_class.name:{:?}",
            current_class.get_streamer_info.first()
        );
        let c = ClassStreamerStrings {
            class: generate_class_strings(current_class.get_streamer_info),
            elements: generate_elements_strings(current_class.get_elements),
        };
        trace!(";generate_class.c.class.name:{:?}", c.class.fName);

        // assert_eq!(
        //     c.class.f_name,
        //     current_class.current_class_name_point_virgule
        // );

        ret.push(c);
    }

    Ok(ret)
}

pub fn populate_db(db: &mut DbStreamer, dump: &'static str) -> Result<()> {
    trace!(";populate_db.call:{}", true);
    let titles = make_titles();
    let classes_str = generate_class(dump).unwrap();

    let mut id_elements = 0;

    for class_str in classes_str {
        let class = &class_str.class;
        trace!(";populate_db.class_name:{}", class.name());

        let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());
        // let he = titles.get(class_name as &str);

        for element_str in class_str.elements {
            let mut streamer_element = StreamerElement::new(
                element_str.name(),
                element_str.etype(),
                element_str.esize(),
                id_elements,
            );

            if let Some(he) = titles.get(class.name()) {
                if let Some(h) = he.get(element_str.name()) {
                    if let Some(title) = h.get("title") {
                        streamer_element.named =
                            streamer_element.named.with_title(title.to_string());
                    }
                }
            }

            id_elements += 1;
            streamer_element.ename = element_str.fTypeName.clone();

            // trace!(";populate_db.element_str:{:?}", element_str);
            // trace!(";populate_db.streamer_element:{:?}", streamer_element);

            matches!(element_str.etype(), Enum::Named(_));
            let streamer = element_str.build_streamer(streamer_element);
            streamer_info.elems.push(streamer);
        }

        let key = format!("{}-{}", streamer_info.name(), streamer_info.clsver());
        streamer_info.id = id_elements;
        db.insert(streamer_info);
    }
    Ok(())
}

fn make_titles() -> HashMap<&'static str, HashMap<&'static str, HashMap<&'static str, &'static str>>>
{
    let mut hh = HashMap::new();
    // TTree
    //si =  {'properties': {'f_name': 'TTree'}, 'elements': {'fMaxEntries': {'title': 'Maximum number of entries in case of circular buffers', 'name': 'fMaxEntries'}, 'fMaxEntryLoop': {'title': 'Maximum number of entries to process', 'name': 'fMaxEntryLoop'}, 'fZipBytes': {'name': 'fZipBytes', 'title': 'Total number of bytes in all branches after compression'}, 'fTotBytes': {'title': 'Total number of bytes in all branches before compression', 'name': 'fTotBytes'}, 'fClusterSize': {'title': '[fNClusterRange] Number of entries in each cluster for a given range.', 'name': 'fClusterSize'}, 'fAliases': {'title': 'List of aliases for expressions based on the tree branches.', 'name': 'fAliases'}, 'fDefaultEntryOffsetLen': {'title': 'Initial Length of fEntryOffset table in the basket buffers', 'name': 'fDefaultEntryOffsetLen'}, 'TAttMarker': {'name': 'TAttMarker', 'title': 'Marker attributes'}, 'fFlushedBytes': {'name': 'fFlushedBytes', 'title': 'Number of auto-flushed bytes'}, 'fUpdate': {'title': 'Update frequency for EntryLoop', 'name': 'fUpdate'}, 'fTreeIndex': {'title': 'Pointer to the tree Index (if any)', 'name': 'fTreeIndex'}, 'fBranchRef': {'title': 'Branch supporting the TRefTable (if any)', 'name': 'fBranchRef'}, 'fAutoFlush': {'name': 'fAutoFlush', 'title': 'Auto-flush tree when fAutoFlush entries written or -fAutoFlush (compressed) bytes produced'}, 'fUserInfo': {'title': 'pointer to a list of user objects associated to this Tree', 'name': 'fUserInfo'}, 'fFriends': {'title': 'pointer to list of friend elements', 'name': 'fFriends'}, 'fEstimate': {'title': 'Number of entries to estimate histogram limits', 'name': 'fEstimate'}, 'TNamed': {'name': 'TNamed', 'title': 'The basis for a named object (name, title)'}, 'fWeight': {'title': 'Tree weight (see TTree::SetWeight)', 'name': 'fWeight'}, 'TAttFill': {'name': 'TAttFill', 'title': 'Fill area attributes'}, 'fTimerInterval': {'name': 'fTimerInterval', 'title': 'Timer interval in milliseconds'}, 'TAttLine': {'title': 'Line attributes', 'name': 'TAttLine'}, 'fMaxVirtualSize': {'name': 'fMaxVirtualSize', 'title': 'Maximum total size of buffers kept in memory'}, 'fSavedBytes': {'name': 'fSavedBytes', 'title': 'Number of autosaved bytes'}, 'fAutoSave': {'title': 'Autosave tree when fAutoSave entries written or -fAutoSave (compressed) bytes produced', 'name': 'fAutoSave'}, 'fIOFeatures': {'title': 'IO features to define for newly-written baskets and branches.', 'name': 'fIOFeatures'}, 'fBranches': {'name': 'fBranches', 'title': 'List of Branches'}, 'fEntries': {'title': 'Number of entries', 'name': 'fEntries'}, 'fScanField': {'name': 'fScanField', 'title': 'Number of runs before prompting in Scan'}, 'fNClusterRange': {'name': 'fNClusterRange', 'title': "Number of Cluster range in addition to the one defined by 'AutoFlush'"}, 'fLeaves': {'name': 'fLeaves', 'title': 'Direct pointers to individual branch leaves'}, 'fClusterRangeEnd': {'title': '[fNClusterRange] Last entry of a cluster range.', 'name': 'fClusterRangeEnd'}, 'fIndex': {'title': 'Index of sorted values', 'name': 'fIndex'}, 'fIndexValues': {'name': 'fIndexValues', 'title': 'Sorted index values'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Maximum number of entries in case of circular buffers",
    );
    he.insert("fMaxEntries", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum number of entries to process");
    he.insert("fMaxEntryLoop", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Total number of bytes in all branches after compression",
    );
    he.insert("fZipBytes", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Total number of bytes in all branches before compression",
    );
    he.insert("fTotBytes", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "[fNClusterRange] Number of entries in each cluster for a given range.",
    );
    he.insert("fClusterSize", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "List of aliases for expressions based on the tree branches.",
    );
    he.insert("fAliases", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Initial Length of fEntryOffset table in the basket buffers",
    );
    he.insert("fDefaultEntryOffsetLen", h);
    let mut h = HashMap::new();
    h.insert("title", "Marker attributes");
    he.insert("TAttMarker", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of auto-flushed bytes");
    he.insert("fFlushedBytes", h);
    let mut h = HashMap::new();
    h.insert("title", "Update frequency for EntryLoop");
    he.insert("fUpdate", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to the tree Index (if any)");
    he.insert("fTreeIndex", h);
    let mut h = HashMap::new();
    h.insert("title", "Branch supporting the TRefTable (if any)");
    he.insert("fBranchRef", h);
    let mut h = HashMap::new();
    h.insert("title", "Auto-flush tree when fAutoFlush entries written or -fAutoFlush (compressed) bytes produced");
    he.insert("fAutoFlush", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "pointer to a list of user objects associated to this Tree",
    );
    he.insert("fUserInfo", h);
    let mut h = HashMap::new();
    h.insert("title", "pointer to list of friend elements");
    he.insert("fFriends", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of entries to estimate histogram limits");
    he.insert("fEstimate", h);
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    let mut h = HashMap::new();
    h.insert("title", "Tree weight (see TTree::SetWeight)");
    he.insert("fWeight", h);
    let mut h = HashMap::new();
    h.insert("title", "Fill area attributes");
    he.insert("TAttFill", h);
    let mut h = HashMap::new();
    h.insert("title", "Timer interval in milliseconds");
    he.insert("fTimerInterval", h);
    let mut h = HashMap::new();
    h.insert("title", "Line attributes");
    he.insert("TAttLine", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum total size of buffers kept in memory");
    he.insert("fMaxVirtualSize", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of autosaved bytes");
    he.insert("fSavedBytes", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Autosave tree when fAutoSave entries written or -fAutoSave (compressed) bytes produced",
    );
    he.insert("fAutoSave", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "IO features to define for newly-written baskets and branches.",
    );
    he.insert("fIOFeatures", h);
    let mut h = HashMap::new();
    h.insert("title", "List of Branches");
    he.insert("fBranches", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of entries");
    he.insert("fEntries", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of runs before prompting in Scan");
    he.insert("fScanField", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Number of Cluster range in addition to the one defined by 'AutoFlush'",
    );
    he.insert("fNClusterRange", h);
    let mut h = HashMap::new();
    h.insert("title", "Direct pointers to individual branch leaves");
    he.insert("fLeaves", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNClusterRange] Last entry of a cluster range.");
    he.insert("fClusterRangeEnd", h);
    let mut h = HashMap::new();
    h.insert("title", "Index of sorted values");
    he.insert("fIndex", h);
    let mut h = HashMap::new();
    h.insert("title", "Sorted index values");
    he.insert("fIndexValues", h);
    hh.insert("TTree", he);
    // TObjArray
    //si =  {'properties': {'f_name': 'TObjArray'}, 'elements': {'TSeqCollection': {'name': 'TSeqCollection', 'title': 'Sequenceable collection ABC'}, 'fLowerBound': {'name': 'fLowerBound', 'title': 'Lower bound of the array'}, 'fLast': {'title': 'Last element in array containing an object', 'name': 'fLast'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Sequenceable collection ABC");
    he.insert("TSeqCollection", h);
    let mut h = HashMap::new();
    h.insert("title", "Lower bound of the array");
    he.insert("fLowerBound", h);
    let mut h = HashMap::new();
    h.insert("title", "Last element in array containing an object");
    he.insert("fLast", h);
    hh.insert("TObjArray", he);
    // TAttCanvas
    //si =  {'properties': {'f_name': 'TAttCanvas'}, 'elements': {'fYdate': {'title': 'X position where to draw the date', 'name': 'fYdate'}, 'fAdate': {'name': 'fAdate', 'title': 'Alignment for the date'}, 'fTitleFromTop': {'name': 'fTitleFromTop', 'title': 'Y distance of Global Title from top'}, 'fXBetween': {'title': 'X distance between pads', 'name': 'fXBetween'}, 'fXdate': {'name': 'fXdate', 'title': 'X position where to draw the date'}, 'fYBetween': {'title': 'Y distance between pads', 'name': 'fYBetween'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "X position where to draw the date");
    he.insert("fYdate", h);
    let mut h = HashMap::new();
    h.insert("title", "Alignment for the date");
    he.insert("fAdate", h);
    let mut h = HashMap::new();
    h.insert("title", "Y distance of Global Title from top");
    he.insert("fTitleFromTop", h);
    let mut h = HashMap::new();
    h.insert("title", "X distance between pads");
    he.insert("fXBetween", h);
    let mut h = HashMap::new();
    h.insert("title", "X position where to draw the date");
    he.insert("fXdate", h);
    let mut h = HashMap::new();
    h.insert("title", "Y distance between pads");
    he.insert("fYBetween", h);
    hh.insert("TAttCanvas", he);
    // TH1D
    //si =  {'elements': {'TH1': {'title': '1-Dim histogram base class', 'name': 'TH1'}, 'TArrayD': {'name': 'TArrayD', 'title': 'Array of doubles'}}, 'properties': {'f_name': 'TH1D'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "1-Dim histogram base class");
    he.insert("TH1", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of doubles");
    he.insert("TArrayD", h);
    hh.insert("TH1D", he);
    // TKey
    //si =  {'elements': {'TNamed': {'title': 'The basis for a named object (name, title)', 'name': 'TNamed'}, 'fKeylen': {'name': 'fKeylen', 'title': 'Number of bytes for the key itself'}, 'fSeekPdir': {'title': 'Location of parent directory on file', 'name': 'fSeekPdir'}, 'fBufferRef': {'title': 'Pointer to the TBuffer object', 'name': 'fBufferRef'}, 'fCycle': {'title': 'Cycle number', 'name': 'fCycle'}, 'fLeft': {'name': 'fLeft', 'title': 'Number of bytes left in current segment'}, 'fObjlen': {'title': 'Length of uncompressed object in bytes', 'name': 'fObjlen'}, 'fClassName': {'title': 'Object Class name', 'name': 'fClassName'}, 'fBuffer': {'name': 'fBuffer', 'title': 'Object buffer'}, 'fVersion': {'title': 'KEY version identifier', 'name': 'fVersion'}, 'fDatime': {'name': 'fDatime', 'title': 'Date/Time of insertion in file'}, 'fNbytes': {'title': 'Number of bytes for the object on file', 'name': 'fNbytes'}, 'fSeekKey': {'title': 'Location of object on file', 'name': 'fSeekKey'}}, 'properties': {'f_name': 'TKey'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bytes for the key itself");
    he.insert("fKeylen", h);
    let mut h = HashMap::new();
    h.insert("title", "Location of parent directory on file");
    he.insert("fSeekPdir", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to the TBuffer object");
    he.insert("fBufferRef", h);
    let mut h = HashMap::new();
    h.insert("title", "Cycle number");
    he.insert("fCycle", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bytes left in current segment");
    he.insert("fLeft", h);
    let mut h = HashMap::new();
    h.insert("title", "Length of uncompressed object in bytes");
    he.insert("fObjlen", h);
    let mut h = HashMap::new();
    h.insert("title", "Object Class name");
    he.insert("fClassName", h);
    let mut h = HashMap::new();
    h.insert("title", "Object buffer");
    he.insert("fBuffer", h);
    let mut h = HashMap::new();
    h.insert("title", "KEY version identifier");
    he.insert("fVersion", h);
    let mut h = HashMap::new();
    h.insert("title", "Date/Time of insertion in file");
    he.insert("fDatime", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bytes for the object on file");
    he.insert("fNbytes", h);
    let mut h = HashMap::new();
    h.insert("title", "Location of object on file");
    he.insert("fSeekKey", h);
    hh.insert("TKey", he);
    // TUUID
    //si =  {'elements': {'fTimeLow': {'title': '60 bit time, lower 32 bits', 'name': 'fTimeLow'}, 'fNode': {'title': '6 node id bytes', 'name': 'fNode'}, 'fTimeMid': {'title': 'middle 16 time bits', 'name': 'fTimeMid'}, 'fTimeHiAndVersion': {'name': 'fTimeHiAndVersion', 'title': 'high 12 time bits + 4 UUID version bits'}, 'fClockSeqHiAndReserved': {'name': 'fClockSeqHiAndReserved', 'title': 'high 6 clock bits + 2 bits reserved'}, 'fClockSeqLow': {'name': 'fClockSeqLow', 'title': 'low 8 clock bits'}}, 'properties': {'f_name': 'TUUID'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "60 bit time, lower 32 bits");
    he.insert("fTimeLow", h);
    let mut h = HashMap::new();
    h.insert("title", "6 node id bytes");
    he.insert("fNode", h);
    let mut h = HashMap::new();
    h.insert("title", "middle 16 time bits");
    he.insert("fTimeMid", h);
    let mut h = HashMap::new();
    h.insert("title", "high 12 time bits + 4 UUID version bits");
    he.insert("fTimeHiAndVersion", h);
    let mut h = HashMap::new();
    h.insert("title", "high 6 clock bits + 2 bits reserved");
    he.insert("fClockSeqHiAndReserved", h);
    let mut h = HashMap::new();
    h.insert("title", "low 8 clock bits");
    he.insert("fClockSeqLow", h);
    hh.insert("TUUID", he);
    // TLeafObject
    //si =  {'properties': {'f_name': 'TLeafObject'}, 'elements': {'TLeaf': {'name': 'TLeaf', 'title': 'Leaf: description of a Branch data type'}, 'fVirtual': {'title': 'Support for polymorphism, when set classname is written with object.', 'name': 'fVirtual'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Support for polymorphism, when set classname is written with object.",
    );
    he.insert("fVirtual", h);
    hh.insert("TLeafObject", he);
    // TArrayD
    //si =  {'properties': {'f_name': 'TArrayD'}, 'elements': {'fArray': {'name': 'fArray', 'title': '[fN] Array of fN doubles'}, 'TArray': {'name': 'TArray', 'title': 'Abstract array base class'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "[fN] Array of fN doubles");
    he.insert("fArray", h);
    let mut h = HashMap::new();
    h.insert("title", "Abstract array base class");
    he.insert("TArray", h);
    hh.insert("TArrayD", he);
    // TLeafF16
    //si =  {'elements': {'TLeaf': {'name': 'TLeaf', 'title': 'Leaf: description of a Branch data type'}, 'fMinimum': {'name': 'fMinimum', 'title': 'Minimum value if leaf range is specified'}, 'fMaximum': {'title': 'Maximum value if leaf range is specified', 'name': 'fMaximum'}}, 'properties': {'f_name': 'TLeafF16'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value if leaf range is specified");
    he.insert("fMinimum", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum value if leaf range is specified");
    he.insert("fMaximum", h);
    hh.insert("TLeafF16", he);
    // TLeafS
    //si =  {'properties': {'f_name': 'TLeafS'}, 'elements': {'TLeaf': {'name': 'TLeaf', 'title': 'Leaf: description of a Branch data type'}, 'fMinimum': {'title': 'Minimum value if leaf range is specified', 'name': 'fMinimum'}, 'fMaximum': {'title': 'Maximum value if leaf range is specified', 'name': 'fMaximum'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value if leaf range is specified");
    he.insert("fMinimum", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum value if leaf range is specified");
    he.insert("fMaximum", h);
    hh.insert("TLeafS", he);
    // TProfile
    //si =  {'properties': {'f_name': 'TProfile'}, 'elements': {'fYmin': {'name': 'fYmin', 'title': 'Lower limit in Y (if set)'}, 'fTsumwy': {'name': 'fTsumwy', 'title': 'Total Sum of weight*Y'}, 'fBinEntries': {'title': 'number of entries per bin', 'name': 'fBinEntries'}, 'fYmax': {'name': 'fYmax', 'title': 'Upper limit in Y (if set)'}, 'fTsumwy2': {'title': 'Total Sum of weight*Y*Y', 'name': 'fTsumwy2'}, 'fErrorMode': {'name': 'fErrorMode', 'title': 'Option to compute errors'}, 'TH1D': {'name': 'TH1D', 'title': '1-Dim histograms (one double per channel)'}, 'fBinSumw2': {'title': 'Array of sum of squares of weights per bin', 'name': 'fBinSumw2'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Lower limit in Y (if set)");
    he.insert("fYmin", h);
    let mut h = HashMap::new();
    h.insert("title", "Total Sum of weight*Y");
    he.insert("fTsumwy", h);
    let mut h = HashMap::new();
    h.insert("title", "number of entries per bin");
    he.insert("fBinEntries", h);
    let mut h = HashMap::new();
    h.insert("title", "Upper limit in Y (if set)");
    he.insert("fYmax", h);
    let mut h = HashMap::new();
    h.insert("title", "Total Sum of weight*Y*Y");
    he.insert("fTsumwy2", h);
    let mut h = HashMap::new();
    h.insert("title", "Option to compute errors");
    he.insert("fErrorMode", h);
    let mut h = HashMap::new();
    h.insert("title", "1-Dim histograms (one double per channel)");
    he.insert("TH1D", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of sum of squares of weights per bin");
    he.insert("fBinSumw2", h);
    hh.insert("TProfile", he);
    // TArrayL
    //si =  {'properties': {'f_name': 'TArrayL'}, 'elements': {'TArray': {'title': 'Abstract array base class', 'name': 'TArray'}, 'fArray': {'name': 'fArray', 'title': '[fN] Array of fN longs'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Abstract array base class");
    he.insert("TArray", h);
    let mut h = HashMap::new();
    h.insert("title", "[fN] Array of fN longs");
    he.insert("fArray", h);
    hh.insert("TArrayL", he);
    // TF1
    //si =  {'properties': {'f_name': 'TF1'}, 'elements': {'fMaximum': {'name': 'fMaximum', 'title': 'Maximum value for plotting'}, 'fParams': {'name': 'fParams', 'title': 'Pointer to Function parameters object (exists only for not-formula functions)'}, 'TAttFill': {'name': 'TAttFill', 'title': 'Fill area attributes'}, 'TAttLine': {'name': 'TAttLine', 'title': 'Line attributes'}, 'fNpx': {'title': 'Number of points used for the graphical representation', 'name': 'fNpx'}, 'fNormIntegral': {'title': 'Integral of the function before being normalized', 'name': 'fNormIntegral'}, 'fComposition': {'title': 'Pointer to composition (NSUM or CONV)', 'name': 'fComposition'}, 'fType': {'title': '', 'name': 'fType'}, 'fParErrors': {'title': 'Array of errors of the fNpar parameters', 'name': 'fParErrors'}, 'fSave': {'title': 'Array of fNsave function values', 'name': 'fSave'}, 'fNDF': {'name': 'fNDF', 'title': 'Number of degrees of freedom in the fit'}, 'fXmin': {'name': 'fXmin', 'title': 'Lower bounds for the range'}, 'fNpar': {'name': 'fNpar', 'title': 'Number of parameters'}, 'fParMax': {'name': 'fParMax', 'title': 'Array of upper limits of the fNpar parameters'}, 'fNpfits': {'name': 'fNpfits', 'title': 'Number of points used in the fit'}, 'fNormalized': {'name': 'fNormalized', 'title': 'Normalization option (false by default)'}, 'fFormula': {'name': 'fFormula', 'title': 'Pointer to TFormula in case when user define formula'}, 'fChisquare': {'title': 'Function fit chisquare', 'name': 'fChisquare'}, 'fMinimum': {'name': 'fMinimum', 'title': 'Minimum value for plotting'}, 'fXmax': {'name': 'fXmax', 'title': 'Upper bounds for the range'}, 'fParMin': {'name': 'fParMin', 'title': 'Array of lower limits of the fNpar parameters'}, 'fNdim': {'name': 'fNdim', 'title': 'Function dimension'}, 'TAttMarker': {'name': 'TAttMarker', 'title': 'Marker attributes'}, 'TNamed': {'title': 'The basis for a named object (name, title)', 'name': 'TNamed'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Maximum value for plotting");
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Pointer to Function parameters object (exists only for not-formula functions)",
    );
    he.insert("fParams", h);
    let mut h = HashMap::new();
    h.insert("title", "Fill area attributes");
    he.insert("TAttFill", h);
    let mut h = HashMap::new();
    h.insert("title", "Line attributes");
    he.insert("TAttLine", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Number of points used for the graphical representation",
    );
    he.insert("fNpx", h);
    let mut h = HashMap::new();
    h.insert("title", "Integral of the function before being normalized");
    he.insert("fNormIntegral", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to composition (NSUM or CONV)");
    he.insert("fComposition", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fType", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of errors of the fNpar parameters");
    he.insert("fParErrors", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of fNsave function values");
    he.insert("fSave", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of degrees of freedom in the fit");
    he.insert("fNDF", h);
    let mut h = HashMap::new();
    h.insert("title", "Lower bounds for the range");
    he.insert("fXmin", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of parameters");
    he.insert("fNpar", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of upper limits of the fNpar parameters");
    he.insert("fParMax", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of points used in the fit");
    he.insert("fNpfits", h);
    let mut h = HashMap::new();
    h.insert("title", "Normalization option (false by default)");
    he.insert("fNormalized", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Pointer to TFormula in case when user define formula",
    );
    he.insert("fFormula", h);
    let mut h = HashMap::new();
    h.insert("title", "Function fit chisquare");
    he.insert("fChisquare", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value for plotting");
    he.insert("fMinimum", h);
    let mut h = HashMap::new();
    h.insert("title", "Upper bounds for the range");
    he.insert("fXmax", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of lower limits of the fNpar parameters");
    he.insert("fParMin", h);
    let mut h = HashMap::new();
    h.insert("title", "Function dimension");
    he.insert("fNdim", h);
    let mut h = HashMap::new();
    h.insert("title", "Marker attributes");
    he.insert("TAttMarker", h);
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    hh.insert("TF1", he);
    // TAttBBox2D
    //si =  {'properties': {'f_name': 'TAttBBox2D'}}
    let he = HashMap::new();
    hh.insert("TAttBBox2D", he);
    // TDirectory
    //si =  {'properties': {'f_name': 'TDirectory'}, 'elements': {'fMother': {'title': 'pointer to mother of the directory', 'name': 'fMother'}, 'TNamed': {'title': 'The basis for a named object (name, title)', 'name': 'TNamed'}, 'fUUID': {'title': 'Unique identifier', 'name': 'fUUID'}, 'fList': {'title': 'List of objects in memory', 'name': 'fList'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "pointer to mother of the directory");
    he.insert("fMother", h);
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    let mut h = HashMap::new();
    h.insert("title", "Unique identifier");
    he.insert("fUUID", h);
    let mut h = HashMap::new();
    h.insert("title", "List of objects in memory");
    he.insert("fList", h);
    hh.insert("TDirectory", he);
    // TStreamerInfo
    //si =  {'elements': {'TVirtualStreamerInfo': {'name': 'TVirtualStreamerInfo', 'title': 'Abstract Interface describing Streamer information for one class'}, 'fCheckSum': {'title': 'Checksum of original class', 'name': 'fCheckSum'}, 'fElements': {'title': 'Array of TStreamerElements', 'name': 'fElements'}, 'fClassVersion': {'name': 'fClassVersion', 'title': 'Class version identifier'}}, 'properties': {'f_name': 'TStreamerInfo'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Abstract Interface describing Streamer information for one class",
    );
    he.insert("TVirtualStreamerInfo", h);
    let mut h = HashMap::new();
    h.insert("title", "Checksum of original class");
    he.insert("fCheckSum", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of TStreamerElements");
    he.insert("fElements", h);
    let mut h = HashMap::new();
    h.insert("title", "Class version identifier");
    he.insert("fClassVersion", h);
    hh.insert("TStreamerInfo", he);
    // TH1F
    //si =  {'properties': {'f_name': 'TH1F'}, 'elements': {'TH1': {'name': 'TH1', 'title': '1-Dim histogram base class'}, 'TArrayF': {'title': 'Array of floats', 'name': 'TArrayF'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "1-Dim histogram base class");
    he.insert("TH1", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of floats");
    he.insert("TArrayF", h);
    hh.insert("TH1F", he);
    // TLeafC
    //si =  {'properties': {'f_name': 'TLeafC'}, 'elements': {'fMaximum': {'name': 'fMaximum', 'title': 'Maximum value if leaf range is specified'}, 'fMinimum': {'name': 'fMinimum', 'title': 'Minimum value if leaf range is specified'}, 'TLeaf': {'title': 'Leaf: description of a Branch data type', 'name': 'TLeaf'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Maximum value if leaf range is specified");
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value if leaf range is specified");
    he.insert("fMinimum", h);
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    hh.insert("TLeafC", he);
    // TStreamerBasicType
    //si =  {'properties': {'f_name': 'TStreamerBasicType'}, 'elements': {'TStreamerElement': {'title': 'Base class for one element (data member) to be Streamed', 'name': 'TStreamerElement'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Base class for one element (data member) to be Streamed",
    );
    he.insert("TStreamerElement", h);
    hh.insert("TStreamerBasicType", he);
    // TH2Poly
    //si =  {'properties': {'f_name': 'TH2Poly'}, 'elements': {'fCells': {'name': 'fCells', 'title': '[fNCells] The array of TLists that store the bins that intersect with each cell. List do not own the contained objects'}, 'fStepY': {'title': 'Dimensions of a partition cell', 'name': 'fStepY'}, 'fStepX': {'title': 'Dimensions of a partition cell', 'name': 'fStepX'}, 'TH2': {'name': 'TH2', 'title': '2-Dim histogram base class'}, 'fNCells': {'title': 'Number of partition cells: fCellX*fCellY', 'name': 'fNCells'}, 'fOverflow': {'name': 'fOverflow', 'title': 'Overflow bins'}, 'fIsEmpty': {'name': 'fIsEmpty', 'title': '[fNCells] The array that returns true if the cell at the given coordinate is empty'}, 'fCompletelyInside': {'name': 'fCompletelyInside', 'title': '[fNCells] The array that returns true if the cell at the given coordinate is completely inside a bin'}, 'fCellX': {'name': 'fCellX', 'title': 'Number of partition cells in the x-direction of the histogram'}, 'fFloat': {'name': 'fFloat', 'title': 'When set to kTRUE, allows the histogram to expand if a bin outside the limits is added.'}, 'fCellY': {'title': 'Number of partition cells in the y-direction of the histogram', 'name': 'fCellY'}, 'fBins': {'title': 'List of bins. The list owns the contained objects', 'name': 'fBins'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "[fNCells] The array of TLists that store the bins that intersect with each cell. List do not own the contained objects");
    he.insert("fCells", h);
    let mut h = HashMap::new();
    h.insert("title", "Dimensions of a partition cell");
    he.insert("fStepY", h);
    let mut h = HashMap::new();
    h.insert("title", "Dimensions of a partition cell");
    he.insert("fStepX", h);
    let mut h = HashMap::new();
    h.insert("title", "2-Dim histogram base class");
    he.insert("TH2", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of partition cells: fCellX*fCellY");
    he.insert("fNCells", h);
    let mut h = HashMap::new();
    h.insert("title", "Overflow bins");
    he.insert("fOverflow", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "[fNCells] The array that returns true if the cell at the given coordinate is empty",
    );
    he.insert("fIsEmpty", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNCells] The array that returns true if the cell at the given coordinate is completely inside a bin");
    he.insert("fCompletelyInside", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Number of partition cells in the x-direction of the histogram",
    );
    he.insert("fCellX", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "When set to kTRUE, allows the histogram to expand if a bin outside the limits is added.",
    );
    he.insert("fFloat", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Number of partition cells in the y-direction of the histogram",
    );
    he.insert("fCellY", h);
    let mut h = HashMap::new();
    h.insert("title", "List of bins. The list owns the contained objects");
    he.insert("fBins", h);
    hh.insert("TH2Poly", he);
    // TVector3
    //si =  {'elements': {'fZ': {'title': '', 'name': 'fZ'}, 'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}, 'fX': {'title': '', 'name': 'fX'}, 'fY': {'name': 'fY', 'title': ''}}, 'properties': {'f_name': 'TVector3'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fZ", h);
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fX", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fY", h);
    hh.insert("TVector3", he);
    // TList
    //si =  {'elements': {'TSeqCollection': {'title': 'Sequenceable collection ABC', 'name': 'TSeqCollection'}}, 'properties': {'f_name': 'TList'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Sequenceable collection ABC");
    he.insert("TSeqCollection", h);
    hh.insert("TList", he);
    // TLeafElement
    //si =  {'elements': {'fType': {'title': 'leaf type', 'name': 'fType'}, 'TLeaf': {'name': 'TLeaf', 'title': 'Leaf: description of a Branch data type'}, 'fID': {'name': 'fID', 'title': 'element serial number in fInfo'}}, 'properties': {'f_name': 'TLeafElement'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "leaf type");
    he.insert("fType", h);
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    let mut h = HashMap::new();
    h.insert("title", "element serial number in fInfo");
    he.insert("fID", h);
    hh.insert("TLeafElement", he);
    // TH2D
    //si =  {'properties': {'f_name': 'TH2D'}, 'elements': {'TArrayD': {'title': 'Array of doubles', 'name': 'TArrayD'}, 'TH2': {'name': 'TH2', 'title': '2-Dim histogram base class'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Array of doubles");
    he.insert("TArrayD", h);
    let mut h = HashMap::new();
    h.insert("title", "2-Dim histogram base class");
    he.insert("TH2", h);
    hh.insert("TH2D", he);
    // TF1Parameters
    //si =  {'properties': {'f_name': 'TF1Parameters'}, 'elements': {'fParameters': {'title': 'parameter values', 'name': 'fParameters'}, 'fParNames': {'name': 'fParNames', 'title': 'parameter names'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "parameter values");
    he.insert("fParameters", h);
    let mut h = HashMap::new();
    h.insert("title", "parameter names");
    he.insert("fParNames", h);
    hh.insert("TF1Parameters", he);
    // TBits
    //si =  {'elements': {'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}, 'fNbits': {'title': 'Highest bit set + 1', 'name': 'fNbits'}, 'fAllBits': {'name': 'fAllBits', 'title': '[fNbytes] array of UChars'}, 'fNbytes': {'name': 'fNbytes', 'title': 'Number of UChars in fAllBits'}}, 'properties': {'f_name': 'TBits'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    let mut h = HashMap::new();
    h.insert("title", "Highest bit set + 1");
    he.insert("fNbits", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNbytes] array of UChars");
    he.insert("fAllBits", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of UChars in fAllBits");
    he.insert("fNbytes", h);
    hh.insert("TBits", he);
    // TStreamerArtificial
    //si =  {'elements': {'TStreamerElement': {'title': 'Base class for one element (data member) to be Streamed', 'name': 'TStreamerElement'}}, 'properties': {'f_name': 'TStreamerArtificial'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Base class for one element (data member) to be Streamed",
    );
    he.insert("TStreamerElement", h);
    hh.insert("TStreamerArtificial", he);
    // TGraph
    //si =  {'properties': {'f_name': 'TGraph'}, 'elements': {'fX': {'name': 'fX', 'title': '[fNpoints] array of X points'}, 'fMaximum': {'name': 'fMaximum', 'title': 'Maximum value for plotting along y'}, 'TAttFill': {'title': 'Fill area attributes', 'name': 'TAttFill'}, 'fNpoints': {'title': 'Number of points <= fMaxSize', 'name': 'fNpoints'}, 'TAttLine': {'name': 'TAttLine', 'title': 'Line attributes'}, 'fY': {'name': 'fY', 'title': '[fNpoints] array of Y points'}, 'fHistogram': {'title': 'Pointer to histogram used for drawing axis', 'name': 'fHistogram'}, 'TAttMarker': {'title': 'Marker attributes', 'name': 'TAttMarker'}, 'fMinimum': {'name': 'fMinimum', 'title': 'Minimum value for plotting along y'}, 'TNamed': {'title': 'The basis for a named object (name, title)', 'name': 'TNamed'}, 'fFunctions': {'name': 'fFunctions', 'title': 'Pointer to list of functions (fits and user)'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "[fNpoints] array of X points");
    he.insert("fX", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum value for plotting along y");
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert("title", "Fill area attributes");
    he.insert("TAttFill", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of points <= fMaxSize");
    he.insert("fNpoints", h);
    let mut h = HashMap::new();
    h.insert("title", "Line attributes");
    he.insert("TAttLine", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNpoints] array of Y points");
    he.insert("fY", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to histogram used for drawing axis");
    he.insert("fHistogram", h);
    let mut h = HashMap::new();
    h.insert("title", "Marker attributes");
    he.insert("TAttMarker", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value for plotting along y");
    he.insert("fMinimum", h);
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to list of functions (fits and user)");
    he.insert("fFunctions", h);
    hh.insert("TGraph", he);
    // TArrayI
    //si =  {'properties': {'f_name': 'TArrayI'}, 'elements': {'fArray': {'name': 'fArray', 'title': '[fN] Array of fN 32 bit integers'}, 'TArray': {'title': 'Abstract array base class', 'name': 'TArray'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "[fN] Array of fN 32 bit integers");
    he.insert("fArray", h);
    let mut h = HashMap::new();
    h.insert("title", "Abstract array base class");
    he.insert("TArray", h);
    hh.insert("TArrayI", he);
    // TClonesArray
    //si =  {'properties': {'f_name': 'TClonesArray'}, 'elements': {'TObjArray': {'name': 'TObjArray', 'title': 'An array of objects'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "An array of objects");
    he.insert("TObjArray", h);
    hh.insert("TClonesArray", he);
    // TProcessID
    //si =  {'elements': {'TNamed': {'name': 'TNamed', 'title': 'The basis for a named object (name, title)'}}, 'properties': {'f_name': 'TProcessID'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    hh.insert("TProcessID", he);
    // TLeaf
    //si =  {'properties': {'f_name': 'TLeaf'}, 'elements': {'fIsRange': {'name': 'fIsRange', 'title': "(=kTRUE if leaf has a range, kFALSE otherwise).  This is equivalent to being a 'leafcount'.  For a TLeafElement the range information is actually store in the TBranchElement."}, 'fOffset': {'name': 'fOffset', 'title': 'Offset in ClonesArray object (if one)'}, 'fIsUnsigned': {'title': '(=kTRUE if unsigned, kFALSE otherwise)', 'name': 'fIsUnsigned'}, 'fLenType': {'title': 'Number of bytes for this data type', 'name': 'fLenType'}, 'fLeafCount': {'title': 'Pointer to Leaf count if variable length (we do not own the counter)', 'name': 'fLeafCount'}, 'TNamed': {'title': 'The basis for a named object (name, title)', 'name': 'TNamed'}, 'fLen': {'name': 'fLen', 'title': "Number of fixed length elements in the leaf's data."}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "(=kTRUE if leaf has a range, kFALSE otherwise).  This is equivalent to being a 'leafcount'.  For a TLeafElement the range information is actually store in the TBranchElement.");
    he.insert("fIsRange", h);
    let mut h = HashMap::new();
    h.insert("title", "Offset in ClonesArray object (if one)");
    he.insert("fOffset", h);
    let mut h = HashMap::new();
    h.insert("title", "(=kTRUE if unsigned, kFALSE otherwise)");
    he.insert("fIsUnsigned", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bytes for this data type");
    he.insert("fLenType", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Pointer to Leaf count if variable length (we do not own the counter)",
    );
    he.insert("fLeafCount", h);
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Number of fixed length elements in the leaf's data.",
    );
    he.insert("fLen", h);
    hh.insert("TLeaf", he);
    // TH1K
    //si =  {'properties': {'f_name': 'TH1K'}, 'elements': {'TH1': {'title': '1-Dim histogram base class', 'name': 'TH1'}, 'fNIn': {'name': 'fNIn', 'title': ''}, 'TArrayF': {'name': 'TArrayF', 'title': 'Array of floats'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "1-Dim histogram base class");
    he.insert("TH1", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fNIn", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of floats");
    he.insert("TArrayF", h);
    hh.insert("TH1K", he);
    // TFile
    //si =  {'elements': {'fBEGIN': {'name': 'fBEGIN', 'title': 'First used byte in file'}, 'fSumBuffer': {'name': 'fSumBuffer', 'title': 'Sum of buffer sizes of objects written so far'}, 'fNbytesInfo': {'title': 'Number of bytes for StreamerInfo record', 'name': 'fNbytesInfo'}, 'fReadCalls': {'title': 'Number of read calls ( not counting the cache calls )', 'name': 'fReadCalls'}, 'fCompress': {'name': 'fCompress', 'title': 'Compression level and algorithm'}, 'fRealName': {'title': 'Effective real file name (not original url)', 'name': 'fRealName'}, 'fOption': {'title': 'File options', 'name': 'fOption'}, 'fUnits': {'title': 'Number of bytes for file pointers', 'name': 'fUnits'}, 'fNbytesFree': {'title': 'Number of bytes for free segments structure', 'name': 'fNbytesFree'}, 'fBytesRead': {'name': 'fBytesRead', 'title': 'Number of bytes read from this file'}, 'fBytesReadExtra': {'title': 'Number of extra bytes (overhead) read by the readahead buffer', 'name': 'fBytesReadExtra'}, 'fSum2Buffer': {'name': 'fSum2Buffer', 'title': 'Sum of squares of buffer sizes of objects written so far'}, 'TDirectoryFile': {'title': 'Describe directory structure in a ROOT file', 'name': 'TDirectoryFile'}, 'fNProcessIDs': {'name': 'fNProcessIDs', 'title': 'Number of TProcessID written to this file'}, 'fBytesWrite': {'title': 'Number of bytes written to this file', 'name': 'fBytesWrite'}, 'fSeekFree': {'name': 'fSeekFree', 'title': 'Location on disk of free segments structure'}, 'fWritten': {'title': 'Number of objects written so far', 'name': 'fWritten'}, 'fFree': {'name': 'fFree', 'title': 'Free segments linked list table'}, 'fSeekInfo': {'name': 'fSeekInfo', 'title': 'Location on disk of StreamerInfo record'}, 'fVersion': {'title': 'File format version', 'name': 'fVersion'}, 'fD': {'name': 'fD', 'title': 'File descriptor'}, 'fEND': {'name': 'fEND', 'title': 'Last used byte in file'}}, 'properties': {'f_name': 'TFile'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "First used byte in file");
    he.insert("fBEGIN", h);
    let mut h = HashMap::new();
    h.insert("title", "Sum of buffer sizes of objects written so far");
    he.insert("fSumBuffer", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bytes for StreamerInfo record");
    he.insert("fNbytesInfo", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Number of read calls ( not counting the cache calls )",
    );
    he.insert("fReadCalls", h);
    let mut h = HashMap::new();
    h.insert("title", "Compression level and algorithm");
    he.insert("fCompress", h);
    let mut h = HashMap::new();
    h.insert("title", "Effective real file name (not original url)");
    he.insert("fRealName", h);
    let mut h = HashMap::new();
    h.insert("title", "File options");
    he.insert("fOption", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bytes for file pointers");
    he.insert("fUnits", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bytes for free segments structure");
    he.insert("fNbytesFree", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bytes read from this file");
    he.insert("fBytesRead", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Number of extra bytes (overhead) read by the readahead buffer",
    );
    he.insert("fBytesReadExtra", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Sum of squares of buffer sizes of objects written so far",
    );
    he.insert("fSum2Buffer", h);
    let mut h = HashMap::new();
    h.insert("title", "Describe directory structure in a ROOT file");
    he.insert("TDirectoryFile", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of TProcessID written to this file");
    he.insert("fNProcessIDs", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bytes written to this file");
    he.insert("fBytesWrite", h);
    let mut h = HashMap::new();
    h.insert("title", "Location on disk of free segments structure");
    he.insert("fSeekFree", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of objects written so far");
    he.insert("fWritten", h);
    let mut h = HashMap::new();
    h.insert("title", "Free segments linked list table");
    he.insert("fFree", h);
    let mut h = HashMap::new();
    h.insert("title", "Location on disk of StreamerInfo record");
    he.insert("fSeekInfo", h);
    let mut h = HashMap::new();
    h.insert("title", "File format version");
    he.insert("fVersion", h);
    let mut h = HashMap::new();
    h.insert("title", "File descriptor");
    he.insert("fD", h);
    let mut h = HashMap::new();
    h.insert("title", "Last used byte in file");
    he.insert("fEND", h);
    hh.insert("TFile", he);
    // TNtuple
    //si =  {'elements': {'fNvar': {'name': 'fNvar', 'title': 'Number of columns'}, 'TTree': {'name': 'TTree', 'title': 'Tree descriptor (the main ROOT I/O class)'}}, 'properties': {'f_name': 'TNtuple'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Number of columns");
    he.insert("fNvar", h);
    let mut h = HashMap::new();
    h.insert("title", "Tree descriptor (the main ROOT I/O class)");
    he.insert("TTree", h);
    hh.insert("TNtuple", he);
    // TF1AbsComposition
    //si =  {'properties': {'f_name': 'TF1AbsComposition'}, 'elements': {'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    hh.insert("TF1AbsComposition", he);
    // THashList
    //si =  {'elements': {'TList': {'title': 'Doubly linked list', 'name': 'TList'}}, 'properties': {'f_name': 'THashList'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Doubly linked list");
    he.insert("TList", h);
    hh.insert("THashList", he);
    // TBranch
    //si =  {'properties': {'f_name': 'TBranch'}, 'elements': {'TNamed': {'title': 'The basis for a named object (name, title)', 'name': 'TNamed'}, 'fCompress': {'title': 'Compression level and algorithm', 'name': 'fCompress'}, 'fFileName': {'name': 'fFileName', 'title': 'Name of file where buffers are stored ("" if in same file as Tree header)'}, 'fWriteBasket': {'name': 'fWriteBasket', 'title': 'Last basket number written'}, 'fOffset': {'title': 'Offset of this branch', 'name': 'fOffset'}, 'fSplitLevel': {'name': 'fSplitLevel', 'title': 'Branch split level'}, 'fBaskets': {'name': 'fBaskets', 'title': '-> List of baskets of this branch'}, 'fIOFeatures': {'title': 'IO features for newly-created baskets.', 'name': 'fIOFeatures'}, 'fTotBytes': {'title': 'Total number of bytes in all leaves before compression', 'name': 'fTotBytes'}, 'TAttFill': {'name': 'TAttFill', 'title': 'Fill area attributes'}, 'fEntryNumber': {'name': 'fEntryNumber', 'title': 'Current entry number (last one filled in this branch)'}, 'fLeaves': {'name': 'fLeaves', 'title': '-> List of leaves of this branch'}, 'fBasketBytes': {'name': 'fBasketBytes', 'title': '[fMaxBaskets] Length of baskets on file'}, 'fBasketSize': {'name': 'fBasketSize', 'title': 'Initial Size of  Basket Buffer'}, 'fMaxBaskets': {'name': 'fMaxBaskets', 'title': 'Maximum number of Baskets so far'}, 'fEntryOffsetLen': {'name': 'fEntryOffsetLen', 'title': 'Initial Length of fEntryOffset table in the basket buffers'}, 'fZipBytes': {'title': 'Total number of bytes in all leaves after compression', 'name': 'fZipBytes'}, 'fBranches': {'name': 'fBranches', 'title': '-> List of Branches of this branch'}, 'fEntries': {'title': 'Number of entries', 'name': 'fEntries'}, 'fBasketSeek': {'title': '[fMaxBaskets] Addresses of baskets on file', 'name': 'fBasketSeek'}, 'fBasketEntry': {'name': 'fBasketEntry', 'title': '[fMaxBaskets] Table of first entry in each basket'}, 'fFirstEntry': {'name': 'fFirstEntry', 'title': 'Number of the first entry in this branch'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    let mut h = HashMap::new();
    h.insert("title", "Compression level and algorithm");
    he.insert("fCompress", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Name of file where buffers are stored (\"\" if in same file as Tree header)",
    );
    he.insert("fFileName", h);
    let mut h = HashMap::new();
    h.insert("title", "Last basket number written");
    he.insert("fWriteBasket", h);
    let mut h = HashMap::new();
    h.insert("title", "Offset of this branch");
    he.insert("fOffset", h);
    let mut h = HashMap::new();
    h.insert("title", "Branch split level");
    he.insert("fSplitLevel", h);
    let mut h = HashMap::new();
    h.insert("title", "-> List of baskets of this branch");
    he.insert("fBaskets", h);
    let mut h = HashMap::new();
    h.insert("title", "IO features for newly-created baskets.");
    he.insert("fIOFeatures", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Total number of bytes in all leaves before compression",
    );
    he.insert("fTotBytes", h);
    let mut h = HashMap::new();
    h.insert("title", "Fill area attributes");
    he.insert("TAttFill", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Current entry number (last one filled in this branch)",
    );
    he.insert("fEntryNumber", h);
    let mut h = HashMap::new();
    h.insert("title", "-> List of leaves of this branch");
    he.insert("fLeaves", h);
    let mut h = HashMap::new();
    h.insert("title", "[fMaxBaskets] Length of baskets on file");
    he.insert("fBasketBytes", h);
    let mut h = HashMap::new();
    h.insert("title", "Initial Size of  Basket Buffer");
    he.insert("fBasketSize", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum number of Baskets so far");
    he.insert("fMaxBaskets", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Initial Length of fEntryOffset table in the basket buffers",
    );
    he.insert("fEntryOffsetLen", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Total number of bytes in all leaves after compression",
    );
    he.insert("fZipBytes", h);
    let mut h = HashMap::new();
    h.insert("title", "-> List of Branches of this branch");
    he.insert("fBranches", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of entries");
    he.insert("fEntries", h);
    let mut h = HashMap::new();
    h.insert("title", "[fMaxBaskets] Addresses of baskets on file");
    he.insert("fBasketSeek", h);
    let mut h = HashMap::new();
    h.insert("title", "[fMaxBaskets] Table of first entry in each basket");
    he.insert("fBasketEntry", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of the first entry in this branch");
    he.insert("fFirstEntry", h);
    hh.insert("TBranch", he);
    // THashTable
    //si =  {'properties': {'f_name': 'THashTable'}, 'elements': {'TCollection': {'title': 'Collection abstract base class', 'name': 'TCollection'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Collection abstract base class");
    he.insert("TCollection", h);
    hh.insert("THashTable", he);
    // TDatime
    //si =  {'properties': {'f_name': 'TDatime'}, 'elements': {'fDatime': {'name': 'fDatime', 'title': 'Date (relative to 1995) + time'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Date (relative to 1995) + time");
    he.insert("fDatime", h);
    hh.insert("TDatime", he);
    // TArrayL64
    //si =  {'elements': {'TArray': {'title': 'Abstract array base class', 'name': 'TArray'}, 'fArray': {'title': '[fN] Array of fN long64s', 'name': 'fArray'}}, 'properties': {'f_name': 'TArrayL64'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Abstract array base class");
    he.insert("TArray", h);
    let mut h = HashMap::new();
    h.insert("title", "[fN] Array of fN long64s");
    he.insert("fArray", h);
    hh.insert("TArrayL64", he);
    // TStreamerLoop
    //si =  {'elements': {'fCountClass': {'title': 'name of the class with the counter', 'name': 'fCountClass'}, 'fCountName': {'name': 'fCountName', 'title': 'name of data member holding the array count'}, 'TStreamerElement': {'title': 'Base class for one element (data member) to be Streamed', 'name': 'TStreamerElement'}, 'fCountVersion': {'title': 'version number of the class with the counter', 'name': 'fCountVersion'}}, 'properties': {'f_name': 'TStreamerLoop'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "name of the class with the counter");
    he.insert("fCountClass", h);
    let mut h = HashMap::new();
    h.insert("title", "name of data member holding the array count");
    he.insert("fCountName", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Base class for one element (data member) to be Streamed",
    );
    he.insert("TStreamerElement", h);
    let mut h = HashMap::new();
    h.insert("title", "version number of the class with the counter");
    he.insert("fCountVersion", h);
    hh.insert("TStreamerLoop", he);
    // TH1S
    //si =  {'properties': {'f_name': 'TH1S'}, 'elements': {'TH1': {'name': 'TH1', 'title': '1-Dim histogram base class'}, 'TArrayS': {'name': 'TArrayS', 'title': 'Array of shorts'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "1-Dim histogram base class");
    he.insert("TH1", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of shorts");
    he.insert("TArrayS", h);
    hh.insert("TH1S", he);
    // TCanvas
    //si =  {'properties': {'f_name': 'TCanvas'}, 'elements': {'fDISPLAY': {'name': 'fDISPLAY', 'title': 'Name of destination screen'}, 'fXsizeReal': {'name': 'fXsizeReal', 'title': 'Current size of canvas along X in CM'}, 'fYsizeReal': {'name': 'fYsizeReal', 'title': 'Current size of canvas along Y in CM'}, 'fWindowHeight': {'title': 'Height of window (including menubar, borders, etc.)', 'name': 'fWindowHeight'}, 'fCh': {'title': 'Height of the canvas along Y (pixels)', 'name': 'fCh'}, 'fWindowTopX': {'title': 'Top X position of window (in pixels)', 'name': 'fWindowTopX'}, 'fCatt': {'name': 'fCatt', 'title': 'Canvas attributes'}, 'fDoubleBuffer': {'name': 'fDoubleBuffer', 'title': 'Double buffer flag (0=off, 1=on)'}, 'fWindowTopY': {'title': 'Top Y position of window (in pixels)', 'name': 'fWindowTopY'}, 'fWindowWidth': {'title': 'Width of window (including borders, etc.)', 'name': 'fWindowWidth'}, 'fRetained': {'title': 'Retain structure flag', 'name': 'fRetained'}, 'TPad': {'name': 'TPad', 'title': 'A Graphics pad'}, 'fHighLightColor': {'title': 'Highlight color of active pad', 'name': 'fHighLightColor'}, 'fCw': {'name': 'fCw', 'title': 'Width of the canvas along X (pixels)'}, 'fXsizeUser': {'name': 'fXsizeUser', 'title': 'User specified size of canvas along X in CM'}, 'fYsizeUser': {'title': 'User specified size of canvas along Y in CM', 'name': 'fYsizeUser'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Name of destination screen");
    he.insert("fDISPLAY", h);
    let mut h = HashMap::new();
    h.insert("title", "Current size of canvas along X in CM");
    he.insert("fXsizeReal", h);
    let mut h = HashMap::new();
    h.insert("title", "Current size of canvas along Y in CM");
    he.insert("fYsizeReal", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Height of window (including menubar, borders, etc.)",
    );
    he.insert("fWindowHeight", h);
    let mut h = HashMap::new();
    h.insert("title", "Height of the canvas along Y (pixels)");
    he.insert("fCh", h);
    let mut h = HashMap::new();
    h.insert("title", "Top X position of window (in pixels)");
    he.insert("fWindowTopX", h);
    let mut h = HashMap::new();
    h.insert("title", "Canvas attributes");
    he.insert("fCatt", h);
    let mut h = HashMap::new();
    h.insert("title", "Double buffer flag (0=off, 1=on)");
    he.insert("fDoubleBuffer", h);
    let mut h = HashMap::new();
    h.insert("title", "Top Y position of window (in pixels)");
    he.insert("fWindowTopY", h);
    let mut h = HashMap::new();
    h.insert("title", "Width of window (including borders, etc.)");
    he.insert("fWindowWidth", h);
    let mut h = HashMap::new();
    h.insert("title", "Retain structure flag");
    he.insert("fRetained", h);
    let mut h = HashMap::new();
    h.insert("title", "A Graphics pad");
    he.insert("TPad", h);
    let mut h = HashMap::new();
    h.insert("title", "Highlight color of active pad");
    he.insert("fHighLightColor", h);
    let mut h = HashMap::new();
    h.insert("title", "Width of the canvas along X (pixels)");
    he.insert("fCw", h);
    let mut h = HashMap::new();
    h.insert("title", "User specified size of canvas along X in CM");
    he.insert("fXsizeUser", h);
    let mut h = HashMap::new();
    h.insert("title", "User specified size of canvas along Y in CM");
    he.insert("fYsizeUser", h);
    hh.insert("TCanvas", he);
    // TCollection
    //si =  {'elements': {'f_name': {'title': 'name of the collection', 'name': 'f_name'}, 'fSize': {'title': 'number of elements in collection', 'name': 'fSize'}, 'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}}, 'properties': {'f_name': 'TCollection'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "name of the collection");
    he.insert("f_name", h);
    let mut h = HashMap::new();
    h.insert("title", "number of elements in collection");
    he.insert("fSize", h);
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    hh.insert("TCollection", he);
    // TH2C
    //si =  {'elements': {'TArrayC': {'title': 'Array of chars', 'name': 'TArrayC'}, 'TH2': {'title': '2-Dim histogram base class', 'name': 'TH2'}}, 'properties': {'f_name': 'TH2C'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Array of chars");
    he.insert("TArrayC", h);
    let mut h = HashMap::new();
    h.insert("title", "2-Dim histogram base class");
    he.insert("TH2", h);
    hh.insert("TH2C", he);
    // TBranchObject
    //si =  {'properties': {'f_name': 'TBranchObject'}, 'elements': {'TBranch': {'name': 'TBranch', 'title': 'Branch descriptor'}, 'fClassName': {'title': 'Class name of referenced object', 'name': 'fClassName'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Branch descriptor");
    he.insert("TBranch", h);
    let mut h = HashMap::new();
    h.insert("title", "Class name of referenced object");
    he.insert("fClassName", h);
    hh.insert("TBranchObject", he);
    // TAttMarker
    //si =  {'elements': {'fMarkerStyle': {'name': 'fMarkerStyle', 'title': 'Marker style'}, 'fMarkerSize': {'name': 'fMarkerSize', 'title': 'Marker size'}, 'fMarkerColor': {'name': 'fMarkerColor', 'title': 'Marker color'}}, 'properties': {'f_name': 'TAttMarker'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Marker style");
    he.insert("fMarkerStyle", h);
    let mut h = HashMap::new();
    h.insert("title", "Marker size");
    he.insert("fMarkerSize", h);
    let mut h = HashMap::new();
    h.insert("title", "Marker color");
    he.insert("fMarkerColor", h);
    hh.insert("TAttMarker", he);
    // TLeafI
    //si =  {'elements': {'TLeaf': {'name': 'TLeaf', 'title': 'Leaf: description of a Branch data type'}, 'fMaximum': {'title': 'Maximum value if leaf range is specified', 'name': 'fMaximum'}, 'fMinimum': {'name': 'fMinimum', 'title': 'Minimum value if leaf range is specified'}}, 'properties': {'f_name': 'TLeafI'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum value if leaf range is specified");
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value if leaf range is specified");
    he.insert("fMinimum", h);
    hh.insert("TLeafI", he);
    // TH2S
    //si =  {'properties': {'f_name': 'TH2S'}, 'elements': {'TH2': {'title': '2-Dim histogram base class', 'name': 'TH2'}, 'TArrayS': {'name': 'TArrayS', 'title': 'Array of shorts'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "2-Dim histogram base class");
    he.insert("TH2", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of shorts");
    he.insert("TArrayS", h);
    hh.insert("TH2S", he);
    // TF1NormSum
    //si =  {'properties': {'f_name': 'TF1NormSum'}, 'elements': {'fCoeffs': {'name': 'fCoeffs', 'title': 'Vector of size afNOfFunctions containing coefficients in front of each function'}, 'fXmin': {'name': 'fXmin', 'title': 'Minimal bound of range of NormSum'}, 'fNOfFunctions': {'title': 'Number of functions to add', 'name': 'fNOfFunctions'}, 'fParNames': {'title': 'Parameter names', 'name': 'fParNames'}, 'fCstIndexes': {'name': 'fCstIndexes', 'title': 'Vector with size of fNOfFunctions containing the index of the constant parameter/ function (the removed ones)'}, 'fScale': {'title': 'Fixed Scale parameter to normalize function (e.g. bin width)', 'name': 'fScale'}, 'fXmax': {'title': 'Maximal bound of range of NormSum', 'name': 'fXmax'}, 'TF1AbsComposition': {'name': 'TF1AbsComposition', 'title': ''}, 'fFunctions': {'title': 'Vector of size fNOfFunctions containing TF1 functions', 'name': 'fFunctions'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Vector of size afNOfFunctions containing coefficients in front of each function",
    );
    he.insert("fCoeffs", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimal bound of range of NormSum");
    he.insert("fXmin", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of functions to add");
    he.insert("fNOfFunctions", h);
    let mut h = HashMap::new();
    h.insert("title", "Parameter names");
    he.insert("fParNames", h);
    let mut h = HashMap::new();
    h.insert("title", "Vector with size of fNOfFunctions containing the index of the constant parameter/ function (the removed ones)");
    he.insert("fCstIndexes", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Fixed Scale parameter to normalize function (e.g. bin width)",
    );
    he.insert("fScale", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximal bound of range of NormSum");
    he.insert("fXmax", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("TF1AbsComposition", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Vector of size fNOfFunctions containing TF1 functions",
    );
    he.insert("fFunctions", h);
    hh.insert("TF1NormSum", he);
    // TProcessUUID
    //si =  {'elements': {'TProcessID': {'name': 'TProcessID', 'title': 'Process Unique Identifier in time and space'}, 'fActive': {'title': 'Table of active UUIDs', 'name': 'fActive'}, 'fUUIDs': {'title': 'Global list of TUUIDs', 'name': 'fUUIDs'}}, 'properties': {'f_name': 'TProcessUUID'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Process Unique Identifier in time and space");
    he.insert("TProcessID", h);
    let mut h = HashMap::new();
    h.insert("title", "Table of active UUIDs");
    he.insert("fActive", h);
    let mut h = HashMap::new();
    h.insert("title", "Global list of TUUIDs");
    he.insert("fUUIDs", h);
    hh.insert("TProcessUUID", he);
    // TStreamerObjectAny
    //si =  {'properties': {'f_name': 'TStreamerObjectAny'}, 'elements': {'TStreamerElement': {'name': 'TStreamerElement', 'title': 'Base class for one element (data member) to be Streamed'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Base class for one element (data member) to be Streamed",
    );
    he.insert("TStreamerElement", h);
    hh.insert("TStreamerObjectAny", he);
    // TStreamerSTL
    //si =  {'elements': {'fSTLtype': {'name': 'fSTLtype', 'title': 'type of STL vector'}, 'fCtype': {'title': 'STL contained type', 'name': 'fCtype'}, 'TStreamerElement': {'title': 'Base class for one element (data member) to be Streamed', 'name': 'TStreamerElement'}}, 'properties': {'f_name': 'TStreamerSTL'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "type of STL vector");
    he.insert("fSTLtype", h);
    let mut h = HashMap::new();
    h.insert("title", "STL contained type");
    he.insert("fCtype", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Base class for one element (data member) to be Streamed",
    );
    he.insert("TStreamerElement", h);
    hh.insert("TStreamerSTL", he);
    // TStreamerObjectPointer
    //si =  {'properties': {'f_name': 'TStreamerObjectPointer'}, 'elements': {'TStreamerElement': {'name': 'TStreamerElement', 'title': 'Base class for one element (data member) to be Streamed'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Base class for one element (data member) to be Streamed",
    );
    he.insert("TStreamerElement", h);
    hh.insert("TStreamerObjectPointer", he);
    // TStreamerBase
    //si =  {'elements': {'TStreamerElement': {'title': 'Base class for one element (data member) to be Streamed', 'name': 'TStreamerElement'}, 'fBaseVersion': {'name': 'fBaseVersion', 'title': 'version number of the base class (used during memberwise streaming)'}}, 'properties': {'f_name': 'TStreamerBase'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Base class for one element (data member) to be Streamed",
    );
    he.insert("TStreamerElement", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "version number of the base class (used during memberwise streaming)",
    );
    he.insert("fBaseVersion", h);
    hh.insert("TStreamerBase", he);
    // TStreamerString
    //si =  {'properties': {'f_name': 'TStreamerString'}, 'elements': {'TStreamerElement': {'name': 'TStreamerElement', 'title': 'Base class for one element (data member) to be Streamed'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Base class for one element (data member) to be Streamed",
    );
    he.insert("TStreamerElement", h);
    hh.insert("TStreamerString", he);
    // TH1I
    //si =  {'elements': {'TArrayI': {'title': 'Array of ints', 'name': 'TArrayI'}, 'TH1': {'title': '1-Dim histogram base class', 'name': 'TH1'}}, 'properties': {'f_name': 'TH1I'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Array of ints");
    he.insert("TArrayI", h);
    let mut h = HashMap::new();
    h.insert("title", "1-Dim histogram base class");
    he.insert("TH1", h);
    hh.insert("TH1I", he);
    // TStreamerSTLstring
    //si =  {'properties': {'f_name': 'TStreamerSTLstring'}, 'elements': {'TStreamerSTL': {'title': 'Streamer element of type STL container', 'name': 'TStreamerSTL'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Streamer element of type STL container");
    he.insert("TStreamerSTL", h);
    hh.insert("TStreamerSTLstring", he);
    // TLeafO
    //si =  {'elements': {'fMaximum': {'name': 'fMaximum', 'title': 'Maximum value if leaf range is specified'}, 'fMinimum': {'title': 'Minimum value if leaf range is specified', 'name': 'fMinimum'}, 'TLeaf': {'title': 'Leaf: description of a Branch data type', 'name': 'TLeaf'}}, 'properties': {'f_name': 'TLeafO'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Maximum value if leaf range is specified");
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value if leaf range is specified");
    he.insert("fMinimum", h);
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    hh.insert("TLeafO", he);
    // TObject
    //si =  {'properties': {'f_name': 'TObject'}, 'elements': {'fUniqueID': {'name': 'fUniqueID', 'title': 'object unique identifier'}, 'fBits': {'name': 'fBits', 'title': 'bit field status word'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "object unique identifier");
    he.insert("fUniqueID", h);
    let mut h = HashMap::new();
    h.insert("title", "bit field status word");
    he.insert("fBits", h);
    hh.insert("TObject", he);
    // TQObject
    //si =  {'properties': {'f_name': 'TQObject'}}
    let he = HashMap::new();
    hh.insert("TQObject", he);
    // TBranchElement
    //si =  {'properties': {'f_name': 'TBranchElement'}, 'elements': {'fCheckSum': {'title': 'CheckSum of class', 'name': 'fCheckSum'}, 'fClassVersion': {'title': 'Version number of class', 'name': 'fClassVersion'}, 'fMaximum': {'name': 'fMaximum', 'title': 'Maximum entries for a TClonesArray or variable array'}, 'TBranch': {'title': 'Branch descriptor', 'name': 'TBranch'}, 'fType': {'name': 'fType', 'title': 'Branch type'}, 'fBranchCount': {'name': 'fBranchCount', 'title': 'pointer to primary branchcount branch'}, 'fStreamerType': {'name': 'fStreamerType', 'title': 'branch streamer type'}, 'fBranchCount2': {'name': 'fBranchCount2', 'title': 'pointer to secondary branchcount branch'}, 'fParentName': {'name': 'fParentName', 'title': 'Name of parent class'}, 'fClassName': {'name': 'fClassName', 'title': 'Class name of referenced object'}, 'fClonesName': {'title': 'Name of class in TClonesArray (if any)', 'name': 'fClonesName'}, 'fID': {'name': 'fID', 'title': 'element serial number in fInfo'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "CheckSum of class");
    he.insert("fCheckSum", h);
    let mut h = HashMap::new();
    h.insert("title", "Version number of class");
    he.insert("fClassVersion", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Maximum entries for a TClonesArray or variable array",
    );
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert("title", "Branch descriptor");
    he.insert("TBranch", h);
    let mut h = HashMap::new();
    h.insert("title", "Branch type");
    he.insert("fType", h);
    let mut h = HashMap::new();
    h.insert("title", "pointer to primary branchcount branch");
    he.insert("fBranchCount", h);
    let mut h = HashMap::new();
    h.insert("title", "branch streamer type");
    he.insert("fStreamerType", h);
    let mut h = HashMap::new();
    h.insert("title", "pointer to secondary branchcount branch");
    he.insert("fBranchCount2", h);
    let mut h = HashMap::new();
    h.insert("title", "Name of parent class");
    he.insert("fParentName", h);
    let mut h = HashMap::new();
    h.insert("title", "Class name of referenced object");
    he.insert("fClassName", h);
    let mut h = HashMap::new();
    h.insert("title", "Name of class in TClonesArray (if any)");
    he.insert("fClonesName", h);
    let mut h = HashMap::new();
    h.insert("title", "element serial number in fInfo");
    he.insert("fID", h);
    hh.insert("TBranchElement", he);
    // TH2
    //si =  {'properties': {'f_name': 'TH2'}, 'elements': {'fTsumwy2': {'name': 'fTsumwy2', 'title': 'Total Sum of weight*Y*Y'}, 'fScalefactor': {'title': 'Scale factor', 'name': 'fScalefactor'}, 'fTsumwy': {'title': 'Total Sum of weight*Y', 'name': 'fTsumwy'}, 'TH1': {'name': 'TH1', 'title': '1-Dim histogram base class'}, 'fTsumwxy': {'name': 'fTsumwxy', 'title': 'Total Sum of weight*X*Y'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Total Sum of weight*Y*Y");
    he.insert("fTsumwy2", h);
    let mut h = HashMap::new();
    h.insert("title", "Scale factor");
    he.insert("fScalefactor", h);
    let mut h = HashMap::new();
    h.insert("title", "Total Sum of weight*Y");
    he.insert("fTsumwy", h);
    let mut h = HashMap::new();
    h.insert("title", "1-Dim histogram base class");
    he.insert("TH1", h);
    let mut h = HashMap::new();
    h.insert("title", "Total Sum of weight*X*Y");
    he.insert("fTsumwxy", h);
    hh.insert("TH2", he);
    // TH2F
    //si =  {'elements': {'TArrayF': {'title': 'Array of floats', 'name': 'TArrayF'}, 'TH2': {'name': 'TH2', 'title': '2-Dim histogram base class'}}, 'properties': {'f_name': 'TH2F'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Array of floats");
    he.insert("TArrayF", h);
    let mut h = HashMap::new();
    h.insert("title", "2-Dim histogram base class");
    he.insert("TH2", h);
    hh.insert("TH2F", he);
    // TBasket
    //si =  {'elements': {'fNevBufSize': {'title': 'Length in Int_t of fEntryOffset OR fixed length of each entry if fEntryOffset is null!', 'name': 'fNevBufSize'}, 'fNevBuf': {'name': 'fNevBuf', 'title': 'Number of entries in basket'}, 'fEntryOffset': {'title': '[fNevBuf] Offset of entries in fBuffer(TKey); generated at runtime.  Special value', 'name': 'fEntryOffset'}, 'fBranch': {'name': 'fBranch', 'title': 'Pointer to the basket support branch'}, 'fBufferSize': {'name': 'fBufferSize', 'title': 'fBuffer length in bytes'}, 'TKey': {'title': 'Header description of a logical record on file.', 'name': 'TKey'}, 'fLast': {'name': 'fLast', 'title': 'Pointer to last used byte in basket'}, 'fHeaderOnly': {'name': 'fHeaderOnly', 'title': 'True when only the basket header must be read/written'}}, 'properties': {'f_name': 'TBasket'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Length in Int_t of fEntryOffset OR fixed length of each entry if fEntryOffset is null!",
    );
    he.insert("fNevBufSize", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of entries in basket");
    he.insert("fNevBuf", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "[fNevBuf] Offset of entries in fBuffer(TKey); generated at runtime.  Special value",
    );
    he.insert("fEntryOffset", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to the basket support branch");
    he.insert("fBranch", h);
    let mut h = HashMap::new();
    h.insert("title", "fBuffer length in bytes");
    he.insert("fBufferSize", h);
    let mut h = HashMap::new();
    h.insert("title", "Header description of a logical record on file.");
    he.insert("TKey", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to last used byte in basket");
    he.insert("fLast", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "True when only the basket header must be read/written",
    );
    he.insert("fHeaderOnly", h);
    hh.insert("TBasket", he);
    // TStreamerBasicPointer
    //si =  {'properties': {'f_name': 'TStreamerBasicPointer'}, 'elements': {'TStreamerElement': {'name': 'TStreamerElement', 'title': 'Base class for one element (data member) to be Streamed'}, 'fCountName': {'title': 'name of data member holding the array count', 'name': 'fCountName'}, 'fCountVersion': {'name': 'fCountVersion', 'title': 'version number of the class with the counter'}, 'fCountClass': {'name': 'fCountClass', 'title': 'name of the class with the counter'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Base class for one element (data member) to be Streamed",
    );
    he.insert("TStreamerElement", h);
    let mut h = HashMap::new();
    h.insert("title", "name of data member holding the array count");
    he.insert("fCountName", h);
    let mut h = HashMap::new();
    h.insert("title", "version number of the class with the counter");
    he.insert("fCountVersion", h);
    let mut h = HashMap::new();
    h.insert("title", "name of the class with the counter");
    he.insert("fCountClass", h);
    hh.insert("TStreamerBasicPointer", he);
    // TProfile2D
    //si =  {'properties': {'f_name': 'TProfile2D'}, 'elements': {'fBinEntries': {'name': 'fBinEntries', 'title': 'Number of entries per bin'}, 'fBinSumw2': {'title': 'Array of sum of squares of weights per bin', 'name': 'fBinSumw2'}, 'fErrorMode': {'title': 'Option to compute errors', 'name': 'fErrorMode'}, 'TH2D': {'name': 'TH2D', 'title': '2-Dim histograms (one double per channel)'}, 'fZmax': {'title': 'Upper limit in Z (if set)', 'name': 'fZmax'}, 'fTsumwz': {'name': 'fTsumwz', 'title': 'Total Sum of weight*Z'}, 'fZmin': {'title': 'Lower limit in Z (if set)', 'name': 'fZmin'}, 'fTsumwz2': {'name': 'fTsumwz2', 'title': 'Total Sum of weight*Z*Z'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Number of entries per bin");
    he.insert("fBinEntries", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of sum of squares of weights per bin");
    he.insert("fBinSumw2", h);
    let mut h = HashMap::new();
    h.insert("title", "Option to compute errors");
    he.insert("fErrorMode", h);
    let mut h = HashMap::new();
    h.insert("title", "2-Dim histograms (one double per channel)");
    he.insert("TH2D", h);
    let mut h = HashMap::new();
    h.insert("title", "Upper limit in Z (if set)");
    he.insert("fZmax", h);
    let mut h = HashMap::new();
    h.insert("title", "Total Sum of weight*Z");
    he.insert("fTsumwz", h);
    let mut h = HashMap::new();
    h.insert("title", "Lower limit in Z (if set)");
    he.insert("fZmin", h);
    let mut h = HashMap::new();
    h.insert("title", "Total Sum of weight*Z*Z");
    he.insert("fTsumwz2", h);
    hh.insert("TProfile2D", he);
    // TConfidenceLevel
    //si =  {'properties': {'f_name': 'TConfidenceLevel'}, 'elements': {'fMCL5S': {'name': 'fMCL5S', 'title': ''}, 'TObject': {'title': 'Basic ROOT object', 'name': 'TObject'}, 'fNNMC': {'name': 'fNNMC', 'title': ''}, 'fStot': {'title': '', 'name': 'fStot'}, 'fTSD': {'name': 'fTSD', 'title': ''}, 'fTSB': {'title': '[fNNMC]', 'name': 'fTSB'}, 'fDtot': {'name': 'fDtot', 'title': ''}, 'fMCL3S': {'title': '', 'name': 'fMCL3S'}, 'fISB': {'title': '[fNNMC]', 'name': 'fISB'}, 'fBtot': {'name': 'fBtot', 'title': ''}, 'fTSS': {'name': 'fTSS', 'title': '[fNNMC]'}, 'fNMC': {'title': '', 'name': 'fNMC'}, 'fISS': {'name': 'fISS', 'title': '[fNNMC]'}, 'fLRB': {'title': '[fNNMC]', 'name': 'fLRB'}, 'fLRS': {'name': 'fLRS', 'title': '[fNNMC]'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fMCL5S", h);
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fNNMC", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fStot", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fTSD", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNNMC]");
    he.insert("fTSB", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fDtot", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fMCL3S", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNNMC]");
    he.insert("fISB", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fBtot", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNNMC]");
    he.insert("fTSS", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fNMC", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNNMC]");
    he.insert("fISS", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNNMC]");
    he.insert("fLRB", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNNMC]");
    he.insert("fLRS", h);
    hh.insert("TConfidenceLevel", he);
    // TObjString
    //si =  {'properties': {'f_name': 'TObjString'}, 'elements': {'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}, 'fString': {'name': 'fString', 'title': 'wrapped TString'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    let mut h = HashMap::new();
    h.insert("title", "wrapped TString");
    he.insert("fString", h);
    hh.insert("TObjString", he);
    // TNtupleD
    //si =  {'elements': {'fNvar': {'name': 'fNvar', 'title': 'Number of columns'}, 'TTree': {'name': 'TTree', 'title': 'Tree descriptor (the main ROOT I/O class)'}}, 'properties': {'f_name': 'TNtupleD'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Number of columns");
    he.insert("fNvar", h);
    let mut h = HashMap::new();
    h.insert("title", "Tree descriptor (the main ROOT I/O class)");
    he.insert("TTree", h);
    hh.insert("TNtupleD", he);
    // TChain
    //si =  {'elements': {'TTree': {'name': 'TTree', 'title': 'Tree descriptor (the main ROOT I/O class)'}, 'fTreeOffset': {'title': '[fTreeOffsetLen] Array of variables', 'name': 'fTreeOffset'}, 'fTreeOffsetLen': {'title': 'Current size of fTreeOffset array', 'name': 'fTreeOffsetLen'}, 'fNtrees': {'title': 'Number of trees', 'name': 'fNtrees'}, 'fFiles': {'name': 'fFiles', 'title': '-> List of file names containing the trees (TChainElement, owned)'}, 'fStatus': {'name': 'fStatus', 'title': '-> List of active/inactive branches (TChainElement, owned)'}}, 'properties': {'f_name': 'TChain'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Tree descriptor (the main ROOT I/O class)");
    he.insert("TTree", h);
    let mut h = HashMap::new();
    h.insert("title", "[fTreeOffsetLen] Array of variables");
    he.insert("fTreeOffset", h);
    let mut h = HashMap::new();
    h.insert("title", "Current size of fTreeOffset array");
    he.insert("fTreeOffsetLen", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of trees");
    he.insert("fNtrees", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "-> List of file names containing the trees (TChainElement, owned)",
    );
    he.insert("fFiles", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "-> List of active/inactive branches (TChainElement, owned)",
    );
    he.insert("fStatus", h);
    hh.insert("TChain", he);
    // TStreamerObject
    //si =  {'properties': {'f_name': 'TStreamerObject'}, 'elements': {'TStreamerElement': {'name': 'TStreamerElement', 'title': 'Base class for one element (data member) to be Streamed'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Base class for one element (data member) to be Streamed",
    );
    he.insert("TStreamerElement", h);
    hh.insert("TStreamerObject", he);
    // TAttFill
    //si =  {'properties': {'f_name': 'TAttFill'}, 'elements': {'fFillColor': {'name': 'fFillColor', 'title': 'Fill area color'}, 'fFillStyle': {'name': 'fFillStyle', 'title': 'Fill area style'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Fill area color");
    he.insert("fFillColor", h);
    let mut h = HashMap::new();
    h.insert("title", "Fill area style");
    he.insert("fFillStyle", h);
    hh.insert("TAttFill", he);
    // TAttPad
    //si =  {'elements': {'fAstat': {'title': 'Alignment for the statistics', 'name': 'fAstat'}, 'fFrameFillStyle': {'title': 'Pad frame fill style', 'name': 'fFrameFillStyle'}, 'fFrameLineWidth': {'name': 'fFrameLineWidth', 'title': 'Pad frame line width'}, 'fYfile': {'title': 'Y position where to draw the file name', 'name': 'fYfile'}, 'fLeftMargin': {'name': 'fLeftMargin', 'title': 'LeftMargin'}, 'fRightMargin': {'title': 'RightMargin', 'name': 'fRightMargin'}, 'fAfile': {'name': 'fAfile', 'title': 'Alignment for the file name'}, 'fBottomMargin': {'name': 'fBottomMargin', 'title': 'BottomMargin'}, 'fXstat': {'name': 'fXstat', 'title': 'X position where to draw the statistics'}, 'fFrameLineColor': {'name': 'fFrameLineColor', 'title': 'Pad frame line color'}, 'fFrameLineStyle': {'name': 'fFrameLineStyle', 'title': 'Pad frame line style'}, 'fTopMargin': {'name': 'fTopMargin', 'title': 'TopMargin'}, 'fFrameBorderSize': {'title': 'Pad frame border size', 'name': 'fFrameBorderSize'}, 'fFrameBorderMode': {'title': 'Pad frame border mode', 'name': 'fFrameBorderMode'}, 'fYstat': {'name': 'fYstat', 'title': 'Y position where to draw the statistics'}, 'fFrameFillColor': {'name': 'fFrameFillColor', 'title': 'Pad frame fill color'}, 'fXfile': {'title': 'X position where to draw the file name', 'name': 'fXfile'}}, 'properties': {'f_name': 'TAttPad'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Alignment for the statistics");
    he.insert("fAstat", h);
    let mut h = HashMap::new();
    h.insert("title", "Pad frame fill style");
    he.insert("fFrameFillStyle", h);
    let mut h = HashMap::new();
    h.insert("title", "Pad frame line width");
    he.insert("fFrameLineWidth", h);
    let mut h = HashMap::new();
    h.insert("title", "Y position where to draw the file name");
    he.insert("fYfile", h);
    let mut h = HashMap::new();
    h.insert("title", "LeftMargin");
    he.insert("fLeftMargin", h);
    let mut h = HashMap::new();
    h.insert("title", "RightMargin");
    he.insert("fRightMargin", h);
    let mut h = HashMap::new();
    h.insert("title", "Alignment for the file name");
    he.insert("fAfile", h);
    let mut h = HashMap::new();
    h.insert("title", "BottomMargin");
    he.insert("fBottomMargin", h);
    let mut h = HashMap::new();
    h.insert("title", "X position where to draw the statistics");
    he.insert("fXstat", h);
    let mut h = HashMap::new();
    h.insert("title", "Pad frame line color");
    he.insert("fFrameLineColor", h);
    let mut h = HashMap::new();
    h.insert("title", "Pad frame line style");
    he.insert("fFrameLineStyle", h);
    let mut h = HashMap::new();
    h.insert("title", "TopMargin");
    he.insert("fTopMargin", h);
    let mut h = HashMap::new();
    h.insert("title", "Pad frame border size");
    he.insert("fFrameBorderSize", h);
    let mut h = HashMap::new();
    h.insert("title", "Pad frame border mode");
    he.insert("fFrameBorderMode", h);
    let mut h = HashMap::new();
    h.insert("title", "Y position where to draw the statistics");
    he.insert("fYstat", h);
    let mut h = HashMap::new();
    h.insert("title", "Pad frame fill color");
    he.insert("fFrameFillColor", h);
    let mut h = HashMap::new();
    h.insert("title", "X position where to draw the file name");
    he.insert("fXfile", h);
    hh.insert("TAttPad", he);
    // TH1C
    //si =  {'elements': {'TH1': {'title': '1-Dim histogram base class', 'name': 'TH1'}, 'TArrayC': {'title': 'Array of chars', 'name': 'TArrayC'}}, 'properties': {'f_name': 'TH1C'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "1-Dim histogram base class");
    he.insert("TH1", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of chars");
    he.insert("TArrayC", h);
    hh.insert("TH1C", he);
    // TGraphErrors
    //si =  {'elements': {'TGraph': {'title': 'Graph graphics class', 'name': 'TGraph'}, 'fEY': {'name': 'fEY', 'title': '[fNpoints] array of Y errors'}, 'fEX': {'name': 'fEX', 'title': '[fNpoints] array of X errors'}}, 'properties': {'f_name': 'TGraphErrors'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Graph graphics class");
    he.insert("TGraph", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNpoints] array of Y errors");
    he.insert("fEY", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNpoints] array of X errors");
    he.insert("fEX", h);
    hh.insert("TGraphErrors", he);
    // TH1
    //si =  {'properties': {'f_name': 'TH1'}, 'elements': {'TAttMarker': {'title': 'Marker attributes', 'name': 'TAttMarker'}, 'fBuffer': {'name': 'fBuffer', 'title': '[fBufferSize] entry buffer'}, 'fEntries': {'title': 'Number of entries', 'name': 'fEntries'}, 'fFunctions': {'name': 'fFunctions', 'title': '->Pointer to list of functions (fits and user)'}, 'TAttLine': {'title': 'Line attributes', 'name': 'TAttLine'}, 'fTsumwx': {'name': 'fTsumwx', 'title': 'Total Sum of weight*X'}, 'fYaxis': {'title': 'Y axis descriptor', 'name': 'fYaxis'}, 'fXaxis': {'title': 'X axis descriptor', 'name': 'fXaxis'}, 'TNamed': {'title': 'The basis for a named object (name, title)', 'name': 'TNamed'}, 'fBarOffset': {'title': '(1000*offset) for bar charts or legos', 'name': 'fBarOffset'}, 'fMaximum': {'name': 'fMaximum', 'title': 'Maximum value for plotting'}, 'fBufferSize': {'title': 'fBuffer size', 'name': 'fBufferSize'}, 'fStatOverflows': {'title': 'Per object flag to use under/overflows in statistics', 'name': 'fStatOverflows'}, 'fSumw2': {'title': 'Array of sum of squares of weights', 'name': 'fSumw2'}, 'fNcells': {'name': 'fNcells', 'title': 'Number of bins(1D), cells (2D) +U/Overflows'}, 'fMinimum': {'name': 'fMinimum', 'title': 'Minimum value for plotting'}, 'fNormFactor': {'name': 'fNormFactor', 'title': 'Normalization factor'}, 'fTsumw': {'title': 'Total Sum of weights', 'name': 'fTsumw'}, 'fTsumwx2': {'title': 'Total Sum of weight*X*X', 'name': 'fTsumwx2'}, 'fContour': {'title': 'Array to display contour levels', 'name': 'fContour'}, 'fOption': {'title': 'Histogram options', 'name': 'fOption'}, 'fBarWidth': {'title': '(1000*width) for bar charts or legos', 'name': 'fBarWidth'}, 'fTsumw2': {'name': 'fTsumw2', 'title': 'Total Sum of squares of weights'}, 'fZaxis': {'title': 'Z axis descriptor', 'name': 'fZaxis'}, 'fBinStatErrOpt': {'name': 'fBinStatErrOpt', 'title': 'Option for bin statistical errors'}, 'TAttFill': {'title': 'Fill area attributes', 'name': 'TAttFill'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Marker attributes");
    he.insert("TAttMarker", h);
    let mut h = HashMap::new();
    h.insert("title", "[fBufferSize] entry buffer");
    he.insert("fBuffer", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of entries");
    he.insert("fEntries", h);
    let mut h = HashMap::new();
    h.insert("title", "->Pointer to list of functions (fits and user)");
    he.insert("fFunctions", h);
    let mut h = HashMap::new();
    h.insert("title", "Line attributes");
    he.insert("TAttLine", h);
    let mut h = HashMap::new();
    h.insert("title", "Total Sum of weight*X");
    he.insert("fTsumwx", h);
    let mut h = HashMap::new();
    h.insert("title", "Y axis descriptor");
    he.insert("fYaxis", h);
    let mut h = HashMap::new();
    h.insert("title", "X axis descriptor");
    he.insert("fXaxis", h);
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    let mut h = HashMap::new();
    h.insert("title", "(1000*offset) for bar charts or legos");
    he.insert("fBarOffset", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum value for plotting");
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert("title", "fBuffer size");
    he.insert("fBufferSize", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Per object flag to use under/overflows in statistics",
    );
    he.insert("fStatOverflows", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of sum of squares of weights");
    he.insert("fSumw2", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bins(1D), cells (2D) +U/Overflows");
    he.insert("fNcells", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value for plotting");
    he.insert("fMinimum", h);
    let mut h = HashMap::new();
    h.insert("title", "Normalization factor");
    he.insert("fNormFactor", h);
    let mut h = HashMap::new();
    h.insert("title", "Total Sum of weights");
    he.insert("fTsumw", h);
    let mut h = HashMap::new();
    h.insert("title", "Total Sum of weight*X*X");
    he.insert("fTsumwx2", h);
    let mut h = HashMap::new();
    h.insert("title", "Array to display contour levels");
    he.insert("fContour", h);
    let mut h = HashMap::new();
    h.insert("title", "Histogram options");
    he.insert("fOption", h);
    let mut h = HashMap::new();
    h.insert("title", "(1000*width) for bar charts or legos");
    he.insert("fBarWidth", h);
    let mut h = HashMap::new();
    h.insert("title", "Total Sum of squares of weights");
    he.insert("fTsumw2", h);
    let mut h = HashMap::new();
    h.insert("title", "Z axis descriptor");
    he.insert("fZaxis", h);
    let mut h = HashMap::new();
    h.insert("title", "Option for bin statistical errors");
    he.insert("fBinStatErrOpt", h);
    let mut h = HashMap::new();
    h.insert("title", "Fill area attributes");
    he.insert("TAttFill", h);
    hh.insert("TH1", he);
    // TGraphAsymmErrors
    //si =  {'properties': {'f_name': 'TGraphAsymmErrors'}, 'elements': {'fEYlow': {'name': 'fEYlow', 'title': '[fNpoints] array of Y low errors'}, 'fEXhigh': {'name': 'fEXhigh', 'title': '[fNpoints] array of X high errors'}, 'fEXlow': {'name': 'fEXlow', 'title': '[fNpoints] array of X low errors'}, 'TGraph': {'name': 'TGraph', 'title': 'Graph graphics class'}, 'fEYhigh': {'name': 'fEYhigh', 'title': '[fNpoints] array of Y high errors'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "[fNpoints] array of Y low errors");
    he.insert("fEYlow", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNpoints] array of X high errors");
    he.insert("fEXhigh", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNpoints] array of X low errors");
    he.insert("fEXlow", h);
    let mut h = HashMap::new();
    h.insert("title", "Graph graphics class");
    he.insert("TGraph", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNpoints] array of Y high errors");
    he.insert("fEYhigh", h);
    hh.insert("TGraphAsymmErrors", he);
    // TRefArray
    //si =  {'elements': {'TSeqCollection': {'title': 'Sequenceable collection ABC', 'name': 'TSeqCollection'}, 'fPID': {'title': 'Pointer to Process Unique Identifier', 'name': 'fPID'}, 'fUIDs': {'name': 'fUIDs', 'title': '[fSize] To store uids of referenced objects'}, 'fLowerBound': {'name': 'fLowerBound', 'title': 'Lower bound of the array'}, 'fLast': {'title': 'Last element in array containing an object', 'name': 'fLast'}}, 'properties': {'f_name': 'TRefArray'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Sequenceable collection ABC");
    he.insert("TSeqCollection", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to Process Unique Identifier");
    he.insert("fPID", h);
    let mut h = HashMap::new();
    h.insert("title", "[fSize] To store uids of referenced objects");
    he.insert("fUIDs", h);
    let mut h = HashMap::new();
    h.insert("title", "Lower bound of the array");
    he.insert("fLowerBound", h);
    let mut h = HashMap::new();
    h.insert("title", "Last element in array containing an object");
    he.insert("fLast", h);
    hh.insert("TRefArray", he);
    // TMultiGraph
    //si =  {'properties': {'f_name': 'TMultiGraph'}, 'elements': {'fMaximum': {'title': 'Maximum value for plotting along y', 'name': 'fMaximum'}, 'fMinimum': {'name': 'fMinimum', 'title': 'Minimum value for plotting along y'}, 'fHistogram': {'name': 'fHistogram', 'title': 'Pointer to histogram used for drawing axis'}, 'TNamed': {'name': 'TNamed', 'title': 'The basis for a named object (name, title)'}, 'fGraphs': {'name': 'fGraphs', 'title': 'Pointer to list of TGraphs'}, 'fFunctions': {'name': 'fFunctions', 'title': 'Pointer to list of functions (fits and user)'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Maximum value for plotting along y");
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value for plotting along y");
    he.insert("fMinimum", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to histogram used for drawing axis");
    he.insert("fHistogram", h);
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to list of TGraphs");
    he.insert("fGraphs", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to list of functions (fits and user)");
    he.insert("fFunctions", h);
    hh.insert("TMultiGraph", he);
    // TFeldmanCousins
    //si =  {'properties': {'f_name': 'TFeldmanCousins'}, 'elements': {'fMuMin': {'name': 'fMuMin', 'title': 'minimum value of signal to use in calculating the tables'}, 'fQUICK': {'title': 'take a short cut to speed up the process of generating a', 'name': 'fQUICK'}, 'fMuStep': {'name': 'fMuStep', 'title': 'the step in signal to use when generating tables'}, 'fUpperLimit': {'title': 'the calculated upper limit', 'name': 'fUpperLimit'}, 'fCL': {'title': 'confidence level as a fraction [e.g. 90% = 0.9]', 'name': 'fCL'}, 'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}, 'fLowerLimit': {'title': 'the calculated lower limit', 'name': 'fLowerLimit'}, 'fMuMax': {'name': 'fMuMax', 'title': 'maximum value of signal to use in calculating the tables'}, 'fNbackground': {'name': 'fNbackground', 'title': 'input number of background events'}, 'fNMuStep': {'title': '= (int)(fMuStep)', 'name': 'fNMuStep'}, 'fNobserved': {'title': 'input number of observed events', 'name': 'fNobserved'}, 'fNMax': {'name': 'fNMax', 'title': '= (int)(fMuMax)'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "minimum value of signal to use in calculating the tables",
    );
    he.insert("fMuMin", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "take a short cut to speed up the process of generating a",
    );
    he.insert("fQUICK", h);
    let mut h = HashMap::new();
    h.insert("title", "the step in signal to use when generating tables");
    he.insert("fMuStep", h);
    let mut h = HashMap::new();
    h.insert("title", "the calculated upper limit");
    he.insert("fUpperLimit", h);
    let mut h = HashMap::new();
    h.insert("title", "confidence level as a fraction [e.g. 90% = 0.9]");
    he.insert("fCL", h);
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    let mut h = HashMap::new();
    h.insert("title", "the calculated lower limit");
    he.insert("fLowerLimit", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "maximum value of signal to use in calculating the tables",
    );
    he.insert("fMuMax", h);
    let mut h = HashMap::new();
    h.insert("title", "input number of background events");
    he.insert("fNbackground", h);
    let mut h = HashMap::new();
    h.insert("title", "= (int)(fMuStep)");
    he.insert("fNMuStep", h);
    let mut h = HashMap::new();
    h.insert("title", "input number of observed events");
    he.insert("fNobserved", h);
    let mut h = HashMap::new();
    h.insert("title", "= (int)(fMuMax)");
    he.insert("fNMax", h);
    hh.insert("TFeldmanCousins", he);
    // TFormula
    //si =  {'properties': {'f_name': 'TFormula'}, 'elements': {'fNdim': {'title': 'Dimension - needed for lambda expressions', 'name': 'fNdim'}, 'TNamed': {'name': 'TNamed', 'title': 'The basis for a named object (name, title)'}, 'fAllParametersSetted': {'name': 'fAllParametersSetted', 'title': 'Flag to control if all parameters are setted'}, 'fParams': {'name': 'fParams', 'title': '|| List of  parameter names'}, 'fFormula': {'name': 'fFormula', 'title': 'String representing the formula expression'}, 'fLinearParts': {'name': 'fLinearParts', 'title': 'Vector of linear functions'}, 'fVectorized': {'title': 'Whether we should use vectorized or regular variables', 'name': 'fVectorized'}, 'fClingParameters': {'title': 'Parameter values', 'name': 'fClingParameters'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Dimension - needed for lambda expressions");
    he.insert("fNdim", h);
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    let mut h = HashMap::new();
    h.insert("title", "Flag to control if all parameters are setted");
    he.insert("fAllParametersSetted", h);
    let mut h = HashMap::new();
    h.insert("title", "|| List of  parameter names");
    he.insert("fParams", h);
    let mut h = HashMap::new();
    h.insert("title", "String representing the formula expression");
    he.insert("fFormula", h);
    let mut h = HashMap::new();
    h.insert("title", "Vector of linear functions");
    he.insert("fLinearParts", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Whether we should use vectorized or regular variables",
    );
    he.insert("fVectorized", h);
    let mut h = HashMap::new();
    h.insert("title", "Parameter values");
    he.insert("fClingParameters", h);
    hh.insert("TFormula", he);
    // TGraphMultiErrors
    //si =  {'elements': {'fExL': {'title': '[fNpoints] array of X low errors', 'name': 'fExL'}, 'fAttLine': {'title': 'The AttLine attributes of the different errors', 'name': 'fAttLine'}, 'fExH': {'name': 'fExH', 'title': '[fNpoints] array of X high errors'}, 'fNYErrors': {'title': 'The amount of different y-errors', 'name': 'fNYErrors'}, 'fEyL': {'title': 'Two dimensional array of Y low errors', 'name': 'fEyL'}, 'fAttFill': {'name': 'fAttFill', 'title': 'The AttFill attributes of the different errors'}, 'fSumErrorsMode': {'name': 'fSumErrorsMode', 'title': 'How y errors are summed: kOnlyFirst = Only First; kSquareSum = Squared Sum; kSum ='}, 'fEyH': {'title': 'Two dimensional array of Y high errors', 'name': 'fEyH'}, 'TGraph': {'title': 'Graph graphics class', 'name': 'TGraph'}}, 'properties': {'f_name': 'TGraphMultiErrors'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "[fNpoints] array of X low errors");
    he.insert("fExL", h);
    let mut h = HashMap::new();
    h.insert("title", "The AttLine attributes of the different errors");
    he.insert("fAttLine", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNpoints] array of X high errors");
    he.insert("fExH", h);
    let mut h = HashMap::new();
    h.insert("title", "The amount of different y-errors");
    he.insert("fNYErrors", h);
    let mut h = HashMap::new();
    h.insert("title", "Two dimensional array of Y low errors");
    he.insert("fEyL", h);
    let mut h = HashMap::new();
    h.insert("title", "The AttFill attributes of the different errors");
    he.insert("fAttFill", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "How y errors are summed: kOnlyFirst = Only First; kSquareSum = Squared Sum; kSum =",
    );
    he.insert("fSumErrorsMode", h);
    let mut h = HashMap::new();
    h.insert("title", "Two dimensional array of Y high errors");
    he.insert("fEyH", h);
    let mut h = HashMap::new();
    h.insert("title", "Graph graphics class");
    he.insert("TGraph", h);
    hh.insert("TGraphMultiErrors", he);
    // TRef
    //si =  {'elements': {'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}}, 'properties': {'f_name': 'TRef'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    hh.insert("TRef", he);
    // TLorentzVector
    //si =  {'properties': {'f_name': 'TLorentzVector'}, 'elements': {'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}, 'fE': {'name': 'fE', 'title': 'time or energy of (x,y,z,t) or (px,py,pz,e)'}, 'fP': {'name': 'fP', 'title': '3 vector component'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    let mut h = HashMap::new();
    h.insert("title", "time or energy of (x,y,z,t) or (px,py,pz,e)");
    he.insert("fE", h);
    let mut h = HashMap::new();
    h.insert("title", "3 vector component");
    he.insert("fP", h);
    hh.insert("TLorentzVector", he);
    // TLimitDataSource
    //si =  {'properties': {'f_name': 'TLimitDataSource'}, 'elements': {'fSignal': {'name': 'fSignal', 'title': 'Packed input signal'}, 'fErrorOnSignal': {'title': 'Packed error sources for signal', 'name': 'fErrorOnSignal'}, 'fIds': {'name': 'fIds', 'title': 'Packed IDs for the different error sources'}, 'fCandidates': {'name': 'fCandidates', 'title': 'Packed input candidates (data)'}, 'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}, 'fDummyTA': {'title': 'Array of dummy object (used for bookeeping)', 'name': 'fDummyTA'}, 'fErrorOnBackground': {'name': 'fErrorOnBackground', 'title': 'Packed error sources for background'}, 'fDummyIds': {'name': 'fDummyIds', 'title': 'Array of dummy object (used for bookeeping)'}, 'fBackground': {'title': 'Packed input background', 'name': 'fBackground'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Packed input signal");
    he.insert("fSignal", h);
    let mut h = HashMap::new();
    h.insert("title", "Packed error sources for signal");
    he.insert("fErrorOnSignal", h);
    let mut h = HashMap::new();
    h.insert("title", "Packed IDs for the different error sources");
    he.insert("fIds", h);
    let mut h = HashMap::new();
    h.insert("title", "Packed input candidates (data)");
    he.insert("fCandidates", h);
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of dummy object (used for bookeeping)");
    he.insert("fDummyTA", h);
    let mut h = HashMap::new();
    h.insert("title", "Packed error sources for background");
    he.insert("fErrorOnBackground", h);
    let mut h = HashMap::new();
    h.insert("title", "Array of dummy object (used for bookeeping)");
    he.insert("fDummyIds", h);
    let mut h = HashMap::new();
    h.insert("title", "Packed input background");
    he.insert("fBackground", h);
    hh.insert("TLimitDataSource", he);
    // TArrayS
    //si =  {'elements': {'fArray': {'name': 'fArray', 'title': '[fN] Array of fN shorts'}, 'TArray': {'name': 'TArray', 'title': 'Abstract array base class'}}, 'properties': {'f_name': 'TArrayS'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "[fN] Array of fN shorts");
    he.insert("fArray", h);
    let mut h = HashMap::new();
    h.insert("title", "Abstract array base class");
    he.insert("TArray", h);
    hh.insert("TArrayS", he);
    // TLeafB
    //si =  {'properties': {'f_name': 'TLeafB'}, 'elements': {'fMaximum': {'title': 'Maximum value if leaf range is specified', 'name': 'fMaximum'}, 'TLeaf': {'name': 'TLeaf', 'title': 'Leaf: description of a Branch data type'}, 'fMinimum': {'title': 'Minimum value if leaf range is specified', 'name': 'fMinimum'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Maximum value if leaf range is specified");
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value if leaf range is specified");
    he.insert("fMinimum", h);
    hh.insert("TLeafB", he);
    // TVirtualPad
    //si =  {'properties': {'f_name': 'TVirtualPad'}, 'elements': {'TAttPad': {'title': 'Pad attributes', 'name': 'TAttPad'}, 'TObject': {'title': 'Basic ROOT object', 'name': 'TObject'}, 'TQObject': {'title': 'Base class for object communication mechanism', 'name': 'TQObject'}, 'TAttFill': {'name': 'TAttFill', 'title': 'Fill area attributes'}, 'TAttLine': {'name': 'TAttLine', 'title': 'Line attributes'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Pad attributes");
    he.insert("TAttPad", h);
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    let mut h = HashMap::new();
    h.insert("title", "Base class for object communication mechanism");
    he.insert("TQObject", h);
    let mut h = HashMap::new();
    h.insert("title", "Fill area attributes");
    he.insert("TAttFill", h);
    let mut h = HashMap::new();
    h.insert("title", "Line attributes");
    he.insert("TAttLine", h);
    hh.insert("TVirtualPad", he);
    // TNamed
    //si =  {'properties': {'f_name': 'TNamed'}, 'elements': {'f_name': {'title': 'object identifier', 'name': 'f_name'}, 'fTitle': {'name': 'fTitle', 'title': 'object title'}, 'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "object identifier");
    he.insert("f_name", h);
    let mut h = HashMap::new();
    h.insert("title", "object title");
    he.insert("fTitle", h);
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    hh.insert("TNamed", he);
    // TAxis
    //si =  {'properties': {'f_name': 'TAxis'}, 'elements': {'fBits2': {'name': 'fBits2', 'title': 'Second bit status word'}, 'fXmax': {'title': 'Upper edge of last bin', 'name': 'fXmax'}, 'fLast': {'title': 'Last bin to display', 'name': 'fLast'}, 'fXbins': {'name': 'fXbins', 'title': 'Bin edges array in X'}, 'fTimeFormat': {'title': 'Date&time format, ex: 09/12/99 12:34:00', 'name': 'fTimeFormat'}, 'fFirst': {'name': 'fFirst', 'title': 'First bin to display'}, 'fNbins': {'name': 'fNbins', 'title': 'Number of bins'}, 'fLabels': {'name': 'fLabels', 'title': 'List of labels'}, 'fModLabs': {'name': 'fModLabs', 'title': 'List of modified labels'}, 'TNamed': {'name': 'TNamed', 'title': 'The basis for a named object (name, title)'}, 'TAttAxis': {'title': 'Axis attributes', 'name': 'TAttAxis'}, 'fTimeDisplay': {'name': 'fTimeDisplay', 'title': 'On/off displaying time values instead of numerics'}, 'fXmin': {'name': 'fXmin', 'title': 'Low edge of first bin'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Second bit status word");
    he.insert("fBits2", h);
    let mut h = HashMap::new();
    h.insert("title", "Upper edge of last bin");
    he.insert("fXmax", h);
    let mut h = HashMap::new();
    h.insert("title", "Last bin to display");
    he.insert("fLast", h);
    let mut h = HashMap::new();
    h.insert("title", "Bin edges array in X");
    he.insert("fXbins", h);
    let mut h = HashMap::new();
    h.insert("title", "Date&time format, ex: 09/12/99 12:34:00");
    he.insert("fTimeFormat", h);
    let mut h = HashMap::new();
    h.insert("title", "First bin to display");
    he.insert("fFirst", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bins");
    he.insert("fNbins", h);
    let mut h = HashMap::new();
    h.insert("title", "List of labels");
    he.insert("fLabels", h);
    let mut h = HashMap::new();
    h.insert("title", "List of modified labels");
    he.insert("fModLabs", h);
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    let mut h = HashMap::new();
    h.insert("title", "Axis attributes");
    he.insert("TAttAxis", h);
    let mut h = HashMap::new();
    h.insert("title", "On/off displaying time values instead of numerics");
    he.insert("fTimeDisplay", h);
    let mut h = HashMap::new();
    h.insert("title", "Low edge of first bin");
    he.insert("fXmin", h);
    hh.insert("TAxis", he);
    // TArrayC
    //si =  {'properties': {'f_name': 'TArrayC'}, 'elements': {'TArray': {'title': 'Abstract array base class', 'name': 'TArray'}, 'fArray': {'title': '[fN] Array of fN chars', 'name': 'fArray'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Abstract array base class");
    he.insert("TArray", h);
    let mut h = HashMap::new();
    h.insert("title", "[fN] Array of fN chars");
    he.insert("fArray", h);
    hh.insert("TArrayC", he);
    // TRefTable
    //si =  {'properties': {'f_name': 'TRefTable'}, 'elements': {'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}, 'fOwner': {'title': 'Object owning this TRefTable', 'name': 'fOwner'}, 'fSize': {'title': 'dummy for backward compatibility', 'name': 'fSize'}, 'fParents': {'name': 'fParents', 'title': 'array of Parent objects  (eg TTree branch) holding the referenced objects'}, 'fProcessGUIDs': {'name': 'fProcessGUIDs', 'title': 'UUIDs of TProcessIDs used in fParentIDs'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    let mut h = HashMap::new();
    h.insert("title", "Object owning this TRefTable");
    he.insert("fOwner", h);
    let mut h = HashMap::new();
    h.insert("title", "dummy for backward compatibility");
    he.insert("fSize", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "array of Parent objects  (eg TTree branch) holding the referenced objects",
    );
    he.insert("fParents", h);
    let mut h = HashMap::new();
    h.insert("title", "UUIDs of TProcessIDs used in fParentIDs");
    he.insert("fProcessGUIDs", h);
    hh.insert("TRefTable", he);
    // TScatter
    //si =  {'elements': {'fSize': {'name': 'fSize', 'title': '[fNpoints] array of marker sizes'}, 'fNpoints': {'title': 'Number of points <= fMaxSize', 'name': 'fNpoints'}, 'fColor': {'name': 'fColor', 'title': '[fNpoints] array of colors'}, 'fMargin': {'title': 'Margin around the plot in %', 'name': 'fMargin'}, 'fMaxMarkerSize': {'title': 'Largest marker size used to paint the markers', 'name': 'fMaxMarkerSize'}, 'TAttMarker': {'name': 'TAttMarker', 'title': 'Marker attributes'}, 'TAttFill': {'name': 'TAttFill', 'title': 'Fill area attributes'}, 'fHistogram': {'title': 'Pointer to histogram used for drawing axis', 'name': 'fHistogram'}, 'TAttLine': {'title': 'Line attributes', 'name': 'TAttLine'}, 'fMinMarkerSize': {'name': 'fMinMarkerSize', 'title': 'Smallest marker size used to paint the markers'}, 'fGraph': {'title': 'Pointer to graph holding X and Y positions', 'name': 'fGraph'}, 'TNamed': {'name': 'TNamed', 'title': 'The basis for a named object (name, title)'}}, 'properties': {'f_name': 'TScatter'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "[fNpoints] array of marker sizes");
    he.insert("fSize", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of points <= fMaxSize");
    he.insert("fNpoints", h);
    let mut h = HashMap::new();
    h.insert("title", "[fNpoints] array of colors");
    he.insert("fColor", h);
    let mut h = HashMap::new();
    h.insert("title", "Margin around the plot in %");
    he.insert("fMargin", h);
    let mut h = HashMap::new();
    h.insert("title", "Largest marker size used to paint the markers");
    he.insert("fMaxMarkerSize", h);
    let mut h = HashMap::new();
    h.insert("title", "Marker attributes");
    he.insert("TAttMarker", h);
    let mut h = HashMap::new();
    h.insert("title", "Fill area attributes");
    he.insert("TAttFill", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to histogram used for drawing axis");
    he.insert("fHistogram", h);
    let mut h = HashMap::new();
    h.insert("title", "Line attributes");
    he.insert("TAttLine", h);
    let mut h = HashMap::new();
    h.insert("title", "Smallest marker size used to paint the markers");
    he.insert("fMinMarkerSize", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to graph holding X and Y positions");
    he.insert("fGraph", h);
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    hh.insert("TScatter", he);
    // TLeafL
    //si =  {'elements': {'fMaximum': {'name': 'fMaximum', 'title': 'Maximum value if leaf range is specified'}, 'TLeaf': {'name': 'TLeaf', 'title': 'Leaf: description of a Branch data type'}, 'fMinimum': {'name': 'fMinimum', 'title': 'Minimum value if leaf range is specified'}}, 'properties': {'f_name': 'TLeafL'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Maximum value if leaf range is specified");
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value if leaf range is specified");
    he.insert("fMinimum", h);
    hh.insert("TLeafL", he);
    // TString
    //si =  {'properties': {'f_name': 'TString'}}
    let he = HashMap::new();
    hh.insert("TString", he);
    // TH2PolyBin
    //si =  {'properties': {'f_name': 'TH2PolyBin'}, 'elements': {'fContent': {'title': 'Bin content', 'name': 'fContent'}, 'fArea': {'title': 'Bin area', 'name': 'fArea'}, 'fPoly': {'name': 'fPoly', 'title': 'Object holding the polygon definition'}, 'fNumber': {'name': 'fNumber', 'title': 'Bin number of the bin in TH2Poly'}, 'fYmin': {'title': 'Y minimum value', 'name': 'fYmin'}, 'fChanged': {'name': 'fChanged', 'title': 'For the 3D Painter'}, 'fYmax': {'name': 'fYmax', 'title': 'Y maximum value'}, 'fXmax': {'title': 'X maximum value', 'name': 'fXmax'}, 'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}, 'fXmin': {'name': 'fXmin', 'title': 'X minimum value'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Bin content");
    he.insert("fContent", h);
    let mut h = HashMap::new();
    h.insert("title", "Bin area");
    he.insert("fArea", h);
    let mut h = HashMap::new();
    h.insert("title", "Object holding the polygon definition");
    he.insert("fPoly", h);
    let mut h = HashMap::new();
    h.insert("title", "Bin number of the bin in TH2Poly");
    he.insert("fNumber", h);
    let mut h = HashMap::new();
    h.insert("title", "Y minimum value");
    he.insert("fYmin", h);
    let mut h = HashMap::new();
    h.insert("title", "For the 3D Painter");
    he.insert("fChanged", h);
    let mut h = HashMap::new();
    h.insert("title", "Y maximum value");
    he.insert("fYmax", h);
    let mut h = HashMap::new();
    h.insert("title", "X maximum value");
    he.insert("fXmax", h);
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    let mut h = HashMap::new();
    h.insert("title", "X minimum value");
    he.insert("fXmin", h);
    hh.insert("TH2PolyBin", he);
    // TMap
    //si =  {'properties': {'f_name': 'TMap'}, 'elements': {'TCollection': {'name': 'TCollection', 'title': 'Collection abstract base class'}, 'fTable': {'title': "Hash table used to store TPair's", 'name': 'fTable'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Collection abstract base class");
    he.insert("TCollection", h);
    let mut h = HashMap::new();
    h.insert("title", "Hash table used to store TPair's");
    he.insert("fTable", h);
    hh.insert("TMap", he);
    // TStreamerElement
    //si =  {'properties': {'f_name': 'TStreamerElement'}, 'elements': {'TNamed': {'name': 'TNamed', 'title': 'The basis for a named object (name, title)'}, 'fMaxIndex': {'title': 'Maximum array index for array dimension "dim"', 'name': 'fMaxIndex'}, 'fType': {'title': 'element type', 'name': 'fType'}, 'fArrayLength': {'name': 'fArrayLength', 'title': 'cumulative size of all array dims'}, 'fTypeName': {'name': 'fTypeName', 'title': 'Data type name of data member'}, 'fSize': {'title': 'sizeof element', 'name': 'fSize'}, 'fArrayDim': {'name': 'fArrayDim', 'title': 'number of array dimensions'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum array index for array dimension \"dim\"");
    he.insert("fMaxIndex", h);
    let mut h = HashMap::new();
    h.insert("title", "element type");
    he.insert("fType", h);
    let mut h = HashMap::new();
    h.insert("title", "cumulative size of all array dims");
    he.insert("fArrayLength", h);
    let mut h = HashMap::new();
    h.insert("title", "Data type name of data member");
    he.insert("fTypeName", h);
    let mut h = HashMap::new();
    h.insert("title", "sizeof element");
    he.insert("fSize", h);
    let mut h = HashMap::new();
    h.insert("title", "number of array dimensions");
    he.insert("fArrayDim", h);
    hh.insert("TStreamerElement", he);
    // TPad
    //si =  {'elements': {'TVirtualPad': {'name': 'TVirtualPad', 'title': 'Abstract base class for Pads and Canvases'}, 'fNextPaletteColor': {'title': 'Next automatic color', 'name': 'fNextPaletteColor'}, 'fXtoPixelk': {'title': 'Conversion coefficient for X World to pixel', 'name': 'fXtoPixelk'}, 'fXtoAbsPixelk': {'title': 'Conversion coefficient for X World to absolute pixel', 'name': 'fXtoAbsPixelk'}, 'fPixeltoXk': {'name': 'fPixeltoXk', 'title': 'Conversion coefficient for pixel to X World'}, 'fNumPaletteColor': {'name': 'fNumPaletteColor', 'title': 'Number of objects with an automatic color'}, 'fXtoPixel': {'name': 'fXtoPixel', 'title': 'xpixel = fXtoPixelk + fXtoPixel*xworld'}, 'fUxmin': {'name': 'fUxmin', 'title': 'Minimum value on the X axis'}, 'fY1': {'title': 'Y of lower Y coordinate', 'name': 'fY1'}, 'fExecs': {'title': 'List of commands to be executed when a pad event occurs', 'name': 'fExecs'}, 'fLogy': {'title': '(=0 if Y linear scale, =1 if log scale)', 'name': 'fLogy'}, 'fPrimitives': {'title': '->List of primitives (subpads)', 'name': 'fPrimitives'}, 'fXUpNDC': {'title': '', 'name': 'fXUpNDC'}, 'fX1': {'name': 'fX1', 'title': 'X of lower X coordinate'}, 'fTitle': {'name': 'fTitle', 'title': 'Pad title'}, 'fAbsPixeltoXk': {'title': 'Conversion coefficient for absolute pixel to X World', 'name': 'fAbsPixeltoXk'}, 'fAbsHNDC': {'name': 'fAbsHNDC', 'title': 'Absolute Height of pad along Y in NDC'}, 'fUymax': {'title': 'Maximum value on the Y axis', 'name': 'fUymax'}, 'fAbsWNDC': {'name': 'fAbsWNDC', 'title': 'Absolute Width of pad along X in NDC'}, 'fBorderSize': {'title': 'pad bordersize in pixels', 'name': 'fBorderSize'}, 'fYtoAbsPixelk': {'title': 'Conversion coefficient for Y World to absolute pixel', 'name': 'fYtoAbsPixelk'}, 'fGridy': {'name': 'fGridy', 'title': 'Set to true if grid along Y'}, 'fPixeltoX': {'name': 'fPixeltoX', 'title': 'xworld = fPixeltoXk + fPixeltoX*xpixel'}, 'fAbsCoord': {'name': 'fAbsCoord', 'title': 'Use absolute coordinates'}, 'fUxmax': {'title': 'Maximum value on the X axis', 'name': 'fUxmax'}, 'fGridx': {'title': 'Set to true if grid along X', 'name': 'fGridx'}, 'fUtoPixel': {'title': 'xpixel = fUtoPixelk + fUtoPixel*undc', 'name': 'fUtoPixel'}, 'fTicky': {'name': 'fTicky', 'title': 'Set to 1 if tick marks along Y'}, 'fFixedAspectRatio': {'name': 'fFixedAspectRatio', 'title': 'True if fixed aspect ratio'}, 'fWNDC': {'title': 'Width of pad along X in Normalized Coordinates (NDC)', 'name': 'fWNDC'}, 'fHNDC': {'name': 'fHNDC', 'title': 'Height of pad along Y in Normalized Coordinates (NDC)'}, 'fNumber': {'title': 'pad number identifier', 'name': 'fNumber'}, 'fCrosshairPos': {'name': 'fCrosshairPos', 'title': 'Position of crosshair'}, 'fAbsYlowNDC': {'title': 'Absolute Y top left corner of pad in NDC [0,1]', 'name': 'fAbsYlowNDC'}, 'fModified': {'title': 'Set to true when pad is modified', 'name': 'fModified'}, 'fPadPaint': {'name': 'fPadPaint', 'title': 'Set to 1 while painting the pad'}, 'fEditable': {'name': 'fEditable', 'title': 'True if canvas is editable'}, 'fYtoPixel': {'title': 'ypixel = fYtoPixelk + fYtoPixel*yworld', 'name': 'fYtoPixel'}, 'fLogx': {'name': 'fLogx', 'title': '(=0 if X linear scale, =1 if log scale)'}, 'fYlowNDC': {'name': 'fYlowNDC', 'title': 'Y bottom left corner of pad in NDC [0,1]'}, 'fX2': {'name': 'fX2', 'title': 'X of upper X coordinate'}, 'fBorderMode': {'name': 'fBorderMode', 'title': 'Bordermode (-1=down, 0 = no border, 1=up)'}, 'fUtoAbsPixelk': {'name': 'fUtoAbsPixelk', 'title': 'Conversion coefficient for U NDC to absolute pixel'}, 'fAbsXlowNDC': {'name': 'fAbsXlowNDC', 'title': 'Absolute X top left corner of pad in NDC [0,1]'}, 'TAttBBox2D': {'title': '2D bounding box attributes', 'name': 'TAttBBox2D'}, 'fVtoPixel': {'name': 'fVtoPixel', 'title': 'ypixel = fVtoPixelk + fVtoPixel*vndc'}, 'fLogz': {'title': '(=0 if Z linear scale, =1 if log scale)', 'name': 'fLogz'}, 'fUtoPixelk': {'name': 'fUtoPixelk', 'title': 'Conversion coefficient for U NDC to pixel'}, 'fVtoAbsPixelk': {'name': 'fVtoAbsPixelk', 'title': 'Conversion coefficient for V NDC to absolute pixel'}, 'fAbsPixeltoYk': {'title': 'Conversion coefficient for absolute pixel to Y World', 'name': 'fAbsPixeltoYk'}, 'fPhi': {'title': 'phi angle   to view as lego/surface', 'name': 'fPhi'}, 'f_name': {'title': 'Pad name', 'name': 'f_name'}, 'fPixeltoY': {'name': 'fPixeltoY', 'title': 'yworld = fPixeltoYk + fPixeltoY*ypixel'}, 'fPixeltoYk': {'name': 'fPixeltoYk', 'title': 'Conversion coefficient for pixel to Y World'}, 'fTickx': {'name': 'fTickx', 'title': 'Set to 1 if tick marks along X'}, 'fAspectRatio': {'title': 'ratio of w/h in case of fixed ratio', 'name': 'fAspectRatio'}, 'fY2': {'title': 'Y of upper Y coordinate', 'name': 'fY2'}, 'fXlowNDC': {'name': 'fXlowNDC', 'title': 'X bottom left corner of pad in NDC [0,1]'}, 'fTheta': {'title': 'theta angle to view as lego/surface', 'name': 'fTheta'}, 'fYUpNDC': {'name': 'fYUpNDC', 'title': ''}, 'fYtoPixelk': {'name': 'fYtoPixelk', 'title': 'Conversion coefficient for Y World to pixel'}, 'fVtoPixelk': {'title': 'Conversion coefficient for V NDC to pixel', 'name': 'fVtoPixelk'}, 'fUymin': {'title': 'Minimum value on the Y axis', 'name': 'fUymin'}, 'fCrosshair': {'title': 'Crosshair type (0 if no crosshair requested)', 'name': 'fCrosshair'}}, 'properties': {'f_name': 'TPad'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Abstract base class for Pads and Canvases");
    he.insert("TVirtualPad", h);
    let mut h = HashMap::new();
    h.insert("title", "Next automatic color");
    he.insert("fNextPaletteColor", h);
    let mut h = HashMap::new();
    h.insert("title", "Conversion coefficient for X World to pixel");
    he.insert("fXtoPixelk", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Conversion coefficient for X World to absolute pixel",
    );
    he.insert("fXtoAbsPixelk", h);
    let mut h = HashMap::new();
    h.insert("title", "Conversion coefficient for pixel to X World");
    he.insert("fPixeltoXk", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of objects with an automatic color");
    he.insert("fNumPaletteColor", h);
    let mut h = HashMap::new();
    h.insert("title", "xpixel = fXtoPixelk + fXtoPixel*xworld");
    he.insert("fXtoPixel", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value on the X axis");
    he.insert("fUxmin", h);
    let mut h = HashMap::new();
    h.insert("title", "Y of lower Y coordinate");
    he.insert("fY1", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "List of commands to be executed when a pad event occurs",
    );
    he.insert("fExecs", h);
    let mut h = HashMap::new();
    h.insert("title", "(=0 if Y linear scale, =1 if log scale)");
    he.insert("fLogy", h);
    let mut h = HashMap::new();
    h.insert("title", "->List of primitives (subpads)");
    he.insert("fPrimitives", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fXUpNDC", h);
    let mut h = HashMap::new();
    h.insert("title", "X of lower X coordinate");
    he.insert("fX1", h);
    let mut h = HashMap::new();
    h.insert("title", "Pad title");
    he.insert("fTitle", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Conversion coefficient for absolute pixel to X World",
    );
    he.insert("fAbsPixeltoXk", h);
    let mut h = HashMap::new();
    h.insert("title", "Absolute Height of pad along Y in NDC");
    he.insert("fAbsHNDC", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum value on the Y axis");
    he.insert("fUymax", h);
    let mut h = HashMap::new();
    h.insert("title", "Absolute Width of pad along X in NDC");
    he.insert("fAbsWNDC", h);
    let mut h = HashMap::new();
    h.insert("title", "pad bordersize in pixels");
    he.insert("fBorderSize", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Conversion coefficient for Y World to absolute pixel",
    );
    he.insert("fYtoAbsPixelk", h);
    let mut h = HashMap::new();
    h.insert("title", "Set to true if grid along Y");
    he.insert("fGridy", h);
    let mut h = HashMap::new();
    h.insert("title", "xworld = fPixeltoXk + fPixeltoX*xpixel");
    he.insert("fPixeltoX", h);
    let mut h = HashMap::new();
    h.insert("title", "Use absolute coordinates");
    he.insert("fAbsCoord", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum value on the X axis");
    he.insert("fUxmax", h);
    let mut h = HashMap::new();
    h.insert("title", "Set to true if grid along X");
    he.insert("fGridx", h);
    let mut h = HashMap::new();
    h.insert("title", "xpixel = fUtoPixelk + fUtoPixel*undc");
    he.insert("fUtoPixel", h);
    let mut h = HashMap::new();
    h.insert("title", "Set to 1 if tick marks along Y");
    he.insert("fTicky", h);
    let mut h = HashMap::new();
    h.insert("title", "True if fixed aspect ratio");
    he.insert("fFixedAspectRatio", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Width of pad along X in Normalized Coordinates (NDC)",
    );
    he.insert("fWNDC", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Height of pad along Y in Normalized Coordinates (NDC)",
    );
    he.insert("fHNDC", h);
    let mut h = HashMap::new();
    h.insert("title", "pad number identifier");
    he.insert("fNumber", h);
    let mut h = HashMap::new();
    h.insert("title", "Position of crosshair");
    he.insert("fCrosshairPos", h);
    let mut h = HashMap::new();
    h.insert("title", "Absolute Y top left corner of pad in NDC [0,1]");
    he.insert("fAbsYlowNDC", h);
    let mut h = HashMap::new();
    h.insert("title", "Set to true when pad is modified");
    he.insert("fModified", h);
    let mut h = HashMap::new();
    h.insert("title", "Set to 1 while painting the pad");
    he.insert("fPadPaint", h);
    let mut h = HashMap::new();
    h.insert("title", "True if canvas is editable");
    he.insert("fEditable", h);
    let mut h = HashMap::new();
    h.insert("title", "ypixel = fYtoPixelk + fYtoPixel*yworld");
    he.insert("fYtoPixel", h);
    let mut h = HashMap::new();
    h.insert("title", "(=0 if X linear scale, =1 if log scale)");
    he.insert("fLogx", h);
    let mut h = HashMap::new();
    h.insert("title", "Y bottom left corner of pad in NDC [0,1]");
    he.insert("fYlowNDC", h);
    let mut h = HashMap::new();
    h.insert("title", "X of upper X coordinate");
    he.insert("fX2", h);
    let mut h = HashMap::new();
    h.insert("title", "Bordermode (-1=down, 0 = no border, 1=up)");
    he.insert("fBorderMode", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Conversion coefficient for U NDC to absolute pixel",
    );
    he.insert("fUtoAbsPixelk", h);
    let mut h = HashMap::new();
    h.insert("title", "Absolute X top left corner of pad in NDC [0,1]");
    he.insert("fAbsXlowNDC", h);
    let mut h = HashMap::new();
    h.insert("title", "2D bounding box attributes");
    he.insert("TAttBBox2D", h);
    let mut h = HashMap::new();
    h.insert("title", "ypixel = fVtoPixelk + fVtoPixel*vndc");
    he.insert("fVtoPixel", h);
    let mut h = HashMap::new();
    h.insert("title", "(=0 if Z linear scale, =1 if log scale)");
    he.insert("fLogz", h);
    let mut h = HashMap::new();
    h.insert("title", "Conversion coefficient for U NDC to pixel");
    he.insert("fUtoPixelk", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Conversion coefficient for V NDC to absolute pixel",
    );
    he.insert("fVtoAbsPixelk", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Conversion coefficient for absolute pixel to Y World",
    );
    he.insert("fAbsPixeltoYk", h);
    let mut h = HashMap::new();
    h.insert("title", "phi angle   to view as lego/surface");
    he.insert("fPhi", h);
    let mut h = HashMap::new();
    h.insert("title", "Pad name");
    he.insert("f_name", h);
    let mut h = HashMap::new();
    h.insert("title", "yworld = fPixeltoYk + fPixeltoY*ypixel");
    he.insert("fPixeltoY", h);
    let mut h = HashMap::new();
    h.insert("title", "Conversion coefficient for pixel to Y World");
    he.insert("fPixeltoYk", h);
    let mut h = HashMap::new();
    h.insert("title", "Set to 1 if tick marks along X");
    he.insert("fTickx", h);
    let mut h = HashMap::new();
    h.insert("title", "ratio of w/h in case of fixed ratio");
    he.insert("fAspectRatio", h);
    let mut h = HashMap::new();
    h.insert("title", "Y of upper Y coordinate");
    he.insert("fY2", h);
    let mut h = HashMap::new();
    h.insert("title", "X bottom left corner of pad in NDC [0,1]");
    he.insert("fXlowNDC", h);
    let mut h = HashMap::new();
    h.insert("title", "theta angle to view as lego/surface");
    he.insert("fTheta", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fYUpNDC", h);
    let mut h = HashMap::new();
    h.insert("title", "Conversion coefficient for Y World to pixel");
    he.insert("fYtoPixelk", h);
    let mut h = HashMap::new();
    h.insert("title", "Conversion coefficient for V NDC to pixel");
    he.insert("fVtoPixelk", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value on the Y axis");
    he.insert("fUymin", h);
    let mut h = HashMap::new();
    h.insert("title", "Crosshair type (0 if no crosshair requested)");
    he.insert("fCrosshair", h);
    hh.insert("TPad", he);
    // TAttLine
    //si =  {'elements': {'fLineColor': {'title': 'Line color', 'name': 'fLineColor'}, 'fLineStyle': {'name': 'fLineStyle', 'title': 'Line style'}, 'fLineWidth': {'name': 'fLineWidth', 'title': 'Line width'}}, 'properties': {'f_name': 'TAttLine'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Line color");
    he.insert("fLineColor", h);
    let mut h = HashMap::new();
    h.insert("title", "Line style");
    he.insert("fLineStyle", h);
    let mut h = HashMap::new();
    h.insert("title", "Line width");
    he.insert("fLineWidth", h);
    hh.insert("TAttLine", he);
    // TLimit
    //si =  {'properties': {'f_name': 'TLimit'}}
    let he = HashMap::new();
    hh.insert("TLimit", he);
    // TBranchRef
    //si =  {'elements': {'fRefTable': {'name': 'fRefTable', 'title': 'pointer to the TRefTable'}, 'TBranch': {'title': 'Branch descriptor', 'name': 'TBranch'}}, 'properties': {'f_name': 'TBranchRef'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "pointer to the TRefTable");
    he.insert("fRefTable", h);
    let mut h = HashMap::new();
    h.insert("title", "Branch descriptor");
    he.insert("TBranch", h);
    hh.insert("TBranchRef", he);
    // TLeafD32
    //si =  {'elements': {'fMaximum': {'name': 'fMaximum', 'title': 'Maximum value if leaf range is specified'}, 'fMinimum': {'name': 'fMinimum', 'title': 'Minimum value if leaf range is specified'}, 'TLeaf': {'title': 'Leaf: description of a Branch data type', 'name': 'TLeaf'}}, 'properties': {'f_name': 'TLeafD32'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Maximum value if leaf range is specified");
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value if leaf range is specified");
    he.insert("fMinimum", h);
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    hh.insert("TLeafD32", he);
    // TSeqCollection
    //si =  {'elements': {'TCollection': {'name': 'TCollection', 'title': 'Collection abstract base class'}}, 'properties': {'f_name': 'TSeqCollection'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Collection abstract base class");
    he.insert("TCollection", h);
    hh.insert("TSeqCollection", he);
    // TDirectoryFile
    //si =  {'properties': {'f_name': 'TDirectoryFile'}, 'elements': {'fModified': {'name': 'fModified', 'title': 'True if directory has been modified'}, 'fSeekDir': {'title': 'Location of directory on file', 'name': 'fSeekDir'}, 'fDatimeM': {'name': 'fDatimeM', 'title': 'Date and time of last modification'}, 'fSeekParent': {'name': 'fSeekParent', 'title': 'Location of parent directory on file'}, 'fNbytesKeys': {'name': 'fNbytesKeys', 'title': 'Number of bytes for the keys'}, 'fDatimeC': {'title': 'Date and time when directory is created', 'name': 'fDatimeC'}, 'fSeekKeys': {'name': 'fSeekKeys', 'title': 'Location of Keys record on file'}, 'fKeys': {'name': 'fKeys', 'title': 'Pointer to keys list in memory'}, 'TDirectory': {'name': 'TDirectory', 'title': 'Describe directory structure in memory'}, 'fFile': {'name': 'fFile', 'title': 'Pointer to current file in memory'}, 'fBufferSize': {'name': 'fBufferSize', 'title': 'Default buffer size to create new TKeys'}, 'fWritable': {'name': 'fWritable', 'title': 'True if directory is writable'}, 'fNbytesName': {'name': 'fNbytesName', 'title': 'Number of bytes in TNamed at creation time'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "True if directory has been modified");
    he.insert("fModified", h);
    let mut h = HashMap::new();
    h.insert("title", "Location of directory on file");
    he.insert("fSeekDir", h);
    let mut h = HashMap::new();
    h.insert("title", "Date and time of last modification");
    he.insert("fDatimeM", h);
    let mut h = HashMap::new();
    h.insert("title", "Location of parent directory on file");
    he.insert("fSeekParent", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bytes for the keys");
    he.insert("fNbytesKeys", h);
    let mut h = HashMap::new();
    h.insert("title", "Date and time when directory is created");
    he.insert("fDatimeC", h);
    let mut h = HashMap::new();
    h.insert("title", "Location of Keys record on file");
    he.insert("fSeekKeys", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to keys list in memory");
    he.insert("fKeys", h);
    let mut h = HashMap::new();
    h.insert("title", "Describe directory structure in memory");
    he.insert("TDirectory", h);
    let mut h = HashMap::new();
    h.insert("title", "Pointer to current file in memory");
    he.insert("fFile", h);
    let mut h = HashMap::new();
    h.insert("title", "Default buffer size to create new TKeys");
    he.insert("fBufferSize", h);
    let mut h = HashMap::new();
    h.insert("title", "True if directory is writable");
    he.insert("fWritable", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of bytes in TNamed at creation time");
    he.insert("fNbytesName", h);
    hh.insert("TDirectoryFile", he);
    // ROOT
    //si =  [':TIOFeatures.properties.f_name:ROOT::TIOFeatures', ':TIOFeatures.elements.fIOBits.name:fIOBits', ':TIOFeatures.elements.fIOBits.title:']
    // TLeafF
    //si =  {'elements': {'TLeaf': {'name': 'TLeaf', 'title': 'Leaf: description of a Branch data type'}, 'fMaximum': {'name': 'fMaximum', 'title': 'Maximum value if leaf range is specified'}, 'fMinimum': {'title': 'Minimum value if leaf range is specified', 'name': 'fMinimum'}}, 'properties': {'f_name': 'TLeafF'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum value if leaf range is specified");
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value if leaf range is specified");
    he.insert("fMinimum", h);
    hh.insert("TLeafF", he);
    // TArrayF
    //si =  {'properties': {'f_name': 'TArrayF'}, 'elements': {'TArray': {'title': 'Abstract array base class', 'name': 'TArray'}, 'fArray': {'title': '[fN] Array of fN floats', 'name': 'fArray'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Abstract array base class");
    he.insert("TArray", h);
    let mut h = HashMap::new();
    h.insert("title", "[fN] Array of fN floats");
    he.insert("fArray", h);
    hh.insert("TArrayF", he);
    // TF1Convolution
    //si =  {'properties': {'f_name': 'TF1Convolution'}, 'elements': {'fNofParams2': {'name': 'fNofParams2', 'title': ''}, 'fCstIndex': {'title': 'Index of the constant parameter f the first function', 'name': 'fCstIndex'}, 'fXmin': {'title': 'Minimal bound of the range of the convolution', 'name': 'fXmin'}, 'fNofPoints': {'name': 'fNofPoints', 'title': 'Number of point for FFT array'}, 'fFlagFFT': {'title': 'Choose FFT or numerical convolution', 'name': 'fFlagFFT'}, 'fParams2': {'title': '', 'name': 'fParams2'}, 'TF1AbsComposition': {'name': 'TF1AbsComposition', 'title': ''}, 'fFunction1': {'name': 'fFunction1', 'title': 'First function to be convolved'}, 'fParNames': {'name': 'fParNames', 'title': "Parameters' names"}, 'fParams1': {'title': '', 'name': 'fParams1'}, 'fFunction2': {'name': 'fFunction2', 'title': 'Second function to be convolved'}, 'fXmax': {'name': 'fXmax', 'title': 'Maximal bound of the range of the convolution'}, 'fNofParams1': {'name': 'fNofParams1', 'title': ''}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fNofParams2", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Index of the constant parameter f the first function",
    );
    he.insert("fCstIndex", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimal bound of the range of the convolution");
    he.insert("fXmin", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of point for FFT array");
    he.insert("fNofPoints", h);
    let mut h = HashMap::new();
    h.insert("title", "Choose FFT or numerical convolution");
    he.insert("fFlagFFT", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fParams2", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("TF1AbsComposition", h);
    let mut h = HashMap::new();
    h.insert("title", "First function to be convolved");
    he.insert("fFunction1", h);
    let mut h = HashMap::new();
    h.insert("title", "Parameters' names");
    he.insert("fParNames", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fParams1", h);
    let mut h = HashMap::new();
    h.insert("title", "Second function to be convolved");
    he.insert("fFunction2", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximal bound of the range of the convolution");
    he.insert("fXmax", h);
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fNofParams1", h);
    hh.insert("TF1Convolution", he);
    // TAttAxis
    //si =  {'properties': {'f_name': 'TAttAxis'}, 'elements': {'fLabelFont': {'name': 'fLabelFont', 'title': 'Font for labels'}, 'fTickLength': {'title': 'Length of tick marks', 'name': 'fTickLength'}, 'fNdivisions': {'name': 'fNdivisions', 'title': 'Number of divisions(10000*n3 + 100*n2 + n1)'}, 'fTitleColor': {'name': 'fTitleColor', 'title': 'Color of axis title'}, 'fLabelOffset': {'title': 'Offset of labels', 'name': 'fLabelOffset'}, 'fTitleFont': {'name': 'fTitleFont', 'title': 'Font for axis title'}, 'fAxisColor': {'title': 'Color of the line axis', 'name': 'fAxisColor'}, 'fLabelColor': {'name': 'fLabelColor', 'title': 'Color of labels'}, 'fLabelSize': {'name': 'fLabelSize', 'title': 'Size of labels'}, 'fTitleOffset': {'name': 'fTitleOffset', 'title': 'Offset of axis title'}, 'fTitleSize': {'name': 'fTitleSize', 'title': 'Size of axis title'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Font for labels");
    he.insert("fLabelFont", h);
    let mut h = HashMap::new();
    h.insert("title", "Length of tick marks");
    he.insert("fTickLength", h);
    let mut h = HashMap::new();
    h.insert("title", "Number of divisions(10000*n3 + 100*n2 + n1)");
    he.insert("fNdivisions", h);
    let mut h = HashMap::new();
    h.insert("title", "Color of axis title");
    he.insert("fTitleColor", h);
    let mut h = HashMap::new();
    h.insert("title", "Offset of labels");
    he.insert("fLabelOffset", h);
    let mut h = HashMap::new();
    h.insert("title", "Font for axis title");
    he.insert("fTitleFont", h);
    let mut h = HashMap::new();
    h.insert("title", "Color of the line axis");
    he.insert("fAxisColor", h);
    let mut h = HashMap::new();
    h.insert("title", "Color of labels");
    he.insert("fLabelColor", h);
    let mut h = HashMap::new();
    h.insert("title", "Size of labels");
    he.insert("fLabelSize", h);
    let mut h = HashMap::new();
    h.insert("title", "Offset of axis title");
    he.insert("fTitleOffset", h);
    let mut h = HashMap::new();
    h.insert("title", "Size of axis title");
    he.insert("fTitleSize", h);
    hh.insert("TAttAxis", he);
    // TLeafD
    //si =  {'elements': {'fMaximum': {'title': 'Maximum value if leaf range is specified', 'name': 'fMaximum'}, 'TLeaf': {'title': 'Leaf: description of a Branch data type', 'name': 'TLeaf'}, 'fMinimum': {'title': 'Minimum value if leaf range is specified', 'name': 'fMinimum'}}, 'properties': {'f_name': 'TLeafD'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Maximum value if leaf range is specified");
    he.insert("fMaximum", h);
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    let mut h = HashMap::new();
    h.insert("title", "Minimum value if leaf range is specified");
    he.insert("fMinimum", h);
    hh.insert("TLeafD", he);
    // TStreamerObjectAnyPointer
    //si =  {'elements': {'TStreamerElement': {'title': 'Base class for one element (data member) to be Streamed', 'name': 'TStreamerElement'}}, 'properties': {'f_name': 'TStreamerObjectAnyPointer'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Base class for one element (data member) to be Streamed",
    );
    he.insert("TStreamerElement", h);
    hh.insert("TStreamerObjectAnyPointer", he);
    // TLeafG
    //si =  {'elements': {'fMinimum': {'title': 'Minimum value if leaf range is specified', 'name': 'fMinimum'}, 'TLeaf': {'name': 'TLeaf', 'title': 'Leaf: description of a Branch data type'}, 'fMaximum': {'name': 'fMaximum', 'title': 'Maximum value if leaf range is specified'}}, 'properties': {'f_name': 'TLeafG'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Minimum value if leaf range is specified");
    he.insert("fMinimum", h);
    let mut h = HashMap::new();
    h.insert("title", "Leaf: description of a Branch data type");
    he.insert("TLeaf", h);
    let mut h = HashMap::new();
    h.insert("title", "Maximum value if leaf range is specified");
    he.insert("fMaximum", h);
    hh.insert("TLeafG", he);
    // TEfficiency
    //si =  {'elements': {'TAttLine': {'name': 'TAttLine', 'title': 'Line attributes'}, 'fWeight': {'name': 'fWeight', 'title': 'Weight for all events (default = 1)'}, 'fBeta_beta': {'name': 'fBeta_beta', 'title': 'Global parameter for prior beta distribution (default = 1)'}, 'fBeta_bin_params': {'name': 'fBeta_bin_params', 'title': 'Parameter for prior beta distribution different bin by bin'}, 'fStatisticOption': {'name': 'fStatisticOption', 'title': 'Defines how the confidence intervals are determined'}, 'TNamed': {'name': 'TNamed', 'title': 'The basis for a named object (name, title)'}, 'fConfLevel': {'title': 'Confidence level (default = 0.683, 1 sigma)', 'name': 'fConfLevel'}, 'fTotalHistogram': {'title': 'Histogram for total number of events', 'name': 'fTotalHistogram'}, 'fPassedHistogram': {'title': 'Histogram for events which passed certain criteria', 'name': 'fPassedHistogram'}, 'TAttFill': {'title': 'Fill area attributes', 'name': 'TAttFill'}, 'TAttMarker': {'title': 'Marker attributes', 'name': 'TAttMarker'}, 'fFunctions': {'name': 'fFunctions', 'title': '->Pointer to list of functions'}, 'fBeta_alpha': {'title': 'Global parameter for prior beta distribution (default = 1)', 'name': 'fBeta_alpha'}}, 'properties': {'f_name': 'TEfficiency'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Line attributes");
    he.insert("TAttLine", h);
    let mut h = HashMap::new();
    h.insert("title", "Weight for all events (default = 1)");
    he.insert("fWeight", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Global parameter for prior beta distribution (default = 1)",
    );
    he.insert("fBeta_beta", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Parameter for prior beta distribution different bin by bin",
    );
    he.insert("fBeta_bin_params", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Defines how the confidence intervals are determined",
    );
    he.insert("fStatisticOption", h);
    let mut h = HashMap::new();
    h.insert("title", "The basis for a named object (name, title)");
    he.insert("TNamed", h);
    let mut h = HashMap::new();
    h.insert("title", "Confidence level (default = 0.683, 1 sigma)");
    he.insert("fConfLevel", h);
    let mut h = HashMap::new();
    h.insert("title", "Histogram for total number of events");
    he.insert("fTotalHistogram", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Histogram for events which passed certain criteria",
    );
    he.insert("fPassedHistogram", h);
    let mut h = HashMap::new();
    h.insert("title", "Fill area attributes");
    he.insert("TAttFill", h);
    let mut h = HashMap::new();
    h.insert("title", "Marker attributes");
    he.insert("TAttMarker", h);
    let mut h = HashMap::new();
    h.insert("title", "->Pointer to list of functions");
    he.insert("fFunctions", h);
    let mut h = HashMap::new();
    h.insert(
        "title",
        "Global parameter for prior beta distribution (default = 1)",
    );
    he.insert("fBeta_alpha", h);
    hh.insert("TEfficiency", he);
    // TVector2
    //si =  {'properties': {'f_name': 'TVector2'}, 'elements': {'fY': {'name': 'fY', 'title': ''}, 'TObject': {'name': 'TObject', 'title': 'Basic ROOT object'}, 'fX': {'name': 'fX', 'title': 'components of the vector'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "");
    he.insert("fY", h);
    let mut h = HashMap::new();
    h.insert("title", "Basic ROOT object");
    he.insert("TObject", h);
    let mut h = HashMap::new();
    h.insert("title", "components of the vector");
    he.insert("fX", h);
    hh.insert("TVector2", he);
    // TArray
    //si =  {'elements': {'fN': {'name': 'fN', 'title': 'Number of array elements'}}, 'properties': {'f_name': 'TArray'}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Number of array elements");
    he.insert("fN", h);
    hh.insert("TArray", he);
    // TH2I
    //si =  {'properties': {'f_name': 'TH2I'}, 'elements': {'TArrayI': {'name': 'TArrayI', 'title': 'Array of ints'}, 'TH2': {'name': 'TH2', 'title': '2-Dim histogram base class'}}}
    let mut he = HashMap::new();
    let mut h = HashMap::new();
    h.insert("title", "Array of ints");
    he.insert("TArrayI", h);
    let mut h = HashMap::new();
    h.insert("title", "2-Dim histogram base class");
    he.insert("TH2", h);
    hh.insert("TH2I", he);
    hh
}
