use anyhow::Result;
use oxyroot::RBuffer;
use paste::paste;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, path};

pub struct TemplateWriter {
    out_dir: path::PathBuf,
    out_root_path: String,
    value_type: String,
    compression: i32,
}

impl Default for TemplateWriter {
    fn default() -> Self {
        TemplateWriter {
            out_dir: "/tmp/rust_test".into(),
            out_root_path: "o.root".into(),
            value_type: "int".into(),
            compression: 207,
        }
    }
}

impl TemplateWriter {
    fn final_file_path(&self) -> PathBuf {
        let mut o = self.out_dir.clone();
        o.push("o.root");
        o
    }

    fn with_outdir<T: AsRef<str>>(mut self, outdir: T) -> Result<TemplateWriter> {
        let s = outdir.as_ref().to_string();
        fs::create_dir_all(&s)?;
        self.out_dir = s.into();
        println!("outdir = {:?}", self.out_dir);
        Ok(self)
    }

    fn with_value_type<T: AsRef<str>>(mut self, value_t: T) -> Result<TemplateWriter> {
        let value_t = value_t.as_ref().to_string();
        self.value_type = value_t;
        println!("value_type = {:?}", self.value_type);
        Ok(self)
    }

    fn with_compression(mut self, value_t: i32) -> Result<TemplateWriter> {
        let value_t = value_t;
        self.compression = value_t;
        println!("compression = {:?}", self.compression);
        Ok(self)
    }

    fn write_root_macro(&self) -> Result<()> {
        println!("outdir = {:?}", self.out_dir.clone());
        let mut macro_path = self.out_dir.clone();
        macro_path.push("gen.C");
        println!("write to {:?}", macro_path);

        let macro_content = format!(
            r#"{{
   {TYPE}  v_i = 0;

   TFile *hfile = hfile = TFile::Open("o.root","RECREATE", "", {COMPRESSION});

   TTree *tree = new TTree("T","data for rust tests");
   tree->BRANCH("v_i",&v_i);
   
   for (int i = -10; i < 10; ++i)
     {{
       v_i = i;
       tree->Fill(); 
     }}   

   tree->Print();
   tree->Write();   
   delete hfile;
}}
"#,
            TYPE = self.value_type,
            COMPRESSION = self.compression
        );

        fs::write(macro_path, macro_content)?;

        Ok(())
    }

    fn execute_macro(&self) -> Result<()> {
        println!("Execute ROOT");

        let out = Command::new("root")
            .arg("-q")
            .arg("gen.C")
            .current_dir(&self.out_dir)
            .output()?;
        println!("{}", String::from_utf8(out.stdout)?);

        Ok(())
    }

    fn tree(&self) -> Result<oxyroot::Tree> {
        let mut f = oxyroot::RootFile::open(self.final_file_path().to_str().unwrap())?;
        let tree = f.get_tree("T")?.unwrap();
        Ok(tree)
    }
}

fn read_i32_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/int")?
        .with_value_type("int")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_i32().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.get_branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

fn read_u32_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/u32")?
        .with_value_type("UInt_t")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_u32().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.get_branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i as u32, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

fn read_i16_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/i16")?
        .with_value_type("short")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_i16().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.get_branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

fn read_u16_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/u16")?
        .with_value_type("short")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_u16().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.get_branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i as u16, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

fn read_i8_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/i8")?
        .with_value_type("Char_t")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_i8().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.get_branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

fn read_i64_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/i64")?
        .with_value_type("ULong64_t")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_i64().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.get_branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

fn read_f32_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/float")?
        .with_value_type("float")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_f32().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.get_branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i as f32, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

fn read_f64_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/f64")?
        .with_value_type("double")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_f64().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.get_branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i as f64, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

#[test]
fn read_tree_i32() {
    read_i32_branch().expect("nooo");
}

#[test]
fn read_tree_u32() {
    read_u32_branch().expect("nooo");
}

#[test]
fn read_tree_i16() {
    read_i16_branch().expect("nooo");
}

#[test]
fn read_tree_u16() {
    read_u16_branch().expect("nooo");
}

#[test]
fn read_tree_i8() {
    read_i8_branch().expect("nooo");
}

#[test]
fn read_tree_i64() {
    read_i64_branch().expect("nooo");
}

#[test]
fn read_tree_f32() {
    read_f32_branch().expect("nooo");
}

#[test]
fn read_tree_f64() {
    read_f64_branch().expect("nooo");
}

macro_rules! read_primitive_branch {
    ($ftype:ty, $root_type:literal) => {{
        read_primitive_branch! {$ftype, $root_type, 207};
        read_primitive_branch! {$ftype, $root_type, 101};
        // read_primitive_branch! {$ftype, $root_type, 404};
        // read_primitive_branch! {$ftype, $root_type, 505};
    }};

    ($ftype:ty, $root_type:literal, $compression: expr) => {{
        fn impl_read_primitive_branch() -> Result<()> {
            let outdir = format!(
                "/tmp/rust/into/{}/{}/{}",
                stringify! {$ftype},
                $root_type,
                stringify! {$compression}
            );

            let temp = TemplateWriter::default()
                .with_outdir(outdir)?
                .with_compression($compression)?
                .with_value_type($root_type)?;

            temp.write_root_macro()?;
            temp.execute_macro()?;

            let tree = temp.tree()?;
            let mut b = tree.get_branch("v_i").unwrap().get_basket_into::<$ftype>();

            for i in -10..10 {
                assert_eq!(i as $ftype, b.next().unwrap());
            }

            assert!(b.next().is_none());
            Ok(())
        }

        impl_read_primitive_branch().expect("nooo");
    }};
}

#[test]
fn read_tree_into_i32() {
    read_primitive_branch!(i32, "Int_t");
    read_primitive_branch!(i32, "int");
}

#[test]
fn read_tree_into_u32() {
    read_primitive_branch!(u32, "UInt_t");
}

// #[test]
// fn read_tree_into_i64() {
//     read_primitive_branch!(i64, "Long_t");
// }
// #[test]
// fn read_tree_into_u64() {
//     read_primitive_branch!(u64, "ULong_t");
// }
#[test]
fn read_tree_into_f32() {
    read_primitive_branch!(f32, "Float_t");
}

#[test]
fn read_tree_into_f64() {
    read_primitive_branch!(f64, "Double_t");
}
