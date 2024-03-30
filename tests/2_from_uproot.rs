use anyhow::Result;
use log::trace;
use oxyroot::{RBuffer, RootFile, SizedSlice, Slice, Unmarshaler, UnmarshalerInto};
use std::fmt::Debug;
use std::path::Path;

#[test]
#[allow(non_snake_case)]
fn open_nested() -> Result<()> {
    let s = "examples/from_uproot/data/HZZ.root";

    let tree = RootFile::open(s)?.get_tree("events")?;
    let NJet = tree.branch("NJet").unwrap().as_iter::<i32>();

    assert_eq!(tree.branch("NJet").unwrap().interpretation(), "i32");

    let n = NJet.count();
    assert_eq!(n, 2421);

    assert_eq!(
        tree.branch("Jet_Py").unwrap().interpretation(),
        "Slice<f32>"
    );

    let Jet_Py = tree.branch("Jet_Py").unwrap().as_iter::<Slice<f32>>();
    assert_eq!(Jet_Py.count(), 2421);

    let nelectron = tree.branch("NElectron").unwrap().as_iter::<i32>();
    assert_eq!(nelectron.count(), 2421);

    let photon_e = tree.branch("Photon_E").unwrap().as_iter::<Slice<f32>>();
    assert_eq!(photon_e.count(), 2421);

    let Electron_Px = tree.branch("Electron_Px").unwrap().as_iter::<Slice<f32>>();
    assert_eq!(Electron_Px.count(), 2421);

    Ok(())
}

#[test]
fn open_simple_root() -> Result<()> {
    let s = "examples/from_uproot/data/simple.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;

    assert_eq!(tree.branch("one").unwrap().item_type_name(), "int32_t");
    assert_eq!(tree.branch("two").unwrap().item_type_name(), "float");
    assert_eq!(tree.branch("three").unwrap().item_type_name(), "char*");
    assert_eq!(tree.branch("one").unwrap().interpretation(), "i32");
    assert_eq!(tree.branch("two").unwrap().interpretation(), "f32");
    assert_eq!(tree.branch("three").unwrap().interpretation(), "String");

    let one = tree
        .branch("one")
        .unwrap()
        .as_iter::<i32>()
        .collect::<Vec<_>>();

    assert_eq!(one, [1, 2, 3, 4]);

    let two = tree
        .branch("two")
        .unwrap()
        .as_iter::<f32>()
        .collect::<Vec<_>>();

    assert_eq!(two, [1.1, 2.2, 3.3, 4.4]);

    let three = tree
        .branch("three")
        .unwrap()
        .as_iter::<String>()
        .collect::<Vec<_>>();

    assert_eq!(three, ["uno", "dos", "tres", "quatro"]);

    Ok(())
}

#[test]
fn open_tree_with_string() -> Result<()> {
    let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    let tree = f.get_tree("tree")?;

    assert_eq!(tree.branch("Beg").unwrap().item_type_name(), "TString");
    assert_eq!(tree.branch("End").unwrap().item_type_name(), "TString");

    assert_eq!(tree.branch("Beg").unwrap().interpretation(), "TString");
    assert_eq!(tree.branch("End").unwrap().interpretation(), "TString");

    tree.branch("Beg")
        .unwrap()
        .as_iter::<String>()
        .enumerate()
        .for_each(|(i, s)| {
            assert_eq!(s, format!("beg-{:03}", i));
        });

    tree.branch("End")
        .unwrap()
        .as_iter::<String>()
        .enumerate()
        .for_each(|(i, s)| {
            assert_eq!(s, format!("end-{:03}", i));
        });

    Ok(())
}

#[test]
fn open_tree_with_stl_string() -> Result<()> {
    let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    let tree = f.get_tree("tree")?;

    assert_eq!(tree.branch("StdStr").unwrap().item_type_name(), "string");
    assert_eq!(tree.branch("StdStr").unwrap().interpretation(), "String");

    tree.branch("StdStr")
        .unwrap()
        .as_iter::<String>()
        .enumerate()
        .for_each(|(i, s)| {
            assert_eq!(s, format!("std-{:03}", i));
        });

    Ok(())
}

#[test]
fn open_tree_with_struct_p3() -> Result<()> {
    // P3 <=> P3 { x: i32, y: f64, z: i32}
    // Stored in three branches P3.x, P3.y, and P3.z
    // tree.branch("P3") will zip the three branches

    let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    let tree = f.get_tree("tree")?;
    let tree = tree;

    assert_eq!(tree.branch("P3").unwrap().item_type_name(), "P3");
    assert_eq!(tree.branch("P3.Px").unwrap().item_type_name(), "int32_t");
    assert_eq!(tree.branch("P3.Py").unwrap().item_type_name(), "double");
    assert_eq!(tree.branch("P3.Pz").unwrap().item_type_name(), "int32_t");
    assert_eq!(tree.branch("P3").unwrap().interpretation(), "P3");
    assert_eq!(tree.branch("P3.Px").unwrap().interpretation(), "i32");
    assert_eq!(tree.branch("P3.Py").unwrap().interpretation(), "f64");
    assert_eq!(tree.branch("P3.Pz").unwrap().interpretation(), "i32");

    tree.branch("P3")
        .unwrap()
        .get_basket(|r| {
            let x = r.read_i32().unwrap();
            let y = r.read_f64().unwrap();
            let z = r.read_i32().unwrap();
            (x, y, z)
        })
        .enumerate()
        .for_each(|(i, (x, y, z))| {
            // trace!("x = {x} y = {y}");
            let i = i as i32;
            assert_eq!(x, i - 1);
            assert_eq!(x, z);
            assert_eq!(y, i as f64);
        });

    Ok(())
}

#[test]
fn open_tree_with_vector_parse() -> Result<()> {
    let s = "examples/from_uproot/data/tree_with_jagged_array.root";

    let mut f = RootFile::open(s)?;

    f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("t1")?;

    assert_eq!(
        tree.branch("int32_array").unwrap().item_type_name(),
        "vector<int32_t>"
    );

    tree.branch("int32_array")
        .unwrap()
        .get_basket(|r| {
            r.do_skip_header().unwrap();
            let size = r.read_i32().unwrap();
            let mut ret: Vec<i32> = Vec::new();
            for _ in 0..size {
                let a = r.read_object_into::<i32>().unwrap();
                trace!("\t a = {:?}", a);
                ret.push(a);
            }
            ret
        })
        .enumerate()
        .for_each(|(i, val)| {
            assert_eq!(val.len(), i % 10);
            val.iter()
                .enumerate()
                .map(|(j, v)| {
                    assert_eq!(*v, (i - i % 10 + j) as i32);
                })
                .for_each(drop);
        });
    Ok(())
}

#[test]
fn open_tree_with_vector_into() -> Result<()> {
    let s = "examples/from_uproot/data/tree_with_jagged_array.root";

    let mut f = RootFile::open(s)?;
    f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);
    let tree = f.get_tree("t1")?;

    assert_eq!(
        tree.branch("int32_array").unwrap().interpretation(),
        "Vec<i32>"
    );

    tree.branch("int32_array")
        .unwrap()
        .as_iter::<Vec<i32>>()
        .enumerate()
        .for_each(|(i, val)| {
            assert_eq!(val.len(), i % 10);
            val.iter()
                .enumerate()
                .map(|(j, v)| {
                    assert_eq!(*v, (i - i % 10 + j) as i32);
                })
                .for_each(drop);
        });
    Ok(())
}

#[test]
fn open_tree_with_slice_i16() -> Result<()> {
    let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";

    let mut f = RootFile::open(s)?;

    f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;

    assert_eq!(
        tree.branch("SliceI16").unwrap().item_type_name(),
        "int16_t[]"
    );
    assert_eq!(
        tree.branch("SliceI32").unwrap().item_type_name(),
        "int32_t[]"
    );
    assert_eq!(
        tree.branch("SliceI64").unwrap().item_type_name(),
        "int64_t[]"
    );

    assert_eq!(
        tree.branch("SliceU16").unwrap().item_type_name(),
        "uint16_t[]"
    );
    assert_eq!(
        tree.branch("SliceU32").unwrap().item_type_name(),
        "uint32_t[]"
    );
    assert_eq!(
        tree.branch("SliceU64").unwrap().item_type_name(),
        "uint64_t[]"
    );

    assert_eq!(tree.branch("SliceF32").unwrap().item_type_name(), "float[]");
    assert_eq!(
        tree.branch("SliceF64").unwrap().item_type_name(),
        "double[]"
    );

    assert_eq!(
        tree.branch("SliceI16").unwrap().interpretation(),
        "Slice<i16>"
    );
    assert_eq!(
        tree.branch("SliceI32").unwrap().interpretation(),
        "Slice<i32>"
    );
    assert_eq!(
        tree.branch("SliceI64").unwrap().interpretation(),
        "Slice<i64>"
    );

    assert_eq!(
        tree.branch("SliceU16").unwrap().interpretation(),
        "Slice<u16>"
    );
    assert_eq!(
        tree.branch("SliceU32").unwrap().interpretation(),
        "Slice<u32>"
    );
    assert_eq!(
        tree.branch("SliceU64").unwrap().interpretation(),
        "Slice<u64>"
    );

    assert_eq!(
        tree.branch("SliceF32").unwrap().interpretation(),
        "Slice<f32>"
    );
    assert_eq!(
        tree.branch("SliceF64").unwrap().interpretation(),
        "Slice<f64>"
    );

    tree.branch("SliceI16")
        .unwrap()
        .as_iter::<oxyroot::Slice<i16>>()
        .map(|a| a.into_vec())
        .enumerate()
        .for_each(|(i, val)| {
            assert_eq!(val.len(), i % 10);

            val.into_iter()
                .map(|v| {
                    assert_eq!(v, i as i16);
                })
                .for_each(drop)
        });

    Ok(())
}

#[test]
fn open_tree_with_slice_i16_into_vec() -> Result<()> {
    let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";

    let mut f = RootFile::open(s)?;

    f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;

    let v: Vec<Vec<i16>> = tree
        .branch("SliceI16")
        .unwrap()
        .as_iter::<oxyroot::Slice<i16>>()
        .map(|s| s.into())
        .collect();

    let good = [
        vec![],
        vec![1],
        vec![2, 2],
        vec![3, 3, 3],
        vec![4, 4, 4, 4],
        vec![5, 5, 5, 5, 5],
        vec![6, 6, 6, 6, 6, 6],
        vec![7, 7, 7, 7, 7, 7, 7],
        vec![8, 8, 8, 8, 8, 8, 8, 8],
        vec![9, 9, 9, 9, 9, 9, 9, 9, 9],
        vec![],
        vec![11],
        vec![12, 12],
        vec![13, 13, 13],
        vec![14, 14, 14, 14],
        vec![15, 15, 15, 15, 15],
        vec![16, 16, 16, 16, 16, 16],
        vec![17, 17, 17, 17, 17, 17, 17],
        vec![18, 18, 18, 18, 18, 18, 18, 18],
        vec![19, 19, 19, 19, 19, 19, 19, 19, 19],
        vec![],
        vec![21],
        vec![22, 22],
        vec![23, 23, 23],
        vec![24, 24, 24, 24],
        vec![25, 25, 25, 25, 25],
        vec![26, 26, 26, 26, 26, 26],
        vec![27, 27, 27, 27, 27, 27, 27],
        vec![28, 28, 28, 28, 28, 28, 28, 28],
        vec![29, 29, 29, 29, 29, 29, 29, 29, 29],
        vec![],
        vec![31],
        vec![32, 32],
        vec![33, 33, 33],
        vec![34, 34, 34, 34],
        vec![35, 35, 35, 35, 35],
        vec![36, 36, 36, 36, 36, 36],
        vec![37, 37, 37, 37, 37, 37, 37],
        vec![38, 38, 38, 38, 38, 38, 38, 38],
        vec![39, 39, 39, 39, 39, 39, 39, 39, 39],
        vec![],
        vec![41],
        vec![42, 42],
        vec![43, 43, 43],
        vec![44, 44, 44, 44],
        vec![45, 45, 45, 45, 45],
        vec![46, 46, 46, 46, 46, 46],
        vec![47, 47, 47, 47, 47, 47, 47],
        vec![48, 48, 48, 48, 48, 48, 48, 48],
        vec![49, 49, 49, 49, 49, 49, 49, 49, 49],
        vec![],
        vec![51],
        vec![52, 52],
        vec![53, 53, 53],
        vec![54, 54, 54, 54],
        vec![55, 55, 55, 55, 55],
        vec![56, 56, 56, 56, 56, 56],
        vec![57, 57, 57, 57, 57, 57, 57],
        vec![58, 58, 58, 58, 58, 58, 58, 58],
        vec![59, 59, 59, 59, 59, 59, 59, 59, 59],
        vec![],
        vec![61],
        vec![62, 62],
        vec![63, 63, 63],
        vec![64, 64, 64, 64],
        vec![65, 65, 65, 65, 65],
        vec![66, 66, 66, 66, 66, 66],
        vec![67, 67, 67, 67, 67, 67, 67],
        vec![68, 68, 68, 68, 68, 68, 68, 68],
        vec![69, 69, 69, 69, 69, 69, 69, 69, 69],
        vec![],
        vec![71],
        vec![72, 72],
        vec![73, 73, 73],
        vec![74, 74, 74, 74],
        vec![75, 75, 75, 75, 75],
        vec![76, 76, 76, 76, 76, 76],
        vec![77, 77, 77, 77, 77, 77, 77],
        vec![78, 78, 78, 78, 78, 78, 78, 78],
        vec![79, 79, 79, 79, 79, 79, 79, 79, 79],
        vec![],
        vec![81],
        vec![82, 82],
        vec![83, 83, 83],
        vec![84, 84, 84, 84],
        vec![85, 85, 85, 85, 85],
        vec![86, 86, 86, 86, 86, 86],
        vec![87, 87, 87, 87, 87, 87, 87],
        vec![88, 88, 88, 88, 88, 88, 88, 88],
        vec![89, 89, 89, 89, 89, 89, 89, 89, 89],
        vec![],
        vec![91],
        vec![92, 92],
        vec![93, 93, 93],
        vec![94, 94, 94, 94],
        vec![95, 95, 95, 95, 95],
        vec![96, 96, 96, 96, 96, 96],
        vec![97, 97, 97, 97, 97, 97, 97],
        vec![98, 98, 98, 98, 98, 98, 98, 98],
        vec![99, 99, 99, 99, 99, 99, 99, 99, 99],
    ];

    assert_eq!(v, good);

    Ok(())
}

#[test]
fn open_tree_with_vector_of_string() -> Result<()> {
    let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";

    let mut f = RootFile::open(s)?;

    f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;

    assert_eq!(
        tree.branch("StlVecStr").unwrap().item_type_name(),
        "vector<string>"
    );
    assert_eq!(
        tree.branch("StlVecStr").unwrap().interpretation(),
        "Vec<String>"
    );

    tree.branch("StlVecStr")
        .unwrap()
        .as_iter::<Vec<String>>()
        .enumerate()
        .for_each(|(i, val)| {
            trace!("StlVecStr: i = {i} val = {:?}", val);
            assert_eq!(val.len(), i % 10);

            val.into_iter()
                .map(|v| {
                    assert_eq!(v, format!("vec-{:03}", i));
                })
                .for_each(drop)
        });

    Ok(())
}

#[test]
fn tree_with_array() -> Result<()> {
    let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";

    let mut f = RootFile::open(s)?;

    f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;

    assert_eq!(
        tree.branch("ArrayI16[10]").unwrap().item_type_name(),
        "int16_t[10]"
    );
    assert_eq!(
        tree.branch("ArrayI16[10]").unwrap().interpretation(),
        "[i16;10]"
    );

    tree.branch("ArrayI16[10]")
        .unwrap()
        .get_basket(|r| {
            let mut buf = [0 as i16; 10];
            r.read_array_i16_into(&mut buf).unwrap();
            buf
        })
        .enumerate()
        .for_each(|(i, buf)| {
            // trace!("buf = {:?}", buf);
            buf.map(|v| assert_eq!(v, i as i16));
        });

    tree.branch("ArrayI16[10]")
        .unwrap()
        .as_iter::<[i16; 10]>()
        .enumerate()
        .for_each(|(i, buf)| {
            // trace!("buf = {:?}", buf);
            buf.map(|v| assert_eq!(v, i as i16));
        });

    Ok(())
}

fn open_tree_with_vector_primitive<T>(name_branch: &str) -> Result<()>
where
    T: UnmarshalerInto<Item = T> + Debug + PartialEq + TryFrom<u8>,
    <T as TryFrom<u8>>::Error: Debug,
{
    let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";

    let mut f = RootFile::open(s)?;
    f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);
    let tree = f.get_tree("tree")?;
    tree.branch(name_branch)
        .unwrap()
        .as_iter::<Vec<T>>()
        .enumerate()
        .for_each(|(i, val)| {
            assert_eq!(val.len(), i % 10);
            val.iter()
                .enumerate()
                .map(|(_j, v)| {
                    let calcul = i as u8;
                    let calcul = T::try_from(calcul).unwrap();
                    assert_eq!(*v, calcul);
                })
                .for_each(drop);
        });
    Ok(())
}

#[test]
fn open_tree_with_vector_i16() -> Result<()> {
    open_tree_with_vector_primitive::<i16>("StlVecI16")
}

#[test]
fn open_tree_with_vector_u16() -> Result<()> {
    open_tree_with_vector_primitive::<u16>("StlVecU16")
}

#[test]
fn open_tree_with_vector_i32() -> Result<()> {
    open_tree_with_vector_primitive::<i32>("StlVecI32")
}

#[test]
fn open_tree_with_vector_u32() -> Result<()> {
    open_tree_with_vector_primitive::<u32>("StlVecU32")
}

#[test]
fn open_tree_with_vector_i64() -> Result<()> {
    open_tree_with_vector_primitive::<i64>("StlVecI64")
}

#[test]
fn open_tree_with_vector_u64() -> Result<()> {
    open_tree_with_vector_primitive::<u64>("StlVecU64")
}

#[test]
fn open_tree_with_vector_f32() -> Result<()> {
    open_tree_with_vector_primitive::<f32>("StlVecF32")
}

#[test]
fn open_tree_with_vector_f64() -> Result<()> {
    open_tree_with_vector_primitive::<f64>("StlVecF64")
}

const SAMPLE_PATH: [&'static str; 4] = [
    "tests_data/from_uproot/sample-6.10.05-uncompressed.root",
    "tests_data/from_uproot/sample-6.10.05-zlib.root",
    "tests_data/from_uproot/sample-6.10.05-lzma.root",
    "tests_data/from_uproot/sample-6.10.05-lz4.root",
];

fn open_sample_n_impl<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut f = RootFile::open(path)?;
    // f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);
    let tree = f.get_tree("sample")?;
    assert_eq!(tree.entries(), 30);
    let b = tree.branch("n").unwrap().as_iter::<i32>();
    assert_eq!(b.count(), tree.entries() as usize);

    let b = tree.branch("n").unwrap().as_iter::<i32>();
    b.enumerate().for_each(|(i, val)| {
        assert_eq!(val, (i % 5) as i32);
    });

    Ok(())
}

#[test]
fn open_sample_n() -> Result<()> {
    for s in SAMPLE_PATH {
        open_sample_n_impl(s)?;
    }
    Ok(())
}

fn open_sample_b_impl<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut f = RootFile::open(path)?;
    // f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);
    let tree = f.get_tree("sample")?;
    assert_eq!(tree.entries(), 30);
    let branch_name = "b";
    let b = tree.branch(branch_name).unwrap().as_iter::<bool>();
    assert_eq!(b.count(), tree.entries() as usize);

    let vv: Vec<_> = tree
        .branch(branch_name)
        .unwrap()
        .as_iter::<bool>()
        .collect();
    let vgood = vec![
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false,
    ];
    assert_eq!(vv, vgood);

    Ok(())
}

#[test]
fn open_sample_b() -> Result<()> {
    for s in SAMPLE_PATH {
        open_sample_b_impl(s)?;
    }
    Ok(())
}

fn open_sample_i4_impl<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut f = RootFile::open(path)?;
    // f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);
    let tree = f.get_tree("sample")?;
    assert_eq!(tree.entries(), 30);
    let branch_name = "i4";
    let b = tree.branch(branch_name).unwrap().as_iter::<i32>();
    assert_eq!(b.count(), tree.entries() as usize);

    let vv: Vec<_> = tree.branch(branch_name).unwrap().as_iter::<i32>().collect();
    let vgood = vec![
        -15, -14, -13, -12, -11, -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14,
    ];
    assert_eq!(vv, vgood);

    Ok(())
}

#[test]
fn open_sample_i4() -> Result<()> {
    for s in SAMPLE_PATH {
        open_sample_i4_impl(s)?;
    }
    Ok(())
}

fn open_sample_ab_impl<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut f = RootFile::open(path)?;
    // f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);
    let tree = f.get_tree("sample")?;
    assert_eq!(tree.entries(), 30);
    let branch_name = "ab";
    let b = tree.branch(branch_name).unwrap().as_iter::<[bool; 3]>();
    assert_eq!(b.count(), tree.entries() as usize);

    let vv: Vec<_> = tree
        .branch(branch_name)
        .unwrap()
        .as_iter::<[bool; 3]>()
        .collect();
    let vgood = vec![
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
    ];

    for (v, vg) in vv.iter().zip(vgood) {
        assert_eq!(v, vg.as_slice());
    }

    Ok(())
}

#[test]
fn open_sample_ab() -> Result<()> {
    for s in SAMPLE_PATH {
        open_sample_ab_impl(s)?;
    }
    Ok(())
}

fn open_sample_ai4_impl<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut f = RootFile::open(path)?;
    // f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);
    let tree = f.get_tree("sample")?;
    assert_eq!(tree.entries(), 30);
    let branch_name = "ai4";
    let b = tree.branch(branch_name).unwrap().as_iter::<[i32; 3]>();
    assert_eq!(b.count(), tree.entries() as usize);

    let vv: Vec<_> = tree
        .branch(branch_name)
        .unwrap()
        .as_iter::<[i32; 3]>()
        .collect();
    let vgood = vec![
        vec![-14, -13, -12],
        vec![-13, -12, -11],
        vec![-12, -11, -10],
        vec![-11, -10, -9],
        vec![-10, -9, -8],
        vec![-9, -8, -7],
        vec![-8, -7, -6],
        vec![-7, -6, -5],
        vec![-6, -5, -4],
        vec![-5, -4, -3],
        vec![-4, -3, -2],
        vec![-3, -2, -1],
        vec![-2, -1, 0],
        vec![-1, 0, 1],
        vec![0, 1, 2],
        vec![1, 2, 3],
        vec![2, 3, 4],
        vec![3, 4, 5],
        vec![4, 5, 6],
        vec![5, 6, 7],
        vec![6, 7, 8],
        vec![7, 8, 9],
        vec![8, 9, 10],
        vec![9, 10, 11],
        vec![10, 11, 12],
        vec![11, 12, 13],
        vec![12, 13, 14],
        vec![13, 14, 15],
        vec![14, 15, 16],
        vec![15, 16, 17],
    ];

    for (v, vg) in vv.iter().zip(vgood) {
        assert_eq!(v, vg.as_slice());
    }

    Ok(())
}

#[test]
fn open_sample_ai4() -> Result<()> {
    for s in SAMPLE_PATH {
        open_sample_ai4_impl(s)?;
    }
    Ok(())
}

fn open_sample_ab_sizedslice_impl<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut f = RootFile::open(path)?;
    // f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);
    let tree = f.get_tree("sample")?;
    assert_eq!(tree.entries(), 30);
    let branch_name = "ab";
    let b = tree.branch(branch_name).unwrap().as_iter::<[bool; 3]>();
    assert_eq!(b.count(), tree.entries() as usize);

    let f = move |r: &mut RBuffer| {
        let mut s = SizedSlice::<bool>::new(3);
        s.unmarshal(r).unwrap();
        s
    };

    let vv: Vec<_> = tree.branch(branch_name).unwrap().get_basket(f).collect();
    let vgood = vec![
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
    ];

    for (v, vg) in vv.into_iter().zip(vgood) {
        let v: Vec<_> = v.into();
        assert_eq!(v, vg);
    }

    Ok(())
}

#[test]
fn open_sample_ab_sizedslice() -> Result<()> {
    for s in SAMPLE_PATH {
        open_sample_ab_sizedslice_impl(s)?;
    }
    Ok(())
}

fn open_sample_ai4_sizedslice_impl<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut f = RootFile::open(path)?;
    // f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);
    let tree = f.get_tree("sample")?;
    assert_eq!(tree.entries(), 30);
    let branch_name = "ai4";
    let b = tree.branch(branch_name).unwrap().as_iter::<[i32; 3]>();
    assert_eq!(b.count(), tree.entries() as usize);

    let f = move |r: &mut RBuffer| {
        let mut s = SizedSlice::<i32>::new(3);
        s.unmarshal(r).unwrap();
        s
    };

    let vv: Vec<_> = tree.branch(branch_name).unwrap().get_basket(f).collect();
    let vgood = vec![
        vec![-14, -13, -12],
        vec![-13, -12, -11],
        vec![-12, -11, -10],
        vec![-11, -10, -9],
        vec![-10, -9, -8],
        vec![-9, -8, -7],
        vec![-8, -7, -6],
        vec![-7, -6, -5],
        vec![-6, -5, -4],
        vec![-5, -4, -3],
        vec![-4, -3, -2],
        vec![-3, -2, -1],
        vec![-2, -1, 0],
        vec![-1, 0, 1],
        vec![0, 1, 2],
        vec![1, 2, 3],
        vec![2, 3, 4],
        vec![3, 4, 5],
        vec![4, 5, 6],
        vec![5, 6, 7],
        vec![6, 7, 8],
        vec![7, 8, 9],
        vec![8, 9, 10],
        vec![9, 10, 11],
        vec![10, 11, 12],
        vec![11, 12, 13],
        vec![12, 13, 14],
        vec![13, 14, 15],
        vec![14, 15, 16],
        vec![15, 16, 17],
    ];

    for (v, vg) in vv.into_iter().zip(vgood) {
        let v: Vec<_> = v.into();
        assert_eq!(v, vg);
    }

    Ok(())
}

#[test]
fn open_sample_ai4_sizedslice() -> Result<()> {
    for s in SAMPLE_PATH {
        open_sample_ai4_sizedslice_impl(s)?;
    }
    Ok(())
}

#[allow(non_snake_case)]
fn open_sample_Ab_impl<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut f = RootFile::open(path)?;
    // f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);
    let tree = f.get_tree("sample")?;
    assert_eq!(tree.entries(), 30);
    let branch_name = "Ab";
    let b = tree.branch(branch_name).unwrap().as_iter::<Slice<bool>>();
    assert_eq!(b.count(), tree.entries() as usize);

    let vv: Vec<_> = tree
        .branch(branch_name)
        .unwrap()
        .as_iter::<Slice<bool>>()
        .collect();
    let vgood = vec![
        vec![],
        vec![true],
        vec![true, true],
        vec![true, true, true],
        vec![true, true, true, true],
        vec![],
        vec![false],
        vec![false, false],
        vec![false, false, false],
        vec![false, false, false, false],
        vec![],
        vec![true],
        vec![true, true],
        vec![true, true, true],
        vec![true, true, true, true],
        vec![],
        vec![false],
        vec![false, false],
        vec![false, false, false],
        vec![false, false, false, false],
        vec![],
        vec![true],
        vec![true, true],
        vec![true, true, true],
        vec![true, true, true, true],
        vec![],
        vec![false],
        vec![false, false],
        vec![false, false, false],
        vec![false, false, false, false],
    ];

    for (v, vg) in vv.into_iter().zip(vgood) {
        assert_eq!(v.into_vec(), vg.as_slice());
    }

    Ok(())
}

#[test]
#[allow(non_snake_case)]
fn open_sample_Ab() -> Result<()> {
    for s in SAMPLE_PATH {
        open_sample_Ab_impl(s)?;
    }
    Ok(())
}

fn open_sample_str_impl<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut f = RootFile::open(path)?;
    // f.keys_name().map(|k| trace!("key = {}", k)).for_each(drop);
    let tree = f.get_tree("sample")?;
    assert_eq!(tree.entries(), 30);
    let branch_name = "str";
    let b = tree.branch(branch_name).unwrap().as_iter::<String>();
    assert_eq!(b.count(), tree.entries() as usize);

    let vv: Vec<_> = tree
        .branch(branch_name)
        .unwrap()
        .as_iter::<String>()
        .collect();

    vv.into_iter().enumerate().for_each(|(i, v)| {
        let s = format!("hey-{i}");
        assert_eq!(s, v);
    });

    Ok(())
}

#[test]
fn open_sample_str() -> Result<()> {
    for s in SAMPLE_PATH {
        open_sample_str_impl(s)?;
    }
    Ok(())
}

//
