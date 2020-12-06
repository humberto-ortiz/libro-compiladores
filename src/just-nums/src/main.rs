use std::fs;
use std::env;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let program_text = fs::read_to_string(filename)?;
    // println!("Input = {}", program_text);

    let program = i32::from_str(&program_text.trim())?;
    println!("section .text\n\
global our_code_starts_here\n\
our_code_starts_here:\n\
\tmov RAX, {}\n\
\tret\n", program);
    Ok(())
}
