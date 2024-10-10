use lazy_static::lazy_static;
use log::trace;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::str::from_utf8;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"==> Dumping object at: 0x[a-z0-9]+, name=(?<name>[A-Za-z0-9_:]+), class=(?<class>[A-Za-z0-9]+)"
    )
    .unwrap();

}

#[derive(Debug, Deserialize)]
struct ElementTitles {
    title: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct StreamerInfoTitles {
    name: String,
    title: String,
    elements: Option<HashMap<String, ElementTitles>>,
}

#[derive(Debug, Deserialize)]
struct StreamerInfosTitles {
    #[serde(flatten)]
    data: HashMap<String, StreamerInfoTitles>,
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

#[derive(Debug, Default)]
pub(crate) struct ClassStrings {
    pub(crate) class: String,
    pub(crate) fName: String,
    pub(crate) fCheckSum: u32,
    pub(crate) fClassVersion: i32,
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

fn generate_class(dump: &str) -> anyhow::Result<Vec<ClassStreamerStrings>> {
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
                WhatIAmReading::Trailer => return Err(anyhow::bail!("unexpected line: {}", line)),
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

fn main() {
    trace!(";populate_db.call:{}", true);

    let content_titles = fs::read_to_string("/tmp/oxyroot.json").unwrap();
    let infos_titles: StreamerInfosTitles = serde_json::from_str(&content_titles).unwrap();

    let dump = fs::read_to_string("/tmp/rust/gen_streamers/gen_all.txt").unwrap();

    // let titles = make_titles();
    let classes_str = generate_class(&dump).unwrap();

    let mut populate_db = Vec::new();

    writeln!(populate_db, "let mut id_elements = 0;").unwrap();

    for class_str in classes_str {
        let class = &class_str.class;
        println!(";populate_db.class_name:{}", class.fName);

        write!(populate_db, r#"// Streamer {}"#, class_str.class.fName).unwrap();

        let stitle = infos_titles.data.get(&class.fName).unwrap();

        write!(
            populate_db,
            r#"
    let class = ClassStrings {{
        class: "{}",
        title: "{}",
        fName: "{}",
        fCheckSum: {},
        fClassVersion: {},
    }};

    "#,
            class.class, stitle.title, class.fName, class.fCheckSum, class.fClassVersion
        )
        .unwrap();

        write!(
            populate_db,
            r"
        let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

        "
        )
        .unwrap();

        // let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

        for element_str in class_str.elements {
            write!(populate_db, r#"//\t element {}"#, element_str.f_name).unwrap();
            let etitle = stitle
                .elements
                .as_ref()
                .unwrap()
                .get(&element_str.f_name)
                .unwrap();
            write!(
                populate_db,
                r#"
                    #[allow(unused_mut)]
                    let mut element_str = ElementStrings {{
                        class: "{}",
                        f_name: "{}",
                        fTitle: "{}",
                        fSize: {},
                        fType: {},
                        fTypeName: "{}",
                        fBaseVersion: None,
                        fCountName: None,
                        fCountClass: None,
                        fCountVersion: None,
                        fSTLtype: None,
                    }};

                    "#,
                element_str.class,
                element_str.f_name,
                etitle.title.replace("\"", "\\\""),
                element_str.fSize,
                element_str.fType,
                element_str.fTypeName,
            )
            .unwrap();

            if let Some(fBaseVersion) = element_str.fBaseVersion {
                writeln!(
                    populate_db,
                    "element_str.fBaseVersion = Some({});",
                    fBaseVersion
                )
                .unwrap();
            }

            if let Some(fCountName) = element_str.fCountName {
                writeln!(
                    populate_db,
                    "element_str.fCountName = Some(\"{}\");",
                    fCountName
                )
                .unwrap();
            }
            if let Some(fCountClass) = element_str.fCountClass {
                writeln!(
                    populate_db,
                    "element_str.fCountClass = Some(\"{}\");",
                    fCountClass
                )
                .unwrap();
            }

            if let Some(fCountVersion) = element_str.fCountVersion {
                writeln!(
                    populate_db,
                    "element_str.fCountVersion = Some({});",
                    fCountVersion
                )
                .unwrap();
            }

            if let Some(fSTLtype) = element_str.fSTLtype {
                writeln!(populate_db, "element_str.fSTLtype = Some({});", fSTLtype).unwrap();
            }

            write!(
                populate_db,
                r#"            let mut streamer_element = StreamerElement::new(
                element_str.name(),
                element_str.etype(),
                element_str.esize(),
                id_elements,
            );

            streamer_element.named =
                            streamer_element.named.with_title("{}".to_string());
            id_elements += 1;
            streamer_element.ename = element_str.fTypeName.to_string();

            matches!(streamer_element.etype, Enum::Named(_));

            let streamer = element_str.build_streamer(streamer_element);
            streamer_info.elems.push(streamer);

            "#,
                element_str.fTitle.replace("\"", "\\\"")
            )
            .unwrap();
        }

        write!(
            populate_db,
            r"
        streamer_info.id = id_elements;
        db.insert(streamer_info);

        "
        )
        .unwrap();
    }

    let populate_db = from_utf8(&populate_db).unwrap();

    let content = format!(
        r#"
// Automatically generated. DO NOT EDIT.

use crate::rdict::{{StreamerElement,StreamerInfo}};
use crate::rdict::streamers::db::DbStreamer;
use crate::rdict::streamers::streamers_db_gen_helpers::{{ClassStrings, ElementStrings}};
use crate::rmeta::Enum;

pub fn populate_db(db: &mut DbStreamer) -> crate::rdict::error::Result<()> {{
    {populate_db}
    Ok(())
}}

"#
    );
    fs::write("/tmp/oxyroot.rs", content).unwrap();
}
