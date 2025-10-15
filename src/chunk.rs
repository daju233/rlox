use std::fmt;

use crate::value::{Value, ValueArray};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    OpConstant = 0,
    OpReturn = 1,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Opcode::OpConstant => "OP_CONSTANT",
            Opcode::OpReturn   => "OP_RETURN",
        };
        write!(f, "{}", name)
    }
}

pub struct Chunk{
    pub code:Vec<u8>,
    pub constants:ValueArray,
    pub lines:Vec<usize>,
    pub count:usize,
}

impl Chunk{
    pub fn new() -> Self{
        Self { 
            code:Vec::<u8>::new(),
            constants:ValueArray::new(),
            lines:Vec::<usize>::new(),
            count:0
        }
    }

    pub fn write_chunk(&mut self, byte:u8, line:usize){
        self.code.push(byte);
        self.lines.push(line);
        self.count += 1;
    }

    pub fn add_constant(&mut self, value:Value) -> usize{
        self.constants.values.push(value);
        self.constants.values.len() - 1
    }
}
