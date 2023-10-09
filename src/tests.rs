use super::*;

use anyhow::Ok;
use serial_test::serial;

use std::fs;
use std::io::Write;
use std::process::{Command, Output, Stdio};

fn compile_and_run(file: &PathBuf, input: &[u8]) -> Result<Output> {
    let instructions = parse_instructions(file, true, false)?;
    let output_path = current_dir()?.join(file.file_stem().expect("File name not found"));
    yasm_x86_64_linux_compiler(&instructions, &output_path)?;
    let mut process = Command::new(&output_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    process.stdin.as_mut().unwrap().write_all(input)?;
    let output = process.wait_with_output()?;
    fs::remove_file(output_path)?;
    Ok(output)
}

#[test]
#[serial]
fn test_hello_world() -> Result<()> {
    let output = compile_and_run(&PathBuf::from("./brainfuck/helloworld.bf"), b"")?;
    assert_eq!(String::from_utf8(output.stdout)?, "Hello World!\n");
    assert_eq!(output.status.code(), Some(0));
    Ok(())
}

#[test]
#[serial]
fn test_in_out() -> Result<()> {
    let output = compile_and_run(&PathBuf::from("./brainfuck/in_out.bf"), b"ab")?;
    assert_eq!(String::from_utf8(output.stdout)?, "ab");
    assert_eq!(output.status.code(), Some(0));
    Ok(())
}

#[test]
#[serial]
fn test_increment_overflow() -> Result<()> {
    let output = compile_and_run(&PathBuf::from("./brainfuck/increment_overflow.bf"), b"")?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        r#"
--- DEBUG ---
pointer: 15000
memory: 255
--- DEBUG ---

--- DEBUG ---
pointer: 15000
memory: 1
--- DEBUG ---
"#
    );
    assert_eq!(output.status.code(), Some(0));
    Ok(())
}

#[test]
#[serial]
fn test_overflow() -> Result<()> {
    let output = compile_and_run(&PathBuf::from("./brainfuck/overflow.bf"), b"")?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        "\x1b[1;31mERROR: overflow exception\x1b[0m\n"
    );
    assert_eq!(output.status.code(), Some(1));
    Ok(())
}

#[test]
#[serial]
fn test_underflow() -> Result<()> {
    let output = compile_and_run(&PathBuf::from("./brainfuck/underflow.bf"), b"")?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        "\x1b[1;31mERROR: underflow exception\x1b[0m\n"
    );
    assert_eq!(output.status.code(), Some(1));
    Ok(())
}
