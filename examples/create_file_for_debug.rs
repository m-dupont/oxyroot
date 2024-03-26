use env_logger::{Builder, Target, WriteStyle};
use oxyroot::{Marshaler, RootFile, Slice, Tree};
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
    fn make_string(n: i32) -> String {
        let mut s = String::new();
        for i in 0..(n * 30) {
            for j in 0..10 {
                s.push_str(&format!("string{},{} ", i, j));
            }
        }
        s
        //format!("evt-{}", 10_i32.pow(n as u32))
    }

    {
        let file = "/tmp/g.root";
        let mut f = RootFile::create(file).unwrap();
        let mut tree = Tree::new("mytree".to_string());
        let it = (0..N).map(|x| make_string(x));
        //
        tree.new_branch("Str".to_string(), it);

        tree.write(&mut f).unwrap();

        f.close().unwrap();

        std::fs::rename(file, "/tmp/a.root").unwrap();
    }

    let mut f = oxyroot::RootFile::open("/tmp/a.root").unwrap();
    let tree = f.get_tree("mytree").unwrap();
    let mut b = tree.branch("Str").unwrap().as_iter::<String>();

    let it = (0..N).map(make_string);

    for (i, (r, w)) in b.zip(it).enumerate() {
        assert_eq!(r, w);
    }

    // println!("tree = {:?}", tree);
    // assert_eq!(Photon_E.count(), 2421);
}
