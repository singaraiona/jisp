#[macro_use]
extern crate jisp;
extern crate nom;

use nom::IResult;
use std::io::{self, Read, Write};
use jisp::parser;
use jisp::eval::{self, AST, init_symbols};

fn ps1() { print!(": "); io::stdout().flush().unwrap(); }

fn main() {
    debug_assert!(::std::mem::size_of::<AST>() == 16, "Sizeof AST is not 16.");
    let mut input = vec![0u8;4096];
    init_symbols();
    ps1();
    loop {
        let size = io::stdin().read(&mut input).expect("STDIN error.");
        match parser::parse(&input[..size]) {
            IResult::Done(_, a) => println!("-> {}", eval::eval_seq(a.as_slice())),
            IResult::Error(e) => println!("-> {:?}", e),
            _ => println!("-> Error"),
        }
        ps1();
    }
}
