use std::num::Wrapping;

pub type Byte = Wrapping<u8>;

pub trait ToBytes {
    fn to_bytes(self) -> Vec<Byte>;
}