use anyhow::Result;
use chrono::Local;
use env_logger::{Builder, Target, WriteStyle};
use log::{error, trace, LevelFilter};
use oxyroot::file::RootFile;
use oxyroot::rbytes::rbuffer::RBuffer;
use oxyroot::root::traits::Named;
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

    let mut Jet_Px = tree.get_branch("Jet_Px").unwrap().get_basket_into::<f32>();
    let mut Jet_Py = tree.get_branch("Jet_Py").unwrap().get_basket_into::<f32>();
    let mut Jet_Pz = tree.get_branch("Jet_Pz").unwrap().get_basket_into::<f32>();

    let NJet = tree.get_branch("NJet").unwrap().get_basket_into::<i32>();

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

    open_HZZ_root();
}
