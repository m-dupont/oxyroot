use anyhow::Result;
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

    pub fn with_outdir<T: AsRef<str>>(mut self, outdir: T) -> Result<TemplateWriter> {
        let s = outdir.as_ref().to_string();
        fs::create_dir_all(&s)?;
        self.out_dir = s.into();
        println!("outdir = {:?}", self.out_dir);
        Ok(self)
    }

    pub fn with_value_type<T: AsRef<str>>(mut self, value_t: T) -> Result<TemplateWriter> {
        let value_t = value_t.as_ref().to_string();
        self.value_type = value_t;
        println!("value_type = {:?}", self.value_type);
        Ok(self)
    }

    pub fn with_compression(mut self, value_t: i32) -> Result<TemplateWriter> {
        let value_t = value_t;
        self.compression = value_t;
        println!("compression = {:?}", self.compression);
        Ok(self)
    }

    pub fn write_root_macro(&self) -> Result<()> {
        println!("outdir = {:?}", self.out_dir.clone());
        let mut macro_path = self.out_dir.clone();
        macro_path.push("gen.C");
        println!("write to {:?}", macro_path);

        let macro_content = format!(
            r#"{{
   {TYPE}  v_i = 0;

   TFile *hfile = hfile = TFile::Open("o.root","RECREATE", "", {COMPRESSION});

   TTree *tree = new TTree("T","data for rust tests");
   tree->Branch("v_i",&v_i);
   
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

    pub fn write_raw_macro(&self, m: &str) -> Result<()> {
        println!("outdir = {:?}", self.out_dir.clone());
        let mut macro_path = self.out_dir.clone();
        macro_path.push("gen.C");
        println!("write to {:?}", macro_path);
        fs::write(macro_path, m)?;
        Ok(())
    }

    pub fn execute_macro(&self) -> Result<()> {
        println!("Execute ROOT");
        let out = Command::new("root")
            .arg("-q")
            .arg("gen.C")
            .current_dir(&self.out_dir)
            .output()?;
        println!("{}", String::from_utf8(out.stdout)?);
        Ok(())
    }

    pub fn tree(&self) -> Result<oxyroot::Tree> {
        let mut f = oxyroot::RootFile::open(self.final_file_path().to_str().unwrap())?;
        let tree = f.get_tree("T")?.unwrap();
        Ok(tree)
    }
}

impl Drop for TemplateWriter {
    fn drop(&mut self) {
        println!("Delete {}", self.out_dir.to_str().unwrap());
        fs::remove_dir_all(&self.out_dir).expect("TODO: panic message");
    }
}
