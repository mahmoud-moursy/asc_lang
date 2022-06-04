#![feature(let_else)]
#![feature(let_chains)]

pub mod tokenizer;
pub mod tokens;
pub mod compiler;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::write;

use tokenizer::tokenize;

use crate::compiler::compile;

fn main() {
    let file = read_to_string("main.asc").unwrap();

    let tokens = tokenize(file);

    println!("{tokens:?}");

    let mut labels = HashMap::new();

    let mut routines = HashMap::new();

    let mut output = Vec::new();

    let mut header_size: usize = 0;

    compile(tokens, &mut output, &mut labels, &mut routines, &mut header_size);

    write("./out.atc", output).unwrap();
}
