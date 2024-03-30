use crate::rbytes::wbuffer::WBuffer;
use crate::rbytes::{Marshaler, MarshallerKind, RVersioner};
use crate::rdict::streamers::make_streamer_for_marshaler_type;
use crate::rtree::basket::Basket;
use crate::rtree::branch::tbranch::{DEFAULT_BASKET_SIZE, DEFAULT_MAX_BASKETS};
use crate::rtree::branch::{TBranch, TBranchElement};
use crate::rtree::leaf::Leaf;
use crate::rtree::tree::WriterTree;
use crate::rtree::wbasket::{BasketBytesWritten, WBasket};
use crate::{rvers, Branch, Named, Object, RootFile};
use log::trace;
use std::fmt::Debug;

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
        let b = self.basket.as_ref().map(|b| &b.basket);
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
        name: &str,
        it: impl Iterator<Item = T> + 'static,
        tree: &mut WriterTree,
    ) -> Self
    where
        U: Marshaler,
    {
        trace!(";WBranch.new.name:{:?}", name);
        trace!(";WBranch.new.code:{:?}", U::root_code());

        let mut tbanch = TBranch::new(name.to_string());

        tbanch.iobits = tree.iobits;
        // branch.compress = f.compression();
        tbanch.basket_size = DEFAULT_BASKET_SIZE;
        tbanch.max_baskets = DEFAULT_MAX_BASKETS;
        tbanch.basket_entry.push(0);

        tbanch.named.title = format!("{}/{}", name, U::root_code());

        let leaf = Leaf::new::<U>(&tbanch);
        trace!("WBranch.new.leaf:{:?}", leaf);
        trace!(";WBranch.new.title:{:?}", &tbanch.named.title);

        let branch = match U::kind() {
            MarshallerKind::Primitive => Branch::Base(tbanch),

            MarshallerKind::Slice { .. } => {
                tbanch.entry_offset_len = 1000;
                let class_name = U::class_name();
                trace!(";WBranch.new.class_name:{:?}", U::class_name());
                let streamer = make_streamer_for_marshaler_type::<U>();
                tree.add_streamer(streamer);

                let branch = TBranchElement::new(class_name, tbanch);

                Branch::Element(branch)
            }
            MarshallerKind::String => {
                tbanch.entry_offset_len = 1000;
                Branch::Base(tbanch)
            }
            MarshallerKind::Struct => {
                todo!()
            }
            MarshallerKind::Array { .. } => Branch::Base(tbanch),
        };

        trace!(";WBranch.new.rust_type_to_kind:{:?}", U::kind());

        let mut branch = Self {
            branch,
            iterator: Box::new(it),
            basket: None,
        };
        // branch.basket = Some(branch.create_new_basket(tree, None));
        branch.branch.tbranch_mut().leaves.push(leaf);
        trace!("WBranch.new.branch:{:?}", branch);
        branch
    }

    pub fn write(
        &mut self,
        tree: &WriterTree,
        file: &mut RootFile,
    ) -> crate::riofs::Result<Option<i32>> {
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
        let ret = match self.iterator.next() {
            Some(item) => {
                // trace!(";WBranch.write.{ident}.item:{:?}", item);
                // self.branch.write(item);
                tbranch.entries += 1;
                tbranch.entry_number += 1;

                let sz_old = basket.wbuf.len();
                trace!(";WBranch.write.{ident}.sz_old:{:?}", sz_old);
                basket.update(sz_old as i64)?;

                assert_eq!(tbranch.leaves.len(), 1);

                for leave in tbranch.leaves.iter_mut() {
                    leave.write_to_buffer(&mut basket.wbuf, &item)?;
                }

                // basket.wbuf.write_object(&item).unwrap();
                let sz_new = basket.wbuf.len();
                trace!(";WBranch.write.{ident}.sz_new:{:?}", sz_new);
                let n = (sz_new - sz_old) as i32;
                if n > basket.basket.nev_size {
                    basket.basket.nev_size = n;
                }

                if sz_new + n as usize >= tbranch.basket_size as usize {
                    self.flush(file)?;
                    self.basket = Some(self.create_new_basket(tree, file));
                }

                Some(n)
            }
            None => None,
        };
        Ok(ret)
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
        let _len = w.len() - 1;
        trace!(";WBranch.marshal.buf.pos:{:?}", w.pos());
        match &self.branch {
            Branch::Base(tb) => tb.marshal(w),
            Branch::Element(te) => te.marshal(w),
        }

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
