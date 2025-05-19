#![allow(dead_code)]

use std::{collections::HashMap, fs};

mod parse;
mod primitive;
mod result;
mod run;
mod value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = fs::read_to_string("test.jsl")?;
    let ast: parse::AST;
    match run::gen_ast_from_code(code.as_str()) {
        Ok(good) => ast = good,
        Err(e) => panic!("{}", e.msg),
    }
    let result = run::run_ast(ast, &mut vec![], &mut HashMap::new());
    if let Some(error) = result.err() {
        println!("{}", error.msg);
    }
    Ok(())
}
