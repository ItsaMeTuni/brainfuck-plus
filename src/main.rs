use crate::lexer::lex;
use crate::ast::gen_ast;
use crate::interpreter::interpret;

mod lexer;
mod ast;
mod interpreter;
mod iterator_helpers;

fn main()
{
    let tokens = lex("");
    let ast = gen_ast(tokens);

    interpret(ast.unwrap(), 10);
}
