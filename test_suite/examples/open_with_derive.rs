// use env_logger::{Builder, Target, WriteStyle};
use oxyroot::ReadFromTree;
use oxyroot::RootFile;

#[derive(Debug, ReadFromTree)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, ReadFromTree)]
struct TwoPoints {
    p1: Point,
    p2: Point,
}

fn main() -> anyhow::Result<()> {
    let file = "test_suite/create_root_files_with_root/t04_04_write_twopoints.root";
    let mut tree = RootFile::open(file).unwrap().get_tree("myTree")?;
    // let mut Photon_E = tree.branch("branch1.").unwrap().as_iter::<String>()?;

    let v = TwoPoints::from_tree(&mut tree)?.collect::<Vec<_>>();
    println!("{:?}", v.len());
    for (i, vv) in v.iter().enumerate() {
        println!("{i} -> {:?}", vv);
    }

    Ok(())
    // assert_eq!(Photon_E.count(), 2421);
}
