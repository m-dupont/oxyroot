// use env_logger::{Builder, Target, WriteStyle};
use oxyroot::{ReaderTree, RootFile, Slice};
use std::io::Write;

use oxyroot::ReadFromTree;

#[derive(Debug, ReadFromTree)]
struct Points {
    x: Slice<f64>,
    y: Slice<f64>,
    weights: Vec<f64>,
}

#[derive(Debug, ReadFromTree)]
struct Point {
    x: f64,
    y: f64,
    weights: Vec<f64>,
}

#[derive(Debug, ReadFromTree)]
#[oxyroot(branch_prefix = "branch1.")]
struct myDetectorData {
    #[oxyroot(rename = "time")]
    times: f64,
    energy: f64,
    correlatedDetectors_v: Vec<f64>,
    #[oxyroot(branch_prefix = "points.")]
    points: Points,
    // #[oxyroot(rename = "branch.origin.")]
    #[oxyroot(branch_prefix = "origin.")]
    origin: Point, // points: VecTree<Points>,
}

fn main() -> anyhow::Result<()> {
    let file =
        "/home/mdupont/Documents/DocumentsSync_data/repositories/customTTreeExample/testFile.root";
    let mut tree = RootFile::open(file).unwrap().get_tree("myTree")?;
    // let mut Photon_E = tree.branch("branch1.").unwrap().as_iter::<String>()?;

    let v = myDetectorData::from_tree(&mut tree)?.collect::<Vec<_>>();
    println!("{:?}", v.len());
    for (i, vv) in v.iter().enumerate() {
        println!("{i} -> {:?}", vv);
    }

    Ok(())
    // assert_eq!(Photon_E.count(), 2421);
}
