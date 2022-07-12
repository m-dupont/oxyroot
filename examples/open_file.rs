use chrono::Local;
use env_logger::{Builder, Target, WriteStyle};
use log::{error, trace, LevelFilter};
use oxyroot::RBuffer;
use oxyroot::RootFile;
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
    let _s = "/home/mdupont/Documents/DocumentsSync/soft/oxyroot/tests_data/0/cernstaff.root";
    let _s =
        "/home/mdupont/Documents/DocumentsSync/soft/oxyroot/tests_data/root-6.14.06/cernstaff.root";
    let _s =
        "/home/mdupont/Documents/DocumentsSync/soft/oxyroot/tests_data/root-6.10.08/cernstaff.root";

    let _s = "/home/mdupont/Documents/DocumentsSync/soft/oxyroot/tests_data/1/cernstaff.root";
    let s = "/tmp/rust/struct/read_int_struct_split=0/o.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s).unwrap();

    // f.get_object("Spectrum").unwrap();
    // f.get_object("T").unwrap();
    let tree = f.get_tree("T").unwrap();
    let tree = tree.unwrap();

    for b in tree.branches() {
        println!("branch = {}, class = {}", b.name(), b.class());
    }

    let fsd = |r: &mut RBuffer| {
        println!("r len = {}", r.len());

        let mut arr = [0 as u8; 100];
        // dbg!(r.read_array_u8(arr.as_mut_slice()).unwrap());

        r.read_array_u8(arr.as_mut_slice()).unwrap();

        println!("arr = {:?}", arr);
        dbg!(r.read_i32().unwrap());
        dbg!(r.read_i32().unwrap());
    };

    println!("###########");
    let v_v: Vec<_> = tree.branch("v_i").unwrap().get_basket(fsd).collect();

    println!("v_v = {:?}", v_v);
}
