use crate::rbytes::Marshaler;
use crate::riofs::Result;
use crate::rtree::basket::Basket;
use crate::rtree::branch::tbranch::{DEFAULT_BASKET_SIZE, DEFAULT_MAX_BASKETS};
use crate::rtree::branch::TBranch;
use crate::rtree::leaf::Leaf;
use crate::rtree::streamer_type::rust_type_to_root_type_code;
use crate::rtree::tree::WriterTree;
use crate::rtree::wbasket::{BasketBytesWritten, WBasket};
use crate::{Branch, RootFile};
use log::trace;
use std::any;
use std::fmt::Debug;
use std::marker::PhantomData;

pub struct WBranch<T>
where
    T: Marshaler,
{
    branch: Branch,
    iterator: Box<dyn Iterator<Item = T>>,
    basket: Option<WBasket<T>>,
    write_basket: i32,
}

impl<T> Debug for WBranch<T>
where
    T: Marshaler,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = match &self.basket {
            None => None,
            Some(b) => Some(&b.basket),
        };
        f.debug_struct("WBranch")
            .field("branch", &self.branch)
            .field("wbasket", &b)
            .finish()
    }
}

impl<T> WBranch<T>
where
    T: Marshaler + Debug + 'static,
{
    pub fn new(
        name: String,
        it: Box<dyn Iterator<Item = T>>,
        tree: &WriterTree<T>,
        f: &RootFile,
    ) -> Self {
        trace!(";WBranch.new.name:{:?}", name);
        trace!(";WBranch.new.code:{:?}", rust_type_to_root_type_code::<T>());

        let mut branch = TBranch::new(name.clone());
        branch.iobits = tree.iobits;
        branch.basket_size = DEFAULT_BASKET_SIZE;
        branch.max_baskets = DEFAULT_MAX_BASKETS;
        branch.basket_entry.push(0);
        let leaf = Leaf::new::<T>(&branch);
        branch.named.title = format!("{}/{}", name, rust_type_to_root_type_code::<T>());

        trace!("WBranch.new.leaf:{:?}", leaf);

        let mut branch = Self {
            branch: Branch::Base(branch),
            iterator: it,
            basket: None,
            write_basket: 0,
        };
        branch.basket = Some(branch.create_new_basket(tree, f));
        trace!("WBranch.new.branch:{:?}", branch);
        branch
    }

    pub fn write(&mut self) -> Option<()> {
        match self.iterator.next() {
            Some(item) => {
                trace!("WBranch.write.item:{:?}", item);
                // self.branch.write(item);
                Some(())
            }
            None => None,
        }
    }

    fn create_new_basket(&mut self, tree: &WriterTree<T>, f: &RootFile) -> WBasket<T> {
        trace!(";WBranch.create_new_basket.call:{:?}", true);
        trace!(
            ";WBranch.create_new_basket.b.write_basket:{:?}",
            self.branch.tbranch().write_basket
        );
        let cycle = self.branch.tbranch().write_basket as i16 + 1;
        let basket_size = self.branch.tbranch().basket_size;
        let basket = Basket::new_from_branch(&self.branch, cycle, basket_size, 0, tree, f);
        trace!(";WBranch.create_new_basket.basket:{:?}", basket);
        let n = self.branch.tbranch().write_basket;
        if n > self.branch.tbranch().max_baskets {
            self.branch.tbranch().max_baskets = n;
        }
        trace!(
            ";WBranch.create_new_basket.b.max_baskets:{:?}",
            self.branch.tbranch().max_baskets
        );
        WBasket::new(basket)
    }

    pub(crate) fn flush(&mut self, file: &mut RootFile) -> crate::riofs::Result<()> {
        trace!(";WBranch.flush.call:{:?}", true);
        let basket = self.basket.as_mut().unwrap();
        let b = basket.write_to_file(file)?;
        trace!(";WBranch.flush.basket_bytes_writter:{:?}", b);
        let BasketBytesWritten {
            tot_bytes,
            zip_bytes,
        } = b;
        self.branch.tbranch().tot_bytes += tot_bytes;
        self.branch.tbranch().zip_bytes += zip_bytes;

        self.branch
            .tbranch()
            .basket_bytes
            .push(basket.basket.key().n_bytes());

        let n = self.branch.tbranch().entries();
        self.branch.tbranch().basket_entry.push(n);
        self.branch
            .tbranch()
            .basket_seek
            .push(basket.basket.key().seek_key());

        trace!(
            ";WBranch.flush.basket_bytes:{:?}",
            self.branch.tbranch().basket_bytes
        );

        trace!(
            ";WBranch.flush.basket_entry:{:?}",
            self.branch.tbranch().basket_entry
        );

        trace!(
            ";WBranch.flush.basket_seek:{:?}",
            self.branch.tbranch().basket_seek
        );

        self.write_basket += 1;
        Ok(())
    }
}
