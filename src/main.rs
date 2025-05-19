#![allow(dead_code)]

use std::{collections::HashMap, fs};

mod parse;
mod primitive;
mod result;
mod run;
mod value;

use result::*;

fn main_helper(input_file: String) -> JSLResult<()> {
    let code = fs::read_to_string(input_file).or(Err(JSLError {
        msg: "could not read file".into(),
    }))?;
    let ast: parse::AST;
    match run::gen_ast_from_code(code.as_str()) {
        Ok(good) => ast = good,
        Err(e) => return Err(e),
    }
    run::run_ast(ast, &mut vec![], &mut HashMap::new())
}

fn print_error(error: String) {
    println!("\x1b[1;31merror:\x1b[0m {error}");
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    if args.len() < 1 {
        print_error("expected jsl file as argument".into());
        return;
    }
    let file = args[0].clone();
    let res = main_helper(file);
    if let Some(err) = res.err() {
        print_error(err.msg);
    }
}
