use std::io;

use lexer::TokenContext;
use crate::parser::parse_module;

mod lexer;
mod parser;
mod typer;
mod flattener;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let mut context = TokenContext::new(&buffer);
    let result = parse_module(&mut context);

    println!("{:#?}", result);

    Ok(())
}
