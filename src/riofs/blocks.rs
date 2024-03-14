use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;

#[derive(Default)]
pub struct FreeSegments {
    pub(crate) first: i64,
    // first free word of segment
    pub(crate) last: i64, // last free word of segment
}

impl FreeSegments {
    pub(crate) fn new(first: i64, last: i64) -> Self {
        FreeSegments { first, last }
    }
}

impl Unmarshaler for FreeSegments {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let vers = r.read_i16()?;

        let is_big_file = vers > 1000;
        let first = if is_big_file {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };
        let last = if is_big_file {
            r.read_i64()?
        } else {
            r.read_i32()? as i64
        };

        self.first = first;
        self.last = last;
        Ok(())
    }
}

#[derive(Default)]
pub struct FreeList(Vec<FreeSegments>);

impl FreeList {
    pub(crate) fn vec(&mut self) -> &mut Vec<FreeSegments> {
        &mut self.0
    }
    pub fn append(&mut self, seg: FreeSegments) {
        self.0.push(seg)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
