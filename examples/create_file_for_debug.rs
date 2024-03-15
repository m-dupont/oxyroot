use env_logger::{Builder, Target, WriteStyle};
use oxyroot::{RootFile, Slice, Tree};
use std::io::Write;

fn main() {
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

    let file = "/tmp/g.root";
    let mut f = RootFile::create(file).unwrap();
    let mut tree = Tree::new("mytree".to_string());
    f.add_tree(&tree).unwrap();
    tree.write(&mut f).unwrap();

    f.close().unwrap();

    std::fs::rename(file, "/tmp/a.root").unwrap();

    // println!("tree = {:?}", tree);
    // assert_eq!(Photon_E.count(), 2421);
}
