use ahash::AHashMap;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    LoopStart(usize),
    LoopEnd(usize),
}

type Token = (char, (usize, usize));

fn lexer(contents: &str) -> Vec<Token> {
    let mut result = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        for (char_index, content) in line.chars().enumerate() {
            match content {
                '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => {
                    result.push((content, (line_index + 1, char_index + 1)));
                }
                _ => {}
            }
        }
    }
    result
}

fn create_loop_targets(tokens: &[Token]) -> Result<AHashMap<usize, usize>> {
    let mut loop_stack = Vec::new();
    let mut targets = AHashMap::new();
    for (index, instruction) in tokens.iter().enumerate() {
        match instruction {
            ('[', _) => {
                loop_stack.push(index);
            }
            (']', location) => {
                let loop_index = loop_stack.pop().ok_or_else(|| {
                    anyhow!("Unmatched ']' at index {}:{}", location.0, location.1)
                })?;
                targets.insert(loop_index, index);
                targets.insert(index, loop_index);
            }
            _ => {}
        }
    }
    Ok(targets)
}

pub fn parse_instructions(instructions_str: &str) -> Result<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let tokens = lexer(instructions_str);
    let loop_targets = create_loop_targets(&tokens)?;
    for (index, instruction_char) in tokens.iter().enumerate() {
        match instruction_char {
            ('>', _) => instructions.push(Instruction::IncrementPointer),
            ('<', _) => instructions.push(Instruction::DecrementPointer),
            ('+', _) => instructions.push(Instruction::IncrementValue),
            ('-', _) => instructions.push(Instruction::DecrementValue),
            ('.', _) => instructions.push(Instruction::Output),
            (',', _) => instructions.push(Instruction::Input),
            ('[', location) => instructions.push(Instruction::LoopStart(
                *loop_targets.get(&index).ok_or_else(|| {
                    anyhow!("Unmatched '[' at index {}:{}", location.0, location.1)
                })?,
            )),
            (']', _) => instructions.push(Instruction::LoopEnd(*loop_targets.get(&index).unwrap())),
            _ => {}
        }
    }
    Ok(instructions)
}
