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
