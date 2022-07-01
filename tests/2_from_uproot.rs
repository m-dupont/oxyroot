use anyhow::Result;
use oxyroot::RootFile;
use regex::internal::Input;

#[test]
fn open_nested() -> Result<()> {
    let s = "examples/from_uproot/data/HZZ.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;
    let tree = f.get_tree("events")?;
    assert!(tree.is_some());
    let tree = tree.unwrap();

    let NJet = tree.get_branch("NJet").unwrap().get_basket_into::<i32>();

    let n = NJet.count();
    assert_eq!(n, 2421);

    let mut Jet_Py = tree.get_branch("Jet_Py").unwrap().get_basket_into::<f32>();
    assert_eq!(Jet_Py.count(), 2773);

    Ok(())
}

#[test]
fn open_simple_root() -> Result<()> {
    let s = "examples/from_uproot/data/simple.root";

    // RootFile::open("old.root").unwrap();
    let mut f = RootFile::open(s)?;

    f.keys().map(|k| println!("key = {}", k)).for_each(drop);

    let tree = f.get_tree("tree")?;
    let tree = tree.unwrap();

    let one = tree
        .get_branch("one")
        .unwrap()
        .get_basket_into::<i32>()
        .collect::<Vec<_>>();

    assert_eq!(one, [1, 2, 3, 4]);

    let two = tree
        .get_branch("two")
        .unwrap()
        .get_basket_into::<f32>()
        .collect::<Vec<_>>();

    assert_eq!(two, [1.1, 2.2, 3.3, 4.4]);

    let three = tree
        .get_branch("three")
        .unwrap()
        .get_basket_into::<String>()
        .collect::<Vec<_>>();

    assert_eq!(three, ["uno", "dos", "tres", "quatro"]);

    Ok(())
}
