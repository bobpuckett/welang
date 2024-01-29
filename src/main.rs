use std::io;

use crate::{modulizer::to_module_tree, typer::type_out};

mod lexer;
mod parser;
mod modulizer;
mod typer;
mod flattener;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    
    let mut tree = to_module_tree(&buffer);
    type_out(&mut tree);

    println!("{:#?}", tree);

    Ok(())
}
