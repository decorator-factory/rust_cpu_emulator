pub mod common;
mod register;
mod memory;
mod instruction;
mod instruction_parser;

use crate::common::Byte;
use crate::register::Register;
use crate::memory::{Memory, InputMap, OutputMap};
use crate::instruction::{Instruction, Argument, CPUMutation};
use crate::instruction_parser::{InstructionParser, DefaultInstructionParser};
use std::num::Wrapping;
use std::fmt::Display;


#[derive(Debug, Clone, Copy)]
pub enum CPUError {
    InvalidOpcode { adr: Byte, opcode: Byte },
    InvalidArgument { instruction: &'static str, adr: Byte },
}


pub struct CPU {
    is_halted: bool,

    pub ram: Memory,

    reg_a: Byte,
    reg_b: Byte,
    reg_c: Byte,
    reg_d: Byte,
    instruction_pointer: Byte,
    stack_pointer: Byte,

    instruction_parser: Box<dyn InstructionParser>,
}


impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "<CPU A={} B={} C={} D={} IP={} SP={} halted={}>",
            self.reg_a, self.reg_b, self.reg_c, self.reg_d,
            self.instruction_pointer, self.stack_pointer, self.is_halted)
    }
}

impl CPU {
    pub fn new(input_mapping: InputMap, output_mapping: OutputMap) -> CPU {
        let instruction_parser = Box::new(DefaultInstructionParser::new());
        CPU {
            is_halted: false,

            ram: Memory::new(input_mapping, output_mapping),

            reg_a: Wrapping(0),
            reg_b: Wrapping(0),
            reg_c: Wrapping(0),
            reg_d: Wrapping(0),
            instruction_pointer: Wrapping(0),
            stack_pointer: Wrapping(0xd0),

            instruction_parser,
        }
    }

    fn fetch_next_instruction(&self) -> Result<(&Instruction, Vec<Argument>, Byte), CPUError> {
        match self.instruction_parser.parse(
            &self.ram,
            self.instruction_pointer,
        ){
            Some((instruction, ip)) => {
                match instruction.parse_args(&self.ram, ip) {
                    Some((args, ip)) => {
                        Ok((instruction, args, ip))
                    }
                    None => Err(CPUError::InvalidArgument {
                        instruction: instruction.name,
                        adr: self.instruction_pointer
                    })
                }

            },
            None => Err(CPUError::InvalidOpcode {
                adr: self.instruction_pointer,
                opcode: self.ram.read(self.instruction_pointer),
            })
        }
    }

    pub fn fill_ram<'a, I: Iterator<Item=&'a u8>>(&mut self, iterator: I) {
        for (i, byte) in iterator.enumerate() {
            if i > 255 {
                panic!("Iterator is longer than 255 values");
            }
            self.ram.write(Wrapping(i as u8), Wrapping(*byte));
        }
    }

    pub fn run_step(&mut self) -> Result<(), CPUError> {
        let (i, args, ip) = self.fetch_next_instruction()?;
        let mutations = i.run(self, args);
        self.instruction_pointer = ip;
        for mutation in mutations {
            match mutation {
                CPUMutation::WriteRegister(r, value) => {
                    self.write_register(r, value);
                },
                CPUMutation::WriteMemory {adr, value} => {
                    self.ram.write(adr, value);
                },
                CPUMutation::Halt => {
                    self.is_halted = true;
                }
            }
        };
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), CPUError>{
        while !self.is_halted {
            self.run_step()?;
        }
        Ok(())
    }

    pub fn run_with_debug<F1, F2>(
        &mut self,
        on_before_step: F1,
        on_after_step: F2,
    ) -> Result<(), CPUError>
        where F1: FnOnce(&CPU) + Copy,
              F2: FnOnce(&CPU) + Copy
    {
        while !self.is_halted {
            on_before_step(self);
            self.run_step()?;
            on_after_step(self);
        }
        Ok(())
    }

    fn read_register (&self, r: Register) -> Byte {
        match r {
            Register::A => self.reg_a,
            Register::B => self.reg_b,
            Register::C => self.reg_c,
            Register::D => self.reg_d,
            Register::IP => self.instruction_pointer,
            Register::SP => self.stack_pointer,
        }
    }

    fn write_register (&mut self, r: Register, value: Byte) {
        match r {
            Register::A => self.reg_a = value,
            Register::B => self.reg_b = value,
            Register::C => self.reg_c = value,
            Register::D => self.reg_d = value,
            Register::IP => self.instruction_pointer = value,
            Register::SP => self.stack_pointer = value,
        }
    }
}
