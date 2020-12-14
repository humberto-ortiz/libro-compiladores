#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub incr); // synthesized by LALRPOP

pub mod ast;

use std::fs;
use std::env;
use crate::ast::Expr;

fn compile(prog : Expr) -> String {
    match prog {
        Expr::Number(n) =>
            format!("section .text\n\
                 global our_code_starts_here\n\
                 our_code_starts_here:\n\
                 \tmov RAX, {}\n\
                 \tret\n", n)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let prog_text = fs::read_to_string(filename)?;
    let prog = incr::ExprParser::new().parse(&prog_text);
    let instr = compile(prog.unwrap());
    println!("{}", instr);
    Ok(())
}
