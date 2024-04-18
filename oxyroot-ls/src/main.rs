use clap::Parser;
use oxyroot::Named;
use oxyroot::Object;
use std::io::Write;
use std::path::PathBuf;

use env_logger::{Builder, Target, WriteStyle};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the file to list
    #[arg(short, long, value_name = "FILE")]
    file: PathBuf,
}

fn main() {
    Builder::new()
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
    let cli = Cli::parse();
    println!("=== {:?} ===", cli.file);

    // ensure file exixts
    let file = cli.file.as_path();
    if !file.exists() {
        eprintln!("file {:?} does not exist", file);
        std::process::exit(1);
    }

    let mut f = oxyroot::RootFile::open(file).expect("Can not open file");

    // trace!("keys = {:?}", keys);

    let keys = f.keys();
    for k in keys {
        println!(
            "> {} name='{}' (title='{}')",
            k.class(),
            k.name(),
            k.title()
        );
        println!("> Data in {}:", k.name());
        let tree = f.get_tree(k.name()).unwrap();
        tree.show();
    }
}
