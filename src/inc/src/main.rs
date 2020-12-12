#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub nums); // synthesized by LALRPOP

#[test]
fn numbers() {
  assert!(nums::NumParser::new().parse("22").is_ok());
  assert!(nums::NumParser::new().parse("123").is_ok());
  assert!(nums::NumParser::new().parse("-22)").is_err());
}

use std::fs;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let foo = fs::read_to_string(filename)?;
    let prog = nums::NumParser::new().parse(&foo);
    println!("{:?}", prog.unwrap());
    Ok(())
}
