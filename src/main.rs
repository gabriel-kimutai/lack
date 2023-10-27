pub mod lexer;
pub mod parser;
pub mod generator;


use generator::Generator;
use parser::NodeExit;
use parser::Parser;

use crate::lexer::*;
use std::fs;
use std::env;
use std::fs::File;
use std::io::Write;


#[allow(dead_code)]
fn print_usage() {
    println!("Input file not provided");
    println!("    Usage: <file.lc>");
}

#[allow(dead_code)]
fn read_file(file_path: &str) -> String {
    let file = fs::read_to_string(file_path.to_string());
    if let Ok(contents) = file {
        return contents;
    } else {
        panic!("Failed to read file {file_path}");
    }
}

fn write_asm(input: String) {
    let file = File::create("./out.asm");
        file.unwrap().write_all(input.as_bytes()).unwrap();
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
    }
    let file_path = &args[1];
    let data: String = read_file(&file_path);

    let mut lex: Lexer = Lexer::from_str(data);
    let tokens: Vec<Token> = lex.tokenize();
    // dbg!(&tokens);
    // generate_asm(&tokens);
    let mut parser: Parser = Parser::from_tok(tokens);
    let node_exit: NodeExit = parser.parse().unwrap();
    let generator: String = Generator::generate(node_exit);
    write_asm(generator);

}
