use oxyroot::RootFile;

#[test]
fn open_g4_ntuples_1000() -> anyhow::Result<()> {
    let s = "tests_data/g4/g4-ntuples.root";
    let tree = RootFile::open(s)?.get_tree("mytree-1000")?;
    assert_eq!(tree.branch("i32").unwrap().item_type_name(), "int32_t");
    assert_eq!(tree.branch("i64").unwrap().item_type_name(), "int32_t");
    assert_eq!(tree.branch("f64").unwrap().item_type_name(), "double");

    assert_eq!(tree.entries(), 1000);
    let bi32 = tree.branch("i32").unwrap().as_iter::<i32>()?;
    assert_eq!(bi32.count() as i64, tree.entries());
    let bi64 = tree.branch("i64").unwrap().as_iter::<i32>()?;
    assert_eq!(bi64.count() as i64, tree.entries());

    let bi32 = tree.branch("i32").unwrap().as_iter::<i32>()?;
    let bi64 = tree.branch("i64").unwrap().as_iter::<i32>()?;

    for (i32, i64) in bi32.zip(bi64) {
        println!("i32: {:?}, i64: {:?}", i32, i64);
        assert_eq!(i32 * i32, i64);
    }

    Ok(())
}

#[test]
fn open_g4_ntuples_10000() -> anyhow::Result<()> {
    let s = "tests_data/g4/g4-ntuples.root";
    let tree = RootFile::open(s)?.get_tree("mytree-10000")?;
    assert_eq!(tree.branch("i32").unwrap().item_type_name(), "int32_t");
    assert_eq!(tree.branch("i64").unwrap().item_type_name(), "int32_t");
    assert_eq!(tree.branch("f64").unwrap().item_type_name(), "double");

    assert_eq!(tree.entries(), 10000);
    let bi32 = tree.branch("i32").unwrap().as_iter::<i32>()?;
    assert_eq!(bi32.count() as i64, tree.entries());
    let bi64 = tree.branch("i64").unwrap().as_iter::<i32>()?;
    assert_eq!(bi64.count() as i64, tree.entries());

    let bi32 = tree.branch("i32").unwrap().as_iter::<i32>()?;
    let bi64 = tree.branch("i64").unwrap().as_iter::<i32>()?;

    for (i32, i64) in bi32.zip(bi64) {
        println!("i32: {:?}, i64: {:?}", i32, i64);
        assert_eq!(i32 * i32, i64);
    }

    Ok(())
}
