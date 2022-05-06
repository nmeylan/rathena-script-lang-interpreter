use crate::lang::chunk::{Chunk, OpCode};

pub struct Disassemble;

impl Disassemble {
    pub fn print(chunks: &mut Vec<Chunk>) {
        for chunk in chunks.iter_mut() {
            loop {
                let maybe_next_op_code = chunk.next_op_code();
                if maybe_next_op_code.is_none() {
                    break;
                }
                let next_op_code = maybe_next_op_code.unwrap();
                match next_op_code {
                    OpCode::Constant => {
                        let maybe_constant = chunk.next_constant();
                        println!("Constant {}", maybe_constant.unwrap());
                    }
                    OpCode::Pop => {}
                    OpCode::GetIdentifier => {
                        let maybe_constant = chunk.next_identifiers();
                        println!("GetGlobal {}", maybe_constant.unwrap());
                    }
                    OpCode::Equal => {}
                    OpCode::Greater => {}
                    OpCode::Less => {}
                    OpCode::Add => {}
                    OpCode::Subtract => {}
                    OpCode::Multiply => {}
                    OpCode::Divide => {}
                    OpCode::Not => {}
                    OpCode::Jump => {}
                    OpCode::Invoke => {}
                    OpCode::Call => {
                        println!("Call with args: {}", chunk.next_bytes().unwrap());
                    }
                    OpCode::Return => {}
                    OpCode::Command => {}
                    OpCode::DefineIdentifier => {}
                    OpCode::SetIdentifier => {}
                }
            }
            chunk.reset();
        }
    }
}