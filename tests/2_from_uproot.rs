use anyhow::Result;
use oxyroot::RootFile;
use regex::internal::Input;
use std::mem;

#[test]
fn open_nested() -> Result<()> {
    let s = "examples/from_uproot/data/HZZ.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;
    let tree = f.get_tree("events")?;
    assert!(tree.is_some());
    let tree = tree.unwrap();

    let NJet = tree.branch("NJet").unwrap().get_basket_into::<i32>();

    let n = NJet.count();
    assert_eq!(n, 2421);

    let mut Jet_Py = tree.branch("Jet_Py").unwrap().get_basket_into::<f32>();
    assert_eq!(Jet_Py.count(), 2773);

    Ok(())
}

#[test]
fn open_simple_root() -> Result<()> {
    let s = "examples/from_uproot/data/simple.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;
    let tree = tree.unwrap();

    let one = tree
        .branch("one")
        .unwrap()
        .get_basket_into::<i32>()
        .collect::<Vec<_>>();

    assert_eq!(one, [1, 2, 3, 4]);

    let two = tree
        .branch("two")
        .unwrap()
        .get_basket_into::<f32>()
        .collect::<Vec<_>>();

    assert_eq!(two, [1.1, 2.2, 3.3, 4.4]);

    let three = tree
        .branch("three")
        .unwrap()
        .get_basket_into::<String>()
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
    let tree = tree.unwrap();
    tree.branch("Beg")
        .unwrap()
        .get_basket_into::<String>()
        .enumerate()
        .for_each(|(i, s)| {
            assert_eq!(s, format!("beg-{:03}", i));
        });

    tree.branch("End")
        .unwrap()
        .get_basket_into::<String>()
        .enumerate()
        .for_each(|(i, s)| {
            assert_eq!(s, format!("end-{:03}", i));
        });

    Ok(())
}

#[test]
fn open_tree_with_struct_P3() -> Result<()> {
    // P3 <=> P3 { x: i32, y: f64, z: i32}
    // Stored in three branches P3.x, P3.y, and P3.z
    // tree.branch("P3") will zip the three branches

    let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    let tree = f.get_tree("tree")?;
    let tree = tree.unwrap();
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
            // println!("x = {x} y = {y}");
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

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("t1")?.unwrap();

    tree.branch("int32_array")
        .unwrap()
        .get_basket(|r| {
            let mut len = r.len() as usize;
            let mut ret: Vec<i32> = Vec::new();
            while len > 0 {
                ret.push(r.read_i32().unwrap());
                len -= mem::size_of::<i32>();
            }
            ret
        })
        .enumerate()
        .for_each(|(i, val)| {
            assert_eq!(val.len(), i % 10);
            // prinytln!("StlVecI16: i = {i} val = {:?}, {}", val, i - i % 10);

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
    f.keys().map(|k| println!("key = {}", k)).for_each(drop);
    let tree = f.get_tree("t1")?.unwrap();
    tree.branch("int32_array")
        .unwrap()
        .get_basket_into::<Vec<i32>>()
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
fn open_tree_with_vector_of_string() -> Result<()> {
    let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";

    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;
    let tree = tree.unwrap();

    tree.branch("StlVecStr")
        .unwrap()
        .get_basket_into::<Vec<String>>()
        .enumerate()
        .for_each(|(i, val)| {
            println!("StlVecStr: i = {i} val = {:?}", val);
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

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;
    let tree = tree.unwrap();

    tree.branch("ArrayI16[10]")
        .unwrap()
        .get_basket(|r| {
            let mut buf = [0 as i16; 10];
            r.read_array_i16(&mut buf).unwrap();
            buf
        })
        .enumerate()
        .for_each(|(i, buf)| {
            // println!("buf = {:?}", buf);
            buf.map(|v| assert_eq!(v, i as i16));
        });

    Ok(())
}
