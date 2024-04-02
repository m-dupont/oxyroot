use oxyroot::{ReaderTree, RootFile};
use oxyroot_derive::FromRootTree;

#[derive(Debug, FromRootTree)]
struct Test {
    a: i32,
    b: Vec<i32>,
}

// struct OwnIteraror<'a> {
//     // tree: &'a ReaderTree,
//     a: Box<dyn Iterator<Item = i32> + 'a>,
//     b: Box<dyn Iterator<Item = i32> + 'a>,
// }
//
// impl<'a> OwnIteraror<'a> {
//     fn new(t: &'a ReaderTree) -> Self {
//         let a = t.branch("a").unwrap().as_iter::<i32>().unwrap();
//         let a = Box::new(a);
//         let b = t.branch("b").unwrap().as_iter::<i32>().unwrap();
//         let b = Box::new(b);
//         OwnIteraror {
//             // tree: t,
//             a: a,
//             b: b,
//         }
//     }
// }
//
// impl Iterator for OwnIteraror<'_> {
//     type Item = Test;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let a = self.a.next()?;
//         let b = self.b.next()?;
//         Some(Test { a, b })
//     }
// }

fn main() {
    println!("Hello, world!");

    let file = "/tmp/a.root";
    let tree = RootFile::open(file).unwrap().get_tree("tree").unwrap();
    // let ownn = OwnIteraror::new(&tree);
    // for t in ownn {
    //     println!("{:?}", t);
    // }

    for t in Test::from_tree(&tree) {
        println!("{:?}", t);
    }
}
