use std::num::Wrapping;

use crate::common::{Byte, ToBytes};
use crate::register::Register;
use crate::memory::Memory;
use crate::CPU;


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CPUMutation {
    WriteRegister(Register, Byte),
    WriteMemory {adr: Byte, value: Byte},
    Halt,
}


#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum ArgType {
    Constant,
    Register,
}

impl ArgType {
    fn parse(&self, b: Byte) -> Option<Argument>{
        match self {
            ArgType::Constant => Some(Argument::Constant(b)),
            ArgType::Register => Register::from_byte(b).map(Argument::Register),
        }
    }
}


#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Argument {
    Constant(Byte),
    Register(Register),
}

impl ToBytes for Argument {
    fn to_bytes(self) -> Vec<Byte> {
        match self {
            Argument::Constant(b) => vec![b],
            Argument::Register(r) => r.to_bytes(),
        }
    }
}



pub type InstructionAction = Box<dyn Fn(&CPU, Vec<Argument>) -> Vec<CPUMutation>>;

pub struct Instruction {
    pub name: &'static str,
    arg_types: Vec<ArgType>,
    action: InstructionAction,
}

impl Instruction {
    pub fn new(name: &'static str, arg_types: Vec<ArgType>, action: InstructionAction) -> Instruction {
        Instruction {name, arg_types, action}
    }

    pub fn parse_args(&self, memory: &Memory, ip: Byte) -> Option<(Vec<Argument>, Byte)> {
        let mut args = Vec::new();
        let mut ip = ip;
        for argt in &self.arg_types {
            let parsed = argt.parse(memory.read(ip));
            match parsed {
                None => return None,
                Some(arg) => args.push(arg),
            }
            ip += Wrapping(1)
        }
        Some((args, ip))
    }

    pub fn run(&self, cpu: &CPU, args: Vec<Argument>) -> Vec<CPUMutation> {
        (self.action)(cpu, args)
    }
}