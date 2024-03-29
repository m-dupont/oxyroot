use anyhow::{anyhow, bail, Result};
use downcast::Any;
use log::trace;
use num::Integer;
use oxyroot;
use oxyroot::{Marshaler, Unmarshaler};
use std::any::type_name;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use test_log::test;

const OUT_DIR: &str = "/tmp/rust/root_write/";

fn write_root_macro_for_simple_branch<T: std::str::FromStr + Debug>(
    out_file: &str,
    ty_cpp: &str,
    bname: &str,
) -> Result<Vec<T>> {
    let macro_content = format!(
        r#"

{{
    TFile *f = new TFile("/tmp/rust/root_write/{}/{}/5.root");
    TTree *t = (TTree*)f->Get("mytree");
    cout << "SIZEOF({ty_cpp}) = " << sizeof({ty_cpp}) << endl;
    {ty_cpp} n = 0;
    t->SetBranchAddress("{bname}", &n);
    //cout << ">>ENTRIES: " << t->GetEntries() << endl;
    for (int i = 0; i < t->GetEntries(); i++)
    {{
        t->GetEntry(i);
        cout <<">>"<< static_cast<int16_t>(n) << endl;
    }}
    f->Close();
}}
"#,
        type_name::<T>(),
        ty_cpp
    );
    println!("write to {:?}", out_file);
    fs::write(out_file, macro_content)?;

    let out = Command::new("root").arg("-q").arg(out_file).output()?;
    eprintln!("err: {}", String::from_utf8(out.stderr)?);
    let out = String::from_utf8(out.stdout)?;
    println!("out: {out}");
    let mut v = Vec::new();
    for line in out.split("\n") {
        if !line.starts_with(">>") {
            continue;
        }
        let line = line.replace(">>", "");
        let val = line.parse::<T>().map_err(|e| anyhow!("parse error:"))?;
        v.push(val);
        println!("line: {line}");
    }
    Ok(v)
}

macro_rules! write_branch {
    ($ty:ty, $N:expr, $ty_cpp: expr) => {{
        let tys = stringify!($ty);
        // let ty_cpps = stringify!($ty_cpp);
        let Ns = stringify!($N);
        let out_dir = format!("{}/{}/{}", OUT_DIR, tys, $ty_cpp);
        fs::create_dir_all(&out_dir)?;
        let out_file = format!("{}/{Ns}.root", out_dir);
        fn gen_it() -> impl Iterator<Item = $ty> {
            (0..$N).map(|x| x as $ty)
        }

        {
            let mut f = oxyroot::RootFile::create(&out_file)?;
            let mut tree = oxyroot::WriterTree::new("mytree");

            tree.new_branch(tys, gen_it());
            tree.write(&mut f)?;
            f.close()?;
        }
        let out_macro = format!("{}/{Ns}.C", out_dir);
        let v = write_root_macro_for_simple_branch::<$ty>(&out_macro, $ty_cpp, tys)?;

        for (i, (r, w)) in v.into_iter().zip(gen_it()).enumerate() {
            assert_eq!(r, w);
        }
    }};
}
// fn write_branch<T, const N: usize>(ty: &str) -> Result<()>
// {
//     let out_dir = format!("{}/{ty}", OUT_DIR);
//     fs::create_dir_all(&out_dir)?;
//     let out_file = format!("{}/{N}.root", out_dir);
//
//     let it = (0..N).map(|x| x as  dyn Marshaler);
//
//     {
//         let mut f = oxyroot::RootFile::create(&out_file)?;
//         let mut tree = oxyroot::Tree::new("mytree");
//
//         tree.new_branch(ty, it);
//         tree.write(&mut f)?;
//         f.close()?;
//     }
//
//     let mut f = oxyroot::RootFile::open(out_file)?;
//     let tree = f.get_tree("mytree")?;
//     let mut b = tree.branch(ty).unwrap().as_iter::<T>();
//
//     let it = (0..N).map(|x| x as T);
//
//     for (i, (r, w)) in b.zip(it).enumerate() {
//         assert_eq!(r, w);
//     }
//
//     Ok(())
// }

#[test]
fn write_i32_branch_int() -> Result<()> {
    write_branch!(i32, 5, "int");
    Ok(())
}

#[test]
fn write_i32_branch_int32_t() -> Result<()> {
    write_branch!(i32, 5, "int32_t");
    Ok(())
}

#[test]
fn write_i32_branch_Int_t() -> Result<()> {
    write_branch!(i32, 15, "Int_t");
    Ok(())
}

#[test]
fn write_u32_branch_unsignedint() -> Result<()> {
    write_branch!(u32, 15, "unsigned int");
    Ok(())
}
#[test]
fn write_u32_branch_uint32_t() -> Result<()> {
    write_branch!(u32, 15, "uint32_t");
    Ok(())
}
#[test]
fn write_u32_branch_UInt_t() -> Result<()> {
    write_branch!(u32, 15, "UInt_t");
    Ok(())
}
#[test]
fn write_f32_branch_float() -> Result<()> {
    write_branch!(f32, 5, "float");
    Ok(())
}
#[test]
fn write_f32_branch_Float_t() -> Result<()> {
    write_branch!(f32, 5, "Float_t");
    Ok(())
}

#[test]
fn write_i16_branch_short() -> Result<()> {
    write_branch!(i16, 5, "short");
    Ok(())
}
#[test]
fn write_i16_branch_Short_t() -> Result<()> {
    write_branch!(i16, 5, "Short_t");
    Ok(())
}
#[test]
fn write_i16_branch_int16_t() -> Result<()> {
    write_branch!(i16, 5, "int16_t");
    Ok(())
}
#[test]
fn write_u16_branch_unsignedshort() -> Result<()> {
    write_branch!(u16, 5, "unsigned short");
    Ok(())
}
#[test]
fn write_u16_branch_uint16_t() -> Result<()> {
    write_branch!(u16, 5, "uint16_t");
    Ok(())
}
#[test]
fn write_u16_branch_UShort_t() -> Result<()> {
    write_branch!(u16, 5, "UShort_t");
    Ok(())
}

#[test]
fn write_i8_branch_Char_t() -> Result<()> {
    write_branch!(i8, 5, "Char_t");
    Ok(())
}

#[test]
fn write_i8_branch_char() -> Result<()> {
    write_branch!(i8, 5, "char");
    Ok(())
}

#[test]
fn write_u8_branch_unsignedchar() -> Result<()> {
    write_branch!(u8, 5, "unsigned char");
    // write_branch!(u8, 5, "uint8_t");
    Ok(())
}
#[test]
fn write_u8_branch_UChar_t() -> Result<()> {
    write_branch!(u8, 5, "UChar_t");
    // write_branch!(u8, 5, "uint8_t");
    Ok(())
}

#[test]
fn write_i64_branch_Long64_t() -> Result<()> {
    write_branch!(i64, 5, "Long64_t");
    Ok(())
}

#[test]
fn write_i64_branch_longlong() -> Result<()> {
    write_branch!(i64, 5, "long long");
    Ok(())
}

#[test]
fn write_u64_branch_ULong64_t() -> Result<()> {
    write_branch!(u64, 5, "ULong64_t");
    Ok(())
}

#[test]
fn write_f64_branch() -> Result<()> {
    write_branch!(f64, 5, "double");
    Ok(())
}

fn write_root_macro_for_vector_branch<T: std::str::FromStr + Debug>(
    out_file: &str,
    ty_cpp: &str,
    input_root: &str,
) -> Result<Vec<Vec<T>>> {
    let macro_content = format!(
        r#"

{{
    TFile *f = new TFile("{input_root}");
    TTree *t = (TTree*)f->Get("mytree");
    cout << "SIZEOF({ty_cpp}) = " << sizeof({ty_cpp}) << endl;
    vector<{ty_cpp}> *v = 0;
    t->SetBranchAddress("vector", &v);
    //cout << ">>ENTRIES: " << t->GetEntries() << endl;
    for (int i = 0; i < t->GetEntries(); i++)
    {{
        t->GetEntry(i);
        cout <<">>"<< v->size() << ";";
        for (int j = 0; j < v->size(); j++)
        {{
            cout << static_cast<int16_t>(v->at(j)) << " ";
        }}
        cout << endl;
    }}
    f->Close();
}}
"#,
    );
    println!("write to {:?}", out_file);
    fs::write(out_file, macro_content)?;

    let out = Command::new("root").arg("-q").arg(out_file).output()?;
    eprintln!("err: {}", String::from_utf8(out.stderr)?);
    let out = String::from_utf8(out.stdout)?;
    println!("out: {out}");
    let mut v = Vec::new();
    for line in out.split("\n") {
        if !line.starts_with(">>") {
            continue;
        }
        let line = line.replace(">>", "");

        let (nentries, data) = line.split_once(";").unwrap();
        let data = data.trim();
        println!("data: {data}");
        println!("nentries: {nentries}");

        let mut datav: Vec<T> = Vec::new();
        for s in data.split(" ") {
            if s.is_empty() {
                continue;
            }
            println!("try to parse '{:?}'", s);
            let val = s.parse::<T>().map_err(|e| anyhow!("parse error:"))?;
            datav.push(val);
        }

        v.push(datav);
        println!("line: {line}");
    }
    Ok(v)
}

macro_rules! write_branch_vector {
    ($ty_item:ty, $N:expr, $ty_cpp: expr) => {{
        let ty = "vector";
        let ty_item = stringify!($ty_item);
        let N = $N;
        let out_dir = format!("{}/{ty}/{ty_item}", OUT_DIR);
        fs::create_dir_all(&out_dir)?;
        let out_file = format!("{}/{N}.root", out_dir);

        fn make_vector(n: i32) -> Vec<$ty_item> {
            let mut ret = Vec::new();
            for i in 0..n {
                ret.push(i as $ty_item);
            }
            ret
        }

        {
            let it = (0..N).map(|x| make_vector(x));
            let mut f = oxyroot::RootFile::create(&out_file)?;
            let mut tree = oxyroot::WriterTree::new("mytree");
            tree.new_branch(ty, it);
            tree.write(&mut f)?;
            f.close()?;
        }

        let out_macro = format!("{}/{N}.C", out_dir);
        let v = write_root_macro_for_vector_branch::<$ty_item>(&out_macro, $ty_cpp, &out_file)?;

        let mut f = oxyroot::RootFile::open(out_file)?;
        let tree = f.get_tree("mytree")?;
        assert_eq!(tree.entries(), N.into());
        let mut b = tree.branch(ty).unwrap().as_iter::<Vec<$ty_item>>();

        let it = (0..N).map(|x| make_vector(x));

        for (i, (r, w)) in b.zip(it).enumerate() {
            assert_eq!(r, w);
        }
    }};
}

#[test]
fn write_vector_i32_int32_t() -> Result<()> {
    write_branch_vector!(i32, 50, "int32_t");
    Ok(())
}
#[test]
fn write_vector_u32_uint32_t() -> Result<()> {
    write_branch_vector!(u32, 50, "uint32_t");
    Ok(())
}

#[test]
fn write_vector_i16_int16_t() -> Result<()> {
    write_branch_vector!(i16, 50, "int16_t");
    Ok(())
}
#[test]
fn write_vector_u16_uint16_t() -> Result<()> {
    write_branch_vector!(u16, 50, "uint16_t");
    Ok(())
}
#[test]
fn write_vector_i8_int8_t() -> Result<()> {
    write_branch_vector!(i8, 50, "Char_t");
    Ok(())
}
#[test]
fn write_vector_u8_uint8_t() -> Result<()> {
    write_branch_vector!(u8, 50, "UChar_t");
    Ok(())
}

#[test]
fn write_bool_branch() -> Result<()> {
    let ty = "bool";
    let N = 5;
    let out_dir = format!("{}/{ty}", OUT_DIR);
    fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/{N}.root", out_dir);

    {
        let it = (0..N).map(|x| x.is_even());
        let mut f = oxyroot::RootFile::create(&out_file)?;
        let mut tree = oxyroot::WriterTree::new("mytree");

        tree.new_branch(ty, it);
        tree.write(&mut f)?;
        f.close()?;
    }

    let mut f = oxyroot::RootFile::open(out_file)?;
    let tree = f.get_tree("mytree")?;
    assert_eq!(tree.entries(), N.into());
    let mut b = tree.branch(ty).unwrap().as_iter::<bool>();

    let it = (0..N).map(|x| x.is_even());

    for (i, (r, w)) in b.zip(it).enumerate() {
        assert_eq!(r, w);
    }
    Ok(())
}

#[test]
fn write_string_branch() -> Result<()> {
    let ty = "String";
    let N = 5;
    let out_dir = format!("{}/{ty}", OUT_DIR);
    fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/{N}.root", out_dir);

    {
        let it = (0..N).map(|x| format!("string{}", x));
        let mut f = oxyroot::RootFile::create(&out_file)?;
        let mut tree = oxyroot::WriterTree::new("mytree");

        tree.new_branch(ty, it);
        tree.write(&mut f)?;
        f.close()?;
    }

    let mut f = oxyroot::RootFile::open(out_file)?;
    let tree = f.get_tree("mytree")?;
    assert_eq!(tree.entries(), N.into());
    let mut b = tree.branch(ty).unwrap().as_iter::<String>();

    let it = (0..N).map(|x| format!("string{}", x));

    for (i, (r, w)) in b.zip(it).enumerate() {
        assert_eq!(r, w);
    }
    Ok(())
}

#[test]
fn write_variable_lenght_string_branch() -> Result<()> {
    let ty = "String_l";
    let N = 500;
    let out_dir = format!("{}/{ty}", OUT_DIR);
    fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/{N}.root", out_dir);

    fn make_string(n: i32) -> String {
        let mut s = String::new();
        for i in 0..n {
            s.push_str(&format!("string{}", i));
        }
        s
    }

    {
        let it = (0..N).map(|x| make_string(x));
        let mut f = oxyroot::RootFile::create(&out_file)?;
        let mut tree = oxyroot::WriterTree::new("mytree");

        tree.new_branch(ty, it);
        tree.write(&mut f)?;
        f.close()?;
    }

    let mut f = oxyroot::RootFile::open(out_file)?;
    let tree = f.get_tree("mytree")?;
    assert_eq!(tree.entries(), N.into());
    let mut b = tree.branch(ty).unwrap().as_iter::<String>();

    let it = (0..N).map(|x| make_string(x));

    for (i, (r, w)) in b.zip(it).enumerate() {
        assert_eq!(r, w);
    }
    Ok(())
}

#[test]
fn write_array_branch() -> Result<()> {
    let ty = "array";
    let N = 15;
    let out_dir = format!("{}/{ty}", OUT_DIR);
    fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/{N}.root", out_dir);

    fn make_string(n: i32) -> [i32; 5] {
        [n, n + 1, n + 2, n + 3, n + 4]
    }

    {
        let it = (0..N).map(|x| make_string(x));
        let mut f = oxyroot::RootFile::create(&out_file)?;
        let mut tree = oxyroot::WriterTree::new("mytree");
        tree.new_branch(ty, it);
        tree.write(&mut f)?;
        f.close()?;
    }

    let mut f = oxyroot::RootFile::open(out_file)?;
    let tree = f.get_tree("mytree")?;
    assert_eq!(tree.entries(), N.into());
    let mut b = tree.branch(ty).unwrap().as_iter::<[i32; 5]>();

    let it = (0..N).map(|x| make_string(x));

    for (i, (r, w)) in b.zip(it).enumerate() {
        assert_eq!(r, w);
    }
    Ok(())
}
