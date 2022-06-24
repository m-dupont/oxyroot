pub mod traits {
    // OBJECT represents a ROOT object
    pub trait Object {
        fn class(&self) -> &'_ str;
    }

    pub trait Named: Object {
        fn name(&self) -> &'_ str {
            unimplemented!()
        }
        fn title(&self) -> &'_ str {
            unimplemented!()
        }
    }

    pub trait List: Object {}
}

pub mod objects {
    use crate::root::traits;

    pub type Object = Box<dyn traits::Object + Sync>;
}
