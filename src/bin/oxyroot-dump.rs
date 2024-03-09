use chrono::Local;
use clap::Parser;
use oxyroot::RootFile;
use oxyroot::{Branch, Tree};
use std::fmt::Debug;
use std::io::Write;
use std::path::PathBuf;

use env_logger;
use env_logger::{Builder, Target, WriteStyle};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the file to list
    #[arg(short, long, value_name = "FILE")]
    file: PathBuf,
}

struct ZipperDumperItem<'a> {
    branch: &'a Branch,
    iterator: Box<dyn Iterator<Item = Box<dyn Debug + 'a>> + 'a>,
}

impl<'a> ZipperDumperItem<'a> {
    pub fn new(branch: &'a Branch) -> ZipperDumperItem {
        // define heare to have branch in scope
        macro_rules! make_box_branch_for_type {
            ($ftype: ty) => {{
                let bo: Box<dyn Iterator<Item = Box<dyn Debug + 'a>> + 'a> = Box::new(
                    branch
                        .as_iter::<$ftype>()
                        .map(|x| Box::new(x) as Box<dyn Debug>),
                );
                bo
            }};
        }

        let it = match branch.interpretation().as_str() {
            "i32" => {
                let bo: Box<dyn Iterator<Item = Box<dyn Debug + 'a>> + 'a> = Box::new(
                    branch
                        .as_iter::<i32>()
                        .map(|x| Box::new(x) as Box<dyn Debug>),
                );
                bo
            }

            "u32" => make_box_branch_for_type!(u32),
            "f64" => make_box_branch_for_type!(f64),
            "f32" => make_box_branch_for_type!(f32),
            "i16" => make_box_branch_for_type!(i16),
            "u16" => make_box_branch_for_type!(u16),
            "i8" => make_box_branch_for_type!(i8),
            "u8" => make_box_branch_for_type!(u8),
            "bool" => make_box_branch_for_type!(bool),
            "String" => make_box_branch_for_type!(String),
            "Vec<i32>" => make_box_branch_for_type!(Vec<i32>),
            "Vec<u32>" => make_box_branch_for_type!(Vec<u32>),
            "Vec<i16>" => make_box_branch_for_type!(Vec<i16>),
            "Vec<u16>" => make_box_branch_for_type!(Vec<u16>),
            "Vec<i8>" => make_box_branch_for_type!(Vec<i8>),
            "Vec<u8>" => make_box_branch_for_type!(Vec<u8>),
            "Vec<bool>" => make_box_branch_for_type!(Vec<bool>),
            "Vec<String>" => make_box_branch_for_type!(Vec<String>),
            "Vec<f64>" => make_box_branch_for_type!(Vec<f64>),
            "Vec<f32>" => make_box_branch_for_type!(Vec<f32>),
            _ => unimplemented!("type_name = {:?}", branch.interpretation()),
        };
        ZipperDumperItem {
            branch: branch,
            iterator: it,
        }
    }
}

pub struct ZiperDumperBranch<'a> {
    tree: &'a Tree,
    iterators: Vec<ZipperDumperItem<'a>>,
}

impl<'a> ZiperDumperBranch<'a> {
    pub fn new(tree: &'a Tree) -> ZiperDumperBranch<'a> {
        let iterators = tree.branches().map(ZipperDumperItem::new).collect();

        ZiperDumperBranch {
            tree: tree,
            iterators: iterators,
        }
    }

    pub fn dump(&mut self) {
        let n = self.tree.entries();

        for i in 0..n {
            for zbv in self.iterators.iter_mut() {
                let it = zbv.iterator.as_mut().next().unwrap();
                println!("[{}][{}]: {:?}", i, zbv.branch.name(), it);
            }
        }
    }
}

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
    let cli = Cli::parse();
    println!("=== {:?} ===", cli.file);

    // ensure file exixts
    let file = cli.file.as_path();
    if !file.exists() {
        eprintln!("file {:?} does not exist", file);
        std::process::exit(1);
    }

    let mut f = RootFile::open(file).unwrap();
    let keys = f
        .keys_name()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    for trees in keys {
        let tree = f.get_tree(&trees).unwrap();
        tree.show();

        let mut zip = ZiperDumperBranch::new(&tree);
        zip.dump();
    }

    // zip.make_iterators();

    // let mut bi32 = tree.branch("v_i").unwrap();
    // bi32.dump();
    // println!("i32: {:?}", vi32);
    // dump(bi32);
    // dump_branch(bi32);
}
