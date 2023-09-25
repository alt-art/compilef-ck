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
    let mut memory: [u8; 30000] = [0; 30000];
    let mut pointer = memory.len() / 2;
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
                if memory[pointer] == 255 {
                    memory[pointer] = 0;
                } else {
                    memory[pointer] += 1;
                }
            }
            Instruction::DecrementValue => {
                if memory[pointer] == 0 {
                    memory[pointer] = 255;
                } else {
                    memory[pointer] -= 1;
                }
            }
            Instruction::Output => {
                print!("{}", memory[pointer] as char);
            }
            Instruction::Input => {
                memory[pointer] = get_char();
            }
            Instruction::LoopStart(target) => {
                if memory[pointer] == 0 {
                    index = target - 1;
                }
            }
            Instruction::LoopEnd(target) => {
                if memory[pointer] != 0 {
                    index = target - 1;
                }
            }
            Instruction::Debug => {
                println!();
                println!("--- DEBUG ---");
                println!("pointer: {pointer}");
                println!("memory: {}", memory[pointer]);
                println!("--- DEBUG ---");
            }
        }
        index += 1;
    }
}
