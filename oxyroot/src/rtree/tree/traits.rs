use crate::error::Error::BranchNotFound;
use crate::{BranchName, Marshaler, UnmarshalerInto};

pub trait ReadFromTree<'a> {
    fn from_branch_tree(
        tree: &'a crate::ReaderTree,
        branch_name: BranchName,
    ) -> crate::Result<impl Iterator<Item = Self>>
    where
        Self: Sized;

    fn from_tree(tree: &'a crate::ReaderTree) -> crate::Result<impl Iterator<Item = Self>>
    where
        Self: Sized,
    {
        Self::from_branch_tree(tree, BranchName::new())
    }
}

pub trait WriteToTree {
    fn to_tree(
        it: impl Iterator<Item = Self> + 'static,
        tree: &mut crate::WriterTree,
    ) -> crate::Result<()>
    where
        Self: Sized,
    {
        Self::to_branch_tree(it, tree, None)
    }

    fn to_branch_tree(
        it: impl Iterator<Item = Self> + 'static,
        tree: &mut crate::WriterTree,
        branch_name: Option<&str>,
    ) -> crate::Result<()>
    where
        Self: Sized;
}

impl<'a, T> ReadFromTree<'a> for T
where
    T: UnmarshalerInto<Item = T> + 'a,
{
    fn from_branch_tree(
        tree: &'a crate::ReaderTree,
        branch_name: BranchName,
    ) -> crate::Result<impl Iterator<Item = Self>> {
        let final_branch_name = branch_name.final_name();

        tree.branch(&final_branch_name)
            .ok_or(BranchNotFound {
                name: final_branch_name,
            })?
            .as_iter::<T>()
    }
}

impl<T> WriteToTree for T
where
    T: Marshaler + 'static,
{
    fn to_branch_tree(
        it: impl Iterator<Item = T> + 'static,
        tree: &mut crate::WriterTree,
        branch_name: Option<&str>,
    ) -> crate::Result<()> {
        tree.new_branch(branch_name.unwrap(), it);
        Ok(())
    }
}
