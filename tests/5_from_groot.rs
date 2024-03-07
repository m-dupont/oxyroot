use oxyroot::RootFile;

#[test]
fn open_g4litetree() -> anyhow::Result<()> {
    let s = "tests_data/from_groot/g4-like.root";
    let tree = RootFile::open(s)?.get_tree("mytree")?;
    assert_eq!(tree.branch("i32").unwrap().item_type_name(), "int32_t");
    assert_eq!(tree.branch("f64").unwrap().item_type_name(), "double");
    assert_eq!(
        tree.branch("slif64").unwrap().item_type_name(),
        "vector<double>"
    );

    #[derive(Debug)]
    struct EventData {
        i32: i32,
        f64: f64,
        slif64: Vec<f64>,
    }

    let mut bi32 = tree.branch("i32").unwrap().as_iter::<i32>();

    println!("nb entries: {}", tree.entries() as usize);
    println!("nb count: {}", tree.branch("i32").unwrap().entries());

    println!("bi32: {:?}", bi32.collect::<Vec<_>>());
    //
    // let v = bi32.next().unwrap();
    // println!("v: {:?}", v);

    let bf64 = tree.branch("f64").unwrap().as_iter::<f64>();
    // let bf64 = tree.branch("f64").unwrap().as_iter::<f64>();

    // let vec_evt = bi32
    //     .zip(bf64)
    //     .map(|(i32, f64)| EventData {
    //         i32,
    //         f64,
    //         slif64: Vec::new(),
    //     })
    //     .collect::<Vec<_>>();
    //
    // println!("vec_evt: {:?}", vec_evt);

    //

    Ok(())
}
