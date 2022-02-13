mod parsing;
mod interpreter;

use parsing::parse_instructions;
use interpreter::execute;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./brainfuck/helloworld.bf")?;
    let mut contents = String::new();
    let mut buffer = BufReader::new(file);
    buffer.read_to_string(&mut contents)?;
    let instructions = parse_instructions(&contents);
    execute(&instructions);
    Ok(())
}
