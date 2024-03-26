# oxyroot

[![Crates.io](https://img.shields.io/crates/v/oxyroot.svg)](https://crates.io/crates/oxyroot)
[![Documentation](https://docs.rs/oxyroot/badge.svg)](https://docs.rs/oxyroot)

Another attempt to make library reading and writing `.root` binary files which are commonly used in particle physics

## Inspiration

To make this library :

- heavy inspiration taken from [groot](https://github.com/go-hep/hep/tree/main/groot) for reading root file, even the
  code
  organisation
- inspiration taken from [uproot](https://github.com/scikit-hep/uproot5) to provide branch interface (for reading basket
  buffer)

## Limitations

For now:

- only writing of primitive types is supported (i.e. `i32`, `f64`, `bool`, `String`, Fixed length arrays). Containers
  will be added.
- can only write uncompressed file

## See also

Another rust implementation of a root reader is [`root-io`](https://crates.io/crates/root-io).

## Getting started

### Example: Iter over a branch tree containing `i32` values

```rust
use oxyroot::RootFile;
let s = "examples/from_uproot/data/HZZ.root";
let tree = RootFile::open(s).unwrap().get_tree("events").unwrap();
let NJet = tree.branch("NJet").unwrap().as_iter::<i32>();
NJet.for_each( | v| trace!("v = {v}"));
```

### # Example: Write i32 values in a branch

```rust
use oxyroot::{RootFile, WriterTree};
let s = "/tmp/simple.root";
let mut file = RootFile::create(s).expect("Can not create file");
let mut tree = WriterTree::new("mytree");
let it = (0..15);
tree.new_branch("it", it);
tree.write( & mut file).expect("Can not write tree");
file.close().expect("Can not close file");
```

### Example: Iter over a branch tree containing `Vec<i32>`  (aka `std::vector<int32_t>`) values

```rust
use oxyroot::RootFile;
let s = "tests/stl_containers/stl_containers.root";
let tree = RootFile::open(s).unwrap().get_tree("tree").unwrap();
let vector_int32 = tree.branch("vector_int32")
.unwrap().as_iter::<Vec<i32> > ()
.collect::<Vec<_ > > ();
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

### Feature

`oxyroot` use [`flate2`](https://crates.io/crates/flate2) to decompress zlib compressed data.
The default backend is `miniz_oxide`, pure Rust crate.  
If you want maximum performance, you can use the zlib-ng C library:

```toml
[dependencies]
oxyroot = { version = "0.1", features = ["zlib-ng"] }
```
