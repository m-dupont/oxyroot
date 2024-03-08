use oxyroot::RootFile;

#[test]
fn open_g4litetree_primitives() -> anyhow::Result<()> {
    let s = "tests_data/from_groot/g4-like.root";
    let tree = RootFile::open(s)?.get_tree("mytree")?;
    assert_eq!(tree.branch("i32").unwrap().item_type_name(), "int32_t");
    assert_eq!(tree.branch("f64").unwrap().item_type_name(), "double");

    assert_eq!(
        tree.branch("i32").unwrap().as_iter::<i32>().count(),
        tree.entries() as usize
    );

    let mut bi32 = tree.branch("i32").unwrap().as_iter::<i32>();
    let mut bf64 = tree.branch("f64").unwrap().as_iter::<f64>();

    for val in bi32.zip(bf64).enumerate() {
        let (i, ((ii, f))) = val;
        assert_eq!(i as i32 + 1, ii);
        assert_eq!(i as f64 + 1.0, f);
    }

    Ok(())
}

#[test]
fn open_g4litetree_vector() -> anyhow::Result<()> {
    let s = "tests_data/from_groot/g4-like.root";
    let tree = RootFile::open(s)?.get_tree("mytree")?;
    assert_eq!(tree.branch("i32").unwrap().item_type_name(), "int32_t");
    assert_eq!(
        tree.branch("slif64").unwrap().item_type_name(),
        "vector<double>"
    );

    assert_eq!(
        tree.branch("slif64").unwrap().as_iter::<Vec<f64>>().count(),
        tree.entries() as usize
    );

    let mut bi32 = tree.branch("i32").unwrap().as_iter::<i32>();
    let bslif64 = tree.branch("slif64").unwrap().as_iter::<Vec<f64>>();

    for val in bi32.zip(bslif64).enumerate() {
        let (i, ((i3, f))) = val;
        assert_eq!(i as i32 + 1, i3);

        for (ii, v) in f.iter().enumerate() {
            assert_eq!(ii as f64 + i as f64, *v);
        }
    }

    Ok(())
}
