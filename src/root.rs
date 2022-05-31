pub mod traits {
    // Object represents a ROOT object
    pub trait Object {
        fn class(&self) -> Option<String> {
            None
        }
    }
}

pub mod objects {
    use crate::root::traits;

    pub type Object = Box<dyn traits::Object + Sync>;
}
