#![feature(let_else)]
#![feature(let_chains)]

pub mod tokenizer;
pub mod tokens;
pub mod compiler;

use std::fs::read_to_string;

use tokenizer::tokenize;

fn main() {
    let file = read_to_string("main.asc").unwrap();

    let tokens = tokenize(file);

    println!("{tokens:?}");
}
