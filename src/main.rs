use std::io;

use crate::{modulizer::to_module_tree, typer::walk_first_pass};

mod flattener;
mod lexer;
mod modulizer;
mod parser;
mod typer;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let mut tree = to_module_tree(&buffer);
    walk_first_pass(&mut tree);

    println!("{:#?}", tree);

    Ok(())
}
