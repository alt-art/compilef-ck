use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use ahash::AHashMap;

#[derive(Debug)]
enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    LoopStart(usize),
    LoopEnd(usize),
}

fn remove_comments(contents: &str) -> String {
    let mut result = String::new();
    for content in contents.chars() {
        match content {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => {
                result.push(content);
            }
            _ => {}
        }
    }
    result
}

fn create_loop_targets(instructions_str: &str) -> AHashMap<usize, usize> {
    let mut loop_stack = Vec::new();
    let mut targets = AHashMap::new();
    for (index, instruction) in instructions_str.chars().enumerate() {
        match instruction {
            '[' => {
                loop_stack.push(index);
            }
            ']' => {
                let loop_index = loop_stack
                    .pop()
                    .expect(format!("Unmatched ']' at index: {}", index).as_str());
                targets.insert(loop_index, index);
                targets.insert(index, loop_index);
            }
            _ => {}
        }
    }
    targets
}

fn parse_instructions(instructions_str: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let loop_targets = create_loop_targets(instructions_str);
    for (index, instruction_char) in instructions_str.chars().enumerate() {
        match instruction_char {
            '>' => instructions.push(Instruction::IncrementPointer),
            '<' => instructions.push(Instruction::DecrementPointer),
            '+' => instructions.push(Instruction::IncrementValue),
            '-' => instructions.push(Instruction::DecrementValue),
            '.' => instructions.push(Instruction::Output),
            ',' => instructions.push(Instruction::Input),
            '[' => instructions.push(Instruction::LoopStart(
                loop_targets.get(&index).expect("Unmatched '['").clone(),
            )),
            ']' => instructions.push(Instruction::LoopEnd(
                loop_targets.get(&index).unwrap().clone(),
            )),
            _ => {}
        }
    }
    instructions
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./brainfuck/helloworld.bf")?;
    let mut contents = String::new();
    let mut buffer = BufReader::new(file);
    buffer.read_to_string(&mut contents)?;
    dbg!(parse_instructions(&remove_comments(&contents)));
    Ok(())
}
