use std::io;

use crate::{modulizer::to_module_tree, typer::type_it};

mod flattener;
mod lexer;
mod modulizer;
mod parser;
mod typer;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let mut tree = to_module_tree(&buffer);
    type_it(&mut tree);

    println!("{:#?}", tree);

    Ok(())
}
