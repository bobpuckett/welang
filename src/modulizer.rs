use std::{collections::HashMap, fs::{metadata, self, read_dir}};

use crate::{parser::{Module, parse_module}, lexer::TokenContext};

#[derive(Debug, Clone)]
pub enum Tree {
    Internal(HashMap<String, Box<Tree>>),
    Leaf(String, Module)
}

pub fn to_module_tree(path: &str) -> Tree {
    let md = metadata(&path);
    match md {
        Ok(val) if val.is_file() => {
            let name = get_mod_name(&path);
            let content = fs::read_to_string(&path);

            match content {
                Ok(source) => Tree::Leaf(name, parse_module(&mut TokenContext::new(&source))),
                Err(_) => todo!("Could not read {}", &path),
            }
        },
        Ok(val) if val.is_dir() => {
            let map = read_dir(path)
                .expect(&format!("Could not read dir: {}", &path))
                .map(|p| {
                    let file_name = p.unwrap().file_name();
                    let next_path = file_name.to_str().unwrap();
                    let module_name = get_mod_name(&next_path);
                    let submodule = to_module_tree(next_path);

                    (module_name, Box::new(submodule))
                })
                .collect::<HashMap<_,_>>();

            Tree::Internal(map)
        },
        Ok(_) => todo!(), // could be a symlink

        Err(msg) => todo!("Could not modularize path {}, {}", &path, msg),
    }
}

fn get_mod_name(path: &str) -> String {
    // TODO: Windows style pathing / no unwraps
    path
        .split('.').next().unwrap().to_owned()
        .split('/').last().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::to_module_tree;

    #[test]
    pub fn can_parse_full_syntax() {
        let tree = to_module_tree("./test/full.we");
        println!("{:#?}", tree);
    }
}