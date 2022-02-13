use std::io;
use std::io::prelude::*;

use crate::parsing::Instruction;

fn get_char() -> u8 {
    let mut buffer = [0];
    io::stdout().lock().flush().unwrap();
    let mut stdin = io::stdin();
    match stdin.read_exact(&mut buffer) {
        Ok(_) => buffer[0],
        Err(_) => 0,
    }
}

pub fn execute(instructions: &[Instruction]) {
    let mut memory = [0; 30000];
    let mut pointer = 0;
    let mut index = 0;
    while index < instructions.len() {
        match instructions[index] {
            Instruction::IncrementPointer => {
                pointer += 1;
                if pointer >= memory.len() {
                    pointer = 0;
                }
            }
            Instruction::DecrementPointer => {
                if pointer == 0 {
                    pointer = memory.len() - 1;
                } else {
                    pointer -= 1;
                }
            }
            Instruction::IncrementValue => {
                memory[pointer] += 1;
            }
            Instruction::DecrementValue => {
                memory[pointer] -= 1;
            }
            Instruction::Output => {
                print!("{}", memory[pointer] as u8 as char);
            }
            Instruction::Input => {
                memory[pointer] = get_char() as isize;
            }
            Instruction::LoopStart(target) => {
                if memory[pointer] == 0 {
                    index = target;
                }
            }
            Instruction::LoopEnd(target) => {
                if memory[pointer] != 0 {
                    index = target;
                }
            }
        }
        index += 1;
    }
}
