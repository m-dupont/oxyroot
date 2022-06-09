use chrono::Local;
use env_logger::{Builder, Target, WriteStyle};
use log::{error, trace, LevelFilter};
use oxyroot::file::RootFile;
use std::io::Write;

fn main() {
    let _stylish_logger = Builder::new()
        .filter(None, LevelFilter::Trace)
        .write_style(WriteStyle::Always)
        .format(|buf, record| {
            let level = record.metadata().level().as_str().to_ascii_uppercase();
            let file = record.file().unwrap_or("");
            let line = record.line().unwrap_or(0);
            let module = record.module_path().unwrap_or("");
            let time = Local::now().format("%Y-%m-%dT%H:%M:%S");
            writeln!(
                buf,
                "{}:[{}]:{}>{} ({}@{})",
                time,
                level,
                module,
                record.args(),
                file,
                line
            )
        })
        .target(Target::Stdout)
        .init();
    // stylish_logger.log(&record());
    trace!("hello");
    error!("info");
    println!("example of opening file");

    // let s = "/home/mdupont/Documents/DocumentsSync/soft/xpad_montecarlo_rust/old.root";
    let s = "/home/mdupont/Documents/DocumentsSync/soft/oxyroot/tests_data/0/cernstaff.root";
    let s =
        "/home/mdupont/Documents/DocumentsSync/soft/oxyroot/tests_data/root-6.14.06/cernstaff.root";
    let s =
        "/home/mdupont/Documents/DocumentsSync/soft/oxyroot/tests_data/root-6.10.08/cernstaff.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s).unwrap();

    // f.get_object("Spectrum").unwrap();
    // f.get_object("T").unwrap();
    f.get_tree("T").unwrap();
}
