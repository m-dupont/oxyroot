use anyhow::Result;
use env_logger::{Builder, Target, WriteStyle};
use log::trace;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

const OUT_DIR: &str = "/tmp/rust/gen_streamers_titles";

fn gen_one_cat_streamers_with_root(class: &str) -> Result<String> {
    println!("gen_one_cat_streamers_with_root class = {}", class);
    let out_dir: PathBuf = OUT_DIR.into();
    let mut out_path = out_dir.clone();

    out_path.push(format!("gen_{CLASS}.log", CLASS = class));

    if out_path.exists() {
        let content = fs::read(out_path)?;
        let s = String::from_utf8(content)?;
        return Ok(s);
    }

    let macro_content = format!(
        r#"{{
auto name = "{CLASS}";


  auto s = TClass::GetClass(name)->GetStreamerInfo();


cout << ";" << name << ".name#" << s->GetName() << endl;
cout << ";" << name << ".title#" << s->GetTitle() << endl;

  auto fArr = s->GetElements();
  for (int i = 0; i <= fArr->GetLast(); i++)
  {{
    TStreamerElement* element = (TStreamerElement*)fArr->At(i);
    auto element_name = element->GetName();
    cout << ";" << name << ".elements." << element_name <<  ".name#" << element->GetName() << endl;
    cout << ";" << name << ".elements." << element_name <<  ".title#" << element->GetTitle() << endl;

    }}
    }}
"#,
        CLASS = class,
    );

    fs::create_dir_all(out_dir.clone())?;

    let mut macro_path = out_dir.clone();
    macro_path.push("gen.C");
    fs::write(macro_path, macro_content)?;

    trace!("Execute ROOT");
    let out = Command::new("root")
        .arg("-q")
        .arg("gen.C")
        .current_dir(&out_dir)
        .output()?;

    fs::write(out_path, &out.stdout)?;
    Ok(String::from_utf8(out.stdout)?)
}

fn gen_cat_streamers_with_root(classes: &[&str]) -> Result<String> {
    let mut out_path: PathBuf = OUT_DIR.into();

    out_path.push("gen_all.txt");
    if out_path.exists() {
        let content = fs::read(out_path)?;
        let s = String::from_utf8(content)?;
        return Ok(s);
    }
    let s = classes
        .iter()
        .map(|class| gen_one_cat_streamers_with_root(class).unwrap())
        .collect();
    fs::write(out_path, &s)?;
    Ok(s)
}

fn main() -> Result<()> {
    Builder::new()
        .parse_default_env()
        // .filter(None, LevelFilter::Trace)
        .write_style(WriteStyle::Always)
        .format(|buf, record| {
            // let level = record.metadata().level().as_str().to_ascii_uppercase();
            // let file = record.file().unwrap_or("");
            // let line = record.line().unwrap_or(0);
            // let module = record.module_path().unwrap_or("");
            // let time = Local::now().format("%Y-%m-%dT%H:%M:%S");
            writeln!(buf, "{}", record.args())
        })
        .target(Target::Stdout)
        .init();

    let classes = vec![
        "TAttAxis",
        "TAttBBox2D",
        "TAttFill",
        "TAttLine",
        "TAttMarker",
        "TAttPad",
        "TDatime",
        "TNamed",
        "TObject",
        "TObjString",
        "TProcessID",
        "TProcessUUID",
        "TQObject",
        "TRef",
        "TString",
        "TUUID",
        "TVirtualPad",
        // rcont
        "TArray",
        "TArrayC",
        "TArrayS",
        "TArrayI",
        "TArrayL",
        "TArrayL64",
        "TArrayF",
        "TArrayD",
        "TBits",
        "TCollection",
        "TClonesArray",
        "TList",
        "THashList",
        "THashTable",
        "TMap",
        "TObjArray",
        "TRefArray",
        "TRefTable",
        "TSeqCollection",
        // rdict
        "TStreamerInfo",
        "TStreamerElement",
        "TStreamerBase",
        "TStreamerBasicType",
        "TStreamerBasicPointer",
        "TStreamerLoop",
        "TStreamerObject",
        "TStreamerObjectPointer",
        "TStreamerObjectAny",
        "TStreamerObjectAnyPointer",
        "TStreamerString",
        "TStreamerSTL",
        "TStreamerSTLstring",
        "TStreamerArtificial",
        // rhist
        // "TAxis",
        // "TConfidenceLevel",
        // "TEfficiency",
        // "TF1",
        // "TF1AbsComposition",
        // "TF1Convolution",
        // "TF1NormSum",
        // "TF1Parameters",
        // "TFormula",
        "TGraph",
        // "TGraphErrors",
        // "TGraphAsymmErrors",
        // "TGraphMultiErrors",
        // "TH1",
        // "TH1C",
        // "TH1D",
        // "TH1F",
        // "TH1I",
        // "TH1K",
        // "TH1S",
        // "TH2",
        // "TH2C",
        // "TH2D",
        // "TH2F",
        // "TH2I",
        // "TH2Poly",
        // "TH2PolyBin",
        // "TH2S",
        // "TLimit",
        // "TLimitDataSource",
        // "TMultiGraph",
        // "TProfile",
        // "TProfile2D",
        // "TScatter", TODO: implement
        // riofs
        "TDirectory",
        "TDirectoryFile",
        "TFile",
        "TKey",
        // rntup
        // "ROOT::Experimental::RNTuple", // FIXME(sbinet): TODO

        // rphys
        "TFeldmanCousins",
        "TLorentzVector",
        "TVector2",
        "TVector3",
        // rtree
        "ROOT::TIOFeatures",
        "TBasket",
        "TBranch",
        "TBranchElement",
        "TBranchObject",
        "TBranchRef",
        "TChain",
        "TLeaf",
        "TLeafElement",
        "TLeafObject",
        "TLeafO",
        "TLeafB",
        "TLeafS",
        "TLeafI",
        "TLeafL",
        "TLeafG",
        "TLeafF",
        "TLeafD",
        "TLeafF16",
        "TLeafD32",
        "TLeafC",
        "TNtuple",
        "TNtupleD",
        "TTree",
        // rpad
        "TAttCanvas",
        "TCanvas",
        "TPad",
    ];

    // let classes = vec!["TLeafElement", "TArrayC"];

    let _dump = gen_cat_streamers_with_root(&classes)?;

    // let classes = generate_class(&dump)?;
    // println!("classes = {:?}", classes);

    // for class in classes {
    //     println!("# class = {}", class);
    //
    // }

    // println!("Dump = {:?}", DUMP);

    // DBSTREAMER.get("hello");
    //
    // println!("DBSTREAMER = {:?}", DBSTREAMER.keys());

    //
    // for class in classes {
    //     println!("# class = {:?}", class);
    //     break;
    // }

    Ok(())
}
