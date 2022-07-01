use anyhow::Result;
use chrono::Local;
use env_logger::{Builder, Target, WriteStyle};
use log::{error, trace, LevelFilter};
use oxyroot::file::RootFile;
use std::io::Write;

fn open_HZZ_root() -> Result<()> {
    let s = "examples/from_uproot/data/HZZ.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("events")?;
    let tree = tree.unwrap();

    for b in tree.branches() {
        println!("branch = {}, entries = {}", b.name(), b.entries());
    }

    println!("entries = {}", tree.entries());

    let mut Jet_Px = tree.branch("Jet_Px").unwrap().get_basket_into::<f32>();
    let mut Jet_Py = tree.branch("Jet_Py").unwrap().get_basket_into::<f32>();
    let mut Jet_Pz = tree.branch("Jet_Pz").unwrap().get_basket_into::<f32>();

    let NJet = tree.branch("NJet").unwrap().get_basket_into::<i32>();

    NJet.take(3000).enumerate().for_each(|(n_entry, n)| {
        println!("n_entry = {n_entry}");

        for i in 0..n {
            println!(
                "\t i = {i}  Jet_Px = {} Jet_Py = {} Jet_Pz = {} ",
                Jet_Px.next().unwrap(),
                Jet_Py.next().unwrap(),
                Jet_Pz.next().unwrap()
            );
        }
    });

    Ok(())
}

fn open_simple_root() -> Result<()> {
    let s = "examples/from_uproot/data/simple.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;
    let tree = tree.unwrap();

    let one = tree
        .branch("one")
        .unwrap()
        .get_basket_into::<i32>()
        .collect::<Vec<_>>();

    assert_eq!(one, [1, 2, 3, 4]);

    let two = tree
        .branch("two")
        .unwrap()
        .get_basket_into::<f32>()
        .collect::<Vec<_>>();

    assert_eq!(two, [1.1, 2.2, 3.3, 4.4]);

    // let f = |r: &mut RBuffer| {
    //     let val = r.read_string().unwrap().to_string();
    //     println!("val = {:?}", val);
    //     val
    // };

    let three = tree
        .branch("three")
        .unwrap()
        .get_basket_into::<String>()
        .collect::<Vec<_>>();

    assert_eq!(three, ["uno", "dos", "tres", "quatro"]);

    Ok(())
}

fn open_small_evnt_tree_fullsplit_root() -> Result<()> {
    let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;
    let tree = tree.unwrap();

    for b in tree.branches() {
        println!("branch = {}, entries = {}", b.name(), b.entries());

        for bb in b.branches() {
            println!("\tbranch = {}, entries = {}", bb.name(), bb.entries());

            for bbb in bb.branches() {
                println!("\t\tbranch = {}, entries = {}", bbb.name(), bbb.entries());
            }
        }
    }

    tree.branch("Beg")
        .unwrap()
        .get_basket_into::<String>()
        .enumerate()
        .for_each(|(i, s)| {
            assert_eq!(s, format!("beg-{:03}", i));
        });

    tree.branch("P3")
        .unwrap()
        .get_basket(|r| {
            let x = r.read_i32().unwrap();
            let y = r.read_f64().unwrap();
            let z = r.read_i32().unwrap();
            (x, y, z)
        })
        .enumerate()
        .for_each(|(i, (x, y, z))| {
            // println!("x = {x} y = {y}");
            let i = i as i32;
            assert_eq!(x, i - 1);
            assert_eq!(x, z);
            assert_eq!(y, i as f64);
        });

    tree.branch("ArrayI16[10]").unwrap();

    // println!("beg = {:?}", beg);

    // let pytree.branch("P3.Px").unwrap().get_basket_into::<f32>();

    Ok(())
}

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

    // open_HZZ_root().expect("NOOOO");
    // open_simple_root().expect("NOOOO");
    open_small_evnt_tree_fullsplit_root().expect("NOOOO");
}
