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

    write("./out.atc", compile(tokens, &[], &mut labels, &mut routines)).unwrap();
}
