#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub nums); // synthesized by LALRPOP

use std::fs;
use std::env;

fn compile(prog : i64) -> String {
    format!("section .text\n\
            global our_code_starts_here\n\
            our_code_starts_here:\n\
            \tmov RAX, {}\n\
            \tret\n", prog)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let prog_text = fs::read_to_string(filename)?;
    let prog = nums::NumParser::new().parse(&prog_text);
    let instr = compile(prog.unwrap());
    println!("{}", instr);
    Ok(())
}
