use std::num::Wrapping;
use crate::common::Byte;


pub type InputMap = Box<dyn Fn(Byte) -> Option<Byte>>;
pub type OutputMap = Box<dyn Fn(Byte, Byte) -> Option<()>>;


pub struct Memory {
    data: [Byte; 256],
    input_mapping: InputMap,
    output_mapping: OutputMap,
}

impl Memory {
    pub fn new(input_mapping: InputMap, output_mapping: OutputMap) -> Memory {
        Memory {
            data: [Wrapping(0); 256],
            input_mapping,
            output_mapping
        }
    }

    pub fn read(&self, adr: Byte) -> Byte {
        if let Some(b) = (self.input_mapping)(adr) {
            b
        } else {
            self.data[adr.0 as usize]
        }
    }

    pub fn write(&mut self, adr: Byte, value: Byte) {
        if let None = (self.output_mapping)(adr, value) {
            self.data[adr.0 as usize] = value;
        }
    }
}