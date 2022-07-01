mod common;

use anyhow::Result;
use common::TemplateWriter;
use oxyroot::RBuffer;

#[test]
fn read_int_struct() -> Result<()> {
    let temp = TemplateWriter::default().with_outdir("/tmp/rust/struct/read_int_struct")?;

    let macro_content = format!(
        r#"{{
    struct sd_t {{
         Int_t a;   
         Int_t b;   
           
     }};
    sd_t sd;

   TFile *hfile = hfile = TFile::Open("o.root","RECREATE", "", {COMPRESSION});

   TTree *tree = new TTree("T","data for rust tests");
   tree->Branch("v_i",&sd);
   
   for (int i = -10; i < 100000; ++i)
     {{
       sd.a = i;
       sd.b = i*13;
       tree->Fill(); 
     }}   

   tree->Print();
   tree->Write();   
   delete hfile;
}}
"#,
        COMPRESSION = 207
    );

    temp.write_raw_macro(&macro_content)?;
    temp.execute_macro()?;

    #[derive(Debug)]
    struct Sd {
        a: i32,
        b: i32,
    };

    let parse = |r: &mut RBuffer| Sd {
        a: r.read_i32().unwrap(),
        b: r.read_i32().unwrap(),
    };

    let tree = temp.tree()?;
    let mut b = tree.branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        let sd = b.next().unwrap();
        assert_eq!(sd.a, i);
        assert_eq!(sd.b, i * 13);
    }

    while let Some(sd) = b.next() {
        assert_eq!(sd.b, sd.a * 13);
    }

    Ok(())
}
