use ahash::AHashMap;
use anyhow::{anyhow, Result};
use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

#[derive(Debug, Clone, Copy)]
pub struct Location(usize, usize);

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

#[derive(Debug)]
pub enum Instruction {
    IncrementPointer(usize),
    DecrementPointer(usize),
    IncrementValue(usize),
    DecrementValue(usize),
    Output,
    Input,
    Debug,
    LoopStart {
        target: Option<usize>,
        location: Location,
    },
    LoopEnd {
        target: Option<usize>,
        location: Location,
    },
}

type Token = (char, Location);

fn lexer(contents: &str) -> Vec<Token> {
    let mut result = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        for (char_index, content) in line.chars().enumerate() {
            match content {
                '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' | '%' => {
                    result.push((content, Location(line_index + 1, char_index + 1)));
                }
                _ => {}
            }
        }
    }
    result
}

pub fn parse_instructions(
    file_path: &PathBuf,
    debug: bool,
    optimize: bool,
) -> Result<Vec<Instruction>> {
    let file = File::open(file_path)?;
    let mut contents = String::new();
    let mut buffer = BufReader::new(file);
    buffer.read_to_string(&mut contents)?;
    let tokens = lexer(&contents);
    let mut instructions = Vec::new();
    let mut token = tokens.into_iter().peekable();
    while let Some(instruction_char) = token.next() {
        let mut times = 1;
        if optimize {
            while let Some(next_instruction_char) = token.peek() {
                match next_instruction_char {
                    ('>' | '<' | '+' | '-', _) => {
                        if next_instruction_char.0 == instruction_char.0 {
                            times += 1;
                            token.next();
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            }
        }
        match instruction_char {
            ('>', _) => instructions.push(Instruction::IncrementPointer(times)),
            ('<', _) => instructions.push(Instruction::DecrementPointer(times)),
            ('+', _) => instructions.push(Instruction::IncrementValue(times)),
            ('-', _) => instructions.push(Instruction::DecrementValue(times)),
            ('.', _) => instructions.push(Instruction::Output),
            (',', _) => instructions.push(Instruction::Input),
            ('%', _) => {
                if debug {
                    instructions.push(Instruction::Debug);
                }
            }
            ('[', location) => instructions.push(Instruction::LoopStart {
                target: None,
                location,
            }),
            (']', location) => instructions.push(Instruction::LoopEnd {
                target: None,
                location,
            }),
            _ => {}
        }
    }
    let mut stack = Vec::new();
    let mut map = AHashMap::new();
    for (index, instruction) in instructions.iter().enumerate() {
        match instruction {
            Instruction::LoopStart { .. } => {
                stack.push(index);
            }
            Instruction::LoopEnd { .. } => {
                let start = stack.pop();
                if let Some(start) = start {
                    map.insert(start, index);
                    map.insert(index, start);
                }
            }
            _ => {}
        }
    }
    for (index, instruction) in instructions.iter_mut().enumerate() {
        match instruction {
            Instruction::LoopStart {
                ref mut target,
                location,
            } => {
                *target = Some(*map.get(&index).ok_or_else(|| {
                    anyhow!("Unmatched '[' at {}:{}", file_path.display(), location)
                })?);
            }
            Instruction::LoopEnd {
                ref mut target,
                location,
            } => {
                *target = Some(*map.get(&index).ok_or_else(|| {
                    anyhow!("Unmatched ']' at {}:{}", file_path.display(), location)
                })?);
            }
            _ => {}
        }
    }
    Ok(instructions)
}
