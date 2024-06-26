mod common;

use anyhow::Result;
use common::TemplateWriter;
use oxyroot::RBuffer;

#[test]
fn read_i32_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/i32/int")?
        .with_value_type("int")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_i32().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

#[test]
fn read_u32_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/u32/UInt_t")?
        .with_value_type("UInt_t")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_u32().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i as u32, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

#[test]
fn read_i16_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/i16/short")?
        .with_value_type("short")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_i16().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

#[test]
fn read_u16_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/u16/short")?
        .with_value_type("short")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_u16().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i as u16, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

#[test]
fn read_i8_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/i8/Char_t")?
        .with_value_type("Char_t")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_i8().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

#[test]
fn read_i64_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/i64/Long64_t")?
        .with_value_type("Long64_t")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_i64().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

#[test]
fn read_u64_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/u64/ULong64_t")?
        .with_value_type("ULong64_t")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_u64().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.branch("v_i").unwrap().get_basket(parse);

    for ii in -10..10 {
        let i = ii as u64;
        assert_eq!(i, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

#[test]
fn read_f32_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/f32/float")?
        .with_value_type("float")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_f32().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i as f32, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
}

#[test]
fn read_f64_branch() -> Result<()> {
    let temp = TemplateWriter::default()
        .with_outdir("/tmp/rust/f64/double")?
        .with_value_type("double")?;

    temp.write_root_macro()?;
    temp.execute_macro()?;

    let parse = |r: &mut RBuffer| {
        let val = r.read_f64().unwrap();
        val
    };

    let tree = temp.tree()?;
    let mut b = tree.branch("v_i").unwrap().get_basket(parse);

    for i in -10..10 {
        assert_eq!(i as f64, b.next().unwrap());
    }

    assert!(b.next().is_none());
    Ok(())
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
            let mut b = tree.branch("v_i").unwrap().as_iter::<$ftype>()?;

            for i in -10..10 {
                assert_eq!(i as $ftype, b.next().unwrap());
            }

            assert!(b.next().is_none());
            Ok(())
        }

        impl_read_primitive_branch()?;
    }};
}

#[test]
#[allow(non_snake_case)]
fn read_tree_into_i8_Char_t() -> Result<()> {
    read_primitive_branch!(i8, "Char_t");
    Ok(())
}

#[test]
#[allow(non_snake_case)]
fn read_tree_into_u8_Char_t() -> Result<()> {
    read_primitive_branch!(u8, "UChar_t");
    Ok(())
}

#[test]
#[allow(non_snake_case)]
fn read_tree_into_i32_Int_t() -> Result<()> {
    read_primitive_branch!(i32, "Int_t");
    Ok(())
}

#[test]
fn read_tree_into_i32_int() -> Result<()> {
    read_primitive_branch!(i32, "int");
    Ok(())
}

#[test]
fn read_tree_into_i32_signed() -> Result<()> {
    read_primitive_branch!(i32, "signed");
    Ok(())
}

#[test]
#[allow(non_snake_case)]
fn read_tree_into_u32_UInt_t() -> Result<()> {
    read_primitive_branch!(u32, "UInt_t");
    Ok(())
}
#[test]
fn read_tree_into_u32_uint32_t() -> Result<()> {
    read_primitive_branch!(u32, "uint32_t");
    Ok(())
}

#[test]
#[allow(non_snake_case)]
fn read_tree_into_f32_Float_t() -> Result<()> {
    read_primitive_branch!(f32, "Float_t");
    Ok(())
}

#[test]
#[allow(non_snake_case)]
fn read_tree_into_f64_Double_t() -> Result<()> {
    read_primitive_branch!(f64, "Double_t");
    Ok(())
}
