extern crate core;

// mod as_any;
mod error;
mod gen_factory;
mod rbase;
mod rbytes;
mod rcolors;
mod rcompress;
mod rcont;
mod rdict;
mod riofs;
mod rmeta;
mod root;
mod rtree;
pub mod rtypes;
mod rusty;
mod rvers;
mod utils;

pub use riofs::file;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
