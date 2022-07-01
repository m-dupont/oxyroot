use chrono::Local;
use env_logger::{Builder, Target, WriteStyle};
use log::{error, trace, LevelFilter};
use oxyroot::file::RootFile;
use oxyroot::rbytes::rbuffer::RBuffer;
use oxyroot::root::traits::Named;
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
    let s = "/tmp/rust/i16/o.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s).unwrap();

    // f.get_object("Spectrum").unwrap();
    // f.get_object("T").unwrap();
    let tree = f.get_tree("T").unwrap();
    let tree = tree.unwrap();

    for b in tree.branches() {
        println!("branch = {}, class = {}", b.name(), b.class());
    }

    // let branch = tree.get_branch("Category").unwrap();
    //
    // let f = |r: &mut RBuffer| {
    //     let val = r.read_i32().unwrap();
    //     val
    // };
    //
    // let f32 = |r: &mut RBuffer| {
    //     let val = r.read_f32().unwrap();
    //     val
    // };
    //
    // let v_cat: Vec<_> = branch.get_basket(f).collect();
    // let branch = tree.get_branch("float").unwrap();
    // let v_float: Vec<_> = branch.get_basket(f32).collect();
    //
    // println!("v_float = {:?}", v_float);
    // println!("v_cat = {:?}", v_cat);
    //
    // let vec_array_int: Vec<_> = tree
    //     .get_branch("array_int")
    //     .unwrap()
    //     .get_basket(|r| (0..5).map(|_| r.read_i32().unwrap()).collect::<Vec<_>>())
    //     .collect();
    //
    // println!("vec_array_int = {:?}", vec_array_int);
    //
    // #[derive(Debug)]
    // struct Sd {
    //     a: i32,
    //     b: i32,
    //     c: f64,
    // };
    //
    // let fsd = |r: &mut RBuffer| Sd {
    //     a: dbg!(r.read_i32().unwrap()),
    //     b: dbg!(r.read_i32().unwrap()),
    //     c: r.read_f64().unwrap(),
    // };
    //
    // let v_sd: Vec<_> = tree.get_branch("sd0").unwrap().get_basket(fsd).collect();
    // println!("v_sd_0 = {:?}", v_sd);
    //
    // println!("###############");
    // let v_sd: Vec<_> = tree.get_branch("sd1").unwrap().get_basket(fsd).collect();
    // println!("v_sd_1 = {:?}", v_sd);
    //
    // let v_sd: Vec<_> = tree.get_branch("sd2").unwrap().get_basket(fsd).collect();
    // println!("v_sd_2 = {:?}", v_sd);

    let fsd = |r: &mut RBuffer| {
        //comment

        // dbg!(r.read_object_any_into());

        // dbg!(r.read_header("vector").unwrap());
        // dbg!(r.read_i32().unwrap());

        println!("r len = {}", r.len());

        let mut arr = [0 as u8; 100];
        // dbg!(r.read_array_u8(arr.as_mut_slice()).unwrap());

        r.read_array_u8(arr.as_mut_slice()).unwrap();

        println!("arr = {:?}", arr);
        dbg!(r.read_i32().unwrap());
        dbg!(r.read_i32().unwrap());
    };

    println!("###########");
    let v_v: Vec<_> = tree.branch("v").unwrap().get_basket(fsd).collect();

    println!("v_v = {:?}", v_v);
}
