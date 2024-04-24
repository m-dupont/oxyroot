use oxyroot::{ReadFromTree, RootFile};

#[test]
fn t04_01_read_tree_points() -> anyhow::Result<()> {
    #[derive(ReadFromTree)]
    struct Points {
        x: i32,
        y: i32,
    }

    let file = "create_root_files_with_root/t04_01_write_tree_points.root";

    let tree = RootFile::open(file)?.get_tree("myTree")?;
    for (i, p) in Points::from_tree(&tree)?.enumerate() {
        assert_eq!(p.x, i as i32);
        assert_eq!(p.y, (i * i) as i32);
    }
    Ok(())
}

#[test]
fn t04_02_write_tree_points_prefix() -> anyhow::Result<()> {
    #[derive(ReadFromTree)]
    #[oxyroot(branch_prefix = "branch.")]
    struct Points {
        x: i32,
        y: i32,
    }

    let file = "create_root_files_with_root/t04_02_write_tree_points_prefix.root";

    let tree = RootFile::open(file)?.get_tree("myTree")?;
    for (i, p) in Points::from_tree(&tree)?.enumerate() {
        assert_eq!(p.x, i as i32);
        assert_eq!(p.y, (i * i) as i32);
    }
    Ok(())
}

#[test]
fn t04_03_write_tree_pointsvector() -> anyhow::Result<()> {
    #[derive(ReadFromTree)]
    struct Points {
        x: Vec<i32>,
        y: Vec<i32>,
    }

    let file = "create_root_files_with_root/t04_03_write_tree_pointsvector.root";

    let tree = RootFile::open(file)?.get_tree("myTree")?;
    for (i, p) in Points::from_tree(&tree)?.enumerate() {
        let xx = vec![i as i32; i];
        let yy: Vec<_> = (0..i as i32).collect();
        assert_eq!(p.x, xx);
        assert_eq!(p.y, yy);
    }
    Ok(())
}

#[test]
fn t04_04_write_twopoints() -> anyhow::Result<()> {
    #[derive(ReadFromTree)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(ReadFromTree)]
    struct TwoPoints {
        p1: Point,
        p2: Point,
    }

    let file = "create_root_files_with_root/t04_04_write_twopoints.root";

    let tree = RootFile::open(file)?.get_tree("myTree")?;
    for (i, p) in TwoPoints::from_tree(&tree)?.enumerate() {
        assert_eq!(p.p1.x, i as i32);
        assert_eq!(p.p2.y, (i * i) as i32);
    }
    Ok(())
}

#[test]
fn t04_05_write_severalpoints() -> anyhow::Result<()> {
    #[derive(ReadFromTree, Debug)]
    #[oxyroot(slicable)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(ReadFromTree, Debug)]
    struct SeveralPoints {
        #[oxyroot(slicable = "Point")]
        points: Vec<Point>,
    }

    let file = "create_root_files_with_root/t04_05_write_severalpoints.root";

    let tree = RootFile::open(file)?.get_tree("myTree")?;

    for (k, p) in SeveralPoints::from_tree(&tree)?.enumerate() {
        println!("k = {k}, p = {:?}", p);
        for (i, pp) in p.points.iter().enumerate() {
            assert_eq!(pp.x, i as i32);
            assert_eq!(pp.y, (i * i) as i32);
        }
    }
    Ok(())
}
