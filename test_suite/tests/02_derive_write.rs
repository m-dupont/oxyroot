use oxyroot::{ReadFromTree, WriteToTree, WriterTree};
use oxyroot::{ReaderTree, RootFile};

const OUT_DIR: &str = "/tmp/rust/derive_write/";

#[test]
fn test_a_i32() -> anyhow::Result<()> {
    let out_dir = format!("{}/a_i32", OUT_DIR);
    std::fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/a.root", out_dir);
    #[derive(Debug, ReadFromTree, WriteToTree, PartialEq)]
    struct Test {
        a: i32,
    }

    fn gent_it() -> impl Iterator<Item = Test> {
        (0..10).map(|x| Test { a: x })
    }

    {
        let mut f = RootFile::create(&out_file)?;
        let mut tree = WriterTree::new("tree");

        Test::to_tree(gent_it(), &mut tree)?;

        tree.write(&mut f)?;
        f.close()?;
    }

    let file = out_file;
    let tree = RootFile::open(file)?.get_tree("tree")?;
    for (t, tt) in Test::from_tree(&tree)?.zip(gent_it()) {
        assert_eq!(t, tt);
    }
    Ok(())
}

#[test]
fn test_a_i32_b_i16() -> anyhow::Result<()> {
    let out_dir = format!("{}/a_i32_b_i16", OUT_DIR);
    std::fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/a.root", out_dir);
    #[derive(Debug, ReadFromTree, WriteToTree, PartialEq)]
    struct Test {
        a: i32,
        b: i16,
    }

    fn gent_it() -> impl Iterator<Item = Test> {
        (0..10).map(|x| Test {
            a: x,
            b: (x + 10) as i16,
        })
    }

    {
        let mut f = RootFile::create(&out_file)?;
        let mut tree = WriterTree::new("tree");
        Test::to_tree(gent_it(), &mut tree)?;

        tree.write(&mut f)?;
        f.close()?;
    }

    let file = out_file;
    let tree = RootFile::open(file)?.get_tree("tree")?;
    for (t, tt) in Test::from_tree(&tree)?.zip(gent_it()) {
        assert_eq!(t, tt);
    }
    Ok(())
}
#[test]
fn test_a_i32_b_i16_c_veci32() -> anyhow::Result<()> {
    let out_dir = format!("{}/a_i32_b_i16_c_veci32", OUT_DIR);
    std::fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/a.root", out_dir);
    #[derive(Debug, ReadFromTree, WriteToTree, PartialEq)]
    struct Test {
        a: i32,
        b: i16,
        c: Vec<i32>,
    }
    fn gent_it() -> impl Iterator<Item = Test> {
        (0..10).map(|x| Test {
            a: x,
            b: (x + 10) as i16,
            c: vec![1; x as usize],
        })
    }

    {
        let mut f = RootFile::create(&out_file)?;
        let mut tree = WriterTree::new("tree");

        Test::to_tree(gent_it(), &mut tree)?;

        tree.write(&mut f)?;
        f.close()?;
    }

    let file = out_file;
    let tree = RootFile::open(file)?.get_tree("tree")?;
    for (t, tt) in Test::from_tree(&tree)?.zip(gent_it()) {
        assert_eq!(t, tt);
    }
    Ok(())
}

#[test]
fn test_a_i32_b_i16_c_veci32_d_string() -> anyhow::Result<()> {
    let out_dir = format!("{}/a_i32_b_i16_c_veci32_d_string", OUT_DIR);
    std::fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/a.root", out_dir);

    #[derive(Debug, ReadFromTree, WriteToTree, PartialEq)]
    struct Test {
        a: i32,
        b: i16,
        c: Vec<i32>,
        d: String,
    }

    fn gent_it() -> impl Iterator<Item = Test> {
        (0..10).map(|x| Test {
            a: x,
            b: (x + 10) as i16,
            c: vec![1; x as usize],
            d: format!("x: {x}"),
        })
    }

    {
        let mut f = RootFile::create(&out_file)?;
        let mut tree = WriterTree::new("tree");

        Test::to_tree(gent_it(), &mut tree)?;

        tree.write(&mut f)?;
        f.close()?;
    }

    let file = out_file;
    let tree = RootFile::open(file)?.get_tree("tree")?;
    for (t, tt) in Test::from_tree(&tree)?.zip(gent_it()) {
        assert_eq!(t, tt);
    }
    Ok(())
}

// #[test]
// fn test_a_i32_b_i16_c_veci32_d_string__ab_c_d() -> anyhow::Result<()> {
//     let out_dir = format!("{}/a_i32_b_i16_c_veci32_d_string__ab_c_d", OUT_DIR);
//     std::fs::create_dir_all(&out_dir)?;
//     let out_file = format!("{}/a.root", out_dir);
//
//     #[derive(Debug, ReadFromTree, WriteToTree, PartialEq)]
//     struct TestAB {
//         a: i32,
//         b: i16,
//     }
//
//     #[derive(Debug, ReadFromTree, WriteToTree, PartialEq)]
//     struct Test {
//         ab: TestAB,
//         c: Vec<i32>,
//         d: String,
//     }
//
//     fn gent_it() -> impl Iterator<Item = Test> {
//         (0..10).map(|x| Test {
//             ab: TestAB {
//                 a: x,
//                 b: (x + 10) as i16,
//             },
//             c: vec![1; x as usize],
//             d: format!("x: {x}"),
//         })
//     }
//
//     {
//         let mut f = RootFile::create(&out_file)?;
//         let mut tree = WriterTree::new("tree");
//
//         Test::to_tree(gent_it(), &mut tree)?;
//
//         tree.write(&mut f)?;
//         f.close()?;
//     }
//
//     let file = out_file;
//     let tree = RootFile::open(file)?.get_tree("tree")?;
//     for (t, tt) in Test::from_tree(&tree)?.zip(gent_it()) {
//         assert_eq!(t, tt);
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
//     #[derive(Debug, ReadFromTree, WriteToTree, PartialEq)]
//     struct TestAB {
//         a: i32,
//         b: i16,
//     }
//
//     #[derive(Debug, ReadFromTree, WriteToTree, PartialEq)]
//     struct TestCD {
//         c: Vec<i32>,
//         d: String,
//     }
//
//     #[derive(Debug, ReadFromTree, WriteToTree, PartialEq)]
//     struct Test {
//         ab: TestAB,
//         cd: TestCD,
//     }
//
//     fn gent_it() -> impl Iterator<Item = Test> {
//         (0..10).map(|x| Test {
//             ab: TestAB {
//                 a: x,
//                 b: (x + 10) as i16,
//             },
//             cd: TestCD {
//                 c: vec![1; x as usize],
//                 d: format!("x: {x}"),
//             },
//         })
//     }
//
//     {
//         let mut f = RootFile::create(&out_file)?;
//         let mut tree = WriterTree::new("tree");
//
//         Test::to_tree(gent_it(), &mut tree)?;
//
//         tree.write(&mut f)?;
//         f.close()?;
//     }
//
//     let file = out_file;
//     let tree = RootFile::open(file)?.get_tree("tree")?;
//     for (t, tt) in Test::from_tree(&tree)?.zip(gent_it()) {
//         assert_eq!(t, tt);
//     }
//     Ok(())
// }
