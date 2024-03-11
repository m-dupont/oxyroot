use chrono::Local;
use clap::Parser;
use oxyroot::{Branch, RBuffer, SizedSlice, Tree, Unmarshaler};
use oxyroot::{RootFile, Slice};
use std::fmt::Debug;
use std::io::Write;
use std::path::PathBuf;

use env_logger;
use env_logger::{Builder, Target, WriteStyle};
use regex::Regex;

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
    pub fn new(branch: &'a Branch) -> Option<ZipperDumperItem> {
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

        macro_rules! make_box_branch_for_sized_slice {
            ($ftype: ty, $n: expr) => {{
                let f = move |r: &mut RBuffer| {
                    let mut s = SizedSlice::<$ftype>::new($n);
                    s.unmarshal(r).unwrap();
                    s
                };
                let bo: Box<dyn Iterator<Item = Box<dyn Debug + 'a>> + 'a> =
                    Box::new(branch.get_basket(f).map(|x| Box::new(x) as Box<dyn Debug>));
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
            "i64" => make_box_branch_for_type!(i64),
            "u64" => make_box_branch_for_type!(u64),
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
            "Slice<bool>" => make_box_branch_for_type!(Slice<bool>),
            "Slice<i32>" => make_box_branch_for_type!(Slice<i32>),
            "Slice<u32>" => make_box_branch_for_type!(Slice<u32>),
            "Slice<f64>" => make_box_branch_for_type!(Slice<f64>),
            "Slice<f32>" => make_box_branch_for_type!(Slice<f32>),
            "Slice<i16>" => make_box_branch_for_type!(Slice<i16>),
            "Slice<u16>" => make_box_branch_for_type!(Slice<u16>),
            "Slice<i8>" => make_box_branch_for_type!(Slice<i8>),
            "Slice<u8>" => make_box_branch_for_type!(Slice<u8>),
            "Slice<i64>" => make_box_branch_for_type!(Slice<i64>),
            "Slice<u64>" => make_box_branch_for_type!(Slice<u64>),
            "Slice<String>" => make_box_branch_for_type!(Slice<String>),
            a => {
                let re = Regex::new(r"\[([A-Za-z0-9]+);([0-9]+)\]").unwrap();
                if let Some(gs) = re.captures(a) {
                    let n = &gs[2];
                    let n = n.parse::<usize>().unwrap();

                    match &gs[1] {
                        "bool" => make_box_branch_for_sized_slice!(bool, n),
                        "i32" => make_box_branch_for_sized_slice!(i32, n),
                        "u32" => make_box_branch_for_sized_slice!(u32, n),
                        "i64" => make_box_branch_for_sized_slice!(i64, n),
                        "u64" => make_box_branch_for_sized_slice!(u64, n),
                        "f64" => make_box_branch_for_sized_slice!(f64, n),
                        "f32" => make_box_branch_for_sized_slice!(f32, n),
                        "i16" => make_box_branch_for_sized_slice!(i16, n),
                        "u16" => make_box_branch_for_sized_slice!(u16, n),
                        "i8" => make_box_branch_for_sized_slice!(i8, n),
                        "u8" => make_box_branch_for_sized_slice!(u8, n),
                        "String" => make_box_branch_for_sized_slice!(String, n),
                        _ => {
                            eprintln!(
                                "Can not interpret type_name = {:?}",
                                branch.interpretation()
                            );
                            return None;
                        }
                    }
                } else {
                    eprintln!(
                        "Can not interpret type_name = {:?}",
                        branch.interpretation()
                    );
                    return None;
                }

                // println!("a = {:?}", &gs[1]);
            }
        };
        Some(ZipperDumperItem {
            branch: branch,
            iterator: it,
        })
    }
}

pub struct ZiperDumperBranch<'a> {
    tree: &'a Tree,
    iterators: Vec<ZipperDumperItem<'a>>,
}

impl<'a> ZiperDumperBranch<'a> {
    pub fn new(tree: &'a Tree) -> ZiperDumperBranch<'a> {
        let iterators = tree
            .branches_r()
            .iter()
            .map(|b| ZipperDumperItem::new(b))
            .filter_map(|x| x)
            .collect::<Vec<_>>();

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
