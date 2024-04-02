const CORE_TYPES: [&str; 8] = [
    "TObject",
    "TFile",
    "TDirectoryFile",
    "TKey",
    "TString",
    "TDatime",
    "TVirtualIndex",
    "TBasket",
];

pub(crate) fn is_core_type(name: &str) -> bool {
    CORE_TYPES.contains(&name)
}

const CXX_BUILTINS: [&str; 53] = [
    "bool",
    "int8_t",
    "uint8_t",
    "int16_t",
    "uint16_t",
    "int32_t",
    "uint32_t",
    "int64_t",
    "uint64_t",
    // C/C++ builtins
    "unsigned",       //:       reflect.TypeOf(uint32(0)),
    "unsigned char",  //:  reflect.TypeOf(uint8(0)),
    "unsigned short", //: reflect.TypeOf(uint16(0)),
    "unsigned int",   //:   reflect.TypeOf(uint32(0)),
    "unsigned long",  //:  reflect.TypeOf(uint64(0)),
    //"int":   reflect.TypeOf(int(0)),
    "char",   //:  reflect.TypeOf(int8(0)),
    "short",  //: reflect.TypeOf(int16(0)),
    "int",    //:   reflect.TypeOf(int32(0)),
    "long",   //:  reflect.TypeOf(int64(0)),
    "float",  //:  reflect.TypeOf(float32(0)),
    "double", //: reflect.TypeOf(float64(0)),
    "string", //: reflect.TypeOf(""),
    // ROOT builtins
    "Bool_t",     //: reflect.TypeOf(true),
    "Byte_t",     //: reflect.TypeOf(uint8(0)),
    "Char_t",     //:    reflect.TypeOf(int8(0)),
    "UChar_t",    //:   reflect.TypeOf(uint8(0)),
    "Short_t",    //:   reflect.TypeOf(int16(0)),
    "UShort_t",   //:  reflect.TypeOf(uint16(0)),
    "Int_t",      //:     reflect.TypeOf(int32(0)),
    "UInt_t",     //:    reflect.TypeOf(uint32(0)),
    "Seek_t",     //:    reflect.TypeOf(int64(0)),  // FIXME(sbinet): not portable
    "Long_t",     //:    reflect.TypeOf(int64(0)),  // FIXME(sbinet): not portable
    "ULong_t",    //:   reflect.TypeOf(uint64(0)), // FIXME(sbinet): not portable
    "Long64_t",   //:  reflect.TypeOf(int64(0)),
    "ULong64_t",  //: reflect.TypeOf(uint64(0)),
    "Float_t",    //:    reflect.TypeOf(float32(0)),
    "Float16_t",  //:  reflect.TypeOf(root.Float16(0)),
    "Double_t",   //:   reflect.TypeOf(float64(0)),
    "Double32_t", //: reflect.TypeOf(root.Double32(0)),
    "Version_t",  //: reflect.TypeOf(int16(0)),
    "Option_t",   //:  reflect.TypeOf(""),
    "Ssiz_t",     //:    reflect.TypeOf(int(0)),
    "Real_t",     //:    reflect.TypeOf(float32(0)),
    "Axis_t",     //: reflect.TypeOf(float64(0)),
    "Stat_t",     //: reflect.TypeOf(float64(0)),
    "Font_t",     //:   reflect.TypeOf(int16(0)),
    "Style_t",    //:  reflect.TypeOf(int16(0)),
    "Marker_t",   //: reflect.TypeOf(int16(0)),
    "Width_t",    //:  reflect.TypeOf(int16(0)),
    "Color_t",    //:  reflect.TypeOf(int16(0)),
    "SCoord_t",   //: reflect.TypeOf(int16(0)),
    "Coord_t",    //:  reflect.TypeOf(float64(0)),
    "Angle_t",    //:  reflect.TypeOf(float32(0)),
    "Size_t",     //:   reflect.TypeOf(float32(0)),
];

pub(crate) fn is_cxx_builtin(name: &str) -> bool {
    CXX_BUILTINS.contains(&name)
}
