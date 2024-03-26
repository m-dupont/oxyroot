use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{Marshaler, MarshallerKind, RVersioner};
use crate::riofs::Result;
use crate::rtree::basket::Basket;
use crate::rtree::branch::tbranch::{DEFAULT_BASKET_SIZE, DEFAULT_MAX_BASKETS};
use crate::rtree::branch::TBranch;
use crate::rtree::leaf::Leaf;
use crate::rtree::streamer_type::rust_type_to_root_type_code;
use crate::rtree::tree::WriterTree;
use crate::rtree::wbasket::{BasketBytesWritten, WBasket};
use crate::{rvers, Branch, Named, Object, RootFile};
use log::trace;
use std::any;
use std::fmt::{format, Debug};
use std::marker::PhantomData;

pub struct WBranch<T>
where
    T: Marshaler,
{
    branch: Branch,
    iterator: Box<dyn Iterator<Item = T>>,
    basket: Option<WBasket>,
}

impl<T> WBranch<T>
where
    T: Marshaler,
{
    pub fn branch(&self) -> &Branch {
        &self.branch
    }
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
    T: Marshaler + 'static,
{
    pub(crate) fn new<U: 'static>(
        name: String,
        it: impl Iterator<Item = T> + 'static,
        tree: &mut WriterTree,
    ) -> Self
    where
        U: Marshaler,
    {
        trace!(";WBranch.new.name:{:?}", name);
        trace!(";WBranch.new.code:{:?}", rust_type_to_root_type_code::<U>());

        let mut branch = TBranch::new(name.clone());

        branch.iobits = tree.iobits;
        // branch.compress = f.compression();
        branch.basket_size = DEFAULT_BASKET_SIZE;
        branch.max_baskets = DEFAULT_MAX_BASKETS;
        branch.basket_entry.push(0);

        branch.named.title = format!("{}/{}", name, rust_type_to_root_type_code::<U>());

        match U::kind() {
            MarshallerKind::Primitive => {}
            MarshallerKind::Array => {
                todo!()
            }
            MarshallerKind::Slice => {
                todo!()
            }
            MarshallerKind::String => branch.entry_offset_len = 1000,
            MarshallerKind::Struct => {
                todo!()
            }
        }

        trace!(";WBranch.new.rust_type_to_kind:{:?}", U::kind());
        trace!(";WBranch.new.title:{:?}", branch.named.title);
        let leaf = Leaf::new::<U>(&branch);
        trace!("WBranch.new.leaf:{:?}", leaf);
        let mut branch = Self {
            branch: Branch::Base(branch),
            iterator: Box::new(it),
            basket: None,
        };
        // branch.basket = Some(branch.create_new_basket(tree, None));
        branch.branch.tbranch_mut().leaves.push(leaf);
        trace!("WBranch.new.branch:{:?}", branch);
        branch
    }

    pub fn write(&mut self, tree: &WriterTree, file: &mut RootFile) -> Option<i32> {
        // trace!(";WBranch.write.call:{:?}", true);
        let basket = match &mut self.basket {
            None => {
                self.basket = Some(self.create_new_basket(tree, file));
                self.basket.as_mut().unwrap() // safe because we juste created it
            }
            Some(b) => b,
        };

        let ident = format!("{}.a{}", self.branch.name(), self.branch.tbranch().entries);
        let tbranch = self.branch.tbranch_mut();
        match self.iterator.next() {
            Some(item) => {
                // trace!(";WBranch.write.{ident}.item:{:?}", item);
                // self.branch.write(item);
                tbranch.entries += 1;
                tbranch.entry_number += 1;

                let szOld = basket.wbuf.len();
                trace!(";WBranch.write.{ident}.szOld:{:?}", szOld);
                basket.update(szOld as i64).unwrap();

                assert_eq!(tbranch.leaves.len(), 1);

                for leave in tbranch.leaves.iter_mut() {
                    leave.write_to_buffer(&mut basket.wbuf, &item).unwrap();
                }

                // basket.wbuf.write_object(&item).unwrap();
                let szNew = basket.wbuf.len();
                trace!(";WBranch.write.{ident}.szNew:{:?}", szNew);
                let n = (szNew - szOld) as i32;
                if n > basket.basket.nev_size {
                    basket.basket.nev_size = n;
                }

                if szNew + n as usize >= tbranch.basket_size as usize {
                    self.flush(file).unwrap();
                    self.basket = Some(self.create_new_basket(tree, file));
                }

                Some(n)
            }
            None => None,
        }
    }

    fn create_new_basket(&mut self, tree: &WriterTree, f: &RootFile) -> WBasket {
        trace!(";WBranch.create_new_basket.call:{:?}", true);
        trace!(
            ";WBranch.create_new_basket.b.write_basket:{:?}",
            self.branch.tbranch_mut().write_basket
        );
        let cycle = self.branch.tbranch_mut().write_basket as i16 + 1;
        let basket_size = self.branch.tbranch_mut().basket_size;
        let offset_len = self.branch.tbranch_mut().entry_offset_len;
        let basket = Basket::new_from_branch(&self.branch, cycle, basket_size, offset_len, tree, f);
        trace!(";WBranch.create_new_basket.basket:{:?}", basket);
        let n = self.branch.tbranch_mut().write_basket;
        if n > self.branch.tbranch_mut().max_baskets {
            self.branch.tbranch_mut().max_baskets = n;
        }
        trace!(
            ";WBranch.create_new_basket.b.max_baskets:{:?}",
            self.branch.tbranch_mut().max_baskets
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
        self.branch.tbranch_mut().tot_bytes += tot_bytes;
        self.branch.tbranch_mut().zip_bytes += zip_bytes;

        self.branch
            .tbranch_mut()
            .basket_bytes
            .push(basket.basket.key().n_bytes());

        let n = self.branch.tbranch_mut().entries();
        self.branch.tbranch_mut().basket_entry.push(n);
        self.branch
            .tbranch_mut()
            .basket_seek
            .push(basket.basket.key().seek_key());

        trace!(
            ";WBranch.flush.basket_bytes:{:?}",
            self.branch.tbranch_mut().basket_bytes
        );

        trace!(
            ";WBranch.flush.basket_entry:{:?}",
            self.branch.tbranch_mut().basket_entry
        );

        trace!(
            ";WBranch.flush.basket_seek:{:?}",
            self.branch.tbranch_mut().basket_seek
        );

        self.branch.tbranch_mut().write_basket += 1;
        Ok(())
    }
}

impl<T> Marshaler for WBranch<T>
where
    T: Marshaler,
{
    fn marshal(&self, w: &mut WBuffer) -> crate::rbytes::Result<i64> {
        let len = w.len() - 1;
        trace!(";WBranch.marshal.buf.pos:{:?}", w.pos());
        let b = self.branch.tbranch();
        b.marshal(w)

        // todo!()
    }
}

impl<T> Object for WBranch<T>
where
    T: Marshaler,
{
    fn class(&self) -> &'_ str {
        self.branch.class()
    }
}

impl<T> Named for WBranch<T>
where
    T: Marshaler,
{
    fn name(&self) -> &'_ str {
        self.branch.name()
    }
}

impl<T> RVersioner for WBranch<T>
where
    T: Marshaler,
{
    fn rversion(&self) -> i16 {
        rvers::BRANCH
    }
}
