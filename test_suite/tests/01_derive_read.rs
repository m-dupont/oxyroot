use oxyroot::ReadFromTree;
use oxyroot::RootFile;
use oxyroot::WriterTree;

const OUT_DIR: &str = "/tmp/rust/derive_read/";

#[test]
fn test_a_i32() -> anyhow::Result<()> {
    let out_dir = format!("{}/a_i32", OUT_DIR);
    std::fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/a.root", out_dir);

    {
        let mut f = RootFile::create(&out_file)?;
        let mut tree = WriterTree::new("tree");

        tree.new_branch("a", 0_i32..10);

        tree.write(&mut f)?;
        f.close()?;
    }

    #[derive(Debug, ReadFromTree, PartialEq)]
    struct Test {
        a: i32,
    }

    let file = out_file;
    let tree = RootFile::open(file)?.get_tree("tree")?;
    let it_a = tree.branch("a").unwrap().as_iter::<i32>()?;
    for (t, a) in Test::from_tree(&tree)?.zip(it_a) {
        let tt = Test { a };
        assert_eq!(t, tt);
    }
    Ok(())
}

#[test]
fn test_a_i32_b_i16() -> anyhow::Result<()> {
    let out_dir = format!("{}/a_i32_b_i16", OUT_DIR);
    std::fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/a.root", out_dir);

    {
        let mut f = RootFile::create(&out_file)?;
        let mut tree = WriterTree::new("tree");

        tree.new_branch("a", 0_i32..10);
        tree.new_branch("b", 10_i16..10 + 10);

        tree.write(&mut f)?;
        f.close()?;
    }

    #[derive(Debug, ReadFromTree, PartialEq)]
    struct Test {
        a: i32,
        b: i16,
    }

    let file = out_file;
    let tree = RootFile::open(file)?.get_tree("tree")?;
    let it_a = tree.branch("a").unwrap().as_iter::<i32>()?;
    let it_b = tree.branch("b").unwrap().as_iter::<i16>()?;
    let it = it_a.zip(it_b).map(|(a, b)| Test { a, b });
    for (tr, tc) in Test::from_tree(&tree)?.zip(it) {
        assert_eq!(tr, tc);
    }
    Ok(())
}
#[test]
fn test_a_i32_b_i16_c_veci32() -> anyhow::Result<()> {
    let out_dir = format!("{}/a_i32_b_i16_c_veci32", OUT_DIR);
    std::fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/a.root", out_dir);

    {
        let mut f = RootFile::create(&out_file)?;
        let mut tree = WriterTree::new("tree");

        tree.new_branch("a", 0_i32..10);
        tree.new_branch("b", 10_i16..10 + 10);
        tree.new_branch("c", (0..10).map(|x| vec![1; x]));

        tree.write(&mut f)?;
        f.close()?;
    }

    #[derive(Debug, ReadFromTree, PartialEq)]
    struct Test {
        a: i32,
        b: i16,
        c: Vec<i32>,
    }

    let file = out_file;
    let tree = RootFile::open(file)?.get_tree("tree")?;
    let it_a = tree.branch("a").unwrap().as_iter::<i32>()?;
    let it_b = tree.branch("b").unwrap().as_iter::<i16>()?;
    let it_c = tree.branch("c").unwrap().as_iter::<Vec<i32>>()?;
    let it = it_a.zip(it_b).zip(it_c).map(|((a, b), c)| Test { a, b, c });
    for (tr, tc) in Test::from_tree(&tree)?.zip(it) {
        assert_eq!(tr, tc);
    }
    Ok(())
}

#[test]
fn test_a_i32_b_i16_c_veci32_d_string() -> anyhow::Result<()> {
    let out_dir = format!("{}/a_i32_b_i16_c_veci32_d_string", OUT_DIR);
    std::fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/a.root", out_dir);

    {
        let mut f = RootFile::create(&out_file)?;
        let mut tree = WriterTree::new("tree");

        tree.new_branch("a", 0_i32..10);
        tree.new_branch("b", 10_i16..10 + 10);
        tree.new_branch("c", (0..10).map(|x| vec![1; x]));

        tree.new_branch("d", (0..10).map(|x| format!("x: {x}")));

        tree.write(&mut f)?;
        f.close()?;
    }

    #[derive(Debug, ReadFromTree, PartialEq)]
    struct Test {
        a: i32,
        b: i16,
        c: Vec<i32>,
        d: String,
    }

    let file = out_file;
    let tree = RootFile::open(file)?.get_tree("tree")?;
    let it_a = tree.branch("a").unwrap().as_iter::<i32>()?;
    let it_b = tree.branch("b").unwrap().as_iter::<i16>()?;
    let it_c = tree.branch("c").unwrap().as_iter::<Vec<i32>>()?;
    let it_d = tree.branch("d").unwrap().as_iter::<String>()?;
    let it = it_a
        .zip(it_b)
        .zip(it_c)
        .zip(it_d)
        .map(|(((a, b), c), d)| Test { a, b, c, d });
    for (tr, tc) in Test::from_tree(&tree)?.zip(it) {
        assert_eq!(tr, tc);
    }
    Ok(())
}

// #[test]
// fn test_a_i32_b_i16_c_veci32_d_string__ab_c_d() -> anyhow::Result<()> {
//     let out_dir = format!("{}/a_i32_b_i16_c_veci32_d_string__ab_c_d", OUT_DIR);
//     std::fs::create_dir_all(&out_dir)?;
//     let out_file = format!("{}/a.root", out_dir);
//
//     {
//         let mut f = RootFile::create(&out_file)?;
//         let mut tree = WriterTree::new("tree");
//
//         tree.new_branch("a", 0_i32..10);
//         tree.new_branch("b", 10_i16..10 + 10);
//         tree.new_branch("c", (0..10).map(|x| vec![1; x]));
//
//         tree.new_branch("d", (0..10).map(|x| format!("x: {x}")));
//
//         tree.write(&mut f)?;
//         f.close()?;
//     }
//
//     #[derive(Debug, ReadFromTree, PartialEq)]
//     struct TestAB {
//         a: i32,
//         b: i16,
//     }
//
//     #[derive(Debug, ReadFromTree, PartialEq)]
//     struct Test {
//         ab: TestAB,
//         c: Vec<i32>,
//         d: String,
//     }
//
//     let file = out_file;
//     let tree = RootFile::open(file)?.get_tree("tree")?;
//     let it_a = tree.branch("a").unwrap().as_iter::<i32>()?;
//     let it_b = tree.branch("b").unwrap().as_iter::<i16>()?;
//     let it_c = tree.branch("c").unwrap().as_iter::<Vec<i32>>()?;
//     let it_d = tree.branch("d").unwrap().as_iter::<String>()?;
//     let it = it_a
//         .zip(it_b)
//         .zip(it_c)
//         .zip(it_d)
//         .map(|(((a, b), c), d)| Test {
//             ab: TestAB { a, b },
//             c,
//             d,
//         });
//     for (tr, tc) in Test::from_tree(&tree)?.zip(it) {
//         assert_eq!(tr, tc);
//     }
//     Ok(())
// }
//
// #[test]
// fn test_a_i32_b_i16_c_veci32_d_string__ab_cd() -> anyhow::Result<()> {
//     let out_dir = format!("{}/a_i32_b_i16_c_veci32_d_string__ab_cd", OUT_DIR);
//     std::fs::create_dir_all(&out_dir)?;
//     let out_file = format!("{}/a.root", out_dir);
//
//     {
//         let mut f = RootFile::create(&out_file)?;
//         let mut tree = WriterTree::new("tree");
//
//         tree.new_branch("a", 0_i32..10);
//         tree.new_branch("b", 10_i16..10 + 10);
//         tree.new_branch("c", (0..10).map(|x| vec![1; x]));
//
//         tree.new_branch("d", (0..10).map(|x| format!("x: {x}")));
//
//         tree.write(&mut f)?;
//         f.close()?;
//     }
//
//     #[derive(Debug, ReadFromTree, PartialEq)]
//     struct TestAB {
//         a: i32,
//         b: i16,
//     }
//
//     #[derive(Debug, ReadFromTree, PartialEq)]
//     struct TestCD {
//         c: Vec<i32>,
//         d: String,
//     }
//
//     #[derive(Debug, ReadFromTree, PartialEq)]
//     struct Test {
//         ab: TestAB,
//         cd: TestCD,
//     }
//
//     let file = out_file;
//     let tree = RootFile::open(file)?.get_tree("tree")?;
//     let it_a = tree.branch("a").unwrap().as_iter::<i32>()?;
//     let it_b = tree.branch("b").unwrap().as_iter::<i16>()?;
//     let it_c = tree.branch("c").unwrap().as_iter::<Vec<i32>>()?;
//     let it_d = tree.branch("d").unwrap().as_iter::<String>()?;
//     let it = it_a
//         .zip(it_b)
//         .zip(it_c)
//         .zip(it_d)
//         .map(|(((a, b), c), d)| Test {
//             ab: TestAB { a, b },
//             cd: TestCD { c, d },
//         });
//     for (tr, tc) in Test::from_tree(&tree)?.zip(it) {
//         assert_eq!(tr, tc);
//     }
//     Ok(())
// }
