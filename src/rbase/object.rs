pub struct Object {
    id: u32,
    bits: u32,
}

impl Default for Object {
    fn default() -> Self {
        Object {
            id: 0x0,
            bits: 0x3000000,
        }
    }
}
