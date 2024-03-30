use crate::rbytes::{MarshallerKind, MarshallerKindStd};
use crate::rdict::streamers::streamer_types::StreamerSTL;
use crate::rdict::StreamerElement;
use crate::rdict::{Streamer, StreamerInfo};
use crate::rmeta::{ESTLType, EnumNamed};
use crate::Marshaler;
use log::trace;

pub(crate) mod db;
pub(crate) mod streamer_types;
pub(crate) mod streamers_db_gen;

pub(crate) fn make_streamer_for_marshaler_type<T: Marshaler>() -> StreamerInfo {
    let st = match T::kind() {
        MarshallerKind::Primitive => {
            todo!()
        }
        MarshallerKind::Array { .. } => {
            todo!()
        }
        MarshallerKind::Slice { std } => match std {
            MarshallerKindStd::Vector { class_name } => {
                trace!(
                    ";make_streamer_for_marshaler_type.slice.vector:{:?}",
                    T::class_name()
                );
                trace!(
                    ";make_streamer_for_marshaler_type.slice.value:{:?}",
                    class_name
                );
                let ename = T::class_name();
                let etype = EnumNamed::from_string(&class_name).unwrap();

                let mut se =
                    StreamerElement::new(&T::class_name(), EnumNamed::Streamer.into(), 24, 0);
                se.ename = ename;

                let si = StreamerSTL {
                    element: se,
                    vtype: ESTLType::STLvector,
                    ctype: etype.into(),
                };
                Streamer::Stl(si)
            }
        },
        MarshallerKind::String => {
            todo!()
        }
        MarshallerKind::Struct => {
            todo!()
        }
    };

    StreamerInfo::new_from_streamerq(T::class_name(), vec![st])
}
