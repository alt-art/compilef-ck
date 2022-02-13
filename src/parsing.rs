use ahash::AHashMap;

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
                    .unwrap_or_else(|| panic!("Unmatched loop end at index {}", index));
                targets.insert(loop_index, index);
                targets.insert(index, loop_index);
            }
            _ => {}
        }
    }
    targets
}

pub fn parse_instructions(instructions_str: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let clean_instructions_str = remove_comments(instructions_str);
    let loop_targets = create_loop_targets(&clean_instructions_str);
    for (index, instruction_char) in clean_instructions_str.chars().enumerate() {
        match instruction_char {
            '>' => instructions.push(Instruction::IncrementPointer),
            '<' => instructions.push(Instruction::DecrementPointer),
            '+' => instructions.push(Instruction::IncrementValue),
            '-' => instructions.push(Instruction::DecrementValue),
            '.' => instructions.push(Instruction::Output),
            ',' => instructions.push(Instruction::Input),
            '[' => instructions.push(Instruction::LoopStart(
                *loop_targets
                    .get(&index)
                    .unwrap_or_else(|| panic!("Unmatched '[' at index: {}", index)),
            )),
            ']' => instructions.push(Instruction::LoopEnd(*loop_targets.get(&index).unwrap())),
            _ => {}
        }
    }
    instructions
}
