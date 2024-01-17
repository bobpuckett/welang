use std::collections::HashMap;

use crate::parser::Node;

#[derive(Debug)]
pub enum Type {
    Integer,
    None,
}

#[derive(Debug)]
pub struct Step {
    pub input_type: Type,
    pub output_type: Type,
}

pub fn flatten(root: Node) -> Vec<Step> {
    return match root {
        Node::Map(map) => {
            if !map.contains_key("main") {
                todo!("No main method provided");
            }

            let main = (**map.get("main").unwrap()).clone();

            match main {
                Node::Function(fn_steps) => fn_steps
                    .iter()
                    .flat_map(|s| flatten_step(&map, s))
                    .into_iter()
                    .collect(),
                _ => todo!("main method was not a method"),
            }
        }
        _ => todo!("Root node was not a map"),
    };
}

fn flatten_step(root: &HashMap<String, Box<Node>>, step: &Box<Node>) -> Vec<Step> {
    match *step.clone() {
        Node::List(_) => todo!(),
        Node::Map(_) => todo!(),
        Node::Function(fun) => fun.iter().flat_map(|f| flatten_step(root, f)).collect(),

        Node::TypeParameter(_, _) => todo!(),

        Node::TypeAlias(_) => todo!(),
        Node::TypeIdentity(_) => todo!(),

        Node::Discard => todo!(),

        Node::IdentifierChain(chain) => todo!(),
        Node::Integer(i) => vec![Step {
            input_type: Type::None,
            output_type: Type::Integer,
        }],
        Node::String(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::TokenContext, parser};

    use super::{flatten, Step};

    #[test]
    pub fn finds_main() {
        // given
        let mut context = TokenContext::new("{ discard: \"nothing\", main: (\"ether\"; print)}");
        let root = *parser::parse_value(&mut context).unwrap();

        let result: Vec<Step> = flatten(root);
    }
}
