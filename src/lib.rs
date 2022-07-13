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
let NJet = tree.branch("NJet").unwrap().as_iter::<i32>();
NJet.for_each(|v| println!("v = {v}"));
```

# Which types can be read from a branch ?

## Primitives and C++ STL standards

oxyroot can iterate over  branch which contains :
- primitive types like i32, f64, bool...
- String (from TString, char* or std::string)
- Vec (from std::vector or array)
- HashMap


| C++ | Rust |
|---------|---------|
| std::string     | String     |
| std::vector     | [Vec](Vec)     |
| std::map     | [HashMap](std::collections::HashMap)    |
| std::set     | [HashSet](std::collections::HashSet)      |
| T*     | [`Slice<T>`](Slice)    |
| T\[N\]     | [array]     |
| TString     | String     |


Examples can be found in tests.

## Structures
Structure serialized in `Branch` can be also read but the parsing code has to be written :

```C++
struct sd_t {
         Int_t a;
         Int_t b;
};

sd_t sd;
tree->Branch("v_i",&sd, 3200000, 0);
```

The `sd` struct can be read with a code like this :


```no_run
use oxyroot::RBuffer;
use oxyroot::RootFile;
struct Sd {
    a: i32,
    b: i32,
};

let parse = |r: &mut RBuffer| Sd {
    a: r.read_i32().unwrap(),
    b: r.read_i32().unwrap(),
};


let s = "tests_data/doc/struct_sd.root";
let tree = RootFile::open(s).unwrap().get_tree("T").unwrap().unwrap();

// branch v_i contains v_i which will be zipped.
let mut b = tree.branch("v_i").unwrap().get_basket(parse);

for i in -10..10 {
    let sd = b.next().unwrap();
}
```




 */

#![deny(unused_must_use)]
// #![deny(unused_imports)]
extern crate core;

mod gen_factory;
mod rbase;
mod rbytes;
mod rcolors;
mod rcompress;
mod rcont;
mod rdict;
mod riofs;
mod rmeta;
mod root;
mod rtree;
mod rtypes;
mod rusty;
mod rvers;
mod utils;

pub use riofs::file::RootFile;
pub use rtree::branch::Branch;
pub use rtree::tree::Tree;

pub use rbytes::rbuffer::RBuffer;
pub use rbytes::Unmarshaler;
pub use rbytes::UnmarshalerInto;

pub use rusty::Slice;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
