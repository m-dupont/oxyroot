// Automatically generated. DO NOT EDIT.

use crate::rdict::streamers::db::DbStreamer;
use crate::rdict::streamers::streamers_db_gen_helpers::{ClassStrings, ElementStrings};
use crate::rdict::{StreamerElement, StreamerInfo};
use crate::rmeta::Enum;

pub fn populate_db(db: &mut DbStreamer) -> crate::rdict::error::Result<()> {
    let mut id_elements = 0;
    // Streamer TAttAxis
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TAttAxis",
        fCheckSum: 1550843710,
        fClassVersion: 4,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element fNdivisions
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNdivisions",
        fTitle: "Number of divisions(10000*n3 + 100*n2 + n1)",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fAxisColor
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fAxisColor",
        fTitle: "Color of the line axis",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLabelColor
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLabelColor",
        fTitle: "Color of labels",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLabelFont
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLabelFont",
        fTitle: "Font for labels",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLabelOffset
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLabelOffset",
        fTitle: "Offset of labels",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLabelSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLabelSize",
        fTitle: "Size of labels",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTickLength
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTickLength",
        fTitle: "Length of tick marks",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTitleOffset
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTitleOffset",
        fTitle: "Offset of axis title",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTitleSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTitleSize",
        fTitle: "Size of axis title",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTitleColor
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTitleColor",
        fTitle: "Color of axis title",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTitleFont
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTitleFont",
        fTitle: "Font for axis title",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TAttBBox2D
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TAttBBox2D",
        fCheckSum: 2443772,
        fClassVersion: 0,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TAttFill
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TAttFill",
        fCheckSum: 4292422290,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element fFillColor
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fFillColor",
        fTitle: "Fill area color",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFillStyle
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fFillStyle",
        fTitle: "Fill area style",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TAttLine
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TAttLine",
        fCheckSum: 2483504457,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element fLineColor
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLineColor",
        fTitle: "Line color",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLineStyle
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLineStyle",
        fTitle: "Line style",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLineWidth
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLineWidth",
        fTitle: "Line width",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TAttMarker
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TAttMarker",
        fCheckSum: 689802220,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element fMarkerColor
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMarkerColor",
        fTitle: "Marker color",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMarkerStyle
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMarkerStyle",
        fTitle: "Marker style",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMarkerSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMarkerSize",
        fTitle: "Marker size",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TAttPad
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TAttPad",
        fCheckSum: 2803232785,
        fClassVersion: 4,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element fLeftMargin
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLeftMargin",
        fTitle: "LeftMargin",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fRightMargin
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fRightMargin",
        fTitle: "RightMargin",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBottomMargin
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fBottomMargin",
        fTitle: "BottomMargin",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTopMargin
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTopMargin",
        fTitle: "TopMargin",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fXfile
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fXfile",
        fTitle: "X position where to draw the file name",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fYfile
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fYfile",
        fTitle: "Y position where to draw the file name",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fAfile
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fAfile",
        fTitle: "Alignment for the file name",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fXstat
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fXstat",
        fTitle: "X position where to draw the statistics",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fYstat
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fYstat",
        fTitle: "Y position where to draw the statistics",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fAstat
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fAstat",
        fTitle: "Alignment for the statistics",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFrameFillColor
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fFrameFillColor",
        fTitle: "Pad frame fill color",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFrameLineColor
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fFrameLineColor",
        fTitle: "Pad frame line color",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFrameFillStyle
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fFrameFillStyle",
        fTitle: "Pad frame fill style",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFrameLineStyle
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fFrameLineStyle",
        fTitle: "Pad frame line style",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFrameLineWidth
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fFrameLineWidth",
        fTitle: "Pad frame line width",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFrameBorderSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fFrameBorderSize",
        fTitle: "Pad frame border size",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFrameBorderMode
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fFrameBorderMode",
        fTitle: "Pad frame border mode",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TDatime
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TDatime",
        fCheckSum: 3024515566,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element fDatime
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fDatime",
        fTitle: "Date (relative to 1995) + time",
        fSize: 4,
        fType: 13,
        fTypeName: "unsigned int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TNamed
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TNamed",
        fCheckSum: 3753331260,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TObject
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TObject",
        fTitle: "Basic ROOT object",
        fSize: 0,
        fType: 66,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fName",
        fTitle: "object identifier",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTitle
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fTitle",
        fTitle: "object title",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TObject
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TObject",
        fCheckSum: 2417737773,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element fUniqueID
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fUniqueID",
        fTitle: "object unique identifier",
        fSize: 4,
        fType: 13,
        fTypeName: "unsigned int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBits
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fBits",
        fTitle: "bit field status word",
        fSize: 4,
        fType: 15,
        fTypeName: "unsigned int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TObjString
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TObjString",
        fCheckSum: 2626570240,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TObject
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TObject",
        fTitle: "Basic ROOT object",
        fSize: 0,
        fType: 66,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fString
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fString",
        fTitle: "wrapped TString",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TProcessID
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TProcessID",
        fCheckSum: 729740665,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TNamed
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TNamed",
        fTitle: "The basis for a named object (name, title)",
        fSize: 0,
        fType: 67,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TProcessUUID
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TProcessUUID",
        fCheckSum: 2599661073,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TProcessID
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TProcessID",
        fTitle: "Process Unique Identifier in time and space",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fUUIDs
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fUUIDs",
        fTitle: "Global list of TUUIDs",
        fSize: 8,
        fType: 64,
        fTypeName: "TList*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fActive
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fActive",
        fTitle: "Table of active UUIDs",
        fSize: 8,
        fType: 64,
        fTypeName: "TBits*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TQObject
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TQObject",
        fCheckSum: 274076,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TRef
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TRef",
        fCheckSum: 2440395009,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TObject
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TObject",
        fTitle: "Basic ROOT object",
        fSize: 0,
        fType: 66,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TString
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TString",
        fCheckSum: 95257,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TUUID
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TUUID",
        fCheckSum: 3528368475,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element fTimeLow
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTimeLow",
        fTitle: "60 bit time, lower 32 bits",
        fSize: 4,
        fType: 13,
        fTypeName: "unsigned int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTimeMid
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTimeMid",
        fTitle: "middle 16 time bits",
        fSize: 2,
        fType: 12,
        fTypeName: "unsigned short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTimeHiAndVersion
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTimeHiAndVersion",
        fTitle: "high 12 time bits + 4 UUID version bits",
        fSize: 2,
        fType: 12,
        fTypeName: "unsigned short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fClockSeqHiAndReserved
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fClockSeqHiAndReserved",
        fTitle: "high 6 clock bits + 2 bits reserved",
        fSize: 1,
        fType: 11,
        fTypeName: "unsigned char",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fClockSeqLow
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fClockSeqLow",
        fTitle: "low 8 clock bits",
        fSize: 1,
        fType: 11,
        fTypeName: "unsigned char",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNode
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNode",
        fTitle: "6 node id bytes",
        fSize: 6,
        fType: 31,
        fTypeName: "unsigned char",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TVirtualPad
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TVirtualPad",
        fCheckSum: 686614457,
        fClassVersion: 3,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TObject
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TObject",
        fTitle: "Basic ROOT object",
        fSize: 0,
        fType: 66,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element TAttLine
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TAttLine",
        fTitle: "Line attributes",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element TAttFill
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TAttFill",
        fTitle: "Fill area attributes",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element TAttPad
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TAttPad",
        fTitle: "Pad attributes",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element TQObject
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TQObject",
        fTitle: "Base class for object communication mechanism",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TArray
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TArray",
        fCheckSum: 7348658,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element fN
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fN",
        fTitle: "Number of array elements",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TArrayC
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TArrayC",
        fCheckSum: 2928122166,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TArray",
        fTitle: "Abstract array base class",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fArray",
        fTitle: "[fN] Array of fN chars",
        fSize: 1,
        fType: 41,
        fTypeName: "char*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fN");
    element_str.fCountClass = Some("TArray");
    element_str.fCountVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TArrayS
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TArrayS",
        fCheckSum: 56398612,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TArray",
        fTitle: "Abstract array base class",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fArray",
        fTitle: "[fN] Array of fN shorts",
        fSize: 2,
        fType: 42,
        fTypeName: "short*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fN");
    element_str.fCountClass = Some("TArray");
    element_str.fCountVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TArrayI
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TArrayI",
        fCheckSum: 3654644167,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TArray",
        fTitle: "Abstract array base class",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fArray",
        fTitle: "[fN] Array of fN 32 bit integers",
        fSize: 4,
        fType: 43,
        fTypeName: "int*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fN");
    element_str.fCountClass = Some("TArray");
    element_str.fCountVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TArrayL
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TArrayL",
        fCheckSum: 4244419721,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TArray",
        fTitle: "Abstract array base class",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fArray",
        fTitle: "[fN] Array of fN longs",
        fSize: 8,
        fType: 44,
        fTypeName: "long*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fN");
    element_str.fCountClass = Some("TArray");
    element_str.fCountVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TArrayL64
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TArrayL64",
        fCheckSum: 417793940,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TArray",
        fTitle: "Abstract array base class",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fArray",
        fTitle: "[fN] Array of fN long64s",
        fSize: 8,
        fType: 56,
        fTypeName: "Long64_t*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fN");
    element_str.fCountClass = Some("TArray");
    element_str.fCountVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TArrayF
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TArrayF",
        fCheckSum: 1510733553,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TArray",
        fTitle: "Abstract array base class",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fArray",
        fTitle: "[fN] Array of fN floats",
        fSize: 4,
        fType: 45,
        fTypeName: "float*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fN");
    element_str.fCountClass = Some("TArray");
    element_str.fCountVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TArrayD
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TArrayD",
        fCheckSum: 1899622196,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TArray",
        fTitle: "Abstract array base class",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fArray",
        fTitle: "[fN] Array of fN doubles",
        fSize: 8,
        fType: 48,
        fTypeName: "double*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fN");
    element_str.fCountClass = Some("TArray");
    element_str.fCountVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TBits
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TBits",
        fCheckSum: 242629704,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TObject
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TObject",
        fTitle: "Basic ROOT object",
        fSize: 0,
        fType: 66,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNbits
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNbits",
        fTitle: "Highest bit set + 1",
        fSize: 4,
        fType: 13,
        fTypeName: "unsigned int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNbytes
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNbytes",
        fTitle: "Number of UChars in fAllBits",
        fSize: 4,
        fType: 13,
        fTypeName: "unsigned int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fAllBits
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fAllBits",
        fTitle: "[fNbytes] array of UChars",
        fSize: 1,
        fType: 51,
        fTypeName: "unsigned char*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fNbytes");
    element_str.fCountClass = Some("TBits");
    element_str.fCountVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TCollection
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TCollection",
        fCheckSum: 1474546588,
        fClassVersion: 3,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TObject
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TObject",
        fTitle: "Basic ROOT object",
        fSize: 0,
        fType: 66,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fName",
        fTitle: "name of the collection",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSize",
        fTitle: "number of elements in collection",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TClonesArray
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TClonesArray",
        fCheckSum: 3066088035,
        fClassVersion: 4,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TObjArray
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TObjArray",
        fTitle: "An array of objects",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(3);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TList
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TList",
        fCheckSum: 1774568379,
        fClassVersion: 5,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TSeqCollection
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TSeqCollection",
        fTitle: "Sequenceable collection ABC",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(0);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer THashList
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "THashList",
        fCheckSum: 3430828481,
        fClassVersion: 0,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TList
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TList",
        fTitle: "Doubly linked list",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(5);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer THashTable
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "THashTable",
        fCheckSum: 3776773014,
        fClassVersion: 0,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TCollection
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TCollection",
        fTitle: "Collection abstract base class",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(3);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TMap
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TMap",
        fCheckSum: 4274299784,
        fClassVersion: 3,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TCollection
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TCollection",
        fTitle: "Collection abstract base class",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(3);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTable
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fTable",
        fTitle: "Hash table used to store TPair's",
        fSize: 8,
        fType: 64,
        fTypeName: "THashTable*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TObjArray
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TObjArray",
        fCheckSum: 2845730130,
        fClassVersion: 3,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TSeqCollection
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TSeqCollection",
        fTitle: "Sequenceable collection ABC",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(0);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLowerBound
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLowerBound",
        fTitle: "Lower bound of the array",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLast
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLast",
        fTitle: "Last element in array containing an object",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TRefArray
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TRefArray",
        fCheckSum: 1207554269,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TSeqCollection
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TSeqCollection",
        fTitle: "Sequenceable collection ABC",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(0);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fPID
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fPID",
        fTitle: "Pointer to Process Unique Identifier",
        fSize: 8,
        fType: 64,
        fTypeName: "TProcessID*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fUIDs
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fUIDs",
        fTitle: "[fSize] To store uids of referenced objects",
        fSize: 4,
        fType: 53,
        fTypeName: "unsigned int*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fSize");
    element_str.fCountClass = Some("TCollection");
    element_str.fCountVersion = Some(3);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLowerBound
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLowerBound",
        fTitle: "Lower bound of the array",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLast
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLast",
        fTitle: "Last element in array containing an object",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TRefTable
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TRefTable",
        fCheckSum: 2357812101,
        fClassVersion: 3,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TObject
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TObject",
        fTitle: "Basic ROOT object",
        fSize: 0,
        fType: 66,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSize",
        fTitle: "dummy for backward compatibility",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fParents
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fParents",
        fTitle: "array of Parent objects  (eg TTree branch) holding the referenced objects",
        fSize: 8,
        fType: 64,
        fTypeName: "TObjArray*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fOwner
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fOwner",
        fTitle: "Object owning this TRefTable",
        fSize: 8,
        fType: 64,
        fTypeName: "TObject*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fProcessGUIDs
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerSTL",
        f_name: "fProcessGUIDs",
        fTitle: "UUIDs of TProcessIDs used in fParentIDs",
        fSize: 24,
        fType: 300,
        fTypeName: "vector<string>",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fSTLtype = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TSeqCollection
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TSeqCollection",
        fCheckSum: 4234951622,
        fClassVersion: 0,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TCollection
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TCollection",
        fTitle: "Collection abstract base class",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(3);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerInfo
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerInfo",
        fCheckSum: 2421581955,
        fClassVersion: 9,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TVirtualStreamerInfo
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TVirtualStreamerInfo",
        fTitle: "Abstract Interface describing Streamer information for one class",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(6);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCheckSum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fCheckSum",
        fTitle: "Checksum of original class",
        fSize: 4,
        fType: 13,
        fTypeName: "unsigned int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fClassVersion
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fClassVersion",
        fTitle: "Class version identifier",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fElements
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fElements",
        fTitle: "Array of TStreamerElements",
        fSize: 8,
        fType: 64,
        fTypeName: "TObjArray*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerElement
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerElement",
        fCheckSum: 3708727891,
        fClassVersion: 4,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TNamed
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TNamed",
        fTitle: "The basis for a named object (name, title)",
        fSize: 0,
        fType: 67,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fType
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fType",
        fTitle: "element type",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSize",
        fTitle: "sizeof element",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fArrayLength
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fArrayLength",
        fTitle: "cumulative size of all array dims",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fArrayDim
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fArrayDim",
        fTitle: "number of array dimensions",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaxIndex
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaxIndex",
        fTitle: "Maximum array index for array dimension \"dim\"",
        fSize: 20,
        fType: 23,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTypeName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fTypeName",
        fTitle: "Data type name of data member",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerBase
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerBase",
        fCheckSum: 153556400,
        fClassVersion: 3,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TStreamerElement
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TStreamerElement",
        fTitle: "Base class for one element (data member) to be Streamed",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBaseVersion
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fBaseVersion",
        fTitle: "version number of the base class (used during memberwise streaming)",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerBasicType
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerBasicType",
        fCheckSum: 4124421197,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TStreamerElement
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TStreamerElement",
        fTitle: "Base class for one element (data member) to be Streamed",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerBasicPointer
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerBasicPointer",
        fCheckSum: 51937339,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TStreamerElement
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TStreamerElement",
        fTitle: "Base class for one element (data member) to be Streamed",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCountVersion
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fCountVersion",
        fTitle: "version number of the class with the counter",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCountName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fCountName",
        fTitle: "name of data member holding the array count",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCountClass
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fCountClass",
        fTitle: "name of the class with the counter",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerLoop
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerLoop",
        fCheckSum: 4082153692,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TStreamerElement
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TStreamerElement",
        fTitle: "Base class for one element (data member) to be Streamed",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCountVersion
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fCountVersion",
        fTitle: "version number of the class with the counter",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCountName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fCountName",
        fTitle: "name of data member holding the array count",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCountClass
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fCountClass",
        fTitle: "name of the class with the counter",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerObject
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerObject",
        fCheckSum: 1651163444,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TStreamerElement
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TStreamerElement",
        fTitle: "Base class for one element (data member) to be Streamed",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerObjectPointer
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerObjectPointer",
        fCheckSum: 1575431499,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TStreamerElement
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TStreamerElement",
        fTitle: "Base class for one element (data member) to be Streamed",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerObjectAny
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerObjectAny",
        fCheckSum: 150466384,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TStreamerElement
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TStreamerElement",
        fTitle: "Base class for one element (data member) to be Streamed",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerObjectAnyPointer
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerObjectAnyPointer",
        fCheckSum: 905975423,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TStreamerElement
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TStreamerElement",
        fTitle: "Base class for one element (data member) to be Streamed",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerString
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerString",
        fCheckSum: 2695532894,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TStreamerElement
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TStreamerElement",
        fTitle: "Base class for one element (data member) to be Streamed",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerSTL
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerSTL",
        fCheckSum: 2314799318,
        fClassVersion: 3,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TStreamerElement
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TStreamerElement",
        fTitle: "Base class for one element (data member) to be Streamed",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSTLtype
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSTLtype",
        fTitle: "type of STL vector",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCtype
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fCtype",
        fTitle: "STL contained type",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerSTLstring
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerSTLstring",
        fCheckSum: 640610209,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TStreamerSTL
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TStreamerSTL",
        fTitle: "Streamer element of type STL container",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(3);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TStreamerArtificial
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TStreamerArtificial",
        fCheckSum: 1445575625,
        fClassVersion: 0,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TStreamerElement
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TStreamerElement",
        fTitle: "Base class for one element (data member) to be Streamed",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TGraph
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TGraph",
        fCheckSum: 100136037,
        fClassVersion: 4,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TNamed
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TNamed",
        fTitle: "The basis for a named object (name, title)",
        fSize: 0,
        fType: 67,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element TAttLine
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TAttLine",
        fTitle: "Line attributes",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element TAttFill
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TAttFill",
        fTitle: "Fill area attributes",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element TAttMarker
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TAttMarker",
        fTitle: "Marker attributes",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNpoints
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNpoints",
        fTitle: "Number of points <= fMaxSize",
        fSize: 4,
        fType: 6,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fX
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fX",
        fTitle: "[fNpoints] array of X points",
        fSize: 8,
        fType: 48,
        fTypeName: "double*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fNpoints");
    element_str.fCountClass = Some("TGraph");
    element_str.fCountVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fY
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fY",
        fTitle: "[fNpoints] array of Y points",
        fSize: 8,
        fType: 48,
        fTypeName: "double*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fNpoints");
    element_str.fCountClass = Some("TGraph");
    element_str.fCountVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFunctions
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fFunctions",
        fTitle: "Pointer to list of functions (fits and user)",
        fSize: 8,
        fType: 64,
        fTypeName: "TList*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fHistogram
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fHistogram",
        fTitle: "Pointer to histogram used for drawing axis",
        fSize: 8,
        fType: 64,
        fTypeName: "TH1F*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMinimum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMinimum",
        fTitle: "Minimum value for plotting along y",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum value for plotting along y",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TDirectory
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TDirectory",
        fCheckSum: 513503088,
        fClassVersion: 5,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TNamed
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TNamed",
        fTitle: "The basis for a named object (name, title)",
        fSize: 0,
        fType: 67,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMother
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fMother",
        fTitle: "pointer to mother of the directory",
        fSize: 8,
        fType: 64,
        fTypeName: "TObject*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fList
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fList",
        fTitle: "List of objects in memory",
        fSize: 8,
        fType: 64,
        fTypeName: "TList*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fUUID
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectAny",
        f_name: "fUUID",
        fTitle: "Unique identifier",
        fSize: 32,
        fType: 62,
        fTypeName: "TUUID",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TDirectoryFile
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TDirectoryFile",
        fCheckSum: 3214885950,
        fClassVersion: 5,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TDirectory
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TDirectory",
        fTitle: "",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(5);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fModified
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fModified",
        fTitle: "True if directory has been modified",
        fSize: 1,
        fType: 18,
        fTypeName: "bool",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fWritable
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fWritable",
        fTitle: "True if directory is writable",
        fSize: 1,
        fType: 18,
        fTypeName: "bool",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fDatimeC
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectAny",
        f_name: "fDatimeC",
        fTitle: "Date and time when directory is created",
        fSize: 16,
        fType: 62,
        fTypeName: "TDatime",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fDatimeM
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectAny",
        f_name: "fDatimeM",
        fTitle: "Date and time of last modification",
        fSize: 16,
        fType: 62,
        fTypeName: "TDatime",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNbytesKeys
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNbytesKeys",
        fTitle: "Number of bytes for the keys",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNbytesName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNbytesName",
        fTitle: "Number of bytes in TNamed at creation time",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBufferSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fBufferSize",
        fTitle: "Default buffer size to create new TKeys",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSeekDir
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSeekDir",
        fTitle: "Location of directory on file",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSeekParent
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSeekParent",
        fTitle: "Location of parent directory on file",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSeekKeys
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSeekKeys",
        fTitle: "Location of Keys record on file",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFile
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fFile",
        fTitle: "Pointer to current file in memory",
        fSize: 8,
        fType: 64,
        fTypeName: "TFile*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fKeys
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fKeys",
        fTitle: "Pointer to keys list in memory",
        fSize: 8,
        fType: 64,
        fTypeName: "TList*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TFile
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TFile",
        fCheckSum: 2489901954,
        fClassVersion: 8,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TDirectoryFile
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TDirectoryFile",
        fTitle: "",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(5);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSumBuffer
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSumBuffer",
        fTitle: "Sum of buffer sizes of objects written so far",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSum2Buffer
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSum2Buffer",
        fTitle: "Sum of squares of buffer sizes of objects written so far",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBytesWrite
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fBytesWrite",
        fTitle: "Number of bytes written to this file",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBytesRead
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fBytesRead",
        fTitle: "Number of bytes read from this file",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBytesReadExtra
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fBytesReadExtra",
        fTitle: "Number of extra bytes (overhead) read by the readahead buffer",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBEGIN
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fBEGIN",
        fTitle: "First used byte in file",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fEND
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fEND",
        fTitle: "Last used byte in file",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSeekFree
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSeekFree",
        fTitle: "Location on disk of free segments structure",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSeekInfo
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSeekInfo",
        fTitle: "Location on disk of StreamerInfo record",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fD
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fD",
        fTitle: "File descriptor",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fVersion
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fVersion",
        fTitle: "File format version",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCompress
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fCompress",
        fTitle: "Compression level and algorithm",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNbytesFree
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNbytesFree",
        fTitle: "Number of bytes for free segments structure",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNbytesInfo
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNbytesInfo",
        fTitle: "Number of bytes for StreamerInfo record",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fWritten
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fWritten",
        fTitle: "Number of objects written so far",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNProcessIDs
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNProcessIDs",
        fTitle: "Number of TProcessID written to this file",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fReadCalls
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fReadCalls",
        fTitle: "Number of read calls ( not counting the cache calls )",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fRealName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fRealName",
        fTitle: "Effective real file name (not original url)",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fOption
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fOption",
        fTitle: "File options",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fUnits
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fUnits",
        fTitle: "Number of bytes for file pointers",
        fSize: 1,
        fType: 1,
        fTypeName: "char",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFree
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fFree",
        fTitle: "Free segments linked list table",
        fSize: 8,
        fType: 64,
        fTypeName: "TList*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TKey
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TKey",
        fCheckSum: 758463703,
        fClassVersion: 4,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TNamed
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TNamed",
        fTitle: "The basis for a named object (name, title)",
        fSize: 0,
        fType: 67,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fVersion
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fVersion",
        fTitle: "Key version identifier",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNbytes
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNbytes",
        fTitle: "Number of bytes for the object on file",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fObjlen
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fObjlen",
        fTitle: "Length of uncompressed object in bytes",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fDatime
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectAny",
        f_name: "fDatime",
        fTitle: "Date/Time of insertion in file",
        fSize: 16,
        fType: 62,
        fTypeName: "TDatime",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fKeylen
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fKeylen",
        fTitle: "Number of bytes for the key itself",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCycle
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fCycle",
        fTitle: "Cycle number",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSeekKey
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSeekKey",
        fTitle: "Location of object on file",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSeekPdir
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSeekPdir",
        fTitle: "Location of parent directory on file",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fClassName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fClassName",
        fTitle: "Object Class name",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLeft
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLeft",
        fTitle: "Number of bytes left in current segment",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBuffer
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fBuffer",
        fTitle: "Object buffer",
        fSize: 8,
        fType: 7,
        fTypeName: "char*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBufferRef
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fBufferRef",
        fTitle: "Pointer to the TBuffer object",
        fSize: 8,
        fType: 64,
        fTypeName: "TBuffer*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TFeldmanCousins
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TFeldmanCousins",
        fCheckSum: 3955179999,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TObject
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TObject",
        fTitle: "Basic ROOT object",
        fSize: 0,
        fType: 66,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCL
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fCL",
        fTitle: "confidence level as a fraction [e.g. 90% = 0.9]",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fUpperLimit
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fUpperLimit",
        fTitle: "the calculated upper limit",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLowerLimit
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLowerLimit",
        fTitle: "the calculated lower limit",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNobserved
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNobserved",
        fTitle: "input number of observed events",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNbackground
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNbackground",
        fTitle: "input number of background events",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMuMin
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMuMin",
        fTitle: "minimum value of signal to use in calculating the tables",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMuMax
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMuMax",
        fTitle: "maximum value of signal to use in calculating the tables",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMuStep
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMuStep",
        fTitle: "the step in signal to use when generating tables",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNMuStep
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNMuStep",
        fTitle: "= (int)(fMuStep)",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNMax
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNMax",
        fTitle: "= (int)(fMuMax)",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fQUICK
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fQUICK",
        fTitle: "take a short cut to speed up the process of generating a",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLorentzVector
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLorentzVector",
        fCheckSum: 3823026593,
        fClassVersion: 4,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TObject
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TObject",
        fTitle: "Basic ROOT object",
        fSize: 0,
        fType: 66,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fP
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObject",
        f_name: "fP",
        fTitle: "3 vector component",
        fSize: 40,
        fType: 61,
        fTypeName: "TVector3",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fE
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fE",
        fTitle: "time or energy of (x,y,z,t) or (px,py,pz,e)",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TVector2
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TVector2",
        fCheckSum: 9025524,
        fClassVersion: 3,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TObject
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TObject",
        fTitle: "Basic ROOT object",
        fSize: 0,
        fType: 66,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fX
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fX",
        fTitle: "components of the vector",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fY
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fY",
        fTitle: "",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TVector3
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TVector3",
        fCheckSum: 2880880158,
        fClassVersion: 3,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TObject
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TObject",
        fTitle: "Basic ROOT object",
        fSize: 0,
        fType: 66,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fX
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fX",
        fTitle: "",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fY
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fY",
        fTitle: "",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fZ
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fZ",
        fTitle: "",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer ROOT::TIOFeatures
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "ROOT::TIOFeatures",
        fCheckSum: 446770960,
        fClassVersion: -1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element fIOBits
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fIOBits",
        fTitle: "",
        fSize: 1,
        fType: 11,
        fTypeName: "unsigned char",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TBasket
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TBasket",
        fCheckSum: 3153532903,
        fClassVersion: 3,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TKey
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TKey",
        fTitle: "Header description of a logical record on file.",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(4);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBufferSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fBufferSize",
        fTitle: "fBuffer length in bytes",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNevBufSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNevBufSize",
        fTitle:
            "Length in Int_t of fEntryOffset OR fixed length of each entry if fEntryOffset is null!",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNevBuf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNevBuf",
        fTitle: "Number of entries in basket",
        fSize: 4,
        fType: 6,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLast
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLast",
        fTitle: "Pointer to last used byte in basket",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fHeaderOnly
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fHeaderOnly",
        fTitle: "True when only the basket header must be read/written",
        fSize: 1,
        fType: 18,
        fTypeName: "bool",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fEntryOffset
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fEntryOffset",
        fTitle:
            "[fNevBuf] Offset of entries in fBuffer(TKey); generated at runtime.  Special value",
        fSize: 4,
        fType: 43,
        fTypeName: "int*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fNevBuf");
    element_str.fCountClass = Some("TBasket");
    element_str.fCountVersion = Some(3);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBranch
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fBranch",
        fTitle: "Pointer to the basket support branch",
        fSize: 8,
        fType: 64,
        fTypeName: "TBranch*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TBranch
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TBranch",
        fCheckSum: 278366892,
        fClassVersion: 13,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TNamed
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TNamed",
        fTitle: "The basis for a named object (name, title)",
        fSize: 0,
        fType: 67,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element TAttFill
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TAttFill",
        fTitle: "Fill area attributes",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCompress
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fCompress",
        fTitle: "Compression level and algorithm",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBasketSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fBasketSize",
        fTitle: "Initial Size of  Basket Buffer",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fEntryOffsetLen
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fEntryOffsetLen",
        fTitle: "Initial Length of fEntryOffset table in the basket buffers",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fWriteBasket
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fWriteBasket",
        fTitle: "Last basket number written",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fEntryNumber
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fEntryNumber",
        fTitle: "Current entry number (last one filled in this branch)",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fIOFeatures
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectAny",
        f_name: "fIOFeatures",
        fTitle: "IO features for newly-created baskets.",
        fSize: 1,
        fType: 62,
        fTypeName: "ROOT::TIOFeatures",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fOffset
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fOffset",
        fTitle: "Offset of this branch",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaxBaskets
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaxBaskets",
        fTitle: "Maximum number of Baskets so far",
        fSize: 4,
        fType: 6,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSplitLevel
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSplitLevel",
        fTitle: "Branch split level",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fEntries
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fEntries",
        fTitle: "Number of entries",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFirstEntry
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fFirstEntry",
        fTitle: "Number of the first entry in this branch",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTotBytes
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTotBytes",
        fTitle: "Total number of bytes in all leaves before compression",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fZipBytes
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fZipBytes",
        fTitle: "Total number of bytes in all leaves after compression",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBranches
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObject",
        f_name: "fBranches",
        fTitle: "-> List of Branches of this branch",
        fSize: 64,
        fType: 61,
        fTypeName: "TObjArray",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLeaves
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObject",
        f_name: "fLeaves",
        fTitle: "-> List of leaves of this branch",
        fSize: 64,
        fType: 61,
        fTypeName: "TObjArray",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBaskets
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObject",
        f_name: "fBaskets",
        fTitle: "-> List of baskets of this branch",
        fSize: 64,
        fType: 61,
        fTypeName: "TObjArray",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBasketBytes
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fBasketBytes",
        fTitle: "[fMaxBaskets] Length of baskets on file",
        fSize: 4,
        fType: 43,
        fTypeName: "int*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fMaxBaskets");
    element_str.fCountClass = Some("TBranch");
    element_str.fCountVersion = Some(13);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBasketEntry
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fBasketEntry",
        fTitle: "[fMaxBaskets] Table of first entry in each basket",
        fSize: 8,
        fType: 56,
        fTypeName: "Long64_t*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fMaxBaskets");
    element_str.fCountClass = Some("TBranch");
    element_str.fCountVersion = Some(13);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBasketSeek
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fBasketSeek",
        fTitle: "[fMaxBaskets] Addresses of baskets on file",
        fSize: 8,
        fType: 56,
        fTypeName: "Long64_t*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fMaxBaskets");
    element_str.fCountClass = Some("TBranch");
    element_str.fCountVersion = Some(13);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFileName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fFileName",
        fTitle: "Name of file where buffers are stored (\"\" if in same file as Tree header)",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TBranchElement
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TBranchElement",
        fCheckSum: 3880738403,
        fClassVersion: 10,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TBranch
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TBranch",
        fTitle: "Branch descriptor",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(13);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fClassName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fClassName",
        fTitle: "Class name of referenced object",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fParentName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fParentName",
        fTitle: "Name of parent class",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fClonesName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fClonesName",
        fTitle: "Name of class in TClonesArray (if any)",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCheckSum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fCheckSum",
        fTitle: "CheckSum of class",
        fSize: 4,
        fType: 13,
        fTypeName: "unsigned int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fClassVersion
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fClassVersion",
        fTitle: "Version number of class",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fID
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fID",
        fTitle: "element serial number in fInfo",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fType
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fType",
        fTitle: "Branch type",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fStreamerType
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fStreamerType",
        fTitle: "branch streamer type",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum entries for a TClonesArray or variable array",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBranchCount
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fBranchCount",
        fTitle: "pointer to primary branchcount branch",
        fSize: 8,
        fType: 64,
        fTypeName: "TBranchElement*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBranchCount2
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fBranchCount2",
        fTitle: "pointer to secondary branchcount branch",
        fSize: 8,
        fType: 64,
        fTypeName: "TBranchElement*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TBranchObject
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TBranchObject",
        fCheckSum: 2758938441,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TBranch
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TBranch",
        fTitle: "Branch descriptor",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(13);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fClassName
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fClassName",
        fTitle: "Class name of referenced object",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TBranchRef
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TBranchRef",
        fCheckSum: 593540093,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TBranch
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TBranch",
        fTitle: "Branch descriptor",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(13);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fRefTable
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fRefTable",
        fTitle: "pointer to the TRefTable",
        fSize: 8,
        fType: 64,
        fTypeName: "TRefTable*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TChain
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TChain",
        fCheckSum: 983483752,
        fClassVersion: 5,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TTree
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TTree",
        fTitle: "Tree descriptor (the main ROOT I/O class)",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(20);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTreeOffsetLen
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTreeOffsetLen",
        fTitle: "Current size of fTreeOffset array",
        fSize: 4,
        fType: 6,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNtrees
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNtrees",
        fTitle: "Number of trees",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTreeOffset
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fTreeOffset",
        fTitle: "[fTreeOffsetLen] Array of variables",
        fSize: 8,
        fType: 56,
        fTypeName: "Long64_t*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fTreeOffsetLen");
    element_str.fCountClass = Some("TChain");
    element_str.fCountVersion = Some(5);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFiles
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fFiles",
        fTitle: "-> List of file names containing the trees (TChainElement, owned)",
        fSize: 8,
        fType: 63,
        fTypeName: "TObjArray*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fStatus
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fStatus",
        fTitle: "-> List of active/inactive branches (TChainElement, owned)",
        fSize: 8,
        fType: 63,
        fTypeName: "TList*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeaf
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeaf",
        fCheckSum: 1830715730,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TNamed
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TNamed",
        fTitle: "The basis for a named object (name, title)",
        fSize: 0,
        fType: 67,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLen
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLen",
        fTitle: "Number of fixed length elements in the leaf's data.",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLenType
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fLenType",
        fTitle: "Number of bytes for this data type",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fOffset
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fOffset",
        fTitle: "Offset in ClonesArray object (if one)",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fIsRange
    #[allow(unused_mut)]
                    let mut element_str = ElementStrings {
                        class: "TStreamerBasicType",
                        f_name: "fIsRange",
                        fTitle: "(=kTRUE if leaf has a range, kFALSE otherwise).  This is equivalent to being a 'leafcount'.  For a TLeafElement the range information is actually store in the TBranchElement.",
                        fSize: 1,
                        fType: 18,
                        fTypeName: "bool",
                        fBaseVersion: None,
                        fCountName: None,
                        fCountClass: None,
                        fCountVersion: None,
                        fSTLtype: None,
                    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fIsUnsigned
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fIsUnsigned",
        fTitle: "(=kTRUE if unsigned, kFALSE otherwise)",
        fSize: 1,
        fType: 18,
        fTypeName: "bool",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLeafCount
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fLeafCount",
        fTitle: "Pointer to Leaf count if variable length (we do not own the counter)",
        fSize: 8,
        fType: 64,
        fTypeName: "TLeaf*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafElement
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafElement",
        fCheckSum: 2689566867,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fID
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fID",
        fTitle: "element serial number in fInfo",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fType
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fType",
        fTitle: "leaf type",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafObject
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafObject",
        fCheckSum: 649755724,
        fClassVersion: 4,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fVirtual
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fVirtual",
        fTitle: "Support for polymorphism, when set classname is written with object.",
        fSize: 1,
        fType: 18,
        fTypeName: "bool",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafO
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafO",
        fCheckSum: 44976339,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMinimum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMinimum",
        fTitle: "Minimum value if leaf range is specified",
        fSize: 1,
        fType: 18,
        fTypeName: "bool",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum value if leaf range is specified",
        fSize: 1,
        fType: 18,
        fTypeName: "bool",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafB
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafB",
        fCheckSum: 253643614,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMinimum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMinimum",
        fTitle: "Minimum value if leaf range is specified",
        fSize: 1,
        fType: 1,
        fTypeName: "char",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum value if leaf range is specified",
        fSize: 1,
        fType: 1,
        fTypeName: "char",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafS
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafS",
        fCheckSum: 353169103,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMinimum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMinimum",
        fTitle: "Minimum value if leaf range is specified",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum value if leaf range is specified",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafI
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafI",
        fCheckSum: 2120920601,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMinimum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMinimum",
        fTitle: "Minimum value if leaf range is specified",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum value if leaf range is specified",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafL
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafL",
        fCheckSum: 3727820898,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMinimum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMinimum",
        fTitle: "Minimum value if leaf range is specified",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum value if leaf range is specified",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafG
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafG",
        fCheckSum: 3970374839,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMinimum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMinimum",
        fTitle: "Minimum value if leaf range is specified",
        fSize: 8,
        fType: 4,
        fTypeName: "long",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum value if leaf range is specified",
        fSize: 8,
        fType: 4,
        fTypeName: "long",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafF
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafF",
        fCheckSum: 987602290,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMinimum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMinimum",
        fTitle: "Minimum value if leaf range is specified",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum value if leaf range is specified",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafD
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafD",
        fCheckSum: 294553462,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMinimum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMinimum",
        fTitle: "Minimum value if leaf range is specified",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum value if leaf range is specified",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafF16
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafF16",
        fCheckSum: 3946182787,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMinimum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMinimum",
        fTitle: "Minimum value if leaf range is specified",
        fSize: 4,
        fType: 19,
        fTypeName: "Float16_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum value if leaf range is specified",
        fSize: 4,
        fType: 19,
        fTypeName: "Float16_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafD32
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafD32",
        fCheckSum: 3789317121,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMinimum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMinimum",
        fTitle: "Minimum value if leaf range is specified",
        fSize: 8,
        fType: 9,
        fTypeName: "Double32_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum value if leaf range is specified",
        fSize: 8,
        fType: 9,
        fTypeName: "Double32_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TLeafC
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TLeafC",
        fCheckSum: 4226003699,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TLeaf
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TLeaf",
        fTitle: "Leaf: description of a Branch data type",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMinimum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMinimum",
        fTitle: "Minimum value if leaf range is specified",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaximum
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaximum",
        fTitle: "Maximum value if leaf range is specified",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TNtuple
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TNtuple",
        fCheckSum: 3097828523,
        fClassVersion: 2,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TTree
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TTree",
        fTitle: "Tree descriptor (the main ROOT I/O class)",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(20);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNvar
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNvar",
        fTitle: "Number of columns",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TNtupleD
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TNtupleD",
        fCheckSum: 2380847219,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TTree
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TTree",
        fTitle: "Tree descriptor (the main ROOT I/O class)",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(20);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNvar
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNvar",
        fTitle: "Number of columns",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TTree
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TTree",
        fCheckSum: 1919213695,
        fClassVersion: 20,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TNamed
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TNamed",
        fTitle: "The basis for a named object (name, title)",
        fSize: 0,
        fType: 67,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(1);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element TAttLine
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TAttLine",
        fTitle: "Line attributes",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element TAttFill
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TAttFill",
        fTitle: "Fill area attributes",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element TAttMarker
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TAttMarker",
        fTitle: "Marker attributes",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(2);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fEntries
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fEntries",
        fTitle: "Number of entries",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTotBytes
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTotBytes",
        fTitle: "Total number of bytes in all branches before compression",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fZipBytes
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fZipBytes",
        fTitle: "Total number of bytes in all branches after compression",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fSavedBytes
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fSavedBytes",
        fTitle: "Number of autosaved bytes",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFlushedBytes
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fFlushedBytes",
        fTitle: "Number of auto-flushed bytes",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fWeight
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fWeight",
        fTitle: "Tree weight (see TTree::SetWeight)",
        fSize: 8,
        fType: 8,
        fTypeName: "double",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTimerInterval
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTimerInterval",
        fTitle: "Timer interval in milliseconds",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fScanField
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fScanField",
        fTitle: "Number of runs before prompting in Scan",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fUpdate
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fUpdate",
        fTitle: "Update frequency for EntryLoop",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fDefaultEntryOffsetLen
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fDefaultEntryOffsetLen",
        fTitle: "Initial Length of fEntryOffset table in the basket buffers",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fNClusterRange
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fNClusterRange",
        fTitle: "Number of Cluster range in addition to the one defined by 'AutoFlush'",
        fSize: 4,
        fType: 6,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaxEntries
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaxEntries",
        fTitle: "Maximum number of entries in case of circular buffers",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaxEntryLoop
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaxEntryLoop",
        fTitle: "Maximum number of entries to process",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fMaxVirtualSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fMaxVirtualSize",
        fTitle: "Maximum total size of buffers kept in memory",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fAutoSave
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fAutoSave",
        fTitle:
            "Autosave tree when fAutoSave entries written or -fAutoSave (compressed) bytes produced",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fAutoFlush
    #[allow(unused_mut)]
                    let mut element_str = ElementStrings {
                        class: "TStreamerBasicType",
                        f_name: "fAutoFlush",
                        fTitle: "Auto-flush tree when fAutoFlush entries written or -fAutoFlush (compressed) bytes produced",
                        fSize: 8,
                        fType: 16,
                        fTypeName: "Long64_t",
                        fBaseVersion: None,
                        fCountName: None,
                        fCountClass: None,
                        fCountVersion: None,
                        fSTLtype: None,
                    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fEstimate
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fEstimate",
        fTitle: "Number of entries to estimate histogram limits",
        fSize: 8,
        fType: 16,
        fTypeName: "Long64_t",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fClusterRangeEnd
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fClusterRangeEnd",
        fTitle: "[fNClusterRange] Last entry of a cluster range.",
        fSize: 8,
        fType: 56,
        fTypeName: "Long64_t*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fNClusterRange");
    element_str.fCountClass = Some("TTree");
    element_str.fCountVersion = Some(20);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fClusterSize
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicPointer",
        f_name: "fClusterSize",
        fTitle: "[fNClusterRange] Number of entries in each cluster for a given range.",
        fSize: 8,
        fType: 56,
        fTypeName: "Long64_t*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fCountName = Some("fNClusterRange");
    element_str.fCountClass = Some("TTree");
    element_str.fCountVersion = Some(20);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fIOFeatures
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectAny",
        f_name: "fIOFeatures",
        fTitle: "IO features to define for newly-written baskets and branches.",
        fSize: 1,
        fType: 62,
        fTypeName: "ROOT::TIOFeatures",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBranches
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObject",
        f_name: "fBranches",
        fTitle: "List of Branches",
        fSize: 64,
        fType: 61,
        fTypeName: "TObjArray",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fLeaves
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObject",
        f_name: "fLeaves",
        fTitle: "Direct pointers to individual branch leaves",
        fSize: 64,
        fType: 61,
        fTypeName: "TObjArray",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fAliases
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fAliases",
        fTitle: "List of aliases for expressions based on the tree branches.",
        fSize: 8,
        fType: 64,
        fTypeName: "TList*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fIndexValues
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectAny",
        f_name: "fIndexValues",
        fTitle: "Sorted index values",
        fSize: 24,
        fType: 62,
        fTypeName: "TArrayD",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fIndex
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectAny",
        f_name: "fIndex",
        fTitle: "Index of sorted values",
        fSize: 24,
        fType: 62,
        fTypeName: "TArrayI",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTreeIndex
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fTreeIndex",
        fTitle: "Pointer to the tree Index (if any)",
        fSize: 8,
        fType: 64,
        fTypeName: "TVirtualIndex*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fFriends
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fFriends",
        fTitle: "pointer to list of friend elements",
        fSize: 8,
        fType: 64,
        fTypeName: "TList*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fUserInfo
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fUserInfo",
        fTitle: "pointer to a list of user objects associated to this Tree",
        fSize: 8,
        fType: 64,
        fTypeName: "TList*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fBranchRef
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectPointer",
        f_name: "fBranchRef",
        fTitle: "Branch supporting the TRefTable (if any)",
        fSize: 8,
        fType: 64,
        fTypeName: "TBranchRef*",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TAttCanvas
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TAttCanvas",
        fCheckSum: 4134953791,
        fClassVersion: 1,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element fXBetween
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fXBetween",
        fTitle: "X distance between pads",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fYBetween
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fYBetween",
        fTitle: "Y distance between pads",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fTitleFromTop
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fTitleFromTop",
        fTitle: "Y distance of Global Title from top",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fXdate
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fXdate",
        fTitle: "X position where to draw the date",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fYdate
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fYdate",
        fTitle: "X position where to draw the date",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fAdate
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fAdate",
        fTitle: "Alignment for the date",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    // Streamer TCanvas
    let class = ClassStrings {
        class: "TStreamerInfo",
        title: "",
        fName: "TCanvas",
        fCheckSum: 232602757,
        fClassVersion: 8,
    };

    let mut streamer_info = StreamerInfo::new(&class.name(), class.fCheckSum, class.rvers());

    //\t element TPad
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBase",
        f_name: "TPad",
        fTitle: "",
        fSize: 0,
        fType: 0,
        fTypeName: "BASE",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    element_str.fBaseVersion = Some(13);
    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCatt
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerObjectAny",
        f_name: "fCatt",
        fTitle: "Canvas attributes",
        fSize: 32,
        fType: 62,
        fTypeName: "TAttCanvas",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fDISPLAY
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerString",
        f_name: "fDISPLAY",
        fTitle: "Name of destination screen",
        fSize: 24,
        fType: 65,
        fTypeName: "TString",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fXsizeUser
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fXsizeUser",
        fTitle: "User specified size of canvas along X in CM",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fYsizeUser
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fYsizeUser",
        fTitle: "User specified size of canvas along Y in CM",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fXsizeReal
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fXsizeReal",
        fTitle: "Current size of canvas along X in CM",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fYsizeReal
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fYsizeReal",
        fTitle: "Current size of canvas along Y in CM",
        fSize: 4,
        fType: 5,
        fTypeName: "float",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fHighLightColor
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fHighLightColor",
        fTitle: "Highlight color of active pad",
        fSize: 2,
        fType: 2,
        fTypeName: "short",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fDoubleBuffer
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fDoubleBuffer",
        fTitle: "Double buffer flag (0=off, 1=on)",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fWindowTopX
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fWindowTopX",
        fTitle: "Top X position of window (in pixels)",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fWindowTopY
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fWindowTopY",
        fTitle: "Top Y position of window (in pixels)",
        fSize: 4,
        fType: 3,
        fTypeName: "int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fWindowWidth
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fWindowWidth",
        fTitle: "Width of window (including borders, etc.)",
        fSize: 4,
        fType: 13,
        fTypeName: "unsigned int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fWindowHeight
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fWindowHeight",
        fTitle: "Height of window (including menubar, borders, etc.)",
        fSize: 4,
        fType: 13,
        fTypeName: "unsigned int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCw
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fCw",
        fTitle: "Width of the canvas along X (pixels)",
        fSize: 4,
        fType: 13,
        fTypeName: "unsigned int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fCh
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fCh",
        fTitle: "Height of the canvas along Y (pixels)",
        fSize: 4,
        fType: 13,
        fTypeName: "unsigned int",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    //\t element fRetained
    #[allow(unused_mut)]
    let mut element_str = ElementStrings {
        class: "TStreamerBasicType",
        f_name: "fRetained",
        fTitle: "Retain structure flag",
        fSize: 1,
        fType: 18,
        fTypeName: "bool",
        fBaseVersion: None,
        fCountName: None,
        fCountClass: None,
        fCountVersion: None,
        fSTLtype: None,
    };

    let mut streamer_element = StreamerElement::new(
        element_str.name(),
        element_str.etype(),
        element_str.esize(),
        id_elements,
    );

    streamer_element.named = streamer_element.named.with_title("".to_string());
    id_elements += 1;
    streamer_element.ename = element_str.fTypeName.to_string();

    matches!(streamer_element.etype, Enum::Named(_));

    let streamer = element_str.build_streamer(streamer_element);
    streamer_info.elems.push(streamer);

    streamer_info.id = id_elements;
    db.insert(streamer_info);

    Ok(())
}
