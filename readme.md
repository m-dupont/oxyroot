oxyroot
=======

Another attempt to make library reading of `.root` binary files which are commonly used in particle physics

# Inspiration

To make this library :

- inspiration taken from [groot](https://github.com/go-hep/hep/tree/main/groot) for reading root file, even the code
  organisation
- inspiration taken from [uproot](https://github.com/scikit-hep/uproot5) to provide branch interface (for reading basket
  buffer)

# Getting started

# Example: Iter over a branch tree containing `i32` values

```rust
use oxyroot::RootFile;
let s = "examples/from_uproot/data/HZZ.root";
let tree = RootFile::open(s).unwrap().get_tree("events").unwrap();
let NJet = tree.branch("NJet").unwrap().as_iter::<i32>();
NJet.for_each( | v| println!("v = {v}"));
```

# Example: Iter over a branch tree containing `Vec<i32>`  (aka `std::vector<int32_t>`) values

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