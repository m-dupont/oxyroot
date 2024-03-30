pub mod traits {
    // OBJECT represents a ROOT object
    pub trait Object {
        fn class(&self) -> &'_ str;
    }

    pub(crate) fn tstring_sizeof(v: &str) -> i32 {
        let n = v.len() as i32;
        if n > 254 {
            n + 1 + 4
        } else {
            n + 1
        }
    }

    pub(crate) fn datime_sizeof() -> i32 {
        4
    }

    pub trait Named: Object {
        fn name(&self) -> &'_ str {
            unimplemented!()
        }
        fn title(&self) -> &'_ str {
            unimplemented!()
        }

        fn size_of(&self) -> i32 {
            tstring_sizeof(self.name()) + tstring_sizeof(self.title())
        }
    }
}

pub mod objects {
    use crate::root::traits;

    pub type Object = Box<dyn traits::Object + Sync>;
}
