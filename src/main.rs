#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::str_to_string
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests;

mod compiler;
mod interpreter;
mod parsing;

use compiler::yasm_x86_64_linux_compiler;
use interpreter::execute;
use parsing::parse_instructions;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

use std::{env::current_dir, fmt::Display, path::PathBuf};

#[derive(Clone, ValueEnum)]
pub enum TargetPlatform {
    Linux,
    Windows,
}

impl Display for TargetPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Linux => write!(f, "linux"),
            Self::Windows => write!(f, "windows"),
        }
    }
}

impl Default for TargetPlatform {
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        return Self::Windows;
        #[cfg(target_os = "linux")]
        return Self::Linux;
    }
}

#[derive(Clone, ValueEnum)]
enum TargetAsm {
    Yasm,
    Fasm,
}

impl Display for TargetAsm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yasm => write!(f, "yasm"),
            Self::Fasm => write!(f, "fasm"),
        }
    }
}

#[derive(Subcommand, Clone)]
enum Mode {
    /// Interprets the brainfuck program in the given file
    Interpret {
        /// The brainfuck file to interpret
        file: PathBuf,
    },
    /// Compiles the brainfuck program in the given file
    Compile {
        /// The brainfuck file to compile
        file: PathBuf,
        /// Output location of binary file
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Target platform that you want to compile
        #[arg(short, long, default_value_t=TargetPlatform::default())]
        target: TargetPlatform,
        /// Target assembler
        #[arg(long, default_value_t=TargetAsm::Yasm)]
        target_asembler: TargetAsm,
    },
}

#[derive(Parser, Clone)]
#[clap(about, author, version)]
struct Opt {
    #[command(subcommand)]
    mode: Mode,
    #[arg(short, long)]
    /// Activate the debug command `%` that shows current value of the pointer and current memory position value
    debug: bool,
    #[arg(short, long)]
    /// Optimize the code by grouping decrement/increment instructions in one single instruction
    optimize: bool,
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    match opt.mode {
        Mode::Interpret { file } => {
            let instructions = parse_instructions(&file, opt.debug, opt.optimize)?;
            execute(&instructions)?;
        }
        Mode::Compile {
            file,
            output,
            target,
            target_asembler,
        } => {
            #[cfg(target_os = "windows")]
            panic!("Cannot compile on windows only interpret");
            let instructions = parse_instructions(&file, opt.debug, opt.optimize)?;
            let output = if let Some(output) = output {
                output
            } else {
                current_dir()?.join(file.file_stem().expect("File name not found"))
            };
            match target_asembler {
                TargetAsm::Yasm => yasm_x86_64_linux_compiler(&instructions, &output, &target)?,
                TargetAsm::Fasm => todo!(),
            }
        }
    }
    Ok(())
}
