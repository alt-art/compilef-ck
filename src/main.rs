mod parsing;

use parsing::parse_instructions;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./brainfuck/helloworld.bf")?;
    let mut contents = String::new();
    let mut buffer = BufReader::new(file);
    buffer.read_to_string(&mut contents)?;
    dbg!(parse_instructions(&contents));
    Ok(())
}
