use std::num::Wrapping;
use std::collections::HashMap;

use crate::common::Byte;
use crate::memory::Memory;
use crate::instruction::{Instruction, Argument as Arg, ArgType, CPUMutation, InstructionAction};


pub struct DefaultInstructionParser {
    instructions: HashMap<Byte, Instruction>
}


impl DefaultInstructionParser {
    pub fn new() -> DefaultInstructionParser {
        let instructions = HashMap::new();
        let mut this = DefaultInstructionParser { instructions };

        this.add_instruction(
            0x00,
            "NUM",
            vec![ArgType::Register, ArgType::Constant],
            Box::new(|_, args| {
                match (args[0], args[1]) {
                    (Arg::Register(r), Arg::Constant(b)) =>
                        vec![CPUMutation::WriteRegister(r, b)],
                    _ => panic!("There's a bug in the argument parsing")
                }
            })
        );

        this.add_instruction(
            0x01,
            "MOV",
            vec![ArgType::Register, ArgType::Register],
            Box::new(|cpu, args| {
                match (args[0], args[1]) {
                    (Arg::Register(dest), Arg::Register(src)) => {
                        vec![CPUMutation::WriteRegister(dest, cpu.read_register(src))]
                    },
                    _ => panic!("There's a bug in the argument parsing")
                }
            })
        );

        this.add_instruction(
            0x02,
            "ADD",
            vec![ArgType::Register, ArgType::Register],
            Box::new(|cpu, args| {
                match (args[0], args[1]) {
                    (Arg::Register(dest), Arg::Register(src)) => {
                        let value = cpu.read_register(dest) + cpu.read_register(src);
                        vec![CPUMutation::WriteRegister(dest, value)]
                    },
                    _ => panic!("There's a bug in the argument parsing")
                }
            })
        );

        this.add_instruction(
            0x03,
            "SUB",
            vec![ArgType::Register, ArgType::Register],
            Box::new(|cpu, args| {
                match (args[0], args[1]) {
                    (Arg::Register(dest), Arg::Register(src)) => {
                        let value = cpu.read_register(dest) - cpu.read_register(src);
                        vec![CPUMutation::WriteRegister(dest, value)]
                    },
                    _ => panic!("There's a bug in the argument parsing")
                }
            })
        );

        this.add_instruction(
            0xff,
            "HLT",
            vec![],
            Box::new(|_, _|
                vec![CPUMutation::Halt]
            )
        );

        this.add_instruction(
            0x30,
            "SET",
            vec![ArgType::Constant, ArgType::Register],
            Box::new(|cpu, args| {
                match (args[0], args[1]) {
                    (Arg::Constant(adr), Arg::Register(src)) => {
                        let value = cpu.read_register(src);
                        vec![CPUMutation::WriteMemory{adr, value}]
                    },
                    _ => panic!("There's a bug in the argument parsing")
                }
            })
        );

        this
    }

    fn add_instruction(
        &mut self,
        opcode: u8,
        name: &'static str,
        arg_types: Vec<ArgType>,
        action: InstructionAction
    ) {
        self.instructions.insert(
            Wrapping(opcode),
            Instruction::new(name, arg_types, action)
        );
    }
}

impl InstructionParser for DefaultInstructionParser {
    fn parse(&self, memory: &Memory, ip: Byte) -> Option<(&Instruction, Byte)> {
        let instruction_byte = memory.read(ip);
        self
            .instructions
            .get(&instruction_byte)
            .map(|i| (i, ip + Wrapping(1)))
    }
}



pub trait InstructionParser {
    // `parse` attempts to grab a new instruction from memory
    // if it succeeds, it returns the parsed instruction and the new I.P.
    fn parse(&self, memory: &Memory, ip: Byte) -> Option<(&Instruction, Byte)>;
}