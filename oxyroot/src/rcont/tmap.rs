use crate::root::traits;
use crate::rtypes::factory::{Factory, FactoryBuilder};
use crate::rtypes::FactoryItemRead;
use crate::{rbase, Object, RBuffer, Unmarshaler};
use log::trace;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub enum Key {
    String(String),
}

#[derive(Default)]
pub struct TMap {
    name: Option<String>,
    obj: rbase::Object,
    objs: HashMap<Key, Box<dyn FactoryItemRead>>,
}

impl TMap {
    pub fn get<T: 'static>(&self, key: &Key) -> Option<&T> {
        match self.objs.get(key) {
            None => None,
            Some(v) => Some(v.downcast_ref::<T>().unwrap()),
        }
    }
}

impl FactoryBuilder for TMap {
    fn register(factory: &mut Factory) {
        let f = || {
            let v = TMap::default();
            let b: Box<dyn FactoryItemRead> = Box::new(v);
            b
        };

        factory.add("TMap", f);
    }
}

impl Object for TMap {
    fn class(&self) -> &'_ str {
        "TMap"
    }
}

impl traits::Named for TMap {
    fn name(&self) -> &'_ str {
        match &self.name {
            None => "TList",
            Some(s) => s,
        }
    }

    fn title(&self) -> &'_ str {
        "A (key,value) map"
    }
}

impl Unmarshaler for TMap {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        if hdr.vers > 2 {
            r.read_object(&mut self.obj)?;
        }

        if hdr.vers > 1 {
            self.name = Some(r.read_string()?.to_string());
        }

        let nobjs = r.read_i32()?;

        trace!(";TMap.unmarshal.nobjs={}", nobjs);

        for _ in 0..nobjs {
            let key = r.read_object_any_into()?.unwrap();
            trace!(";TMap.unmarshal.key={:?}", key);

            let value = r.read_object_any_into()?.unwrap();

            match key.class() {
                "TObjString" => {
                    let key = key.downcast_ref::<crate::rbase::TObjString>().unwrap();
                    self.objs.insert(Key::String(key.to_string()), value);
                }
                _ => {
                    unimplemented!("TMap.unmarshal: key class={}", key.class());
                }
            }
        }

        Ok(())
    }
}
