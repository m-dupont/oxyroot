use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::str::from_utf8;

#[derive(Debug, Deserialize)]
struct Element {
    title: String,
    fSize: i32,
    fType: i32,
    fTypeName: String,
    class: String,
    name: String,
    fBaseVersion: Option<i32>,
    fCountName: Option<String>,
    fCountClass: Option<String>,
    fCountVersion: Option<i32>,
    fSTLtype: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct StreamerInfo {
    name: String,
    fClassVersion: i32,
    fCheckSum: u32,
    class: String,
    elements: Option<HashMap<String, Element>>,
}

#[derive(Debug, Deserialize)]
struct StreamerInfos {
    #[serde(flatten)]
    data: HashMap<String, StreamerInfo>,
}

fn main() {
    let content = fs::read_to_string("/tmp/oxyroot.json").unwrap();
    // println!("{:#?}", content);
    let info: StreamerInfos = serde_json::from_str(&content).unwrap();
    // println!("{:#?}", info);

    let mut populate_db = Vec::new();

    writeln!(populate_db, "let mut id_elements = 0;").unwrap();

    for (_, streamer) in info.data {
        // println!("{:#?}", streamer);

        write!(
            populate_db,
            r#"
    let class = ClassStrings {{
        class: "{}".to_string(),
        fName: "{}".to_string(),
        fCheckSum: {},
        fClassVersion: {},
    }};

        "#,
            streamer.class, streamer.name, streamer.fCheckSum, streamer.fClassVersion
        )
        .unwrap();

        write!(
            populate_db,
            r"
        let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

        "
        )
        .unwrap();

        if let Some(elements) = streamer.elements {
            for (_, element) in elements {
                println!("{:?}", element);
                write!(
                    populate_db,
                    r#"
                    #[allow(unused_mut)]
                    let mut element_str = ElementStrings {{
                        class: "{}".to_string(),
                        f_name: "{}".to_string(),
                        fTitle: "{}".to_string(),
                        fSize: {},
                        fType: {},
                        fTypeName: "{}".to_string(),
                        fBaseVersion: None,
                        fCountName: None,
                        fCountClass: None,
                        fCountVersion: None,
                        fSTLtype: None,
                    }};

                    "#,
                    element.class,
                    element.name,
                    element.title.replace("\"", "\\\""),
                    element.fSize,
                    element.fType,
                    element.fTypeName,
                )
                .unwrap();

                if let Some(fBaseVersion) = element.fBaseVersion {
                    writeln!(
                        populate_db,
                        "element_str.fBaseVersion = Some({});",
                        fBaseVersion
                    )
                    .unwrap();
                }

                if let Some(fCountName) = element.fCountName {
                    writeln!(
                        populate_db,
                        "element_str.fCountName = Some(\"{}\".to_string());",
                        fCountName
                    )
                    .unwrap();
                }
                if let Some(fCountClass) = element.fCountClass {
                    writeln!(
                        populate_db,
                        "element_str.fCountClass = Some(\"{}\".to_string());",
                        fCountClass
                    )
                    .unwrap();
                }

                if let Some(fCountVersion) = element.fCountVersion {
                    writeln!(
                        populate_db,
                        "element_str.fCountVersion = Some({});",
                        fCountVersion
                    )
                    .unwrap();
                }

                if let Some(fSTLtype) = element.fSTLtype {
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
            streamer_element.ename = element_str.fTypeName.clone();

            matches!(streamer_element.etype, Enum::Named(_));

            let streamer = element_str.build_streamer(streamer_element);
            streamer_info.elems.push(streamer);

            "#,
                    element.title.replace("\"", "\\\"")
                )
                .unwrap();
            }
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
