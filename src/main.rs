use crate::{chunk::Chunk, utils::debug::disassemble_chunk};

mod chunk;
mod utils;
mod common;
mod value;
fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(chunk::Opcode::OpConstant as u8, 123);
    chunk.write_chunk(constant as u8, 123);
    chunk.write_chunk(chunk::Opcode::OpReturn as u8, 123);
    disassemble_chunk(&mut chunk, "test_chunk");
}
