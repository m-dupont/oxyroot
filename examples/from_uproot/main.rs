use anyhow::Result;
use chrono::Local;
use env_logger::{Builder, Target, WriteStyle};
use log::{error, trace, LevelFilter};
use oxyroot::RootFile;
use oxyroot::Unmarshaler;
use oxyroot::{RBuffer, Slice};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io::Write;
use std::mem;

fn open_HZZ_root() -> Result<()> {
    let s = "examples/from_uproot/data/HZZ.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("events")?;
    let tree = tree.unwrap();

    for b in tree.branches() {
        println!(
            "branch = {},\tclass_name = {},\tentries = {}",
            b.name(),
            b.item_type_name(),
            b.entries()
        );
    }

    // panic!("plop");

    println!("entries = {}", tree.entries());

    let mut Jet_Px = tree.branch("Jet_Px").unwrap().as_iter::<f32>();
    let mut Jet_Py = tree.branch("Jet_Py").unwrap().as_iter::<f32>();
    let mut Jet_Pz = tree.branch("Jet_Pz").unwrap().as_iter::<f32>();

    let NJet = tree.branch("NJet").unwrap().as_iter::<i32>();

    NJet.take(3000).enumerate().for_each(|(n_entry, n)| {
        println!("n_entry = {n_entry}");

        for i in 0..n {
            println!(
                "\t i = {i}  Jet_Px = {} Jet_Py = {} Jet_Pz = {} ",
                Jet_Px.next().unwrap(),
                Jet_Py.next().unwrap(),
                Jet_Pz.next().unwrap()
            );
        }
    });

    Ok(())
}

fn open_simple_root() -> Result<()> {
    let s = "examples/from_uproot/data/simple.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;
    let tree = tree.unwrap();

    for b in tree.branches() {
        println!(
            "branch = {},\tclass_name = {},\tentries = {}",
            b.name(),
            b.item_type_name(),
            b.entries()
        );
    }

    let one = tree
        .branch("one")
        .unwrap()
        .as_iter::<i32>()
        .collect::<Vec<_>>();

    assert_eq!(one, [1, 2, 3, 4]);

    let two = tree
        .branch("two")
        .unwrap()
        .as_iter::<f32>()
        .collect::<Vec<_>>();

    assert_eq!(two, [1.1, 2.2, 3.3, 4.4]);

    // let f = |r: &mut RBuffer| {
    //     let val = r.read_string().unwrap().to_string();
    //     println!("val = {:?}", val);
    //     val
    // };

    let three = tree
        .branch("three")
        .unwrap()
        .as_iter::<String>()
        .collect::<Vec<_>>();

    assert_eq!(three, ["uno", "dos", "tres", "quatro"]);

    Ok(())
}

fn open_small_evnt_tree_fullsplit_root() -> Result<()> {
    let s = "examples/from_uproot/data/small-evnt-tree-fullsplit.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;
    let tree = tree.unwrap();

    for b in tree.branches() {
        println!(
            "branch = {},\tclass_name = {},\tentries = {}",
            b.name(),
            b.item_type_name(),
            b.entries()
        );

        for bb in b.branches() {
            println!(
                "\tbranch = {},\tclass_name = {},\tentries = {}",
                bb.name(),
                bb.item_type_name(),
                bb.entries()
            );

            for bbb in bb.branches() {
                println!(
                    "branch = {},\tclass_name = {},\tentries = {}",
                    bbb.name(),
                    bbb.item_type_name(),
                    bbb.entries()
                );
            }
        }
    }

    tree.branch("SliceI16")
        .unwrap()
        .as_iter::<oxyroot::Slice<i16>>()
        .enumerate()
        .for_each(|(i, val)| println!("SliceI16: i = {i} val = {:?}", val));

    // panic!("plop");

    tree.branch("Beg")
        .unwrap()
        .as_iter::<String>()
        .enumerate()
        .for_each(|(i, s)| {
            assert_eq!(s, format!("beg-{:03}", i));
        });

    tree.branch("P3")
        .unwrap()
        .get_basket(|r| {
            let x = r.read_i32().unwrap();
            let y = r.read_f64().unwrap();
            let z = r.read_i32().unwrap();
            (x, y, z)
        })
        .enumerate()
        .for_each(|(i, (x, y, z))| {
            // println!("x = {x} y = {y}");
            let i = i as i32;
            assert_eq!(x, i - 1);
            assert_eq!(x, z);
            assert_eq!(y, i as f64);
        });

    tree.branch("ArrayI16[10]")
        .unwrap()
        .get_basket(|r| {
            let mut buf = [0 as i16; 10];
            r.read_array_i16(&mut buf).unwrap();
            buf
            // buf.to_vec()
        })
        .enumerate()
        .for_each(|(i, buf)| {
            // println!("buf = {:?}", buf);
            buf.map(|v| assert_eq!(v, i as i16));
        });

    tree.branch("StlVecI16")
        .unwrap()
        .as_iter::<Vec<i16>>()
        .enumerate()
        .for_each(|(i, val)| println!("StlVecI16: i = {i} val = {:?}", val));

    tree.branch("StlVecStr")
        .unwrap()
        .as_iter::<Vec<String>>()
        .enumerate()
        .for_each(|(i, val)| println!("StlVecStr: i = {i} val = {:?}", val));

    tree.branch("End")
        .unwrap()
        .as_iter::<String>()
        .enumerate()
        .for_each(|(i, val)| println!("End: i = {i} val = {:?}", val));

    tree.branch("StdStr")
        .unwrap()
        .as_iter::<String>()
        .enumerate()
        .for_each(|(i, val)| println!("StdStr: i = {i} val = {:?}", val));

    Ok(())
}

fn tree_with_jagged_array() -> Result<()> {
    let s = "examples/from_uproot/data/tree_with_jagged_array.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("t1")?.unwrap();
    // let tree = tree.unwrap();

    for b in tree.branches() {
        println!(
            "branch = {},\tclass_name = {},\tentries = {}",
            b.name(),
            b.item_type_name(),
            b.entries()
        )
    }

    tree.branch("int32_array")
        .unwrap()
        .get_basket(|r| {
            r.do_skip_header().unwrap();

            let size = r.read_i32().unwrap();
            let mut len = r.len() as usize;
            trace!("len = {}", len);
            let mut ret: Vec<i32> = Vec::new();
            while len > 0 {
                ret.push(r.read_i32().unwrap());
                len -= mem::size_of::<i32>();
            }

            ret
        })
        .enumerate()
        .for_each(|(i, val)| {
            assert_eq!(val.len(), i % 10);
            // prinytln!("StlVecI16: i = {i} val = {:?}, {}", val, i - i % 10);

            val.iter()
                .enumerate()
                .map(|(j, v)| {
                    assert_eq!(*v, (i - i % 10 + j) as i32);
                })
                .for_each(drop);
        });

    Ok(())
}

fn tree_with_stl_containers() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    for b in tree.branches() {
        println!(
            "branch = {},\tclass_name = {},\tentries = {}",
            b.name(),
            b.item_type_name(),
            b.entries()
        )
    }

    let v = tree
        .branch("string")
        .expect("No branch string")
        .as_iter::<String>()
        .collect::<Vec<_>>();

    assert_eq!(v, ["one", "two", "three", "four", "five"]);

    let v = tree
        .branch("tstring")
        .expect("No branch tstring")
        .as_iter::<String>()
        .collect::<Vec<_>>();

    assert_eq!(v, ["one", "two", "three", "four", "five"]);

    let v = tree
        .branch("vector_int32")
        .expect("No branch vector_int32")
        .as_iter::<Vec<i32>>()
        .collect::<Vec<_>>();

    assert_eq!(
        v,
        [
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 5]
        ]
    );
    let v = tree
        .branch("vector_string")
        .expect("No branch vector_string")
        .as_iter::<Vec<String>>()
        .collect::<Vec<_>>();

    assert_eq!(
        v,
        [
            vec!["one"],
            vec!["one", "two"],
            vec!["one", "two", "three"],
            vec!["one", "two", "three", "four"],
            vec!["one", "two", "three", "four", "five"]
        ]
    );

    let v = tree
        .branch("vector_vector_int32")
        .expect("No branch vector_vector_int32")
        .as_iter::<Vec<Vec<i32>>>()
        .map(|a| a.to_vec())
        .collect::<Vec<_>>();

    println!("v = {:?}", v);

    assert_eq!(
        v,
        [
            vec![vec![1]],
            vec![vec![1], vec![1, 2]],
            vec![vec![1], vec![1, 2], vec![1, 2, 3]],
            vec![vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 2, 3, 4]],
            vec![
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4, 5],
            ],
        ]
    );

    let v = tree
        .branch("vector_vector_string")
        .expect("No branch vector_vector_string")
        .as_iter::<Vec<Vec<String>>>()
        .map(|a| a.to_vec())
        .collect::<Vec<_>>();

    println!("v = {:?}", v);

    assert_eq!(
        v,
        [
            vec![vec!["one"]],
            vec![vec!["one"], vec!["one", "two"]],
            vec![vec!["one"], vec!["one", "two"], vec!["one", "two", "three"]],
            vec![
                vec!["one"],
                vec!["one", "two"],
                vec!["one", "two", "three"],
                vec!["one", "two", "three", "four"],
            ],
            vec![
                vec!["one"],
                vec!["one", "two"],
                vec!["one", "two", "three"],
                vec!["one", "two", "three", "four"],
                vec!["one", "two", "three", "four", "five"],
            ]
        ]
    );

    let v = tree
        .branch("vector_vector_tstring")
        .expect("No branch vector_vector_tstring")
        .as_iter::<Vec<Vec<String>>>()
        .map(|a| a.to_vec())
        .collect::<Vec<_>>();

    println!("v = {:?}", v);

    assert_eq!(
        v,
        [
            vec![vec!["one"]],
            vec![vec!["one"], vec!["one", "two"]],
            vec![vec!["one"], vec!["one", "two"], vec!["one", "two", "three"]],
            vec![
                vec!["one"],
                vec!["one", "two"],
                vec!["one", "two", "three"],
                vec!["one", "two", "three", "four"],
            ],
            vec![
                vec!["one"],
                vec!["one", "two"],
                vec!["one", "two", "three"],
                vec!["one", "two", "three", "four"],
                vec!["one", "two", "three", "four", "five"],
            ]
        ]
    );

    let v = tree
        .branch("vector_set_int32")
        .expect("No branch vector_set_int32")
        .as_iter::<Vec<Vec<i32>>>()
        .map(|a| a.to_vec())
        .collect::<Vec<_>>();

    println!("v = {:?}", v);

    assert_eq!(
        v,
        [
            vec![vec![1]],
            vec![vec![1], vec![1, 2]],
            vec![vec![1], vec![1, 2], vec![1, 2, 3]],
            vec![vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 2, 3, 4]],
            vec![
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4, 5],
            ],
        ]
    );

    let v = tree
        .branch("vector_set_string")
        .expect("No branch vector_set_string")
        .as_iter::<Vec<HashSet<String>>>()
        .map(|a| a.to_vec())
        .collect::<Vec<_>>();

    println!("v = {:?}", v);

    let good = [
        vec![HashSet::from(["one"])],
        vec![HashSet::from(["one"]), HashSet::from(["one", "two"])],
        vec![
            HashSet::from(["one"]),
            HashSet::from(["one", "two"]),
            HashSet::from(["one", "three", "two"]),
        ],
        vec![
            HashSet::from(["one"]),
            HashSet::from(["one", "two"]),
            HashSet::from(["one", "three", "two"]),
            HashSet::from(["four", "one", "three", "two"]),
        ],
        vec![
            HashSet::from(["one"]),
            HashSet::from(["one", "two"]),
            HashSet::from(["one", "three", "two"]),
            HashSet::from(["four", "one", "three", "two"]),
            HashSet::from(["five", "four", "one", "three", "two"]),
        ],
    ];

    v.iter()
        .zip(good.iter())
        .for_each(|(row_read, row_provided)| {
            row_provided
                .iter()
                .zip(row_provided.iter())
                .for_each(|(set_read, set_provided)| {
                    assert_eq!(set_read, set_provided);
                });

            println!("row_read = {:?}", row_read);
            println!("row_provided = {:?}", row_provided);
        });

    let v = tree
        .branch("set_int32")
        .expect("No branch set_int32")
        .as_iter::<HashSet<i32>>()
        .collect::<Vec<_>>();

    assert_eq!(
        v,
        [
            HashSet::from([1]),
            HashSet::from([1, 2]),
            HashSet::from([1, 2, 3]),
            HashSet::from([1, 2, 3, 4]),
            HashSet::from([1, 2, 3, 4, 5])
        ]
    );

    let v = tree
        .branch("set_string")
        .expect("No branch set_string")
        .as_iter::<HashSet<String>>()
        .collect::<Vec<_>>();

    assert_eq!(
        v,
        [
            HashSet::from(["one"].map(|s| s.to_string())),
            HashSet::from(["one", "two"].map(|s| s.to_string())),
            HashSet::from(["one", "two", "three"].map(|s| s.to_string())),
            HashSet::from(["one", "two", "three", "four"].map(|s| s.to_string())),
            HashSet::from(["one", "two", "three", "four", "five"].map(|s| s.to_string()))
        ]
    );

    let v = tree
        .branch("map_int32_int16")
        .expect("No branch map_int32_int16")
        .as_iter::<HashMap<i32, i16>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([(1, 1)]),
        HashMap::from_iter([(1, 1), (2, 2)]),
        HashMap::from_iter([(1, 1), (2, 2), (3, 3)]),
        HashMap::from_iter([(1, 1), (2, 2), (3, 3), (4, 4)]),
        HashMap::from_iter([(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]),
    ];

    assert_eq!(v, good);

    let v = tree
        .branch("map_int32_vector_int16")
        .expect("No branch map_int32_vector_int16")
        .as_iter::<HashMap<i32, Vec<i16>>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([(1, vec![1])]),
        HashMap::from_iter([(1, vec![1]), (2, vec![1, 2])]),
        HashMap::from_iter([(1, vec![1]), (2, vec![1, 2]), (3, vec![1, 2, 3])]),
        HashMap::from_iter([
            (1, vec![1]),
            (2, vec![1, 2]),
            (3, vec![1, 2, 3]),
            (4, vec![1, 2, 3, 4]),
        ]),
        HashMap::from_iter([
            (1, vec![1]),
            (2, vec![1, 2]),
            (3, vec![1, 2, 3]),
            (4, vec![1, 2, 3, 4]),
            (5, vec![1, 2, 3, 4, 5]),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);

    let v = tree
        .branch("map_int32_vector_string")
        .expect("No branch map_int32_vector_string")
        .as_iter::<HashMap<i32, Vec<String>>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([(1, vec!["one".to_string()])]),
        HashMap::from_iter([
            (1, vec!["one".to_string()]),
            (2, vec!["one".to_string(), "two".to_string()]),
        ]),
        HashMap::from_iter([
            (1, vec!["one".to_string()]),
            (2, vec!["one".to_string(), "two".to_string()]),
            (
                3,
                vec!["one".to_string(), "two".to_string(), "three".to_string()],
            ),
        ]),
        HashMap::from_iter([
            (1, vec!["one".to_string()]),
            (2, vec!["one".to_string(), "two".to_string()]),
            (
                3,
                vec!["one".to_string(), "two".to_string(), "three".to_string()],
            ),
            (
                4,
                vec![
                    "one".to_string(),
                    "two".to_string(),
                    "three".to_string(),
                    "four".to_string(),
                ],
            ),
        ]),
        HashMap::from_iter([
            (1, vec!["one".to_string()]),
            (2, vec!["one".to_string(), "two".to_string()]),
            (
                3,
                vec!["one".to_string(), "two".to_string(), "three".to_string()],
            ),
            (
                4,
                vec![
                    "one".to_string(),
                    "two".to_string(),
                    "three".to_string(),
                    "four".to_string(),
                ],
            ),
            (
                5,
                vec![
                    "one".to_string(),
                    "two".to_string(),
                    "three".to_string(),
                    "four".to_string(),
                    "five".to_string(),
                ],
            ),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);

    let v = tree
        .branch("map_int32_set_int16")
        .expect("No branch map_int32_set_int16")
        .as_iter::<HashMap<i32, HashSet<i16>>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([(1, HashSet::from([1]))]),
        HashMap::from_iter([(1, HashSet::from([1])), (2, HashSet::from([1, 2]))]),
        HashMap::from_iter([
            (1, HashSet::from([1])),
            (2, HashSet::from([1, 2])),
            (3, HashSet::from([1, 2, 3])),
        ]),
        HashMap::from_iter([
            (1, HashSet::from([1])),
            (2, HashSet::from([1, 2])),
            (3, HashSet::from([1, 2, 3])),
            (4, HashSet::from([1, 2, 3, 4])),
        ]),
        HashMap::from_iter([
            (1, HashSet::from([1])),
            (2, HashSet::from([1, 2])),
            (3, HashSet::from([1, 2, 3])),
            (4, HashSet::from([1, 2, 3, 4])),
            (5, HashSet::from([1, 2, 3, 4, 5])),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);

    let v = tree
        .branch("map_int32_set_string")
        .expect("No branch map_int32_set_string")
        .as_iter::<HashMap<i32, HashSet<String>>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([(1, HashSet::from(["one".to_string()]))]),
        HashMap::from_iter([
            (1, HashSet::from(["one".to_string()])),
            (2, HashSet::from(["one".to_string(), "two".to_string()])),
        ]),
        HashMap::from_iter([
            (1, HashSet::from(["one".to_string()])),
            (2, HashSet::from(["one".to_string(), "two".to_string()])),
            (
                3,
                HashSet::from(["one".to_string(), "three".to_string(), "two".to_string()]),
            ),
        ]),
        HashMap::from_iter([
            (1, HashSet::from(["one".to_string()])),
            (2, HashSet::from(["one".to_string(), "two".to_string()])),
            (
                3,
                HashSet::from(["one".to_string(), "three".to_string(), "two".to_string()]),
            ),
            (
                4,
                HashSet::from([
                    "four".to_string(),
                    "one".to_string(),
                    "three".to_string(),
                    "two".to_string(),
                ]),
            ),
        ]),
        HashMap::from_iter([
            (1, HashSet::from(["one".to_string()])),
            (2, HashSet::from(["one".to_string(), "two".to_string()])),
            (
                3,
                HashSet::from(["one".to_string(), "three".to_string(), "two".to_string()]),
            ),
            (
                4,
                HashSet::from([
                    "four".to_string(),
                    "one".to_string(),
                    "three".to_string(),
                    "two".to_string(),
                ]),
            ),
            (
                5,
                HashSet::from([
                    "five".to_string(),
                    "four".to_string(),
                    "one".to_string(),
                    "three".to_string(),
                    "two".to_string(),
                ]),
            ),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);

    let v = tree
        .branch("map_string_int16")
        .expect("No branch map_string_int16")
        .as_iter::<HashMap<String, i16>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([("one".to_string(), 1)]),
        HashMap::from_iter([("one".to_string(), 1), ("two".to_string(), 2)]),
        HashMap::from_iter([
            ("one".to_string(), 1),
            ("three".to_string(), 3),
            ("two".to_string(), 2),
        ]),
        HashMap::from_iter([
            ("four".to_string(), 4),
            ("one".to_string(), 1),
            ("three".to_string(), 3),
            ("two".to_string(), 2),
        ]),
        HashMap::from_iter([
            ("five".to_string(), 5),
            ("four".to_string(), 4),
            ("one".to_string(), 1),
            ("three".to_string(), 3),
            ("two".to_string(), 2),
        ]),
    ];

    assert_eq!(v, good);

    let v = tree
        .branch("map_string_vector_int16")
        .expect("No branch map_string_vector_int16")
        .as_iter::<HashMap<String, Vec<i16>>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([("one".to_string(), vec![1])]),
        HashMap::from_iter([
            ("one".to_string(), vec![1]),
            ("two".to_string(), vec![1, 2]),
        ]),
        HashMap::from_iter([
            ("one".to_string(), vec![1]),
            ("three".to_string(), vec![1, 2, 3]),
            ("two".to_string(), vec![1, 2]),
        ]),
        HashMap::from_iter([
            ("four".to_string(), vec![1, 2, 3, 4]),
            ("one".to_string(), vec![1]),
            ("three".to_string(), vec![1, 2, 3]),
            ("two".to_string(), vec![1, 2]),
        ]),
        HashMap::from_iter([
            ("five".to_string(), vec![1, 2, 3, 4, 5]),
            ("four".to_string(), vec![1, 2, 3, 4]),
            ("one".to_string(), vec![1]),
            ("three".to_string(), vec![1, 2, 3]),
            ("two".to_string(), vec![1, 2]),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);

    let v = tree
        .branch("map_string_vector_string")
        .expect("No branch map_string_vector_string")
        .as_iter::<HashMap<String, Vec<String>>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([("one".to_string(), vec!["one".to_string()])]),
        HashMap::from_iter([
            ("one".to_string(), vec!["one".to_string()]),
            (
                "two".to_string(),
                vec!["one".to_string(), "two".to_string()],
            ),
        ]),
        HashMap::from_iter([
            ("one".to_string(), vec!["one".to_string()]),
            (
                "three".to_string(),
                vec!["one".to_string(), "two".to_string(), "three".to_string()],
            ),
            (
                "two".to_string(),
                vec!["one".to_string(), "two".to_string()],
            ),
        ]),
        HashMap::from_iter([
            (
                "four".to_string(),
                vec![
                    "one".to_string(),
                    "two".to_string(),
                    "three".to_string(),
                    "four".to_string(),
                ],
            ),
            ("one".to_string(), vec!["one".to_string()]),
            (
                "three".to_string(),
                vec!["one".to_string(), "two".to_string(), "three".to_string()],
            ),
            (
                "two".to_string(),
                vec!["one".to_string(), "two".to_string()],
            ),
        ]),
        HashMap::from_iter([
            (
                "five".to_string(),
                vec![
                    "one".to_string(),
                    "two".to_string(),
                    "three".to_string(),
                    "four".to_string(),
                    "five".to_string(),
                ],
            ),
            (
                "four".to_string(),
                vec![
                    "one".to_string(),
                    "two".to_string(),
                    "three".to_string(),
                    "four".to_string(),
                ],
            ),
            ("one".to_string(), vec!["one".to_string()]),
            (
                "three".to_string(),
                vec!["one".to_string(), "two".to_string(), "three".to_string()],
            ),
            (
                "two".to_string(),
                vec!["one".to_string(), "two".to_string()],
            ),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);

    let v = tree
        .branch("map_string_set_int16")
        .expect("No branch map_string_set_int16")
        .as_iter::<HashMap<String, HashSet<i16>>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([("one".to_string(), HashSet::from([1]))]),
        HashMap::from_iter([
            ("one".to_string(), HashSet::from([1])),
            ("two".to_string(), HashSet::from([1, 2])),
        ]),
        HashMap::from_iter([
            ("one".to_string(), HashSet::from([1])),
            ("three".to_string(), HashSet::from([1, 2, 3])),
            ("two".to_string(), HashSet::from([1, 2])),
        ]),
        HashMap::from_iter([
            ("four".to_string(), HashSet::from([1, 2, 3, 4])),
            ("one".to_string(), HashSet::from([1])),
            ("three".to_string(), HashSet::from([1, 2, 3])),
            ("two".to_string(), HashSet::from([1, 2])),
        ]),
        HashMap::from_iter([
            ("five".to_string(), HashSet::from([1, 2, 3, 4, 5])),
            ("four".to_string(), HashSet::from([1, 2, 3, 4])),
            ("one".to_string(), HashSet::from([1])),
            ("three".to_string(), HashSet::from([1, 2, 3])),
            ("two".to_string(), HashSet::from([1, 2])),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);

    let v = tree
        .branch("map_string_set_string")
        .expect("No branch map_string_set_string")
        .as_iter::<HashMap<String, HashSet<String>>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([("one".to_string(), HashSet::from(["one".to_string()]))]),
        HashMap::from_iter([
            ("one".to_string(), HashSet::from(["one".to_string()])),
            (
                "two".to_string(),
                HashSet::from(["one".to_string(), "two".to_string()]),
            ),
        ]),
        HashMap::from_iter([
            ("one".to_string(), HashSet::from(["one".to_string()])),
            (
                "three".to_string(),
                HashSet::from(["one".to_string(), "three".to_string(), "two".to_string()]),
            ),
            (
                "two".to_string(),
                HashSet::from(["one".to_string(), "two".to_string()]),
            ),
        ]),
        HashMap::from_iter([
            (
                "four".to_string(),
                HashSet::from([
                    "four".to_string(),
                    "one".to_string(),
                    "three".to_string(),
                    "two".to_string(),
                ]),
            ),
            ("one".to_string(), HashSet::from(["one".to_string()])),
            (
                "three".to_string(),
                HashSet::from(["one".to_string(), "three".to_string(), "two".to_string()]),
            ),
            (
                "two".to_string(),
                HashSet::from(["one".to_string(), "two".to_string()]),
            ),
        ]),
        HashMap::from_iter([
            (
                "five".to_string(),
                HashSet::from([
                    "five".to_string(),
                    "four".to_string(),
                    "one".to_string(),
                    "three".to_string(),
                    "two".to_string(),
                ]),
            ),
            (
                "four".to_string(),
                HashSet::from([
                    "four".to_string(),
                    "one".to_string(),
                    "three".to_string(),
                    "two".to_string(),
                ]),
            ),
            ("one".to_string(), HashSet::from(["one".to_string()])),
            (
                "three".to_string(),
                HashSet::from(["one".to_string(), "three".to_string(), "two".to_string()]),
            ),
            (
                "two".to_string(),
                HashSet::from(["one".to_string(), "two".to_string()]),
            ),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);

    let v = tree
        .branch("map_string_string")
        .expect("No branch map_string_string")
        .as_iter::<HashMap<String, String>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([("one".to_string(), "ONE".to_string())]),
        HashMap::from_iter([
            ("one".to_string(), "ONE".to_string()),
            ("two".to_string(), "TWO".to_string()),
        ]),
        HashMap::from_iter([
            ("one".to_string(), "ONE".to_string()),
            ("three".to_string(), "THREE".to_string()),
            ("two".to_string(), "TWO".to_string()),
        ]),
        HashMap::from_iter([
            ("four".to_string(), "FOUR".to_string()),
            ("one".to_string(), "ONE".to_string()),
            ("three".to_string(), "THREE".to_string()),
            ("two".to_string(), "TWO".to_string()),
        ]),
        HashMap::from_iter([
            ("five".to_string(), "FIVE".to_string()),
            ("four".to_string(), "FOUR".to_string()),
            ("one".to_string(), "ONE".to_string()),
            ("three".to_string(), "THREE".to_string()),
            ("two".to_string(), "TWO".to_string()),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);

    let v = tree
        .branch("map_string_tstring")
        .expect("No branch map_string_tstring")
        .as_iter::<HashMap<String, String>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([("one".to_string(), "ONE".to_string())]),
        HashMap::from_iter([
            ("one".to_string(), "ONE".to_string()),
            ("two".to_string(), "TWO".to_string()),
        ]),
        HashMap::from_iter([
            ("one".to_string(), "ONE".to_string()),
            ("three".to_string(), "THREE".to_string()),
            ("two".to_string(), "TWO".to_string()),
        ]),
        HashMap::from_iter([
            ("four".to_string(), "FOUR".to_string()),
            ("one".to_string(), "ONE".to_string()),
            ("three".to_string(), "THREE".to_string()),
            ("two".to_string(), "TWO".to_string()),
        ]),
        HashMap::from_iter([
            ("five".to_string(), "FIVE".to_string()),
            ("four".to_string(), "FOUR".to_string()),
            ("one".to_string(), "ONE".to_string()),
            ("three".to_string(), "THREE".to_string()),
            ("two".to_string(), "TWO".to_string()),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);

    let v = tree
        .branch("map_int32_vector_vector_int16")
        .expect("No branch map_int32_vector_vector_int16")
        .as_iter::<HashMap<i32, Vec<Vec<i16>>>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([(1, vec![vec![1]])]),
        HashMap::from_iter([(1, vec![vec![1]]), (2, vec![vec![1], vec![1, 2]])]),
        HashMap::from_iter([
            (1, vec![vec![1]]),
            (2, vec![vec![1], vec![1, 2]]),
            (3, vec![vec![1], vec![1, 2], vec![1, 2, 3]]),
        ]),
        HashMap::from_iter([
            (1, vec![vec![1]]),
            (2, vec![vec![1], vec![1, 2]]),
            (3, vec![vec![1], vec![1, 2], vec![1, 2, 3]]),
            (
                4,
                vec![vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 2, 3, 4]],
            ),
        ]),
        HashMap::from_iter([
            (1, vec![vec![1]]),
            (2, vec![vec![1], vec![1, 2]]),
            (3, vec![vec![1], vec![1, 2], vec![1, 2, 3]]),
            (
                4,
                vec![vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 2, 3, 4]],
            ),
            (
                5,
                vec![
                    vec![1],
                    vec![1, 2],
                    vec![1, 2, 3],
                    vec![1, 2, 3, 4],
                    vec![1, 2, 3, 4, 5],
                ],
            ),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);

    let v = tree
        .branch("map_int32_vector_set_int16")
        .expect("No branch map_int32_vector_set_int16")
        .as_iter::<HashMap<i32, Vec<HashSet<i16>>>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([(1, vec![HashSet::from([1])])]),
        HashMap::from_iter([
            (1, vec![HashSet::from([1])]),
            (2, vec![HashSet::from([1]), HashSet::from([1, 2])]),
        ]),
        HashMap::from_iter([
            (1, vec![HashSet::from([1])]),
            (2, vec![HashSet::from([1]), HashSet::from([1, 2])]),
            (
                3,
                vec![
                    HashSet::from([1]),
                    HashSet::from([1, 2]),
                    HashSet::from([1, 2, 3]),
                ],
            ),
        ]),
        HashMap::from_iter([
            (1, vec![HashSet::from([1])]),
            (2, vec![HashSet::from([1]), HashSet::from([1, 2])]),
            (
                3,
                vec![
                    HashSet::from([1]),
                    HashSet::from([1, 2]),
                    HashSet::from([1, 2, 3]),
                ],
            ),
            (
                4,
                vec![
                    HashSet::from([1]),
                    HashSet::from([1, 2]),
                    HashSet::from([1, 2, 3]),
                    HashSet::from([1, 2, 3, 4]),
                ],
            ),
        ]),
        HashMap::from_iter([
            (1, vec![HashSet::from([1])]),
            (2, vec![HashSet::from([1]), HashSet::from([1, 2])]),
            (
                3,
                vec![
                    HashSet::from([1]),
                    HashSet::from([1, 2]),
                    HashSet::from([1, 2, 3]),
                ],
            ),
            (
                4,
                vec![
                    HashSet::from([1]),
                    HashSet::from([1, 2]),
                    HashSet::from([1, 2, 3]),
                    HashSet::from([1, 2, 3, 4]),
                ],
            ),
            (
                5,
                vec![
                    HashSet::from([1]),
                    HashSet::from([1, 2]),
                    HashSet::from([1, 2, 3]),
                    HashSet::from([1, 2, 3, 4]),
                    HashSet::from([1, 2, 3, 4, 5]),
                ],
            ),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);

    Ok(())
}

fn tree_with_stl_containers_tmp() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_int32_vector_set_int16")
        .expect("No branch map_int32_vector_set_int16")
        .as_iter::<HashMap<i32, Vec<HashSet<i16>>>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([(1, vec![HashSet::from([1])])]),
        HashMap::from_iter([
            (1, vec![HashSet::from([1])]),
            (2, vec![HashSet::from([1]), HashSet::from([1, 2])]),
        ]),
        HashMap::from_iter([
            (1, vec![HashSet::from([1])]),
            (2, vec![HashSet::from([1]), HashSet::from([1, 2])]),
            (
                3,
                vec![
                    HashSet::from([1]),
                    HashSet::from([1, 2]),
                    HashSet::from([1, 2, 3]),
                ],
            ),
        ]),
        HashMap::from_iter([
            (1, vec![HashSet::from([1])]),
            (2, vec![HashSet::from([1]), HashSet::from([1, 2])]),
            (
                3,
                vec![
                    HashSet::from([1]),
                    HashSet::from([1, 2]),
                    HashSet::from([1, 2, 3]),
                ],
            ),
            (
                4,
                vec![
                    HashSet::from([1]),
                    HashSet::from([1, 2]),
                    HashSet::from([1, 2, 3]),
                    HashSet::from([1, 2, 3, 4]),
                ],
            ),
        ]),
        HashMap::from_iter([
            (1, vec![HashSet::from([1])]),
            (2, vec![HashSet::from([1]), HashSet::from([1, 2])]),
            (
                3,
                vec![
                    HashSet::from([1]),
                    HashSet::from([1, 2]),
                    HashSet::from([1, 2, 3]),
                ],
            ),
            (
                4,
                vec![
                    HashSet::from([1]),
                    HashSet::from([1, 2]),
                    HashSet::from([1, 2, 3]),
                    HashSet::from([1, 2, 3, 4]),
                ],
            ),
            (
                5,
                vec![
                    HashSet::from([1]),
                    HashSet::from([1, 2]),
                    HashSet::from([1, 2, 3]),
                    HashSet::from([1, 2, 3, 4]),
                    HashSet::from([1, 2, 3, 4, 5]),
                ],
            ),
        ]),
    ];

    assert_eq!(v, good);

    println!("v = {:?}", v);
    Ok(())
}

fn main() {
    let _stylish_logger = Builder::new()
        .filter(None, LevelFilter::Trace)
        .write_style(WriteStyle::Always)
        .format(|buf, record| {
            let level = record.metadata().level().as_str().to_ascii_uppercase();
            let file = record.file().unwrap_or("");
            let line = record.line().unwrap_or(0);
            let module = record.module_path().unwrap_or("");
            let time = Local::now().format("%Y-%m-%dT%H:%M:%S");
            writeln!(
                buf,
                "{}:[{}]:{}>{} ({}@{})",
                time,
                level,
                module,
                record.args(),
                file,
                line
            )
        })
        .target(Target::Stdout)
        .init();
    // stylish_logger.log(&record());
    trace!("hello");
    error!("info");
    println!("example of opening file");

    // open_HZZ_root().expect("NOOOO");
    // open_simple_root().expect("NOOOO");
    // open_small_evnt_tree_fullsplit_root().expect("NOOOO");
    // tree_with_jagged_array().expect("NOOOO");
    tree_with_stl_containers().expect("NOOOO");
    // tree_with_stl_containers_tmp().expect("NOOOO");
}
