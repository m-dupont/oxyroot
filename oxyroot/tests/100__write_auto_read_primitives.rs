use anyhow::Result;
use num::Integer;
use oxyroot;
use std::fs;
use test_log::test;

const OUT_DIR: &str = "/tmp/rust/write/";

macro_rules! write_branch {
    ($ty:ty, $N:expr) => {{
        let tys = stringify!($ty);
        let ns = stringify!($N);
        let out_dir = format!("{}/{}", OUT_DIR, tys);
        fs::create_dir_all(&out_dir)?;
        let out_file = format!("{}/{ns}.root", out_dir);
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

        let mut f = oxyroot::RootFile::open(out_file)?;
        let tree = f.get_tree("mytree")?;
        assert_eq!(tree.entries(), $N.into());
        let b = tree.branch(tys).unwrap().as_iter::<$ty>()?;

        for (_i, (r, w)) in b.zip(gen_it()).enumerate() {
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
fn write_i32_branch() -> Result<()> {
    write_branch!(i32, 5);
    write_branch!(i32, 15);
    write_branch!(u32, 15);
    Ok(())
}
#[test]
fn write_f32_branch() -> Result<()> {
    write_branch!(f32, 5);
    Ok(())
}

#[test]
fn write_i16_branch() -> Result<()> {
    write_branch!(i16, 5);
    write_branch!(u16, 5);
    Ok(())
}
#[test]
fn write_i8_branch() -> Result<()> {
    write_branch!(i8, 5);
    write_branch!(u8, 5);
    Ok(())
}

#[test]
fn write_i64_branch() -> Result<()> {
    write_branch!(i64, 5);
    write_branch!(u64, 5);
    Ok(())
}

#[test]
fn write_f64_branch() -> Result<()> {
    write_branch!(f64, 5);
    Ok(())
}

#[test]
fn write_bool_branch() -> Result<()> {
    let ty = "bool";
    let n = 5;
    let out_dir = format!("{}/{ty}", OUT_DIR);
    fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/{n}.root", out_dir);

    {
        let it = (0..n).map(|x| x.is_even());
        let mut f = oxyroot::RootFile::create(&out_file)?;
        let mut tree = oxyroot::WriterTree::new("mytree");

        tree.new_branch(ty, it);
        tree.write(&mut f)?;
        f.close()?;
    }

    let mut f = oxyroot::RootFile::open(out_file)?;
    let tree = f.get_tree("mytree")?;
    assert_eq!(tree.entries(), n.into());
    let mut b = tree.branch(ty).unwrap().as_iter::<bool>()?;

    let it = (0..n).map(|x| x.is_even());

    for (i, (r, w)) in b.zip(it).enumerate() {
        assert_eq!(r, w);
    }
    Ok(())
}

#[test]
fn write_string_branch() -> Result<()> {
    let ty = "String";
    let n = 5;
    let out_dir = format!("{}/{ty}", OUT_DIR);
    fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/{n}.root", out_dir);

    {
        let it = (0..n).map(|x| format!("string{}", x));
        let mut f = oxyroot::RootFile::create(&out_file)?;
        let mut tree = oxyroot::WriterTree::new("mytree");

        tree.new_branch(ty, it);
        tree.write(&mut f)?;
        f.close()?;
    }

    let mut f = oxyroot::RootFile::open(out_file)?;
    let tree = f.get_tree("mytree")?;
    assert_eq!(tree.entries(), n.into());
    let b = tree.branch(ty).unwrap().as_iter::<String>()?;

    let it = (0..n).map(|x| format!("string{}", x));

    for (_i, (r, w)) in b.zip(it).enumerate() {
        assert_eq!(r, w);
    }
    Ok(())
}

#[test]
fn write_variable_lenght_string_branch() -> Result<()> {
    let ty = "String_l";
    let n = 500;
    let out_dir = format!("{}/{ty}", OUT_DIR);
    fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/{n}.root", out_dir);

    fn make_string(n: i32) -> String {
        let mut s = String::new();
        for i in 0..n {
            s.push_str(&format!("string{}", i));
        }
        s
    }

    {
        let it = (0..n).map(|x| make_string(x));
        let mut f = oxyroot::RootFile::create(&out_file)?;
        let mut tree = oxyroot::WriterTree::new("mytree");

        tree.new_branch(ty, it);
        tree.write(&mut f)?;
        f.close()?;
    }

    let mut f = oxyroot::RootFile::open(out_file)?;
    let tree = f.get_tree("mytree")?;
    assert_eq!(tree.entries(), n.into());
    let b = tree.branch(ty).unwrap().as_iter::<String>()?;

    let it = (0..n).map(|x| make_string(x));

    for (_i, (r, w)) in b.zip(it).enumerate() {
        assert_eq!(r, w);
    }
    Ok(())
}

#[test]
fn write_array_branch() -> Result<()> {
    let ty = "array";
    let n = 15;
    let out_dir = format!("{}/{ty}", OUT_DIR);
    fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/{n}.root", out_dir);

    fn make_string(n: i32) -> [i32; 5] {
        [n, n + 1, n + 2, n + 3, n + 4]
    }

    {
        let it = (0..n).map(|x| make_string(x));
        let mut f = oxyroot::RootFile::create(&out_file)?;
        let mut tree = oxyroot::WriterTree::new("mytree");
        tree.new_branch(ty, it);
        tree.write(&mut f)?;
        f.close()?;
    }

    let mut f = oxyroot::RootFile::open(out_file)?;
    let tree = f.get_tree("mytree")?;
    assert_eq!(tree.entries(), n.into());
    let b = tree.branch(ty).unwrap().as_iter::<[i32; 5]>()?;

    let it = (0..n).map(|x| make_string(x));

    for (_i, (r, w)) in b.zip(it).enumerate() {
        assert_eq!(r, w);
    }
    Ok(())
}

macro_rules! write_branch_vector {
    ($ty_item:ty, $N:expr) => {{
        let ty = "vector";
        let ty_item = stringify!($ty_item);
        let n = $N;
        let out_dir = format!("{}/{ty}/{ty_item}", OUT_DIR);
        fs::create_dir_all(&out_dir)?;
        let out_file = format!("{}/{n}.root", out_dir);

        fn make_vector(n: i32) -> Vec<$ty_item> {
            let mut ret = Vec::new();
            for i in 0..n {
                ret.push(i as $ty_item);
            }
            ret
        }

        {
            let it = (0..n).map(|x| make_vector(x));
            let mut f = oxyroot::RootFile::create(&out_file)?;
            let mut tree = oxyroot::WriterTree::new("mytree");
            tree.new_branch(ty, it);
            tree.write(&mut f)?;
            f.close()?;
        }

        let mut f = oxyroot::RootFile::open(out_file)?;
        let tree = f.get_tree("mytree")?;
        assert_eq!(tree.entries(), n.into());
        let b = tree.branch(ty).unwrap().as_iter::<Vec<$ty_item>>()?;

        let it = (0..n).map(|x| make_vector(x));

        for (_i, (r, w)) in b.zip(it).enumerate() {
            assert_eq!(r, w);
        }
    }};
}

#[test]
fn write_vector_i32_5() -> Result<()> {
    write_branch_vector!(i32, 5);
    Ok(())
}

#[test]
fn write_vector_i32_100() -> Result<()> {
    write_branch_vector!(i32, 100);
    Ok(())
}

#[test]
fn write_vector_i16_100() -> Result<()> {
    write_branch_vector!(i16, 100);
    Ok(())
}

#[test]
fn write_vector_i8_100() -> Result<()> {
    write_branch_vector!(i8, 100);
    Ok(())
}

#[test]
fn write_vector_i64_100() -> Result<()> {
    write_branch_vector!(i64, 100);
    Ok(())
}

#[test]
fn write_vector_u32_100() -> Result<()> {
    write_branch_vector!(u32, 100);
    Ok(())
}

#[test]
fn write_vector_u16_100() -> Result<()> {
    write_branch_vector!(u16, 100);
    Ok(())
}

#[test]
fn write_vector_u8_100() -> Result<()> {
    write_branch_vector!(u8, 100);
    Ok(())
}

#[test]
fn write_vector_u64_100() -> Result<()> {
    write_branch_vector!(u64, 100);
    Ok(())
}

#[test]
fn write_vector_f32_100() -> Result<()> {
    write_branch_vector!(f32, 100);
    Ok(())
}

#[test]
fn write_vector_f64_100() -> Result<()> {
    write_branch_vector!(f64, 100);
    Ok(())
}
