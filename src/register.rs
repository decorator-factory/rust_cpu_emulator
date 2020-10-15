use std::num::Wrapping;
use crate::common::{Byte, ToBytes};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    D,
    IP,
    SP,
}

impl Register {
    pub fn from_byte(b: Byte) -> Option<Register> {
        match b.0 {
            0x00 => Some(Register::A),
            0x01 => Some(Register::B),
            0x02 => Some(Register::C),
            0x03 => Some(Register::D),
            0x04 => Some(Register::IP),

            0x06 => Some(Register::SP),

            _    => None,
        }
    }
}

impl ToBytes for Register {
    fn to_bytes(self) -> Vec<Byte> {
        match self {
            Register::A => vec![Wrapping(0x00)],
            Register::B => vec![Wrapping(0x01)],
            Register::C => vec![Wrapping(0x02)],
            Register::D => vec![Wrapping(0x03)],
            Register::IP => vec![Wrapping(0x04)],
            Register::SP => vec![Wrapping(0x06)],
        }
    }
}