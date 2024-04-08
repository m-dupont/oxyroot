use oxyroot::{ReadFromTree, RootFile, WriteToTree, WriterTree};

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
