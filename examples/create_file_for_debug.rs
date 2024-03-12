use env_logger::{Builder, Target, WriteStyle};
use oxyroot::{RootFile, Slice};
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

    let file = "/tmp/a.root";
    let mut tree = RootFile::create(file).unwrap();
    // println!("tree = {:?}", tree);
    // assert_eq!(Photon_E.count(), 2421);
}
