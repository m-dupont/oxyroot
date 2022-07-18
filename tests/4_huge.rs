use anyhow::Result;
use oxyroot::RootFile;
use std::collections::{HashMap, HashSet};

#[test]
fn tree_with_int_array_25() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/huge/huge.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    let tree = f.get_tree("tree")?.unwrap();

    assert_eq!(
        tree.branch("int_array_25").unwrap().item_type_name(),
        "int32_t[25]"
    );

    let v = tree
        .branch("int_array_25")
        .expect("No branch int_array_25")
        .as_iter::<[i32; 25]>()
        .collect::<Vec<_>>();

    assert_eq!(v.len(), tree.entries() as usize);

    v.iter().enumerate().for_each(|(i, int_array_25)| {
        int_array_25.iter().enumerate().for_each(|(k, val)| {
            assert_eq!((i + k) as i32, *val);
        });
    });

    Ok(())
}

#[test]
fn tree_with_int_vector() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/huge/huge.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    let tree = f.get_tree("tree")?.unwrap();

    assert_eq!(
        tree.branch("int_vector").unwrap().item_type_name(),
        "vector<int32_t>"
    );

    let v = tree
        .branch("int_vector")
        .expect("No branch int_vector")
        .as_iter::<Vec<i32>>()
        .collect::<Vec<_>>();

    assert_eq!(v.len(), tree.entries() as usize);

    v.iter().enumerate().for_each(|(i, int_vector)| {
        assert_eq!(int_vector.len(), i % 25);
        int_vector.iter().enumerate().for_each(|(k, val)| {
            assert_eq!((i + k) as i32, *val);
        });
    });

    Ok(())
}
