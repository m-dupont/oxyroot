use oxyroot::WriterTree;
use oxyroot::{ReadFromTree, RootFile, WriteToTree};

const OUT_DIR: &str = "/tmp/rust/derive_read_attributes/";

#[test]
fn test_a_i32() -> anyhow::Result<()> {
    let out_dir = format!("{}/a_i32", OUT_DIR);
    std::fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/a.root", out_dir);
    #[derive(Debug, WriteToTree, PartialEq)]
    struct WTest {
        a: i32,
    }

    #[derive(Debug, ReadFromTree, PartialEq)]
    struct RTest {
        #[oxyroot(rename = "a")]
        b: i32,
    }

    impl RTest {
        fn eq_wtest(&self, w: &WTest) {
            assert_eq!(self.b, w.a);
        }
    }

    fn gent_it() -> impl Iterator<Item = WTest> {
        (0..10).map(|x| WTest { a: x })
    }

    {
        let mut f = RootFile::create(&out_file)?;
        let mut tree = WriterTree::new("tree");

        WTest::to_tree(gent_it(), &mut tree)?;

        tree.write(&mut f)?;
        f.close()?;
    }

    let file = out_file;
    let tree = RootFile::open(file)?.get_tree("tree")?;
    for (t, tt) in RTest::from_tree(&tree)?.zip(gent_it()) {
        t.eq_wtest(&tt);
    }
    Ok(())
}

#[test]
fn test_local_prefix_i32() -> anyhow::Result<()> {
    let out_dir = format!("{}/local_prefix_i32", OUT_DIR);
    std::fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/a.root", out_dir);
    #[derive(Debug, WriteToTree, PartialEq)]
    struct WTest {
        branch_a: i32,
        zweig_b: i32,
    }

    #[derive(Debug, ReadFromTree, PartialEq)]
    struct RTest {
        #[oxyroot(branch_prefix = "branch_")]
        a: i32,
        #[oxyroot(branch_prefix = "zweig_")]
        b: i32,
    }

    impl RTest {
        fn eq_wtest(&self, w: &WTest) {
            assert_eq!(self.b, w.zweig_b);
            assert_eq!(self.a, w.branch_a);
        }
    }

    fn gent_it() -> impl Iterator<Item = WTest> {
        (0..10).map(|x| WTest {
            zweig_b: x,
            branch_a: x * x,
        })
    }

    {
        let mut f = RootFile::create(&out_file)?;
        let mut tree = WriterTree::new("tree");

        WTest::to_tree(gent_it(), &mut tree)?;

        tree.write(&mut f)?;
        f.close()?;
    }

    let file = out_file;
    let tree = RootFile::open(file)?.get_tree("tree")?;
    for (t, tt) in RTest::from_tree(&tree)?.zip(gent_it()) {
        t.eq_wtest(&tt);
    }
    Ok(())
}

#[test]
fn test_global_prefix_i32() -> anyhow::Result<()> {
    let out_dir = format!("{}/global_prefix_i32", OUT_DIR);
    std::fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/a.root", out_dir);
    #[derive(Debug, WriteToTree, PartialEq)]
    struct WTest {
        branch_a: i32,
        branch_b: i32,
    }

    #[derive(Debug, ReadFromTree, PartialEq)]
    #[oxyroot(branch_prefix = "branch_")]
    struct RTest {
        a: i32,
        b: i32,
    }

    impl RTest {
        fn eq_wtest(&self, w: &WTest) {
            assert_eq!(self.b, w.branch_b);
            assert_eq!(self.a, w.branch_a);
        }
    }

    fn gent_it() -> impl Iterator<Item = WTest> {
        (0..10).map(|x| WTest {
            branch_b: x,
            branch_a: x * x,
        })
    }

    {
        let mut f = RootFile::create(&out_file)?;
        let mut tree = WriterTree::new("tree");

        WTest::to_tree(gent_it(), &mut tree)?;

        tree.write(&mut f)?;
        f.close()?;
    }

    let file = out_file;
    let tree = RootFile::open(file)?.get_tree("tree")?;
    for (t, tt) in RTest::from_tree(&tree)?.zip(gent_it()) {
        t.eq_wtest(&tt);
    }
    Ok(())
}

#[test]
fn test_global_prefix_absolute_name_i32() -> anyhow::Result<()> {
    let out_dir = format!("{}/global_prefix_absolute_name_i32", OUT_DIR);
    std::fs::create_dir_all(&out_dir)?;
    let out_file = format!("{}/a.root", out_dir);
    #[derive(Debug, WriteToTree, PartialEq)]
    struct WTest {
        branch_a: i32,
        branch_b: i32,
        zweig_c: i32,
    }

    #[derive(Debug, ReadFromTree, PartialEq)]
    #[oxyroot(branch_prefix = "branch_")]
    struct RTest {
        a: i32,
        b: i32,
        #[oxyroot(absolute_name = "zweig_c")]
        c: i32,
    }

    impl RTest {
        fn eq_wtest(&self, w: &WTest) {
            assert_eq!(self.b, w.branch_b);
            assert_eq!(self.a, w.branch_a);
            assert_eq!(self.c, w.zweig_c);
        }
    }

    fn gent_it() -> impl Iterator<Item = WTest> {
        (0..10).map(|x| WTest {
            branch_b: x,
            branch_a: x * x,
            zweig_c: x * x * x,
        })
    }

    {
        let mut f = RootFile::create(&out_file)?;
        let mut tree = WriterTree::new("tree");

        WTest::to_tree(gent_it(), &mut tree)?;

        tree.write(&mut f)?;
        f.close()?;
    }

    let file = out_file;
    let tree = RootFile::open(file)?.get_tree("tree")?;
    for (t, tt) in RTest::from_tree(&tree)?.zip(gent_it()) {
        t.eq_wtest(&tt);
    }
    Ok(())
}
