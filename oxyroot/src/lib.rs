/*!
This crate aims to provide a way to open and write  data in [ROOT](https://root.cern.ch/) file format
and particularly `Tree` and `Branch` inside `Tree`.
This crate is in fact a port of [groot](https://pkg.go.dev/go-hep.org/x/hep/groot) written
in Go and  [uproot](https://github.com/scikit-hep/uproot5) written in Python.

To read from a branch and write to a branch, the API used is iterator based :
- [`as_iter`](enum.Branch.html#method.as_iter) to read from a branch. The type to read is provided as a type parameter.
This type has to implement `Unmarshaler` trait.
- [`new_branch`](type.WriterTree.html#method.new_branch) from [`WriterTree`](type.WriterTree.html) to write to a branch. The method
[`write`](type.WriterTree.html#method.write) from [`WriterTree`](crate::WriterTree) is used
to write the data to the file by exhausting provided iterators. The type to write has to implement
`Marshaler` trait. The type is deduced from the iterator.

# Example: Show branches from a tree

```rust
use oxyroot::RootFile;
let s = "examples/from_uproot/data/simple.root";
let tree = RootFile::open(s).expect("Can not open file").get_tree("tree").unwrap();
tree.show();
```

will display

```ignore
name                           | typename                       | interpretation
-------------------------------+-------------------------------+-------------------------------
one                            | int32_t                        | i32
two                            | float                          | f32
three                          | char*                          | String
```

# Example: Iter over a branch tree containing `i32` values

```rust
use oxyroot::RootFile;
let s = "examples/from_uproot/data/simple.root";
let tree = RootFile::open(s).expect("Can not open file").get_tree("tree").unwrap();
let one = tree.branch("one").unwrap().as_iter::<i32>().expect("wrong type");
one.for_each(|v| println!("v = {v}"));
```

# Example: Write i32 values in a branch

```rust
use oxyroot::{RootFile, WriterTree};
let s = "/tmp/simple.root";
let mut file = RootFile::create(s).expect("Can not create file");
let mut tree = WriterTree::new("mytree");
let it = (0..15);
tree.new_branch("it", it);
tree.write(&mut file).expect("Can not write tree");
file.close().expect("Can not close file");
```

# Example: Iter over a branch tree containing `Vec<i32>`  (aka `std::vector<int32_t>`) values

```rust
use oxyroot::RootFile;
let s = "tests/stl_containers/stl_containers.root";
let tree = RootFile::open(s).expect("Can not open file").get_tree("tree").unwrap();
let vector_int32 = tree.branch("vector_int32")
                   .unwrap().as_iter::<Vec<i32>>().expect("wrong type")
                   .collect::<Vec<_>>();
assert_eq!(
    vector_int32,
    [
        vec![1],
        vec![1, 2],
        vec![1, 2, 3],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4, 5]
    ]
);
```

# Which types can be read from a branch?

## Primitives and C++ STL standards

oxyroot can iterate over  branch which contains :
- primitive types like i32, f64, bool...
- String (from TString, char* or std::string)
- Vec (from std::vector or array)
- HashMap


| C++ | Rust |
|---------|---------|
| std::string     | [String](String)     |
| std::vector     | [Vec](Vec)     |
| std::map     | [HashMap](std::collections::HashMap)    |
| std::set     | [HashSet](std::collections::HashSet)      |
| T*     | [`Slice<T>`](Slice)    |
| T\[N\]     | [array]     |
| TString     | [String]     |


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
let tree = RootFile::open(s).expect("Can not open file").get_tree("T").unwrap();

// branch v_i contains v_i which will be zipped.
let mut b = tree.branch("v_i").unwrap().get_basket(parse);

for i in -10..10 {
    let sd = b.next().unwrap();
}
```

# Which types can be written to a branch?

## Primitives and C++ STL standards

oxyroot can iterate over  branch which contains :
- primitive types like i32, f64, bool...
- String (will appear as a char*)
- Vec (to std::vector)
- HashMap (not yet implemented)


 */

#![allow(dead_code)]
#![deny(unused_must_use)]
// #![deny(unused_imports)]

// Show which crate feature enables conditionally compiled APIs in documentation.
#![cfg_attr(doc_cfg, feature(doc_cfg))]

extern crate core;

mod error;
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
pub use rtree::tree::ReaderTree;

pub use rtree::tree::ReadFromTree;
pub use rtree::tree::WriteToTree;

pub use rtree::tree::WriterTree;
// pub use rtree::tree::Tree;

pub use rbytes::rbuffer::RBuffer;
pub use rbytes::Unmarshaler;
pub use rbytes::UnmarshalerInto;

pub use rbytes::Marshaler;

pub use rusty::SizedSlice;
pub use rusty::Slice;

pub use root::traits::Named;
pub use root::traits::Object;

pub use error::Result;

/// Derive macro available if oxyroot is built with `features = ["derive"]`.
#[cfg(feature = "derive")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use oxyroot_derive::{ReadFromTree, WriteToTree};

pub use rtree::branch_name::BranchName;
pub use rtree::tree::StateCallBack;
