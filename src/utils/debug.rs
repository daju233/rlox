
use crate::Chunk;

pub fn disassemble_chunk(chunk:&mut Chunk, name:&str){
    println!("== {} ==",name);
    let mut offset = 0;
    while offset < chunk.code.len() {
       offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk:&mut Chunk, offset:usize) -> usize{
    print!("{:04} ",offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }
    let instruction:u8 = chunk.code[offset];
    match instruction{
        0 => constant_instruction("OP_CONSTANT", chunk, offset),
        1 => simple_instruction("OP_RETURN", offset),
        _ => {
            println!("Unknown opcode {:?} \n",instruction);
            offset + 1
        }
    }
}

fn simple_instruction(name:&str, offset:usize) -> usize{
    print!("{}\n",name);
    offset + 1
}

fn constant_instruction(name:&str, chunk:&mut Chunk, offset:usize) -> usize{
    let constant = chunk.code[offset + 1];
    print!("{:<16} {:>4} '", name, constant);
    chunk.constants.print_value(chunk.constants.values[constant as usize]);
    print!("'\n");
    return offset + 2
}