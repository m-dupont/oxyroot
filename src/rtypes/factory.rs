use crate::rbase;
use crate::rbytes::Unmarshaler2;
use lazy_static::lazy_static;
// use std::any::Any;
use std::collections::HashMap;

// use crate::as_any::{AsAny, Downcast};
use crate::rcont::list::List;
use crate::root::objects;
use crate::root::traits;
use trait_set::trait_set;

use downcast::{downcast, Any};

use anyhow::{bail, Result};

enum Value {
    Tlist(List),
    NotInitialized,
}

/// Types of values stored in the Factory. There are fonction able to instantiate one type of `Box<dyn FactoryItem>`
pub type FactoryBuilderValue = fn() -> Box<dyn FactoryItem>;

trait_set! {
    /// Trait of values stored in the Factory
    pub trait FactoryItem = Any + Unmarshaler2 + traits::Named;
}

downcast!(dyn FactoryItem);

pub trait FactoryBuilder {
    // fn make_factory_builder() -> FactoryBuilderValue;
    // fn make_factory_name() -> &'static str;

    fn register(factory: &mut Factory);
}

pub struct Factory<'a> {
    map: HashMap<&'a str, FactoryBuilderValue>,
}

impl<'a> Factory<'a> {
    pub fn new() -> Factory<'a> {
        Factory {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, s: &'a str, f: FactoryBuilderValue) {
        self.map.insert(s, f);
    }

    pub fn get(&self, s: &'a str) -> Option<&FactoryBuilderValue> {
        self.map.get(s)
    }

    pub fn get_as_box(&self, s: &'a str) -> Option<Box<dyn FactoryItem>> {
        let s = self.get(s);
        if let Some(fct) = s {
            let v = fct();
            let vec: Box<dyn FactoryItem> = v;
            return Some(vec);
        }
        None
    }

    // pub fn unbox()

    pub fn get_as_boxtyped<T>(&self, s: &'a str) -> Result<Box<T>>
    where
        T: FactoryItem,
    {
        if let Some(boxed) = self.get_as_box(s) {
            if let Ok(v) = boxed.downcast::<T>() {
                return Ok(v);
            }

            // let b: bool = (*boxed).is::<T>();
            //
            // if b {
            //     return Some(Box::new((*boxed).downcast_ref::<T>().unwrap()));
            // }

            // let boxed = Box::into_inner(boxed);
            // let boxed = *boxed;
            // if let Some(down) = (*boxed).downcast_ref::<T>() {
            //     *boxed = down;
            //     let vec: Box<&T> =
            //     return Some(boxed);
            // }

            // return Some(Box::new((*boxed).downcast_ref::<T>()));
        }

        bail!("plop")
    }
    //
    pub fn len(&self) -> usize {
        self.map.len()
    }
}

lazy_static! {
    pub static ref FACTORY: Factory<'static> = {
        let mut f = Factory::new();
        // f.add(List::make_factory_name(), List::make_factory_builder());
        List::register(&mut f);

        f
    };
}

// #[macro_export]
// macro_rules! factory_get_box {
//     ( $key:expr  ) => {
//
//     };
// }

#[cfg(test)]
mod tests {
    // use crate::as_any::Downcast;
    use crate::rbytes::Unmarshaler2;
    use crate::rcont::list::List;
    use crate::root::traits;
    use crate::root::traits::{Named, Object};
    use crate::rtypes::factory::{Factory, FactoryItem, FACTORY};
    use std::any::Any;

    #[test]
    fn gen_factory_all_steps() {
        let mut factory = Factory::new();
        let f = || {
            let v: List = List::new();
            let b: Box<dyn FactoryItem> = Box::new(v);
            b
        };
        factory.add("VEC", f);

        let fct = factory.get("VE");
        assert!(fct.is_none());
        let fct = factory.get("VEC");
        assert!(fct.is_some());

        assert_eq!(factory.len(), 1);

        let vec = fct.unwrap();
        let vec = vec();

        let boxf: Box<dyn FactoryItem> = vec;
        let vec = boxf.downcast_ref::<List>();

        assert!(vec.is_ok());

        let vec = boxf.downcast::<List>();

        // let component: dyn FactoryItem = *vec;
        // let vec = component.downcast_ref::<List>();
        // let mut vec: Box<List> = vec.

        // assert_eq!(vec.class().unwrap().as_ref(), "TList");

        // vec.push(43);
        // vec.push(43);
        // vec.push(43);
        //
        // assert_eq!(vec.len(), 3);
    }

    #[test]
    fn factory_get_typed() {
        assert_eq!(FACTORY.len(), 1);
        assert!(FACTORY.get("TList").is_some());
        assert!(FACTORY.get_as_box("TList").is_some());

        assert!(FACTORY.get_as_boxtyped::<List>("TList").is_ok());
        // assert!(FACTORY.get_as_boxtyped::<dyn Unmarshaler2>("TList").is_ok());

        if let Ok(v) = FACTORY.get_as_boxtyped::<List>("TList") {
            assert_eq!(v.class().unwrap(), "TList");
            assert_eq!(v.name(), "TList");
        } else {
        }
    }

    #[test]
    fn factory_static() {
        assert!(FACTORY.get("TList").is_some());
        assert!(FACTORY.get_as_box("TList").is_some());

        // let b: Box<dyn traits::Named> = FACTORY.get_as_box("TList").unwrap();

        assert!((*FACTORY.get_as_box("TList").unwrap())
            .downcast_ref::<List>()
            .is_ok());

        if let Ok(v) = (*FACTORY.get_as_box("TList").unwrap()).downcast_ref::<List>() {
            assert_eq!(v.class().unwrap(), "TList");
            assert_eq!(v.name(), "TList");
        } else {
        }
    }
}
