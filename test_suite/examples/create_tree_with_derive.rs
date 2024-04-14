use oxyroot::{ReadFromTree, RootFile, WriteToTree, WriterTree};

#[derive(Debug, WriteToTree)]
struct WStruct {
    aw: i32,
    bw: String,
}

#[derive(Debug, ReadFromTree)]
struct RStruct {
    #[oxyroot(rename = "aw")]
    ar: i32,
    bw: String,
}

fn main() -> anyhow::Result<()> {
    fn gent_it() -> impl Iterator<Item = WStruct> {
        (0..10).map(|x| WStruct {
            aw: x,
            bw: format!("x => {x}"),
        })
    }

    {
        let mut f = RootFile::create("/tmp/a.root")?;
        let mut tree = WriterTree::new("tree");

        WStruct::to_tree(gent_it(), &mut tree)?;

        tree.write(&mut f)?;
        f.close()?;
    }

    let file = "/tmp/a.root";
    let tree = RootFile::open(file)?.get_tree("tree")?;
    for (t, tt) in RStruct::from_tree(&tree)?.zip(gent_it()) {
        println!("r: {t:?}, w = {tt:?}")
    }

    Ok(())
}
