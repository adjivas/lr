extern crate lalrpop_util;

pub mod lexer;
pub mod ast;
pub mod parser; // synthesized by LALRPOP

fn main() {
//    let lexer: lexer::Lexer = lexer::Lexer::new("++\n\n");
//    println!("P{:?}", parser::ProgramParser::new().parse(lexer) );

    let lexer: lexer::Lexer = lexer::Lexer::new("++/\n\n");
    println!("P{:?}", parser::ProgramParser::new().parse(lexer) );

    let lexer: lexer::Lexer = lexer::Lexer::new("+++\n/\n\n");
    println!("P{:?}", parser::ProgramParser::new().parse(lexer) );

//    let lexer: lexer::Lexer = lexer::Lexer::new("++-\n\n");
//    println!("P{:?}", parser::ProgramParser::new().parse(lexer) );

//    let lexer: lexer::Lexer = lexer::Lexer::new("++@\n\n");
//    println!("P{:?}", parser::ProgramParser::new().parse(lexer) );
}
