use crate::rbytes::{RVersioner, Unmarshaler};
use lazy_static::lazy_static;
use log::trace;
// use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;

// use crate::as_any::{AsAny, Downcast};

use crate::root::traits;
use trait_set::trait_set;

use crate::Marshaler;
use downcast::{downcast, Any};

use crate::rtypes::Error;
use crate::rtypes::Result;

/// Types of values stored in the Factory. There are fonction able to instantiate one type of `Box<dyn FactoryItem>`
pub type FactoryBuilderValue = fn() -> Box<dyn FactoryItemRead>;

trait_set! {
    /// Trait of values stored in the Factory
    pub trait FactoryItemRead = Any + Unmarshaler + traits::Named;
    pub trait FactoryItemWrite = Any + Marshaler + traits::Named + RVersioner ;
}

impl Marshaler for &dyn FactoryItemWrite {
    fn marshal(&self, w: &mut crate::rbytes::WBuffer) -> crate::rbytes::Result<i64> {
        (*self).marshal(w)
    }
}

downcast!(dyn FactoryItemRead);

impl Debug for Box<dyn FactoryItemRead> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Box<dyn FactoryItemRead>")
            .field("class", &self.class())
            .finish()
    }
}

impl Debug for Box<dyn FactoryItemWrite> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Box<dyn FactoryItemWrite>")
            .field("class", &self.class())
            .finish()
    }
}
impl<'a> Debug for &'a dyn FactoryItemWrite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Box<dyn FactoryItemWrite>")
            .field("class", &self.class())
            .finish()
    }
}

pub trait FactoryBuilder {
    fn register(factory: &mut Factory);
}

#[macro_export]
macro_rules! factory_fn_register_impl {
    (  $t:ty, $n:literal  ) => {
        // impl $t {
        //     pub fn new() -> Self {
        //         Self {
        //             ..Default::default()
        //         }
        //     }
        // }

        impl $crate::root::traits::Object for $t {
            fn class(&self) -> &'_ str {
                $n
            }
        }

        impl $crate::rtypes::factory::FactoryBuilder for $t {
            fn register(factory: &mut $crate::rtypes::factory::Factory) {
                let f = || {
                    let v: Self = Self::default();
                    let b: Box<dyn $crate::rtypes::factory::FactoryItemRead> = Box::new(v);
                    b
                };

                factory.add($n, f);
            }
        }
    };
}

#[macro_export]
macro_rules! factory_all_for_register_impl {
    (  $t:ty, $n:literal  ) => {
        impl $crate::root::traits::Named for $t {}

        $crate::factory_fn_register_impl! {$t, $n}
    };

    (  $t:ty, $n:literal, $vers: expr  ) => {
        impl $crate::rbytes::RVersioner for $t {
            fn rversion(&self) -> i16 {
                $vers
            }
        }

        $crate::factory_all_for_register_impl! {$t, $n}
    };
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
        trace!("FACTORY: add: {}", s);
        let ret = self.map.insert(s, f);

        if ret.is_some() {
            panic!("key '{}' was already in factory", s);
        }
    }

    pub fn get(&self, s: &'a str) -> Result<&FactoryBuilderValue> {
        trace!("get: {}", s);
        self.map
            .get(s)
            .ok_or_else(|| Error::ClassNameNotRegisteredInFactory(s.into()))
    }

    #[allow(dead_code)] // used in tests
    pub fn get_as_box(&self, s: &'a str) -> Option<Box<dyn FactoryItemRead>> {
        let s = self.get(s);
        if let Ok(fct) = s {
            let v = fct();
            let vec: Box<dyn FactoryItemRead> = v;
            return Some(vec);
        }
        None
    }

    #[allow(dead_code)] // used in tests
    pub fn get_as_boxtyped<T>(&self, s: &'a str) -> Result<Box<T>>
    where
        T: FactoryItemRead,
    {
        if let Some(boxed) = self.get_as_box(s) {
            if let Ok(v) = boxed.downcast::<T>() {
                return Ok(v);
            }
        }

        unimplemented!("should not happen")
    }

    #[allow(dead_code)] // used in tests
    pub fn len(&self) -> usize {
        self.map.len()
    }
}

lazy_static! {

    pub static ref FACTORY: Factory<'static> = {
        use crate::rcont::list::ReaderList;
        use crate::rcont::objarray::ReaderObjArray;
        use crate::rdict::StreamerInfo;
        use crate::rdict::streamers::streamer_types::StreamerBase;
        use crate::rdict::streamers::streamer_types::StreamerString;
        use crate::rdict::streamers::streamer_types::StreamerBasicType;
        use crate::rdict::streamers::streamer_types::StreamerObject;
        use crate::rdict::streamers::streamer_types::StreamerObjectPointer;
        use crate::rdict::streamers::streamer_types::StreamerObjectAny;
        use crate::rdict::streamers::streamer_types::StreamerBasicPointer;

        let mut f = Factory::new();
        // f.add(LIST::make_factory_name(), LIST::make_factory_builder());
        ReaderList::register(&mut f);
        ReaderObjArray::register(&mut f);
        StreamerInfo::register(&mut f);
        StreamerBase::register(&mut f);
        StreamerString::register(&mut f);
        StreamerBasicType::register(&mut f);
        StreamerObject::register(&mut f);
        StreamerObjectPointer::register(&mut f);
        StreamerObjectAny::register(&mut f);
        StreamerBasicPointer::register(&mut f);
        crate::rdict::streamers::streamer_types::StreamerSTL::register(&mut f);
        crate::rdict::streamers::streamer_types::StreamerSTLstring::register(&mut f);
        crate::rbase::ObjString::register(&mut f);
        crate::rbase::AttLine::register(&mut f);
        crate::rbase::AttFill::register(&mut f);
        crate::rbase::AttMarker::register(&mut f);
        crate::rtree::tree::Tree::register(&mut f);
        crate::rtree::branch::TBranch::register(&mut f);
        crate::rtree::branch::TBranchElement::register(&mut f);
        crate::rtree::leaf::TLeaf::register(&mut f);
        crate::rtree::leaf::LeafI::register(&mut f);
        crate::rtree::leaf::LeafF::register(&mut f);
        crate::rtree::leaf::LeafD::register(&mut f);
        crate::rtree::leaf::LeafB::register(&mut f);
        crate::rtree::leaf::LeafL::register(&mut f);
        crate::rtree::leaf::LeafO::register(&mut f);
        crate::rtree::leaf::LeafS::register(&mut f);
        crate::rtree::leaf::LeafC::register(&mut f);
        crate::rtree::leaf::LeafElement::register(&mut f);
        crate::rtree::basket::Basket::register(&mut f);


        f
    };
}

#[cfg(test)]
mod tests {
    // use crate::as_any::Downcast;
    use crate::rcont::list::ReaderList;
    use crate::root::traits::{Named, Object};
    use crate::rtypes::factory::{Factory, FactoryItemRead, FACTORY};

    #[test]
    fn gen_factory_all_steps() {
        let mut factory = Factory::new();
        let f = || {
            let v: ReaderList = ReaderList::new();
            let b: Box<dyn FactoryItemRead> = Box::new(v);
            b
        };
        factory.add("VEC", f);

        let fct = factory.get("VE");
        assert!(fct.is_err());
        let fct = factory.get("VEC");
        assert!(fct.is_ok());

        assert_eq!(factory.len(), 1);

        let vec = fct.unwrap();
        let vec = vec();

        let boxf: Box<dyn FactoryItemRead> = vec;
        let vec = boxf.downcast_ref::<ReaderList>();

        assert!(vec.is_ok());

        let vec = boxf.downcast::<ReaderList>();
        assert!(vec.is_ok());

        let vec = vec.unwrap();

        // let component: dyn FactoryItem = *vec;
        // let vec = component.downcast_ref::<LIST>();
        // let mut vec: Box<LIST> = vec.

        // assert_eq!(vec.class().unwrap().as_ref(), "TList");

        // vec.push(43);
        // vec.push(43);
        // vec.push(43);
        //
        // assert_eq!(vec.len(), 3);
    }

    #[test]
    fn factory_get_typed() {
        // assert_eq!(FACTORY.len(), 1);
        assert!(FACTORY.get("TList").is_ok());
        assert!(FACTORY.get_as_box("TList").is_some());

        assert!(FACTORY.get_as_boxtyped::<ReaderList>("TList").is_ok());
        // assert!(FACTORY.get_as_boxtyped::<dyn Unmarshaler2>("TList").is_ok());

        if let Ok(v) = FACTORY.get_as_boxtyped::<ReaderList>("TList") {
            assert_eq!(v.class(), "TList");
            assert_eq!(v.name(), "TList");
        } else {
        }
    }

    #[test]
    fn factory_static() {
        assert!(FACTORY.get("TList").is_ok());
        assert!(FACTORY.get_as_box("TList").is_some());

        // let b: Box<dyn traits::NAMED> = FACTORY.get_as_box("TList").unwrap();

        assert!((*FACTORY.get_as_box("TList").unwrap())
            .downcast_ref::<ReaderList>()
            .is_ok());

        if let Ok(v) = (*FACTORY.get_as_box("TList").unwrap()).downcast_ref::<ReaderList>() {
            assert_eq!(v.class(), "TList");
            assert_eq!(v.name(), "TList");
        } else {
        }
    }
}
