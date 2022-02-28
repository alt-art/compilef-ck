mod interpreter;
mod parsing;

use interpreter::execute;
use parsing::parse_instructions;

use anyhow::Result;
use clap::Parser;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(about, author, version)]
enum Opt {
    /// Interprets the brainfuck program in the given file
    Interpret {
        /// The brainfuck file to interpret
        file: PathBuf,
    },
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    match opt {
        Opt::Interpret { file } => {
            let file = File::open(file)?;
            let mut contents = String::new();
            let mut buffer = BufReader::new(file);
            buffer.read_to_string(&mut contents)?;
            let instructions = parse_instructions(&contents);
            execute(&instructions);
        }
    }
    Ok(())
}
