use oxyroot::WriteToTree;
use oxyroot::{RootFile, WriterTree};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
// use std::sync::mpsc::Receiver;

use oxyroot::rtree::StateCallBack;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct MyStruct {
    a: i32,
    b: i16,
}

struct Channel<T> {
    current: Rc<RefCell<Option<T>>>,
}

struct Sender<T> {
    channel: Channel<T>,
}

impl<T> Sender<T> {
    fn send(&self, value: Option<T>) {
        let mut current = self.channel.current.borrow_mut();
        *current = value;
    }
}

struct Receiver<T> {
    channel: Channel<T>,
}

fn make_channel<T>() -> (Sender<T>, Receiver<T>) {
    let current = Rc::new(RefCell::new(None));
    let channel = Channel {
        current: current.clone(),
    };

    let sender = Sender { channel: channel };
    let receiver = Receiver {
        channel: Channel {
            current: current.clone(),
        },
    };
    (sender, receiver)
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.channel.current.borrow_mut();
        let ret = current.take();
        ret
    }
}

struct OwnIterator {
    // it: Box<dyn Iterator<Item = MyStruct>>,
    b: Option<i16>,
    current: Option<MyStruct>,
}

impl WriteToTree for MyStruct {
    fn to_branch_tree(
        mut it: impl Iterator<Item = Self> + 'static,
        tree: &mut WriterTree,
        branch_name: Option<&str>,
    ) -> oxyroot::Result<()> {
        let (send_a, recv_a) = make_channel();

        let func = move |s: StateCallBack| {
            println!("OwnIterator: s = {:?}", s);

            match s {
                StateCallBack::Before => {
                    match it.next() {
                        None => {
                            send_a.send(None);
                        }
                        Some(a) => {
                            println!("a = {:?}", a);

                            let MyStruct { a, b } = a;
                            send_a.send(Some(a));
                        }
                    };
                }
                StateCallBack::Branch(_) => {}
                StateCallBack::After => {}
            }
        };

        tree.new_branch("a", recv_a.into_iter());

        tree.add_callback(Box::new(func));
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let mut f = RootFile::create("/tmp/a.root")?;
    let mut tree = oxyroot::WriterTree::new("tree");

    let structs = vec![
        MyStruct { a: 1, b: 2 },
        MyStruct { a: 3, b: 4 },
        MyStruct { a: 5, b: 6 },
    ];

    let it_structs = structs.into_iter();

    MyStruct::to_tree(it_structs, &mut tree)?;
    // ito.register_callback(&mut tree);

    let it = (0..10);

    let func = |s: StateCallBack| {
        println!("s = {:?}", s);
    };
    // tree.add_callback(func);

    tree.new_branch("it", it);
    tree.write(&mut f)?;

    f.close()?;

    // tree.new_branch("mystruct", structs.into_iter());

    Ok(())
}
