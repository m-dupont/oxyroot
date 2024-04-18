use env_logger::{Builder, Target, WriteStyle};

use std::io::Write;

use oxyroot::ReadFromTree;
use oxyroot::{RootFile, Slice};
#[derive(Debug, ReadFromTree)]
struct myDetectorData {
    #[oxyroot(rename = "branch1.time")]
    time: f64,
}

fn main() -> anyhow::Result<()> {
    let _stylish_logger = Builder::new()
        .parse_default_env()
        // .filter(None, LevelFilter::Trace)
        .write_style(WriteStyle::Always)
        .format(|buf, record| {
            // let level = record.metadata().level().as_str().to_ascii_uppercase();
            // let file = record.file().unwrap_or("");
            // let line = record.line().unwrap_or(0);
            // let module = record.module_path().unwrap_or("");
            // let time = Local::now().format("%Y-%m-%dT%H:%M:%S");
            writeln!(buf, "{}", record.args())
        })
        .target(Target::Stdout)
        .init();

    let file =
        "/home/mdupont/Documents/DocumentsSync_data/repositories/customTTreeExample/testFile.root";
    let mut tree = RootFile::open(file).unwrap().get_tree("myTree")?;
    // let mut Photon_E = tree.branch("branch1.").unwrap().as_iter::<String>()?;

    let v = myDetectorData::from_tree(&mut tree)?.collect::<Vec<_>>();
    println!("{:?}", v.len());
    println!("{:?}", v);

    Ok(())
    // assert_eq!(Photon_E.count(), 2421);
}
