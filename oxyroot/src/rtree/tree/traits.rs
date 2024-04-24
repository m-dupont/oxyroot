use crate::error::Error::BranchNotFound;
use crate::{BranchName, Marshaler, ReaderTree, Slice, UnmarshalerInto};

pub enum ReadFromTreeResult<T> {
    OneValue(T),
    Slice(Slice<T>),
}

impl<T> ReadFromTreeResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            ReadFromTreeResult::OneValue(v) => v,
            ReadFromTreeResult::Slice(_) => {
                panic!("should not be here ")
            }
        }
    }

    pub fn unwrap_slice(self) -> Slice<T> {
        match self {
            ReadFromTreeResult::OneValue(_v) => panic!("should not be here "),
            ReadFromTreeResult::Slice(b) => b,
        }
    }
}

pub trait ReadFromTree<'a> {
    fn from_branch_tree(
        tree: &'a crate::ReaderTree,
        branch_name: BranchName,
    ) -> crate::Result<impl Iterator<Item = ReadFromTreeResult<Self>>>
    where
        Self: Sized;

    fn from_branch_tree_sliced(
        tree: &'a crate::ReaderTree,
        branch_name: BranchName,
    ) -> crate::Result<impl Iterator<Item = ReadFromTreeResult<Self>>>
    where
        Self: Sized;

    fn from_tree(tree: &'a crate::ReaderTree) -> crate::Result<impl Iterator<Item = Self>>
    where
        Self: Sized,
    {
        Ok(
            Self::from_branch_tree(tree, BranchName::new())?.map(|t| match t {
                ReadFromTreeResult::OneValue(v) => v,
                ReadFromTreeResult::Slice(_) => {
                    panic!("should not be here ")
                }
            }),
        )
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
    ) -> crate::Result<impl Iterator<Item = ReadFromTreeResult<Self>>> {
        let final_branch_name = branch_name.final_name();

        Ok(tree
            .branch(&final_branch_name)
            .ok_or(BranchNotFound {
                name: final_branch_name,
            })?
            .as_iter::<T>()?
            .map(|t| ReadFromTreeResult::OneValue(t)))
    }

    /// this implementation exists to satisfy the compiler whcih need a concrete return type.
    fn from_branch_tree_sliced(
        tree: &'a ReaderTree,
        branch_name: BranchName,
    ) -> crate::Result<impl Iterator<Item = ReadFromTreeResult<Self>>>
    where
        Self: Sized,
    {
        let final_branch_name = branch_name.final_name();
        Ok(tree
            .branch(&final_branch_name)
            .ok_or(BranchNotFound {
                name: final_branch_name,
            })?
            .as_iter::<T>()?
            .map(|t| ReadFromTreeResult::OneValue(t)))
    }
}

// impl<'a, T> ReadFromTree<'a> for Sliced<T>
// where
//     T: ReadFromTree<'a>,
// {
//     fn from_branch_tree(
//         tree: &'a ReaderTree,
//         branch_name: BranchName,
//     ) -> crate::Result<impl Iterator<Item = ReadFromTreeResult<Self>>>
//     where
//         Self: Sized,
//     {
//         let final_branch_name = branch_name.final_name();
//
//         <Slice<T>>::from_tree(tree, branch_name);
//
//         todo!()
//     }
// }

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
