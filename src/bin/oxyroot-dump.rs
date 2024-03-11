use chrono::Local;
use clap::Parser;
use oxyroot::{Branch, RBuffer, SizedSlice, Tree, Unmarshaler};
use oxyroot::{RootFile, Slice};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
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

trait Dumpable: Debug {
    fn dump(&self) -> String;
}

macro_rules! impl_dumpable_primitive {
    ($ftype:ty) => {
        impl Dumpable for $ftype {
            fn dump(&self) -> String {
                format!("{:?}", self)
            }
        }
    };
}

impl_dumpable_primitive!(i32);
impl_dumpable_primitive!(u32);
impl_dumpable_primitive!(f64);
impl_dumpable_primitive!(f32);
impl_dumpable_primitive!(i16);
impl_dumpable_primitive!(u16);
impl_dumpable_primitive!(i8);
impl_dumpable_primitive!(u8);
impl_dumpable_primitive!(i64);
impl_dumpable_primitive!(u64);
impl_dumpable_primitive!(bool);
impl_dumpable_primitive!(String);

impl<T> Dumpable for Vec<T>
where
    T: Dumpable,
{
    fn dump(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T> Dumpable for Slice<T>
where
    T: Dumpable,
{
    fn dump(&self) -> String {
        format!("{:?}", self.inner())
    }
}

impl<T> Dumpable for SizedSlice<T>
where
    T: Dumpable,
{
    fn dump(&self) -> String {
        format!("{:?}", self.inner())
    }
}

impl<T> Dumpable for HashSet<T>
where
    T: Dumpable,
{
    fn dump(&self) -> String {
        format!("{:?}", self)
    }
}

impl<K, V> Dumpable for HashMap<K, V>
where
    K: Dumpable,
    V: Dumpable,
{
    fn dump(&self) -> String {
        format!("{:?}", self)
    }
}

struct ZipperDumperItem<'a> {
    branch: &'a Branch,
    iterator: Box<dyn Iterator<Item = Box<dyn Dumpable + 'a>> + 'a>,
}

impl<'a> ZipperDumperItem<'a> {
    pub fn new(branch: &'a Branch) -> Option<ZipperDumperItem> {
        // define heare to have branch in scope
        macro_rules! make_box_branch_for_type {
            ($ftype: ty) => {{
                let bo: Box<dyn Iterator<Item = Box<dyn Dumpable + 'a>> + 'a> = Box::new(
                    branch
                        .as_iter::<$ftype>()
                        .map(|x| Box::new(x) as Box<dyn Dumpable>),
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
                let bo: Box<dyn Iterator<Item = Box<dyn Dumpable + 'a>> + 'a> = Box::new(
                    branch
                        .get_basket(f)
                        .map(|x| Box::new(x) as Box<dyn Dumpable>),
                );
                bo
            }};
        }

        type TString = String;

        let it = match branch.interpretation().as_str() {
            "u8" => make_box_branch_for_type!(u8),
            "Vec<u8>" => make_box_branch_for_type!(Vec<u8>),
            "Vec<Vec<u8>>" => make_box_branch_for_type!(Vec<Vec<u8>>),
            "Slice<u8>" => make_box_branch_for_type!(Slice<u8>),
            "i8" => make_box_branch_for_type!(i8),
            "Vec<i8>" => make_box_branch_for_type!(Vec<i8>),
            "Vec<Vec<i8>>" => make_box_branch_for_type!(Vec<Vec<i8>>),
            "Slice<i8>" => make_box_branch_for_type!(Slice<i8>),
            "i32" => make_box_branch_for_type!(i32),
            "Vec<i32>" => make_box_branch_for_type!(Vec<i32>),
            "Vec<Vec<i32>>" => make_box_branch_for_type!(Vec<Vec<i32>>),
            "Slice<i32>" => make_box_branch_for_type!(Slice<i32>),
            "u32" => make_box_branch_for_type!(u32),
            "Vec<u32>" => make_box_branch_for_type!(Vec<u32>),
            "Vec<Vec<u32>>" => make_box_branch_for_type!(Vec<Vec<u32>>),
            "Slice<u32>" => make_box_branch_for_type!(Slice<u32>),
            "i16" => make_box_branch_for_type!(i16),
            "Vec<i16>" => make_box_branch_for_type!(Vec<i16>),
            "Vec<Vec<i16>>" => make_box_branch_for_type!(Vec<Vec<i16>>),
            "Slice<i16>" => make_box_branch_for_type!(Slice<i16>),
            "u16" => make_box_branch_for_type!(u16),
            "Vec<u16>" => make_box_branch_for_type!(Vec<u16>),
            "Vec<Vec<u16>>" => make_box_branch_for_type!(Vec<Vec<u16>>),
            "Slice<u16>" => make_box_branch_for_type!(Slice<u16>),
            "bool" => make_box_branch_for_type!(bool),
            "Vec<bool>" => make_box_branch_for_type!(Vec<bool>),
            "Vec<Vec<bool>>" => make_box_branch_for_type!(Vec<Vec<bool>>),
            "Slice<bool>" => make_box_branch_for_type!(Slice<bool>),
            "f32" => make_box_branch_for_type!(f32),
            "Vec<f32>" => make_box_branch_for_type!(Vec<f32>),
            "Vec<Vec<f32>>" => make_box_branch_for_type!(Vec<Vec<f32>>),
            "Slice<f32>" => make_box_branch_for_type!(Slice<f32>),
            "f64" => make_box_branch_for_type!(f64),
            "Vec<f64>" => make_box_branch_for_type!(Vec<f64>),
            "Vec<Vec<f64>>" => make_box_branch_for_type!(Vec<Vec<f64>>),
            "Slice<f64>" => make_box_branch_for_type!(Slice<f64>),
            "String" => make_box_branch_for_type!(String),
            "Vec<String>" => make_box_branch_for_type!(Vec<String>),
            "Vec<Vec<String>>" => make_box_branch_for_type!(Vec<Vec<String>>),
            "Slice<String>" => make_box_branch_for_type!(Slice<String>),
            "TString" => make_box_branch_for_type!(TString),
            "Vec<TString>" => make_box_branch_for_type!(Vec<TString>),
            "Vec<Vec<TString>>" => make_box_branch_for_type!(Vec<Vec<TString>>),
            "Slice<TString>" => make_box_branch_for_type!(Slice<TString>),
            "Vec<HashSet<u8>>" => make_box_branch_for_type!(Vec<HashSet<u8>>),
            "Vec<HashSet<i8>>" => make_box_branch_for_type!(Vec<HashSet<i8>>),
            "Vec<HashSet<i32>>" => make_box_branch_for_type!(Vec<HashSet<i32>>),
            "Vec<HashSet<u32>>" => make_box_branch_for_type!(Vec<HashSet<u32>>),
            "Vec<HashSet<i16>>" => make_box_branch_for_type!(Vec<HashSet<i16>>),
            "Vec<HashSet<u16>>" => make_box_branch_for_type!(Vec<HashSet<u16>>),
            "Vec<HashSet<bool>>" => make_box_branch_for_type!(Vec<HashSet<bool>>),
            "Vec<HashSet<String>>" => make_box_branch_for_type!(Vec<HashSet<String>>),
            "Vec<HashSet<TString>>" => make_box_branch_for_type!(Vec<HashSet<TString>>),
            "HashSet<u8>" => make_box_branch_for_type!(HashSet<u8>),
            "HashMap<u8,u8>" => make_box_branch_for_type!(HashMap<u8,u8>),
            "HashMap<u8,Vec<u8>>" => make_box_branch_for_type!(HashMap<u8,Vec<u8>>),
            "HashMap<u8,Vec<Vec<u8>>>" => make_box_branch_for_type!(HashMap<u8,Vec<Vec<u8>>>),
            "HashMap<u8,Slice<u8>>" => make_box_branch_for_type!(HashMap<u8,Slice<u8>>),
            "HashMap<u8,i8>" => make_box_branch_for_type!(HashMap<u8,i8>),
            "HashMap<u8,Vec<i8>>" => make_box_branch_for_type!(HashMap<u8,Vec<i8>>),
            "HashMap<u8,Vec<Vec<i8>>>" => make_box_branch_for_type!(HashMap<u8,Vec<Vec<i8>>>),
            "HashMap<u8,Slice<i8>>" => make_box_branch_for_type!(HashMap<u8,Slice<i8>>),
            "HashMap<u8,i32>" => make_box_branch_for_type!(HashMap<u8,i32>),
            "HashMap<u8,Vec<i32>>" => make_box_branch_for_type!(HashMap<u8,Vec<i32>>),
            "HashMap<u8,Vec<Vec<i32>>>" => make_box_branch_for_type!(HashMap<u8,Vec<Vec<i32>>>),
            "HashMap<u8,Slice<i32>>" => make_box_branch_for_type!(HashMap<u8,Slice<i32>>),
            "HashMap<u8,u32>" => make_box_branch_for_type!(HashMap<u8,u32>),
            "HashMap<u8,Vec<u32>>" => make_box_branch_for_type!(HashMap<u8,Vec<u32>>),
            "HashMap<u8,Vec<Vec<u32>>>" => make_box_branch_for_type!(HashMap<u8,Vec<Vec<u32>>>),
            "HashMap<u8,Slice<u32>>" => make_box_branch_for_type!(HashMap<u8,Slice<u32>>),
            "HashMap<u8,i16>" => make_box_branch_for_type!(HashMap<u8,i16>),
            "HashMap<u8,Vec<i16>>" => make_box_branch_for_type!(HashMap<u8,Vec<i16>>),
            "HashMap<u8,Vec<Vec<i16>>>" => make_box_branch_for_type!(HashMap<u8,Vec<Vec<i16>>>),
            "HashMap<u8,Slice<i16>>" => make_box_branch_for_type!(HashMap<u8,Slice<i16>>),
            "HashMap<u8,u16>" => make_box_branch_for_type!(HashMap<u8,u16>),
            "HashMap<u8,Vec<u16>>" => make_box_branch_for_type!(HashMap<u8,Vec<u16>>),
            "HashMap<u8,Vec<Vec<u16>>>" => make_box_branch_for_type!(HashMap<u8,Vec<Vec<u16>>>),
            "HashMap<u8,Slice<u16>>" => make_box_branch_for_type!(HashMap<u8,Slice<u16>>),
            "HashMap<u8,bool>" => make_box_branch_for_type!(HashMap<u8,bool>),
            "HashMap<u8,Vec<bool>>" => make_box_branch_for_type!(HashMap<u8,Vec<bool>>),
            "HashMap<u8,Vec<Vec<bool>>>" => make_box_branch_for_type!(HashMap<u8,Vec<Vec<bool>>>),
            "HashMap<u8,Slice<bool>>" => make_box_branch_for_type!(HashMap<u8,Slice<bool>>),
            "HashMap<u8,f32>" => make_box_branch_for_type!(HashMap<u8,f32>),
            "HashMap<u8,Vec<f32>>" => make_box_branch_for_type!(HashMap<u8,Vec<f32>>),
            "HashMap<u8,Vec<Vec<f32>>>" => make_box_branch_for_type!(HashMap<u8,Vec<Vec<f32>>>),
            "HashMap<u8,Slice<f32>>" => make_box_branch_for_type!(HashMap<u8,Slice<f32>>),
            "HashMap<u8,f64>" => make_box_branch_for_type!(HashMap<u8,f64>),
            "HashMap<u8,Vec<f64>>" => make_box_branch_for_type!(HashMap<u8,Vec<f64>>),
            "HashMap<u8,Vec<Vec<f64>>>" => make_box_branch_for_type!(HashMap<u8,Vec<Vec<f64>>>),
            "HashMap<u8,Slice<f64>>" => make_box_branch_for_type!(HashMap<u8,Slice<f64>>),
            "HashMap<u8,String>" => make_box_branch_for_type!(HashMap<u8,String>),
            "HashMap<u8,Vec<String>>" => make_box_branch_for_type!(HashMap<u8,Vec<String>>),
            "HashMap<u8,Vec<Vec<String>>>" => {
                make_box_branch_for_type!(HashMap<u8,Vec<Vec<String>>>)
            }
            "HashMap<u8,Slice<String>>" => make_box_branch_for_type!(HashMap<u8,Slice<String>>),
            "HashMap<u8,TString>" => make_box_branch_for_type!(HashMap<u8,TString>),
            "HashMap<u8,Vec<TString>>" => make_box_branch_for_type!(HashMap<u8,Vec<TString>>),
            "HashMap<u8,Vec<Vec<TString>>>" => {
                make_box_branch_for_type!(HashMap<u8,Vec<Vec<TString>>>)
            }
            "HashMap<u8,Slice<TString>>" => make_box_branch_for_type!(HashMap<u8,Slice<TString>>),
            "HashMap<u8,HashSet<u8>>" => make_box_branch_for_type!(HashMap<u8,HashSet<u8>>),
            "HashMap<u8,Vec<HashSet<u8>>>" => {
                make_box_branch_for_type!(HashMap<u8,Vec<HashSet<u8>>>)
            }
            "HashMap<u8,HashSet<i8>>" => make_box_branch_for_type!(HashMap<u8,HashSet<i8>>),
            "HashMap<u8,Vec<HashSet<i8>>>" => {
                make_box_branch_for_type!(HashMap<u8,Vec<HashSet<i8>>>)
            }
            "HashMap<u8,HashSet<i32>>" => make_box_branch_for_type!(HashMap<u8,HashSet<i32>>),
            "HashMap<u8,Vec<HashSet<i32>>>" => {
                make_box_branch_for_type!(HashMap<u8,Vec<HashSet<i32>>>)
            }
            "HashMap<u8,HashSet<u32>>" => make_box_branch_for_type!(HashMap<u8,HashSet<u32>>),
            "HashMap<u8,Vec<HashSet<u32>>>" => {
                make_box_branch_for_type!(HashMap<u8,Vec<HashSet<u32>>>)
            }
            "HashMap<u8,HashSet<i16>>" => make_box_branch_for_type!(HashMap<u8,HashSet<i16>>),
            "HashMap<u8,Vec<HashSet<i16>>>" => {
                make_box_branch_for_type!(HashMap<u8,Vec<HashSet<i16>>>)
            }
            "HashMap<u8,HashSet<u16>>" => make_box_branch_for_type!(HashMap<u8,HashSet<u16>>),
            "HashMap<u8,Vec<HashSet<u16>>>" => {
                make_box_branch_for_type!(HashMap<u8,Vec<HashSet<u16>>>)
            }
            "HashMap<u8,HashSet<bool>>" => make_box_branch_for_type!(HashMap<u8,HashSet<bool>>),
            "HashMap<u8,Vec<HashSet<bool>>>" => {
                make_box_branch_for_type!(HashMap<u8,Vec<HashSet<bool>>>)
            }
            "HashMap<u8,HashSet<String>>" => make_box_branch_for_type!(HashMap<u8,HashSet<String>>),
            "HashMap<u8,Vec<HashSet<String>>>" => {
                make_box_branch_for_type!(HashMap<u8,Vec<HashSet<String>>>)
            }
            "HashMap<u8,HashSet<TString>>" => {
                make_box_branch_for_type!(HashMap<u8,HashSet<TString>>)
            }
            "HashMap<u8,Vec<HashSet<TString>>>" => {
                make_box_branch_for_type!(HashMap<u8,Vec<HashSet<TString>>>)
            }
            "HashSet<i8>" => make_box_branch_for_type!(HashSet<i8>),
            "HashMap<i8,u8>" => make_box_branch_for_type!(HashMap<i8,u8>),
            "HashMap<i8,Vec<u8>>" => make_box_branch_for_type!(HashMap<i8,Vec<u8>>),
            "HashMap<i8,Vec<Vec<u8>>>" => make_box_branch_for_type!(HashMap<i8,Vec<Vec<u8>>>),
            "HashMap<i8,Slice<u8>>" => make_box_branch_for_type!(HashMap<i8,Slice<u8>>),
            "HashMap<i8,i8>" => make_box_branch_for_type!(HashMap<i8,i8>),
            "HashMap<i8,Vec<i8>>" => make_box_branch_for_type!(HashMap<i8,Vec<i8>>),
            "HashMap<i8,Vec<Vec<i8>>>" => make_box_branch_for_type!(HashMap<i8,Vec<Vec<i8>>>),
            "HashMap<i8,Slice<i8>>" => make_box_branch_for_type!(HashMap<i8,Slice<i8>>),
            "HashMap<i8,i32>" => make_box_branch_for_type!(HashMap<i8,i32>),
            "HashMap<i8,Vec<i32>>" => make_box_branch_for_type!(HashMap<i8,Vec<i32>>),
            "HashMap<i8,Vec<Vec<i32>>>" => make_box_branch_for_type!(HashMap<i8,Vec<Vec<i32>>>),
            "HashMap<i8,Slice<i32>>" => make_box_branch_for_type!(HashMap<i8,Slice<i32>>),
            "HashMap<i8,u32>" => make_box_branch_for_type!(HashMap<i8,u32>),
            "HashMap<i8,Vec<u32>>" => make_box_branch_for_type!(HashMap<i8,Vec<u32>>),
            "HashMap<i8,Vec<Vec<u32>>>" => make_box_branch_for_type!(HashMap<i8,Vec<Vec<u32>>>),
            "HashMap<i8,Slice<u32>>" => make_box_branch_for_type!(HashMap<i8,Slice<u32>>),
            "HashMap<i8,i16>" => make_box_branch_for_type!(HashMap<i8,i16>),
            "HashMap<i8,Vec<i16>>" => make_box_branch_for_type!(HashMap<i8,Vec<i16>>),
            "HashMap<i8,Vec<Vec<i16>>>" => make_box_branch_for_type!(HashMap<i8,Vec<Vec<i16>>>),
            "HashMap<i8,Slice<i16>>" => make_box_branch_for_type!(HashMap<i8,Slice<i16>>),
            "HashMap<i8,u16>" => make_box_branch_for_type!(HashMap<i8,u16>),
            "HashMap<i8,Vec<u16>>" => make_box_branch_for_type!(HashMap<i8,Vec<u16>>),
            "HashMap<i8,Vec<Vec<u16>>>" => make_box_branch_for_type!(HashMap<i8,Vec<Vec<u16>>>),
            "HashMap<i8,Slice<u16>>" => make_box_branch_for_type!(HashMap<i8,Slice<u16>>),
            "HashMap<i8,bool>" => make_box_branch_for_type!(HashMap<i8,bool>),
            "HashMap<i8,Vec<bool>>" => make_box_branch_for_type!(HashMap<i8,Vec<bool>>),
            "HashMap<i8,Vec<Vec<bool>>>" => make_box_branch_for_type!(HashMap<i8,Vec<Vec<bool>>>),
            "HashMap<i8,Slice<bool>>" => make_box_branch_for_type!(HashMap<i8,Slice<bool>>),
            "HashMap<i8,f32>" => make_box_branch_for_type!(HashMap<i8,f32>),
            "HashMap<i8,Vec<f32>>" => make_box_branch_for_type!(HashMap<i8,Vec<f32>>),
            "HashMap<i8,Vec<Vec<f32>>>" => make_box_branch_for_type!(HashMap<i8,Vec<Vec<f32>>>),
            "HashMap<i8,Slice<f32>>" => make_box_branch_for_type!(HashMap<i8,Slice<f32>>),
            "HashMap<i8,f64>" => make_box_branch_for_type!(HashMap<i8,f64>),
            "HashMap<i8,Vec<f64>>" => make_box_branch_for_type!(HashMap<i8,Vec<f64>>),
            "HashMap<i8,Vec<Vec<f64>>>" => make_box_branch_for_type!(HashMap<i8,Vec<Vec<f64>>>),
            "HashMap<i8,Slice<f64>>" => make_box_branch_for_type!(HashMap<i8,Slice<f64>>),
            "HashMap<i8,String>" => make_box_branch_for_type!(HashMap<i8,String>),
            "HashMap<i8,Vec<String>>" => make_box_branch_for_type!(HashMap<i8,Vec<String>>),
            "HashMap<i8,Vec<Vec<String>>>" => {
                make_box_branch_for_type!(HashMap<i8,Vec<Vec<String>>>)
            }
            "HashMap<i8,Slice<String>>" => make_box_branch_for_type!(HashMap<i8,Slice<String>>),
            "HashMap<i8,TString>" => make_box_branch_for_type!(HashMap<i8,TString>),
            "HashMap<i8,Vec<TString>>" => make_box_branch_for_type!(HashMap<i8,Vec<TString>>),
            "HashMap<i8,Vec<Vec<TString>>>" => {
                make_box_branch_for_type!(HashMap<i8,Vec<Vec<TString>>>)
            }
            "HashMap<i8,Slice<TString>>" => make_box_branch_for_type!(HashMap<i8,Slice<TString>>),
            "HashMap<i8,HashSet<u8>>" => make_box_branch_for_type!(HashMap<i8,HashSet<u8>>),
            "HashMap<i8,Vec<HashSet<u8>>>" => {
                make_box_branch_for_type!(HashMap<i8,Vec<HashSet<u8>>>)
            }
            "HashMap<i8,HashSet<i8>>" => make_box_branch_for_type!(HashMap<i8,HashSet<i8>>),
            "HashMap<i8,Vec<HashSet<i8>>>" => {
                make_box_branch_for_type!(HashMap<i8,Vec<HashSet<i8>>>)
            }
            "HashMap<i8,HashSet<i32>>" => make_box_branch_for_type!(HashMap<i8,HashSet<i32>>),
            "HashMap<i8,Vec<HashSet<i32>>>" => {
                make_box_branch_for_type!(HashMap<i8,Vec<HashSet<i32>>>)
            }
            "HashMap<i8,HashSet<u32>>" => make_box_branch_for_type!(HashMap<i8,HashSet<u32>>),
            "HashMap<i8,Vec<HashSet<u32>>>" => {
                make_box_branch_for_type!(HashMap<i8,Vec<HashSet<u32>>>)
            }
            "HashMap<i8,HashSet<i16>>" => make_box_branch_for_type!(HashMap<i8,HashSet<i16>>),
            "HashMap<i8,Vec<HashSet<i16>>>" => {
                make_box_branch_for_type!(HashMap<i8,Vec<HashSet<i16>>>)
            }
            "HashMap<i8,HashSet<u16>>" => make_box_branch_for_type!(HashMap<i8,HashSet<u16>>),
            "HashMap<i8,Vec<HashSet<u16>>>" => {
                make_box_branch_for_type!(HashMap<i8,Vec<HashSet<u16>>>)
            }
            "HashMap<i8,HashSet<bool>>" => make_box_branch_for_type!(HashMap<i8,HashSet<bool>>),
            "HashMap<i8,Vec<HashSet<bool>>>" => {
                make_box_branch_for_type!(HashMap<i8,Vec<HashSet<bool>>>)
            }
            "HashMap<i8,HashSet<String>>" => make_box_branch_for_type!(HashMap<i8,HashSet<String>>),
            "HashMap<i8,Vec<HashSet<String>>>" => {
                make_box_branch_for_type!(HashMap<i8,Vec<HashSet<String>>>)
            }
            "HashMap<i8,HashSet<TString>>" => {
                make_box_branch_for_type!(HashMap<i8,HashSet<TString>>)
            }
            "HashMap<i8,Vec<HashSet<TString>>>" => {
                make_box_branch_for_type!(HashMap<i8,Vec<HashSet<TString>>>)
            }
            "HashSet<i32>" => make_box_branch_for_type!(HashSet<i32>),
            "HashMap<i32,u8>" => make_box_branch_for_type!(HashMap<i32,u8>),
            "HashMap<i32,Vec<u8>>" => make_box_branch_for_type!(HashMap<i32,Vec<u8>>),
            "HashMap<i32,Vec<Vec<u8>>>" => make_box_branch_for_type!(HashMap<i32,Vec<Vec<u8>>>),
            "HashMap<i32,Slice<u8>>" => make_box_branch_for_type!(HashMap<i32,Slice<u8>>),
            "HashMap<i32,i8>" => make_box_branch_for_type!(HashMap<i32,i8>),
            "HashMap<i32,Vec<i8>>" => make_box_branch_for_type!(HashMap<i32,Vec<i8>>),
            "HashMap<i32,Vec<Vec<i8>>>" => make_box_branch_for_type!(HashMap<i32,Vec<Vec<i8>>>),
            "HashMap<i32,Slice<i8>>" => make_box_branch_for_type!(HashMap<i32,Slice<i8>>),
            "HashMap<i32,i32>" => make_box_branch_for_type!(HashMap<i32,i32>),
            "HashMap<i32,Vec<i32>>" => make_box_branch_for_type!(HashMap<i32,Vec<i32>>),
            "HashMap<i32,Vec<Vec<i32>>>" => make_box_branch_for_type!(HashMap<i32,Vec<Vec<i32>>>),
            "HashMap<i32,Slice<i32>>" => make_box_branch_for_type!(HashMap<i32,Slice<i32>>),
            "HashMap<i32,u32>" => make_box_branch_for_type!(HashMap<i32,u32>),
            "HashMap<i32,Vec<u32>>" => make_box_branch_for_type!(HashMap<i32,Vec<u32>>),
            "HashMap<i32,Vec<Vec<u32>>>" => make_box_branch_for_type!(HashMap<i32,Vec<Vec<u32>>>),
            "HashMap<i32,Slice<u32>>" => make_box_branch_for_type!(HashMap<i32,Slice<u32>>),
            "HashMap<i32,i16>" => make_box_branch_for_type!(HashMap<i32,i16>),
            "HashMap<i32,Vec<i16>>" => make_box_branch_for_type!(HashMap<i32,Vec<i16>>),
            "HashMap<i32,Vec<Vec<i16>>>" => make_box_branch_for_type!(HashMap<i32,Vec<Vec<i16>>>),
            "HashMap<i32,Slice<i16>>" => make_box_branch_for_type!(HashMap<i32,Slice<i16>>),
            "HashMap<i32,u16>" => make_box_branch_for_type!(HashMap<i32,u16>),
            "HashMap<i32,Vec<u16>>" => make_box_branch_for_type!(HashMap<i32,Vec<u16>>),
            "HashMap<i32,Vec<Vec<u16>>>" => make_box_branch_for_type!(HashMap<i32,Vec<Vec<u16>>>),
            "HashMap<i32,Slice<u16>>" => make_box_branch_for_type!(HashMap<i32,Slice<u16>>),
            "HashMap<i32,bool>" => make_box_branch_for_type!(HashMap<i32,bool>),
            "HashMap<i32,Vec<bool>>" => make_box_branch_for_type!(HashMap<i32,Vec<bool>>),
            "HashMap<i32,Vec<Vec<bool>>>" => make_box_branch_for_type!(HashMap<i32,Vec<Vec<bool>>>),
            "HashMap<i32,Slice<bool>>" => make_box_branch_for_type!(HashMap<i32,Slice<bool>>),
            "HashMap<i32,f32>" => make_box_branch_for_type!(HashMap<i32,f32>),
            "HashMap<i32,Vec<f32>>" => make_box_branch_for_type!(HashMap<i32,Vec<f32>>),
            "HashMap<i32,Vec<Vec<f32>>>" => make_box_branch_for_type!(HashMap<i32,Vec<Vec<f32>>>),
            "HashMap<i32,Slice<f32>>" => make_box_branch_for_type!(HashMap<i32,Slice<f32>>),
            "HashMap<i32,f64>" => make_box_branch_for_type!(HashMap<i32,f64>),
            "HashMap<i32,Vec<f64>>" => make_box_branch_for_type!(HashMap<i32,Vec<f64>>),
            "HashMap<i32,Vec<Vec<f64>>>" => make_box_branch_for_type!(HashMap<i32,Vec<Vec<f64>>>),
            "HashMap<i32,Slice<f64>>" => make_box_branch_for_type!(HashMap<i32,Slice<f64>>),
            "HashMap<i32,String>" => make_box_branch_for_type!(HashMap<i32,String>),
            "HashMap<i32,Vec<String>>" => make_box_branch_for_type!(HashMap<i32,Vec<String>>),
            "HashMap<i32,Vec<Vec<String>>>" => {
                make_box_branch_for_type!(HashMap<i32,Vec<Vec<String>>>)
            }
            "HashMap<i32,Slice<String>>" => make_box_branch_for_type!(HashMap<i32,Slice<String>>),
            "HashMap<i32,TString>" => make_box_branch_for_type!(HashMap<i32,TString>),
            "HashMap<i32,Vec<TString>>" => make_box_branch_for_type!(HashMap<i32,Vec<TString>>),
            "HashMap<i32,Vec<Vec<TString>>>" => {
                make_box_branch_for_type!(HashMap<i32,Vec<Vec<TString>>>)
            }
            "HashMap<i32,Slice<TString>>" => make_box_branch_for_type!(HashMap<i32,Slice<TString>>),
            "HashMap<i32,HashSet<u8>>" => make_box_branch_for_type!(HashMap<i32,HashSet<u8>>),
            "HashMap<i32,Vec<HashSet<u8>>>" => {
                make_box_branch_for_type!(HashMap<i32,Vec<HashSet<u8>>>)
            }
            "HashMap<i32,HashSet<i8>>" => make_box_branch_for_type!(HashMap<i32,HashSet<i8>>),
            "HashMap<i32,Vec<HashSet<i8>>>" => {
                make_box_branch_for_type!(HashMap<i32,Vec<HashSet<i8>>>)
            }
            "HashMap<i32,HashSet<i32>>" => make_box_branch_for_type!(HashMap<i32,HashSet<i32>>),
            "HashMap<i32,Vec<HashSet<i32>>>" => {
                make_box_branch_for_type!(HashMap<i32,Vec<HashSet<i32>>>)
            }
            "HashMap<i32,HashSet<u32>>" => make_box_branch_for_type!(HashMap<i32,HashSet<u32>>),
            "HashMap<i32,Vec<HashSet<u32>>>" => {
                make_box_branch_for_type!(HashMap<i32,Vec<HashSet<u32>>>)
            }
            "HashMap<i32,HashSet<i16>>" => make_box_branch_for_type!(HashMap<i32,HashSet<i16>>),
            "HashMap<i32,Vec<HashSet<i16>>>" => {
                make_box_branch_for_type!(HashMap<i32,Vec<HashSet<i16>>>)
            }
            "HashMap<i32,HashSet<u16>>" => make_box_branch_for_type!(HashMap<i32,HashSet<u16>>),
            "HashMap<i32,Vec<HashSet<u16>>>" => {
                make_box_branch_for_type!(HashMap<i32,Vec<HashSet<u16>>>)
            }
            "HashMap<i32,HashSet<bool>>" => make_box_branch_for_type!(HashMap<i32,HashSet<bool>>),
            "HashMap<i32,Vec<HashSet<bool>>>" => {
                make_box_branch_for_type!(HashMap<i32,Vec<HashSet<bool>>>)
            }
            "HashMap<i32,HashSet<String>>" => {
                make_box_branch_for_type!(HashMap<i32,HashSet<String>>)
            }
            "HashMap<i32,Vec<HashSet<String>>>" => {
                make_box_branch_for_type!(HashMap<i32,Vec<HashSet<String>>>)
            }
            "HashMap<i32,HashSet<TString>>" => {
                make_box_branch_for_type!(HashMap<i32,HashSet<TString>>)
            }
            "HashMap<i32,Vec<HashSet<TString>>>" => {
                make_box_branch_for_type!(HashMap<i32,Vec<HashSet<TString>>>)
            }
            "HashSet<u32>" => make_box_branch_for_type!(HashSet<u32>),
            "HashMap<u32,u8>" => make_box_branch_for_type!(HashMap<u32,u8>),
            "HashMap<u32,Vec<u8>>" => make_box_branch_for_type!(HashMap<u32,Vec<u8>>),
            "HashMap<u32,Vec<Vec<u8>>>" => make_box_branch_for_type!(HashMap<u32,Vec<Vec<u8>>>),
            "HashMap<u32,Slice<u8>>" => make_box_branch_for_type!(HashMap<u32,Slice<u8>>),
            "HashMap<u32,i8>" => make_box_branch_for_type!(HashMap<u32,i8>),
            "HashMap<u32,Vec<i8>>" => make_box_branch_for_type!(HashMap<u32,Vec<i8>>),
            "HashMap<u32,Vec<Vec<i8>>>" => make_box_branch_for_type!(HashMap<u32,Vec<Vec<i8>>>),
            "HashMap<u32,Slice<i8>>" => make_box_branch_for_type!(HashMap<u32,Slice<i8>>),
            "HashMap<u32,i32>" => make_box_branch_for_type!(HashMap<u32,i32>),
            "HashMap<u32,Vec<i32>>" => make_box_branch_for_type!(HashMap<u32,Vec<i32>>),
            "HashMap<u32,Vec<Vec<i32>>>" => make_box_branch_for_type!(HashMap<u32,Vec<Vec<i32>>>),
            "HashMap<u32,Slice<i32>>" => make_box_branch_for_type!(HashMap<u32,Slice<i32>>),
            "HashMap<u32,u32>" => make_box_branch_for_type!(HashMap<u32,u32>),
            "HashMap<u32,Vec<u32>>" => make_box_branch_for_type!(HashMap<u32,Vec<u32>>),
            "HashMap<u32,Vec<Vec<u32>>>" => make_box_branch_for_type!(HashMap<u32,Vec<Vec<u32>>>),
            "HashMap<u32,Slice<u32>>" => make_box_branch_for_type!(HashMap<u32,Slice<u32>>),
            "HashMap<u32,i16>" => make_box_branch_for_type!(HashMap<u32,i16>),
            "HashMap<u32,Vec<i16>>" => make_box_branch_for_type!(HashMap<u32,Vec<i16>>),
            "HashMap<u32,Vec<Vec<i16>>>" => make_box_branch_for_type!(HashMap<u32,Vec<Vec<i16>>>),
            "HashMap<u32,Slice<i16>>" => make_box_branch_for_type!(HashMap<u32,Slice<i16>>),
            "HashMap<u32,u16>" => make_box_branch_for_type!(HashMap<u32,u16>),
            "HashMap<u32,Vec<u16>>" => make_box_branch_for_type!(HashMap<u32,Vec<u16>>),
            "HashMap<u32,Vec<Vec<u16>>>" => make_box_branch_for_type!(HashMap<u32,Vec<Vec<u16>>>),
            "HashMap<u32,Slice<u16>>" => make_box_branch_for_type!(HashMap<u32,Slice<u16>>),
            "HashMap<u32,bool>" => make_box_branch_for_type!(HashMap<u32,bool>),
            "HashMap<u32,Vec<bool>>" => make_box_branch_for_type!(HashMap<u32,Vec<bool>>),
            "HashMap<u32,Vec<Vec<bool>>>" => make_box_branch_for_type!(HashMap<u32,Vec<Vec<bool>>>),
            "HashMap<u32,Slice<bool>>" => make_box_branch_for_type!(HashMap<u32,Slice<bool>>),
            "HashMap<u32,f32>" => make_box_branch_for_type!(HashMap<u32,f32>),
            "HashMap<u32,Vec<f32>>" => make_box_branch_for_type!(HashMap<u32,Vec<f32>>),
            "HashMap<u32,Vec<Vec<f32>>>" => make_box_branch_for_type!(HashMap<u32,Vec<Vec<f32>>>),
            "HashMap<u32,Slice<f32>>" => make_box_branch_for_type!(HashMap<u32,Slice<f32>>),
            "HashMap<u32,f64>" => make_box_branch_for_type!(HashMap<u32,f64>),
            "HashMap<u32,Vec<f64>>" => make_box_branch_for_type!(HashMap<u32,Vec<f64>>),
            "HashMap<u32,Vec<Vec<f64>>>" => make_box_branch_for_type!(HashMap<u32,Vec<Vec<f64>>>),
            "HashMap<u32,Slice<f64>>" => make_box_branch_for_type!(HashMap<u32,Slice<f64>>),
            "HashMap<u32,String>" => make_box_branch_for_type!(HashMap<u32,String>),
            "HashMap<u32,Vec<String>>" => make_box_branch_for_type!(HashMap<u32,Vec<String>>),
            "HashMap<u32,Vec<Vec<String>>>" => {
                make_box_branch_for_type!(HashMap<u32,Vec<Vec<String>>>)
            }
            "HashMap<u32,Slice<String>>" => make_box_branch_for_type!(HashMap<u32,Slice<String>>),
            "HashMap<u32,TString>" => make_box_branch_for_type!(HashMap<u32,TString>),
            "HashMap<u32,Vec<TString>>" => make_box_branch_for_type!(HashMap<u32,Vec<TString>>),
            "HashMap<u32,Vec<Vec<TString>>>" => {
                make_box_branch_for_type!(HashMap<u32,Vec<Vec<TString>>>)
            }
            "HashMap<u32,Slice<TString>>" => make_box_branch_for_type!(HashMap<u32,Slice<TString>>),
            "HashMap<u32,HashSet<u8>>" => make_box_branch_for_type!(HashMap<u32,HashSet<u8>>),
            "HashMap<u32,Vec<HashSet<u8>>>" => {
                make_box_branch_for_type!(HashMap<u32,Vec<HashSet<u8>>>)
            }
            "HashMap<u32,HashSet<i8>>" => make_box_branch_for_type!(HashMap<u32,HashSet<i8>>),
            "HashMap<u32,Vec<HashSet<i8>>>" => {
                make_box_branch_for_type!(HashMap<u32,Vec<HashSet<i8>>>)
            }
            "HashMap<u32,HashSet<i32>>" => make_box_branch_for_type!(HashMap<u32,HashSet<i32>>),
            "HashMap<u32,Vec<HashSet<i32>>>" => {
                make_box_branch_for_type!(HashMap<u32,Vec<HashSet<i32>>>)
            }
            "HashMap<u32,HashSet<u32>>" => make_box_branch_for_type!(HashMap<u32,HashSet<u32>>),
            "HashMap<u32,Vec<HashSet<u32>>>" => {
                make_box_branch_for_type!(HashMap<u32,Vec<HashSet<u32>>>)
            }
            "HashMap<u32,HashSet<i16>>" => make_box_branch_for_type!(HashMap<u32,HashSet<i16>>),
            "HashMap<u32,Vec<HashSet<i16>>>" => {
                make_box_branch_for_type!(HashMap<u32,Vec<HashSet<i16>>>)
            }
            "HashMap<u32,HashSet<u16>>" => make_box_branch_for_type!(HashMap<u32,HashSet<u16>>),
            "HashMap<u32,Vec<HashSet<u16>>>" => {
                make_box_branch_for_type!(HashMap<u32,Vec<HashSet<u16>>>)
            }
            "HashMap<u32,HashSet<bool>>" => make_box_branch_for_type!(HashMap<u32,HashSet<bool>>),
            "HashMap<u32,Vec<HashSet<bool>>>" => {
                make_box_branch_for_type!(HashMap<u32,Vec<HashSet<bool>>>)
            }
            "HashMap<u32,HashSet<String>>" => {
                make_box_branch_for_type!(HashMap<u32,HashSet<String>>)
            }
            "HashMap<u32,Vec<HashSet<String>>>" => {
                make_box_branch_for_type!(HashMap<u32,Vec<HashSet<String>>>)
            }
            "HashMap<u32,HashSet<TString>>" => {
                make_box_branch_for_type!(HashMap<u32,HashSet<TString>>)
            }
            "HashMap<u32,Vec<HashSet<TString>>>" => {
                make_box_branch_for_type!(HashMap<u32,Vec<HashSet<TString>>>)
            }
            "HashSet<i16>" => make_box_branch_for_type!(HashSet<i16>),
            "HashMap<i16,u8>" => make_box_branch_for_type!(HashMap<i16,u8>),
            "HashMap<i16,Vec<u8>>" => make_box_branch_for_type!(HashMap<i16,Vec<u8>>),
            "HashMap<i16,Vec<Vec<u8>>>" => make_box_branch_for_type!(HashMap<i16,Vec<Vec<u8>>>),
            "HashMap<i16,Slice<u8>>" => make_box_branch_for_type!(HashMap<i16,Slice<u8>>),
            "HashMap<i16,i8>" => make_box_branch_for_type!(HashMap<i16,i8>),
            "HashMap<i16,Vec<i8>>" => make_box_branch_for_type!(HashMap<i16,Vec<i8>>),
            "HashMap<i16,Vec<Vec<i8>>>" => make_box_branch_for_type!(HashMap<i16,Vec<Vec<i8>>>),
            "HashMap<i16,Slice<i8>>" => make_box_branch_for_type!(HashMap<i16,Slice<i8>>),
            "HashMap<i16,i32>" => make_box_branch_for_type!(HashMap<i16,i32>),
            "HashMap<i16,Vec<i32>>" => make_box_branch_for_type!(HashMap<i16,Vec<i32>>),
            "HashMap<i16,Vec<Vec<i32>>>" => make_box_branch_for_type!(HashMap<i16,Vec<Vec<i32>>>),
            "HashMap<i16,Slice<i32>>" => make_box_branch_for_type!(HashMap<i16,Slice<i32>>),
            "HashMap<i16,u32>" => make_box_branch_for_type!(HashMap<i16,u32>),
            "HashMap<i16,Vec<u32>>" => make_box_branch_for_type!(HashMap<i16,Vec<u32>>),
            "HashMap<i16,Vec<Vec<u32>>>" => make_box_branch_for_type!(HashMap<i16,Vec<Vec<u32>>>),
            "HashMap<i16,Slice<u32>>" => make_box_branch_for_type!(HashMap<i16,Slice<u32>>),
            "HashMap<i16,i16>" => make_box_branch_for_type!(HashMap<i16,i16>),
            "HashMap<i16,Vec<i16>>" => make_box_branch_for_type!(HashMap<i16,Vec<i16>>),
            "HashMap<i16,Vec<Vec<i16>>>" => make_box_branch_for_type!(HashMap<i16,Vec<Vec<i16>>>),
            "HashMap<i16,Slice<i16>>" => make_box_branch_for_type!(HashMap<i16,Slice<i16>>),
            "HashMap<i16,u16>" => make_box_branch_for_type!(HashMap<i16,u16>),
            "HashMap<i16,Vec<u16>>" => make_box_branch_for_type!(HashMap<i16,Vec<u16>>),
            "HashMap<i16,Vec<Vec<u16>>>" => make_box_branch_for_type!(HashMap<i16,Vec<Vec<u16>>>),
            "HashMap<i16,Slice<u16>>" => make_box_branch_for_type!(HashMap<i16,Slice<u16>>),
            "HashMap<i16,bool>" => make_box_branch_for_type!(HashMap<i16,bool>),
            "HashMap<i16,Vec<bool>>" => make_box_branch_for_type!(HashMap<i16,Vec<bool>>),
            "HashMap<i16,Vec<Vec<bool>>>" => make_box_branch_for_type!(HashMap<i16,Vec<Vec<bool>>>),
            "HashMap<i16,Slice<bool>>" => make_box_branch_for_type!(HashMap<i16,Slice<bool>>),
            "HashMap<i16,f32>" => make_box_branch_for_type!(HashMap<i16,f32>),
            "HashMap<i16,Vec<f32>>" => make_box_branch_for_type!(HashMap<i16,Vec<f32>>),
            "HashMap<i16,Vec<Vec<f32>>>" => make_box_branch_for_type!(HashMap<i16,Vec<Vec<f32>>>),
            "HashMap<i16,Slice<f32>>" => make_box_branch_for_type!(HashMap<i16,Slice<f32>>),
            "HashMap<i16,f64>" => make_box_branch_for_type!(HashMap<i16,f64>),
            "HashMap<i16,Vec<f64>>" => make_box_branch_for_type!(HashMap<i16,Vec<f64>>),
            "HashMap<i16,Vec<Vec<f64>>>" => make_box_branch_for_type!(HashMap<i16,Vec<Vec<f64>>>),
            "HashMap<i16,Slice<f64>>" => make_box_branch_for_type!(HashMap<i16,Slice<f64>>),
            "HashMap<i16,String>" => make_box_branch_for_type!(HashMap<i16,String>),
            "HashMap<i16,Vec<String>>" => make_box_branch_for_type!(HashMap<i16,Vec<String>>),
            "HashMap<i16,Vec<Vec<String>>>" => {
                make_box_branch_for_type!(HashMap<i16,Vec<Vec<String>>>)
            }
            "HashMap<i16,Slice<String>>" => make_box_branch_for_type!(HashMap<i16,Slice<String>>),
            "HashMap<i16,TString>" => make_box_branch_for_type!(HashMap<i16,TString>),
            "HashMap<i16,Vec<TString>>" => make_box_branch_for_type!(HashMap<i16,Vec<TString>>),
            "HashMap<i16,Vec<Vec<TString>>>" => {
                make_box_branch_for_type!(HashMap<i16,Vec<Vec<TString>>>)
            }
            "HashMap<i16,Slice<TString>>" => make_box_branch_for_type!(HashMap<i16,Slice<TString>>),
            "HashMap<i16,HashSet<u8>>" => make_box_branch_for_type!(HashMap<i16,HashSet<u8>>),
            "HashMap<i16,Vec<HashSet<u8>>>" => {
                make_box_branch_for_type!(HashMap<i16,Vec<HashSet<u8>>>)
            }
            "HashMap<i16,HashSet<i8>>" => make_box_branch_for_type!(HashMap<i16,HashSet<i8>>),
            "HashMap<i16,Vec<HashSet<i8>>>" => {
                make_box_branch_for_type!(HashMap<i16,Vec<HashSet<i8>>>)
            }
            "HashMap<i16,HashSet<i32>>" => make_box_branch_for_type!(HashMap<i16,HashSet<i32>>),
            "HashMap<i16,Vec<HashSet<i32>>>" => {
                make_box_branch_for_type!(HashMap<i16,Vec<HashSet<i32>>>)
            }
            "HashMap<i16,HashSet<u32>>" => make_box_branch_for_type!(HashMap<i16,HashSet<u32>>),
            "HashMap<i16,Vec<HashSet<u32>>>" => {
                make_box_branch_for_type!(HashMap<i16,Vec<HashSet<u32>>>)
            }
            "HashMap<i16,HashSet<i16>>" => make_box_branch_for_type!(HashMap<i16,HashSet<i16>>),
            "HashMap<i16,Vec<HashSet<i16>>>" => {
                make_box_branch_for_type!(HashMap<i16,Vec<HashSet<i16>>>)
            }
            "HashMap<i16,HashSet<u16>>" => make_box_branch_for_type!(HashMap<i16,HashSet<u16>>),
            "HashMap<i16,Vec<HashSet<u16>>>" => {
                make_box_branch_for_type!(HashMap<i16,Vec<HashSet<u16>>>)
            }
            "HashMap<i16,HashSet<bool>>" => make_box_branch_for_type!(HashMap<i16,HashSet<bool>>),
            "HashMap<i16,Vec<HashSet<bool>>>" => {
                make_box_branch_for_type!(HashMap<i16,Vec<HashSet<bool>>>)
            }
            "HashMap<i16,HashSet<String>>" => {
                make_box_branch_for_type!(HashMap<i16,HashSet<String>>)
            }
            "HashMap<i16,Vec<HashSet<String>>>" => {
                make_box_branch_for_type!(HashMap<i16,Vec<HashSet<String>>>)
            }
            "HashMap<i16,HashSet<TString>>" => {
                make_box_branch_for_type!(HashMap<i16,HashSet<TString>>)
            }
            "HashMap<i16,Vec<HashSet<TString>>>" => {
                make_box_branch_for_type!(HashMap<i16,Vec<HashSet<TString>>>)
            }
            "HashSet<u16>" => make_box_branch_for_type!(HashSet<u16>),
            "HashMap<u16,u8>" => make_box_branch_for_type!(HashMap<u16,u8>),
            "HashMap<u16,Vec<u8>>" => make_box_branch_for_type!(HashMap<u16,Vec<u8>>),
            "HashMap<u16,Vec<Vec<u8>>>" => make_box_branch_for_type!(HashMap<u16,Vec<Vec<u8>>>),
            "HashMap<u16,Slice<u8>>" => make_box_branch_for_type!(HashMap<u16,Slice<u8>>),
            "HashMap<u16,i8>" => make_box_branch_for_type!(HashMap<u16,i8>),
            "HashMap<u16,Vec<i8>>" => make_box_branch_for_type!(HashMap<u16,Vec<i8>>),
            "HashMap<u16,Vec<Vec<i8>>>" => make_box_branch_for_type!(HashMap<u16,Vec<Vec<i8>>>),
            "HashMap<u16,Slice<i8>>" => make_box_branch_for_type!(HashMap<u16,Slice<i8>>),
            "HashMap<u16,i32>" => make_box_branch_for_type!(HashMap<u16,i32>),
            "HashMap<u16,Vec<i32>>" => make_box_branch_for_type!(HashMap<u16,Vec<i32>>),
            "HashMap<u16,Vec<Vec<i32>>>" => make_box_branch_for_type!(HashMap<u16,Vec<Vec<i32>>>),
            "HashMap<u16,Slice<i32>>" => make_box_branch_for_type!(HashMap<u16,Slice<i32>>),
            "HashMap<u16,u32>" => make_box_branch_for_type!(HashMap<u16,u32>),
            "HashMap<u16,Vec<u32>>" => make_box_branch_for_type!(HashMap<u16,Vec<u32>>),
            "HashMap<u16,Vec<Vec<u32>>>" => make_box_branch_for_type!(HashMap<u16,Vec<Vec<u32>>>),
            "HashMap<u16,Slice<u32>>" => make_box_branch_for_type!(HashMap<u16,Slice<u32>>),
            "HashMap<u16,i16>" => make_box_branch_for_type!(HashMap<u16,i16>),
            "HashMap<u16,Vec<i16>>" => make_box_branch_for_type!(HashMap<u16,Vec<i16>>),
            "HashMap<u16,Vec<Vec<i16>>>" => make_box_branch_for_type!(HashMap<u16,Vec<Vec<i16>>>),
            "HashMap<u16,Slice<i16>>" => make_box_branch_for_type!(HashMap<u16,Slice<i16>>),
            "HashMap<u16,u16>" => make_box_branch_for_type!(HashMap<u16,u16>),
            "HashMap<u16,Vec<u16>>" => make_box_branch_for_type!(HashMap<u16,Vec<u16>>),
            "HashMap<u16,Vec<Vec<u16>>>" => make_box_branch_for_type!(HashMap<u16,Vec<Vec<u16>>>),
            "HashMap<u16,Slice<u16>>" => make_box_branch_for_type!(HashMap<u16,Slice<u16>>),
            "HashMap<u16,bool>" => make_box_branch_for_type!(HashMap<u16,bool>),
            "HashMap<u16,Vec<bool>>" => make_box_branch_for_type!(HashMap<u16,Vec<bool>>),
            "HashMap<u16,Vec<Vec<bool>>>" => make_box_branch_for_type!(HashMap<u16,Vec<Vec<bool>>>),
            "HashMap<u16,Slice<bool>>" => make_box_branch_for_type!(HashMap<u16,Slice<bool>>),
            "HashMap<u16,f32>" => make_box_branch_for_type!(HashMap<u16,f32>),
            "HashMap<u16,Vec<f32>>" => make_box_branch_for_type!(HashMap<u16,Vec<f32>>),
            "HashMap<u16,Vec<Vec<f32>>>" => make_box_branch_for_type!(HashMap<u16,Vec<Vec<f32>>>),
            "HashMap<u16,Slice<f32>>" => make_box_branch_for_type!(HashMap<u16,Slice<f32>>),
            "HashMap<u16,f64>" => make_box_branch_for_type!(HashMap<u16,f64>),
            "HashMap<u16,Vec<f64>>" => make_box_branch_for_type!(HashMap<u16,Vec<f64>>),
            "HashMap<u16,Vec<Vec<f64>>>" => make_box_branch_for_type!(HashMap<u16,Vec<Vec<f64>>>),
            "HashMap<u16,Slice<f64>>" => make_box_branch_for_type!(HashMap<u16,Slice<f64>>),
            "HashMap<u16,String>" => make_box_branch_for_type!(HashMap<u16,String>),
            "HashMap<u16,Vec<String>>" => make_box_branch_for_type!(HashMap<u16,Vec<String>>),
            "HashMap<u16,Vec<Vec<String>>>" => {
                make_box_branch_for_type!(HashMap<u16,Vec<Vec<String>>>)
            }
            "HashMap<u16,Slice<String>>" => make_box_branch_for_type!(HashMap<u16,Slice<String>>),
            "HashMap<u16,TString>" => make_box_branch_for_type!(HashMap<u16,TString>),
            "HashMap<u16,Vec<TString>>" => make_box_branch_for_type!(HashMap<u16,Vec<TString>>),
            "HashMap<u16,Vec<Vec<TString>>>" => {
                make_box_branch_for_type!(HashMap<u16,Vec<Vec<TString>>>)
            }
            "HashMap<u16,Slice<TString>>" => make_box_branch_for_type!(HashMap<u16,Slice<TString>>),
            "HashMap<u16,HashSet<u8>>" => make_box_branch_for_type!(HashMap<u16,HashSet<u8>>),
            "HashMap<u16,Vec<HashSet<u8>>>" => {
                make_box_branch_for_type!(HashMap<u16,Vec<HashSet<u8>>>)
            }
            "HashMap<u16,HashSet<i8>>" => make_box_branch_for_type!(HashMap<u16,HashSet<i8>>),
            "HashMap<u16,Vec<HashSet<i8>>>" => {
                make_box_branch_for_type!(HashMap<u16,Vec<HashSet<i8>>>)
            }
            "HashMap<u16,HashSet<i32>>" => make_box_branch_for_type!(HashMap<u16,HashSet<i32>>),
            "HashMap<u16,Vec<HashSet<i32>>>" => {
                make_box_branch_for_type!(HashMap<u16,Vec<HashSet<i32>>>)
            }
            "HashMap<u16,HashSet<u32>>" => make_box_branch_for_type!(HashMap<u16,HashSet<u32>>),
            "HashMap<u16,Vec<HashSet<u32>>>" => {
                make_box_branch_for_type!(HashMap<u16,Vec<HashSet<u32>>>)
            }
            "HashMap<u16,HashSet<i16>>" => make_box_branch_for_type!(HashMap<u16,HashSet<i16>>),
            "HashMap<u16,Vec<HashSet<i16>>>" => {
                make_box_branch_for_type!(HashMap<u16,Vec<HashSet<i16>>>)
            }
            "HashMap<u16,HashSet<u16>>" => make_box_branch_for_type!(HashMap<u16,HashSet<u16>>),
            "HashMap<u16,Vec<HashSet<u16>>>" => {
                make_box_branch_for_type!(HashMap<u16,Vec<HashSet<u16>>>)
            }
            "HashMap<u16,HashSet<bool>>" => make_box_branch_for_type!(HashMap<u16,HashSet<bool>>),
            "HashMap<u16,Vec<HashSet<bool>>>" => {
                make_box_branch_for_type!(HashMap<u16,Vec<HashSet<bool>>>)
            }
            "HashMap<u16,HashSet<String>>" => {
                make_box_branch_for_type!(HashMap<u16,HashSet<String>>)
            }
            "HashMap<u16,Vec<HashSet<String>>>" => {
                make_box_branch_for_type!(HashMap<u16,Vec<HashSet<String>>>)
            }
            "HashMap<u16,HashSet<TString>>" => {
                make_box_branch_for_type!(HashMap<u16,HashSet<TString>>)
            }
            "HashMap<u16,Vec<HashSet<TString>>>" => {
                make_box_branch_for_type!(HashMap<u16,Vec<HashSet<TString>>>)
            }
            "HashSet<bool>" => make_box_branch_for_type!(HashSet<bool>),
            "HashMap<bool,u8>" => make_box_branch_for_type!(HashMap<bool,u8>),
            "HashMap<bool,Vec<u8>>" => make_box_branch_for_type!(HashMap<bool,Vec<u8>>),
            "HashMap<bool,Vec<Vec<u8>>>" => make_box_branch_for_type!(HashMap<bool,Vec<Vec<u8>>>),
            "HashMap<bool,Slice<u8>>" => make_box_branch_for_type!(HashMap<bool,Slice<u8>>),
            "HashMap<bool,i8>" => make_box_branch_for_type!(HashMap<bool,i8>),
            "HashMap<bool,Vec<i8>>" => make_box_branch_for_type!(HashMap<bool,Vec<i8>>),
            "HashMap<bool,Vec<Vec<i8>>>" => make_box_branch_for_type!(HashMap<bool,Vec<Vec<i8>>>),
            "HashMap<bool,Slice<i8>>" => make_box_branch_for_type!(HashMap<bool,Slice<i8>>),
            "HashMap<bool,i32>" => make_box_branch_for_type!(HashMap<bool,i32>),
            "HashMap<bool,Vec<i32>>" => make_box_branch_for_type!(HashMap<bool,Vec<i32>>),
            "HashMap<bool,Vec<Vec<i32>>>" => make_box_branch_for_type!(HashMap<bool,Vec<Vec<i32>>>),
            "HashMap<bool,Slice<i32>>" => make_box_branch_for_type!(HashMap<bool,Slice<i32>>),
            "HashMap<bool,u32>" => make_box_branch_for_type!(HashMap<bool,u32>),
            "HashMap<bool,Vec<u32>>" => make_box_branch_for_type!(HashMap<bool,Vec<u32>>),
            "HashMap<bool,Vec<Vec<u32>>>" => make_box_branch_for_type!(HashMap<bool,Vec<Vec<u32>>>),
            "HashMap<bool,Slice<u32>>" => make_box_branch_for_type!(HashMap<bool,Slice<u32>>),
            "HashMap<bool,i16>" => make_box_branch_for_type!(HashMap<bool,i16>),
            "HashMap<bool,Vec<i16>>" => make_box_branch_for_type!(HashMap<bool,Vec<i16>>),
            "HashMap<bool,Vec<Vec<i16>>>" => make_box_branch_for_type!(HashMap<bool,Vec<Vec<i16>>>),
            "HashMap<bool,Slice<i16>>" => make_box_branch_for_type!(HashMap<bool,Slice<i16>>),
            "HashMap<bool,u16>" => make_box_branch_for_type!(HashMap<bool,u16>),
            "HashMap<bool,Vec<u16>>" => make_box_branch_for_type!(HashMap<bool,Vec<u16>>),
            "HashMap<bool,Vec<Vec<u16>>>" => make_box_branch_for_type!(HashMap<bool,Vec<Vec<u16>>>),
            "HashMap<bool,Slice<u16>>" => make_box_branch_for_type!(HashMap<bool,Slice<u16>>),
            "HashMap<bool,bool>" => make_box_branch_for_type!(HashMap<bool,bool>),
            "HashMap<bool,Vec<bool>>" => make_box_branch_for_type!(HashMap<bool,Vec<bool>>),
            "HashMap<bool,Vec<Vec<bool>>>" => {
                make_box_branch_for_type!(HashMap<bool,Vec<Vec<bool>>>)
            }
            "HashMap<bool,Slice<bool>>" => make_box_branch_for_type!(HashMap<bool,Slice<bool>>),
            "HashMap<bool,f32>" => make_box_branch_for_type!(HashMap<bool,f32>),
            "HashMap<bool,Vec<f32>>" => make_box_branch_for_type!(HashMap<bool,Vec<f32>>),
            "HashMap<bool,Vec<Vec<f32>>>" => make_box_branch_for_type!(HashMap<bool,Vec<Vec<f32>>>),
            "HashMap<bool,Slice<f32>>" => make_box_branch_for_type!(HashMap<bool,Slice<f32>>),
            "HashMap<bool,f64>" => make_box_branch_for_type!(HashMap<bool,f64>),
            "HashMap<bool,Vec<f64>>" => make_box_branch_for_type!(HashMap<bool,Vec<f64>>),
            "HashMap<bool,Vec<Vec<f64>>>" => make_box_branch_for_type!(HashMap<bool,Vec<Vec<f64>>>),
            "HashMap<bool,Slice<f64>>" => make_box_branch_for_type!(HashMap<bool,Slice<f64>>),
            "HashMap<bool,String>" => make_box_branch_for_type!(HashMap<bool,String>),
            "HashMap<bool,Vec<String>>" => make_box_branch_for_type!(HashMap<bool,Vec<String>>),
            "HashMap<bool,Vec<Vec<String>>>" => {
                make_box_branch_for_type!(HashMap<bool,Vec<Vec<String>>>)
            }
            "HashMap<bool,Slice<String>>" => make_box_branch_for_type!(HashMap<bool,Slice<String>>),
            "HashMap<bool,TString>" => make_box_branch_for_type!(HashMap<bool,TString>),
            "HashMap<bool,Vec<TString>>" => make_box_branch_for_type!(HashMap<bool,Vec<TString>>),
            "HashMap<bool,Vec<Vec<TString>>>" => {
                make_box_branch_for_type!(HashMap<bool,Vec<Vec<TString>>>)
            }
            "HashMap<bool,Slice<TString>>" => {
                make_box_branch_for_type!(HashMap<bool,Slice<TString>>)
            }
            "HashMap<bool,HashSet<u8>>" => make_box_branch_for_type!(HashMap<bool,HashSet<u8>>),
            "HashMap<bool,Vec<HashSet<u8>>>" => {
                make_box_branch_for_type!(HashMap<bool,Vec<HashSet<u8>>>)
            }
            "HashMap<bool,HashSet<i8>>" => make_box_branch_for_type!(HashMap<bool,HashSet<i8>>),
            "HashMap<bool,Vec<HashSet<i8>>>" => {
                make_box_branch_for_type!(HashMap<bool,Vec<HashSet<i8>>>)
            }
            "HashMap<bool,HashSet<i32>>" => make_box_branch_for_type!(HashMap<bool,HashSet<i32>>),
            "HashMap<bool,Vec<HashSet<i32>>>" => {
                make_box_branch_for_type!(HashMap<bool,Vec<HashSet<i32>>>)
            }
            "HashMap<bool,HashSet<u32>>" => make_box_branch_for_type!(HashMap<bool,HashSet<u32>>),
            "HashMap<bool,Vec<HashSet<u32>>>" => {
                make_box_branch_for_type!(HashMap<bool,Vec<HashSet<u32>>>)
            }
            "HashMap<bool,HashSet<i16>>" => make_box_branch_for_type!(HashMap<bool,HashSet<i16>>),
            "HashMap<bool,Vec<HashSet<i16>>>" => {
                make_box_branch_for_type!(HashMap<bool,Vec<HashSet<i16>>>)
            }
            "HashMap<bool,HashSet<u16>>" => make_box_branch_for_type!(HashMap<bool,HashSet<u16>>),
            "HashMap<bool,Vec<HashSet<u16>>>" => {
                make_box_branch_for_type!(HashMap<bool,Vec<HashSet<u16>>>)
            }
            "HashMap<bool,HashSet<bool>>" => make_box_branch_for_type!(HashMap<bool,HashSet<bool>>),
            "HashMap<bool,Vec<HashSet<bool>>>" => {
                make_box_branch_for_type!(HashMap<bool,Vec<HashSet<bool>>>)
            }
            "HashMap<bool,HashSet<String>>" => {
                make_box_branch_for_type!(HashMap<bool,HashSet<String>>)
            }
            "HashMap<bool,Vec<HashSet<String>>>" => {
                make_box_branch_for_type!(HashMap<bool,Vec<HashSet<String>>>)
            }
            "HashMap<bool,HashSet<TString>>" => {
                make_box_branch_for_type!(HashMap<bool,HashSet<TString>>)
            }
            "HashMap<bool,Vec<HashSet<TString>>>" => {
                make_box_branch_for_type!(HashMap<bool,Vec<HashSet<TString>>>)
            }
            "HashSet<String>" => make_box_branch_for_type!(HashSet<String>),
            "HashMap<String,u8>" => make_box_branch_for_type!(HashMap<String,u8>),
            "HashMap<String,Vec<u8>>" => make_box_branch_for_type!(HashMap<String,Vec<u8>>),
            "HashMap<String,Vec<Vec<u8>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<Vec<u8>>>)
            }
            "HashMap<String,Slice<u8>>" => make_box_branch_for_type!(HashMap<String,Slice<u8>>),
            "HashMap<String,i8>" => make_box_branch_for_type!(HashMap<String,i8>),
            "HashMap<String,Vec<i8>>" => make_box_branch_for_type!(HashMap<String,Vec<i8>>),
            "HashMap<String,Vec<Vec<i8>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<Vec<i8>>>)
            }
            "HashMap<String,Slice<i8>>" => make_box_branch_for_type!(HashMap<String,Slice<i8>>),
            "HashMap<String,i32>" => make_box_branch_for_type!(HashMap<String,i32>),
            "HashMap<String,Vec<i32>>" => make_box_branch_for_type!(HashMap<String,Vec<i32>>),
            "HashMap<String,Vec<Vec<i32>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<Vec<i32>>>)
            }
            "HashMap<String,Slice<i32>>" => make_box_branch_for_type!(HashMap<String,Slice<i32>>),
            "HashMap<String,u32>" => make_box_branch_for_type!(HashMap<String,u32>),
            "HashMap<String,Vec<u32>>" => make_box_branch_for_type!(HashMap<String,Vec<u32>>),
            "HashMap<String,Vec<Vec<u32>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<Vec<u32>>>)
            }
            "HashMap<String,Slice<u32>>" => make_box_branch_for_type!(HashMap<String,Slice<u32>>),
            "HashMap<String,i16>" => make_box_branch_for_type!(HashMap<String,i16>),
            "HashMap<String,Vec<i16>>" => make_box_branch_for_type!(HashMap<String,Vec<i16>>),
            "HashMap<String,Vec<Vec<i16>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<Vec<i16>>>)
            }
            "HashMap<String,Slice<i16>>" => make_box_branch_for_type!(HashMap<String,Slice<i16>>),
            "HashMap<String,u16>" => make_box_branch_for_type!(HashMap<String,u16>),
            "HashMap<String,Vec<u16>>" => make_box_branch_for_type!(HashMap<String,Vec<u16>>),
            "HashMap<String,Vec<Vec<u16>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<Vec<u16>>>)
            }
            "HashMap<String,Slice<u16>>" => make_box_branch_for_type!(HashMap<String,Slice<u16>>),
            "HashMap<String,bool>" => make_box_branch_for_type!(HashMap<String,bool>),
            "HashMap<String,Vec<bool>>" => make_box_branch_for_type!(HashMap<String,Vec<bool>>),
            "HashMap<String,Vec<Vec<bool>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<Vec<bool>>>)
            }
            "HashMap<String,Slice<bool>>" => make_box_branch_for_type!(HashMap<String,Slice<bool>>),
            "HashMap<String,f32>" => make_box_branch_for_type!(HashMap<String,f32>),
            "HashMap<String,Vec<f32>>" => make_box_branch_for_type!(HashMap<String,Vec<f32>>),
            "HashMap<String,Vec<Vec<f32>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<Vec<f32>>>)
            }
            "HashMap<String,Slice<f32>>" => make_box_branch_for_type!(HashMap<String,Slice<f32>>),
            "HashMap<String,f64>" => make_box_branch_for_type!(HashMap<String,f64>),
            "HashMap<String,Vec<f64>>" => make_box_branch_for_type!(HashMap<String,Vec<f64>>),
            "HashMap<String,Vec<Vec<f64>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<Vec<f64>>>)
            }
            "HashMap<String,Slice<f64>>" => make_box_branch_for_type!(HashMap<String,Slice<f64>>),
            "HashMap<String,String>" => make_box_branch_for_type!(HashMap<String,String>),
            "HashMap<String,Vec<String>>" => make_box_branch_for_type!(HashMap<String,Vec<String>>),
            "HashMap<String,Vec<Vec<String>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<Vec<String>>>)
            }
            "HashMap<String,Slice<String>>" => {
                make_box_branch_for_type!(HashMap<String,Slice<String>>)
            }
            "HashMap<String,TString>" => make_box_branch_for_type!(HashMap<String,TString>),
            "HashMap<String,Vec<TString>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<TString>>)
            }
            "HashMap<String,Vec<Vec<TString>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<Vec<TString>>>)
            }
            "HashMap<String,Slice<TString>>" => {
                make_box_branch_for_type!(HashMap<String,Slice<TString>>)
            }
            "HashMap<String,HashSet<u8>>" => make_box_branch_for_type!(HashMap<String,HashSet<u8>>),
            "HashMap<String,Vec<HashSet<u8>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<HashSet<u8>>>)
            }
            "HashMap<String,HashSet<i8>>" => make_box_branch_for_type!(HashMap<String,HashSet<i8>>),
            "HashMap<String,Vec<HashSet<i8>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<HashSet<i8>>>)
            }
            "HashMap<String,HashSet<i32>>" => {
                make_box_branch_for_type!(HashMap<String,HashSet<i32>>)
            }
            "HashMap<String,Vec<HashSet<i32>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<HashSet<i32>>>)
            }
            "HashMap<String,HashSet<u32>>" => {
                make_box_branch_for_type!(HashMap<String,HashSet<u32>>)
            }
            "HashMap<String,Vec<HashSet<u32>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<HashSet<u32>>>)
            }
            "HashMap<String,HashSet<i16>>" => {
                make_box_branch_for_type!(HashMap<String,HashSet<i16>>)
            }
            "HashMap<String,Vec<HashSet<i16>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<HashSet<i16>>>)
            }
            "HashMap<String,HashSet<u16>>" => {
                make_box_branch_for_type!(HashMap<String,HashSet<u16>>)
            }
            "HashMap<String,Vec<HashSet<u16>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<HashSet<u16>>>)
            }
            "HashMap<String,HashSet<bool>>" => {
                make_box_branch_for_type!(HashMap<String,HashSet<bool>>)
            }
            "HashMap<String,Vec<HashSet<bool>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<HashSet<bool>>>)
            }
            "HashMap<String,HashSet<String>>" => {
                make_box_branch_for_type!(HashMap<String,HashSet<String>>)
            }
            "HashMap<String,Vec<HashSet<String>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<HashSet<String>>>)
            }
            "HashMap<String,HashSet<TString>>" => {
                make_box_branch_for_type!(HashMap<String,HashSet<TString>>)
            }
            "HashMap<String,Vec<HashSet<TString>>>" => {
                make_box_branch_for_type!(HashMap<String,Vec<HashSet<TString>>>)
            }
            "HashSet<TString>" => make_box_branch_for_type!(HashSet<TString>),
            "HashMap<TString,u8>" => make_box_branch_for_type!(HashMap<TString,u8>),
            "HashMap<TString,Vec<u8>>" => make_box_branch_for_type!(HashMap<TString,Vec<u8>>),
            "HashMap<TString,Vec<Vec<u8>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<Vec<u8>>>)
            }
            "HashMap<TString,Slice<u8>>" => make_box_branch_for_type!(HashMap<TString,Slice<u8>>),
            "HashMap<TString,i8>" => make_box_branch_for_type!(HashMap<TString,i8>),
            "HashMap<TString,Vec<i8>>" => make_box_branch_for_type!(HashMap<TString,Vec<i8>>),
            "HashMap<TString,Vec<Vec<i8>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<Vec<i8>>>)
            }
            "HashMap<TString,Slice<i8>>" => make_box_branch_for_type!(HashMap<TString,Slice<i8>>),
            "HashMap<TString,i32>" => make_box_branch_for_type!(HashMap<TString,i32>),
            "HashMap<TString,Vec<i32>>" => make_box_branch_for_type!(HashMap<TString,Vec<i32>>),
            "HashMap<TString,Vec<Vec<i32>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<Vec<i32>>>)
            }
            "HashMap<TString,Slice<i32>>" => make_box_branch_for_type!(HashMap<TString,Slice<i32>>),
            "HashMap<TString,u32>" => make_box_branch_for_type!(HashMap<TString,u32>),
            "HashMap<TString,Vec<u32>>" => make_box_branch_for_type!(HashMap<TString,Vec<u32>>),
            "HashMap<TString,Vec<Vec<u32>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<Vec<u32>>>)
            }
            "HashMap<TString,Slice<u32>>" => make_box_branch_for_type!(HashMap<TString,Slice<u32>>),
            "HashMap<TString,i16>" => make_box_branch_for_type!(HashMap<TString,i16>),
            "HashMap<TString,Vec<i16>>" => make_box_branch_for_type!(HashMap<TString,Vec<i16>>),
            "HashMap<TString,Vec<Vec<i16>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<Vec<i16>>>)
            }
            "HashMap<TString,Slice<i16>>" => make_box_branch_for_type!(HashMap<TString,Slice<i16>>),
            "HashMap<TString,u16>" => make_box_branch_for_type!(HashMap<TString,u16>),
            "HashMap<TString,Vec<u16>>" => make_box_branch_for_type!(HashMap<TString,Vec<u16>>),
            "HashMap<TString,Vec<Vec<u16>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<Vec<u16>>>)
            }
            "HashMap<TString,Slice<u16>>" => make_box_branch_for_type!(HashMap<TString,Slice<u16>>),
            "HashMap<TString,bool>" => make_box_branch_for_type!(HashMap<TString,bool>),
            "HashMap<TString,Vec<bool>>" => make_box_branch_for_type!(HashMap<TString,Vec<bool>>),
            "HashMap<TString,Vec<Vec<bool>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<Vec<bool>>>)
            }
            "HashMap<TString,Slice<bool>>" => {
                make_box_branch_for_type!(HashMap<TString,Slice<bool>>)
            }
            "HashMap<TString,f32>" => make_box_branch_for_type!(HashMap<TString,f32>),
            "HashMap<TString,Vec<f32>>" => make_box_branch_for_type!(HashMap<TString,Vec<f32>>),
            "HashMap<TString,Vec<Vec<f32>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<Vec<f32>>>)
            }
            "HashMap<TString,Slice<f32>>" => make_box_branch_for_type!(HashMap<TString,Slice<f32>>),
            "HashMap<TString,f64>" => make_box_branch_for_type!(HashMap<TString,f64>),
            "HashMap<TString,Vec<f64>>" => make_box_branch_for_type!(HashMap<TString,Vec<f64>>),
            "HashMap<TString,Vec<Vec<f64>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<Vec<f64>>>)
            }
            "HashMap<TString,Slice<f64>>" => make_box_branch_for_type!(HashMap<TString,Slice<f64>>),
            "HashMap<TString,String>" => make_box_branch_for_type!(HashMap<TString,String>),
            "HashMap<TString,Vec<String>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<String>>)
            }
            "HashMap<TString,Vec<Vec<String>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<Vec<String>>>)
            }
            "HashMap<TString,Slice<String>>" => {
                make_box_branch_for_type!(HashMap<TString,Slice<String>>)
            }
            "HashMap<TString,TString>" => make_box_branch_for_type!(HashMap<TString,TString>),
            "HashMap<TString,Vec<TString>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<TString>>)
            }
            "HashMap<TString,Vec<Vec<TString>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<Vec<TString>>>)
            }
            "HashMap<TString,Slice<TString>>" => {
                make_box_branch_for_type!(HashMap<TString,Slice<TString>>)
            }
            "HashMap<TString,HashSet<u8>>" => {
                make_box_branch_for_type!(HashMap<TString,HashSet<u8>>)
            }
            "HashMap<TString,Vec<HashSet<u8>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<HashSet<u8>>>)
            }
            "HashMap<TString,HashSet<i8>>" => {
                make_box_branch_for_type!(HashMap<TString,HashSet<i8>>)
            }
            "HashMap<TString,Vec<HashSet<i8>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<HashSet<i8>>>)
            }
            "HashMap<TString,HashSet<i32>>" => {
                make_box_branch_for_type!(HashMap<TString,HashSet<i32>>)
            }
            "HashMap<TString,Vec<HashSet<i32>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<HashSet<i32>>>)
            }
            "HashMap<TString,HashSet<u32>>" => {
                make_box_branch_for_type!(HashMap<TString,HashSet<u32>>)
            }
            "HashMap<TString,Vec<HashSet<u32>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<HashSet<u32>>>)
            }
            "HashMap<TString,HashSet<i16>>" => {
                make_box_branch_for_type!(HashMap<TString,HashSet<i16>>)
            }
            "HashMap<TString,Vec<HashSet<i16>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<HashSet<i16>>>)
            }
            "HashMap<TString,HashSet<u16>>" => {
                make_box_branch_for_type!(HashMap<TString,HashSet<u16>>)
            }
            "HashMap<TString,Vec<HashSet<u16>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<HashSet<u16>>>)
            }
            "HashMap<TString,HashSet<bool>>" => {
                make_box_branch_for_type!(HashMap<TString,HashSet<bool>>)
            }
            "HashMap<TString,Vec<HashSet<bool>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<HashSet<bool>>>)
            }
            "HashMap<TString,HashSet<String>>" => {
                make_box_branch_for_type!(HashMap<TString,HashSet<String>>)
            }
            "HashMap<TString,Vec<HashSet<String>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<HashSet<String>>>)
            }
            "HashMap<TString,HashSet<TString>>" => {
                make_box_branch_for_type!(HashMap<TString,HashSet<TString>>)
            }
            "HashMap<TString,Vec<HashSet<TString>>>" => {
                make_box_branch_for_type!(HashMap<TString,Vec<HashSet<TString>>>)
            }
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
                        "TString" => make_box_branch_for_sized_slice!(String, n),
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
                println!("[{}][{}]: {}", i, zbv.branch.name(), it.dump());
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
        println!(">>> tree: {:?}", trees);
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
