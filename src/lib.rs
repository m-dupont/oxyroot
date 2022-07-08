/*!
This crate aims to provide a way to open data saved in [ROOT](https://root.cern.ch/) file format
and particularly `Tree` and `Branch` inside `Tree`.
This crate is in fact a port of [groot](https://pkg.go.dev/go-hep.org/x/hep/groot) written
in Go and  [uproot](https://github.com/scikit-hep/uproot5) written in Python.
# Example: Iter over a branch tree containing `i32` values

```rust
use oxyroot::RootFile;
let s = "examples/from_uproot/data/HZZ.root";
let tree = RootFile::open(s).unwrap().get_tree("events").unwrap().unwrap();
let NJet = tree.branch("NJet").unwrap().get_basket_into::<i32>();
NJet.for_each(|v| println!("v = {v}"));
```

# Which types can be read from a branch ?

oxyroot can iterate over  branch which contains :
- primitive types like i32, f64, bool...
- String (from TString, char* or std::string)
- Vec (from std::vec or array)

Examples can be found in `tests`

# Structures
Structure serialized in `Branch` can

 */

#![deny(unused_must_use)]
// #![deny(unused_imports)]
extern crate core;

mod gen_factory;
mod rbase;
pub mod rbytes;
pub mod rcolors;
mod rcompress;
mod rcont;
mod rdict;
mod riofs;
pub mod rmeta;
pub mod root;
pub mod rtree;
pub mod rtypes;
mod rusty;
mod rvers;
mod utils;

pub use riofs::file;

pub use rbytes::rbuffer::RBuffer;
pub use riofs::file::RootFile;
pub use rtree::branch::TBranch;
pub use rtree::tree::Tree;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
