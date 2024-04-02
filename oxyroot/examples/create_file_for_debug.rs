use env_logger::{Builder, Target, WriteStyle};
use oxyroot::{RootFile, WriterTree};
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

    let n = 10;
    fn make_string(n: i32) -> Vec<i32> {
        (0..n).map(|x| x).collect()
    }

    println!("make_string(0) = {:?}", make_string(0));
    println!("make_string(1) = {:?}", make_string(1));
    println!("make_string(2) = {:?}", make_string(2));
    println!("make_string(3) = {:?}", make_string(3));
    println!("make_string(4) = {:?}", make_string(4));
    println!("make_string(5) = {:?}", make_string(5));

    // fn make_string(n: i32) -> i32 {
    //     3
    // }

    {
        let file = "/tmp/g.root";
        let mut f = RootFile::create(file).unwrap();
        let mut tree = WriterTree::new("tree");
        let it = (0..n).map(|x| make_string(x));
        //
        tree.new_branch("a", (10..10 + n));
        tree.new_branch("b", it);

        tree.write(&mut f).unwrap();

        f.close().unwrap();

        std::fs::rename(file, "/tmp/a.root").unwrap();
    }

    // let mut f = oxyroot::RootFile::open("/tmp/a.root").unwrap();
    // let tree = f.get_tree("tree").unwrap();
    // let mut b = tree.branch("vector_int32").unwrap().as_iter::<Vec<i32>>();
    //
    // fn make_string2(n: i32) -> Vec<i32> {
    //     (0..n).map(|x| x).collect()
    // }
    //
    // let it = (0..n).map(make_string2);
    //
    // for (i, (r, w)) in b.zip(it).enumerate() {
    //     assert_eq!(r, w);
    // }
}
