use std::io;

use lexer::TokenContext;
use parser::parse_value;

mod lexer;
mod parser;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let mut context = TokenContext::new(&buffer);
    let result = parse_value(&mut context);

    println!("{:#?}", result);

    Ok(())
}
