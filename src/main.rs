use std::io;

use crate::modulizer::to_module_tree;

mod construct;
mod flattener;
mod lexer;
mod modulizer;
mod parser;
mod typer;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let tree = to_module_tree(&buffer);

    println!("{:#?}", tree);

    Ok(())
}
