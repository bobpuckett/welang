use std::{
    collections::HashMap,
    fs::{self, metadata, read_dir},
};

use crate::{
    lexer::TokenContext,
    parser::{parse_module, Node, Type, Value},
};

pub fn to_module_tree(path: &str) -> Node {
    let md = metadata(&path);
    match md {
        Ok(val) if val.is_file() => {
            let content = fs::read_to_string(&path);

            match content {
                Ok(source) => Node {
                    in_type: Type::None,
                    out_type: Type::None, // TODO: Is None really the right type here?
                    value: Box::new(parse_module(&mut TokenContext::new(&source))),
                },
                Err(_) => todo!("Could not read {}", &path),
            }
        }
        Ok(val) if val.is_dir() => {
            let map = read_dir(path)
                .expect(&format!("Could not read dir: {}", &path))
                .map(|p| {
                    let file_name = p.unwrap().file_name();
                    let next_path = file_name.to_str().unwrap();
                    let module_name = get_mod_name(&next_path);
                    let submodule = to_module_tree(next_path);

                    (module_name, submodule)
                })
                .collect::<HashMap<_, _>>();

            Node {
                in_type: Type::None,
                out_type: Type::None,
                value: Box::new(Value::Map(map)),
            }
        }
        Ok(_) => todo!(), // could be a symlink

        Err(msg) => todo!("Could not modularize path {}, {}", &path, msg),
    }
}

fn get_mod_name(path: &str) -> String {
    // TODO: Windows style pathing / no unwraps
    path.split('.')
        .next()
        .unwrap()
        .to_owned()
        .split('/')
        .last()
        .unwrap()
        .to_owned()
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
