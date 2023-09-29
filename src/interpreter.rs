use std::io;
use std::io::prelude::*;

use anyhow::Result;

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

pub fn execute(instructions: &[Instruction]) -> Result<()> {
    let mut memory: [u8; 30000] = [0; 30000];
    let mut pointer = memory.len() / 2;
    let mut index = 0;
    while index < instructions.len() {
        match instructions[index] {
            Instruction::IncrementPointer(value) => {
                let increment = pointer + value;
                if increment > memory.len() - 1 {
                    anyhow::bail!("Pointer overflow");
                }
                pointer = increment;
            }
            Instruction::DecrementPointer(value) => {
                if pointer < value {
                    anyhow::bail!("Pointer underflow");
                }
                pointer -= value;
            }
            Instruction::IncrementValue(value) => {
                memory[pointer] = memory[pointer].wrapping_add(value as u8);
            }
            Instruction::DecrementValue(value) => {
                memory[pointer] = memory[pointer].wrapping_sub(value as u8);
            }
            Instruction::Output => {
                print!("{}", memory[pointer] as char);
            }
            Instruction::Input => {
                memory[pointer] = get_char();
            }
            Instruction::LoopStart {
                target: Some(target),
                ..
            } => {
                if memory[pointer] == 0 {
                    index = target - 1;
                }
            }
            Instruction::LoopEnd {
                target: Some(target),
                ..
            } => {
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
            Instruction::LoopStart { .. } | Instruction::LoopEnd { .. } => {
                unreachable!("LoopStart and LoopEnd should have a target");
            }
        }
        index += 1;
    }
    Ok(())
}
