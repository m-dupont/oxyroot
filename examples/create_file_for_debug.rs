use env_logger::{Builder, Target, WriteStyle};
use oxyroot::{Marshaler, ReaderTree, RootFile, Slice, WriterTree};
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

    let N = 15;
    fn make_string(n: i32) -> [i32; 5] {
        [n, n + 1, n + 2, n + 3, n + 4]
    }

    {
        let file = "/tmp/g.root";
        let mut f = RootFile::create(file).unwrap();
        let mut tree = WriterTree::new("mytree");
        let it = (0..N).map(|x| make_string(x));
        //
        tree.new_branch("ArrF64", it);

        tree.write(&mut f).unwrap();

        f.close().unwrap();

        std::fs::rename(file, "/tmp/a.root").unwrap();
    }

    let mut f = oxyroot::RootFile::open("/tmp/a.root").unwrap();
    let tree = f.get_tree("mytree").unwrap();
    let mut b = tree.branch("ArrF64").unwrap().as_iter::<[i32; 5]>();

    fn make_string2(n: i32) -> [i32; 5] {
        [n, n + 2, n + 2, n + 3, n + 4]
    }

    let it = (0..N).map(make_string2);

    for (i, (r, w)) in b.zip(it).enumerate() {
        assert_eq!(r, w);
    }

    // println!("tree = {:?}", tree);
    // assert_eq!(Photon_E.count(), 2421);
}
