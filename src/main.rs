use std::io;

use crate::modulizer::to_module_tree;

mod lexer;
mod parser;
mod modulizer;
mod typer;
mod flattener;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    
    let tree = to_module_tree(&buffer);

    println!("{:#?}", tree);

    Ok(())
}
