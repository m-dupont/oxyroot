use anyhow::Result;
use oxyroot::RootFile;
use std::collections::{HashMap, HashSet};

#[test]
fn tree_with_stl_containers__string() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("string")
        .expect("No branch string")
        .get_basket_into::<String>()
        .collect::<Vec<_>>();

    assert_eq!(v, ["one", "two", "three", "four", "five"]);

    Ok(())
}

#[test]
fn tree_with_stl_containers__tstring() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("tstring")
        .expect("No branch string")
        .get_basket_into::<String>()
        .collect::<Vec<_>>();

    assert_eq!(v, ["one", "two", "three", "four", "five"]);

    Ok(())
}

#[test]
fn tree_with_stl_containers__vector_int32() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("vector_int32")
        .expect("No branch vector_int32")
        .get_basket_into::<Vec<i32>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__vector_string() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("vector_string")
        .expect("No branch vector_string")
        .get_basket_into::<Vec<String>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__vector_tstring() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("vector_tstring")
        .expect("No branch vector_tstring")
        .get_basket_into::<Vec<String>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__vector_vector_i32() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("vector_vector_int32")
        .expect("No branch vector_vector_int32")
        .get_basket_into::<Vec<Vec<i32>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__vector_vector_string() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("vector_vector_string")
        .expect("No branch vector_vector_string")
        .get_basket_into::<Vec<Vec<String>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__vector_vector_tstring() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("vector_vector_tstring")
        .expect("No branch vector_vector_tstring")
        .get_basket_into::<Vec<Vec<String>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__vector_set_int32() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("vector_set_int32")
        .expect("No branch vector_set_int32")
        .get_basket_into::<Vec<Vec<i32>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__vector_set_string() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("vector_set_string")
        .expect("No branch vector_set_string")
        .get_basket_into::<Vec<HashSet<String>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__set_int32() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("set_int32")
        .expect("No branch set_int32")
        .get_basket_into::<HashSet<i32>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__set_string() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("set_string")
        .expect("No branch set_string")
        .get_basket_into::<HashSet<String>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_int32_int16() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_int32_int16")
        .expect("No branch map_int32_int16")
        .get_basket_into::<HashMap<i32, i16>>()
        .collect::<Vec<_>>();

    let good = [
        HashMap::from_iter([(1, 1)]),
        HashMap::from_iter([(1, 1), (2, 2)]),
        HashMap::from_iter([(1, 1), (2, 2), (3, 3)]),
        HashMap::from_iter([(1, 1), (2, 2), (3, 3), (4, 4)]),
        HashMap::from_iter([(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]),
    ];

    assert_eq!(v, good);

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_int32_vector_int16() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_int32_vector_int16")
        .expect("No branch map_int32_vector_int16")
        .get_basket_into::<HashMap<i32, Vec<i16>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_int32_vector_string() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_int32_vector_string")
        .expect("No branch map_int32_vector_string")
        .get_basket_into::<HashMap<i32, Vec<String>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_int32_set_int16() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_int32_set_int16")
        .expect("No branch map_int32_set_int16")
        .get_basket_into::<HashMap<i32, HashSet<i16>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_int32_set_string() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_int32_set_string")
        .expect("No branch map_int32_set_string")
        .get_basket_into::<HashMap<i32, HashSet<String>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_string_int16() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_string_int16")
        .expect("No branch map_string_int16")
        .get_basket_into::<HashMap<String, i16>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_string_vector_int16() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_string_vector_int16")
        .expect("No branch map_string_vector_int16")
        .get_basket_into::<HashMap<String, Vec<i16>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_string_vector_string() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_string_vector_string")
        .expect("No branch map_string_vector_string")
        .get_basket_into::<HashMap<String, Vec<String>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_string_set_int16() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_string_set_int16")
        .expect("No branch map_string_set_int16")
        .get_basket_into::<HashMap<String, HashSet<i16>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_string_set_string() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_string_set_string")
        .expect("No branch map_string_set_string")
        .get_basket_into::<HashMap<String, HashSet<String>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_string_string() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_string_string")
        .expect("No branch map_string_string")
        .get_basket_into::<HashMap<String, String>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_string_tstring() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_string_tstring")
        .expect("No branch map_string_tstring")
        .get_basket_into::<HashMap<String, String>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_int32_vector_vector_int16() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_int32_vector_vector_int16")
        .expect("No branch map_int32_vector_vector_int16")
        .get_basket_into::<HashMap<i32, Vec<Vec<i16>>>>()
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

    Ok(())
}

#[test]
fn tree_with_stl_containers__map_int32_vector_set_int16() -> Result<()> {
    // From https://raw.githubusercontent.com/scikit-hep/scikit-hep-testdata/main/dev/make-root/stl_containers.C

    let s = "tests/stl_containers/stl_containers.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?.unwrap();

    let v = tree
        .branch("map_int32_vector_set_int16")
        .expect("No branch map_int32_vector_set_int16")
        .get_basket_into::<HashMap<i32, Vec<HashSet<i16>>>>()
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

    Ok(())
}
