pub mod traits {
    // Object represents a ROOT object
    pub trait Object {
        fn class(&self) -> Option<String> {
            None
        }
    }

    pub trait Named: Object {
        fn name(&self) -> &'_ str;
        fn title(&self) -> &'_ str;
    }
}

pub mod objects {
    use crate::root::traits;

    pub type Object = Box<dyn traits::Object + Sync>;
}
