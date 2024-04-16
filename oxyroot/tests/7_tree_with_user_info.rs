use anyhow::Result;
use log::trace;
use oxyroot::RootFile;

#[test]
#[allow(non_snake_case)]
fn tree_with_user_info_with_map() -> Result<()> {
    let s = "tests/7_userinfo/tree_with_user_info_with_map.root";
    let mut f = RootFile::open(s)?;
    let tree = f.get_tree("tree")?;
    let user_info = tree.user_info();
    assert!(user_info.is_some());
    let user_info = user_info.unwrap();
    assert_eq!(user_info.len(), 2);

    let m = user_info.at::<oxyroot::rcont::TMap>(0);

    let key = oxyroot::rcont::tmap::Key::String("key!".to_string());
    let value = m.get::<oxyroot::rbase::TObjString>(&key);
    assert!(value.is_some());
    let value = value.unwrap();
    assert_eq!(value.to_string(), "value!");

    let s = user_info.at::<oxyroot::rbase::TObjString>(1);
    assert_eq!(s.to_string(), "info!");

    Ok(())
}
