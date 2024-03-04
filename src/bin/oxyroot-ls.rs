use clap::{Parser, Subcommand};
use oxyroot::Named;
use oxyroot::Object;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the file to list
    #[arg(short, long, value_name = "FILE")]
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    println!("=== {:?} ===", cli.file);

    // ensure file exixts
    let file = cli.file.as_path();
    if !file.exists() {
        eprintln!("file {:?} does not exist", file);
        std::process::exit(1);
    }

    let mut f = oxyroot::RootFile::open(file).expect("Can not open file");
    let keys = f.keys_name().collect::<Vec<_>>();

    // println!("keys = {:?}", keys);

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
