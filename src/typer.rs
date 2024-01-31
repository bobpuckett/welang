use crate::parser::{Node, Type, Value};

pub fn type_out(node: &mut Node) {
    match node.value.as_mut() {
        Value::Module { usings: _, map } => {
            for part in map.iter_mut() {
                type_out(part.1);
            }
        }
        Value::Array(value) => {
            if value.len() == 0 {
                assert_of(
                    "array",
                    &node,
                    vec![Type::Unknown, Type::None],
                    vec![
                        Type::Array(Box::new(Type::Unknown)),
                        Type::Array(Box::new(Type::None)),
                    ],
                );

                node.in_type = Type::None;
                node.out_type = Type::Array(Box::new(Type::None));
            } else {
                panic!("tried to handle a non-zero array");
            }
        }
        Value::Map(value) => {
            if value.len() == 0 {
                assert_of(
                    "map",
                    node,
                    vec![Type::Unknown, Type::None],
                    vec![Type::Context, Type::None],
                );
                node.in_type = Type::None;
                node.out_type = Type::Context;
            } else {
                todo!("map waaas not empty")
            }
        }
        Value::Function(steps) => {
            if steps.len() == 0 {
                assert_of(
                    "function",
                    node,
                    vec![Type::Unknown, Type::None],
                    vec![Type::Unknown, Type::None],
                );
                node.in_type = Type::None;
                node.out_type = Type::None;
            } else {
                for step in steps.iter_mut() {
                    type_out(step);
                }

                let first = steps.first();
                node.in_type = first.unwrap().in_type.clone();

                let last = steps.last();
                node.out_type = last.unwrap().out_type.clone();
            }
        }
        Value::TypeAlias(_) => todo!(),
        Value::TypeIdentity(_) => todo!(),
        Value::Discard => {
            assert_of(
                "discard",
                node,
                vec![Type::Unknown, Type::None],
                vec![Type::Unknown, Type::None],
            );
            node.in_type = Type::None;
            node.out_type = Type::None;
        }
        Value::Integer(_) => {
            assert_of("integer", node, vec![Type::None], vec![Type::Atom]);
            node.in_type = Type::None;
            node.out_type = Type::Atom;
        }
        Value::IdentifierChain(_) => todo!(),
        Value::String(_) => {
            assert_of(
                "string",
                node,
                vec![Type::None],
                vec![Type::Array(Box::new(Type::Atom))],
            );

            node.in_type = Type::None;
            node.out_type = Type::Array(Box::new(Type::Atom));
        }
    }
}

fn assert_of(name: &str, node: &Node, in_type: Vec<Type>, out_type: Vec<Type>) {
    if !in_type.contains(&node.in_type) {
        panic!(
            "{} must have a {:#?} input type, but found: {:#?}",
            name, in_type, node.in_type
        );
    }
    if !out_type.contains(&node.out_type) {
        panic!(
            "{} must have a {:#?} output type, but found: {:#?}",
            name, out_type, node.out_type
        );
    }
}

#[cfg(test)]
mod tests {
    use super::type_out;
    use crate::{
        lexer::TokenContext,
        modulizer::to_module_tree,
        parser::{parse_module, Node, Type, Value},
    };
    use std::collections::HashMap;
    #[test]
    pub fn types_simple_declarations() {
        let mut node = Node {
            in_type: Type::None,
            out_type: Type::None,
            value: Box::new(parse_module(&mut TokenContext::new(
                r#"function: ()
                array: []
                map: {}
                int: 10
                string: "hi"
                discard: _"#,
            ))),
        };
        type_out(&mut node);

        println!("typed: {:#?}", node);

        if let Value::Module { usings: _, map } = *node.value.clone() {
            // function
            let function = map.get("function").expect("function was not in the map");
            assert_eq!(function.in_type, Type::None);
            assert_eq!(function.out_type, Type::None);
            assert_eq!(*function.value, Value::Function(vec![]));

            // array
            let array = map.get("array").expect("array was not in the map");
            assert_eq!(array.in_type, Type::None);
            assert_eq!(array.out_type, Type::Array(Box::new(Type::None)));
            assert_eq!(*array.value, Value::Array(vec![]));

            // map
            let sub_map = map.get("map").expect("map was not in the map");
            assert_eq!(sub_map.in_type, Type::None);
            assert_eq!(sub_map.out_type, Type::Context);
            assert_eq!(*sub_map.value, Value::Map(HashMap::new()));

            // string
            let string = map.get("string").expect("string was not in the map");
            assert_eq!(string.in_type, Type::None);
            assert_eq!(string.out_type, Type::Array(Box::new(Type::Atom)));
            assert_eq!(*string.value, Value::String("hi".to_string()));

            // int
            let int = map.get("int").expect("int was not in the map");
            assert_eq!(int.in_type, Type::None);
            assert_eq!(int.out_type, Type::Atom);
            assert_eq!(*int.value, Value::Integer(10));

            // discard
            let discard = map.get("discard").expect("discard was not in the map");
            assert_eq!(discard.in_type, Type::None);
            assert_eq!(discard.out_type, Type::None);
            assert_eq!(*discard.value, Value::Discard);
        } else {
            panic!("node was not module");
        }
    }

    #[test]
    pub fn types_function_with_atom() {
        let mut node = Node {
            in_type: Type::None,
            out_type: Type::None,
            value: Box::new(parse_module(&mut TokenContext::new(r#"atomized: (100)"#))),
        };
        type_out(&mut node);

        if let Value::Module { usings: _, map } = *node.value.clone() {
            let atomized = map.get("atomized").unwrap();
            assert_eq!(atomized.in_type, Type::None);
            assert_eq!(atomized.out_type, Type::Atom);
        } else {
            panic!("node was not module");
        }
    }

    #[test]
    pub fn types_function_with_reference() {}

    #[test]
    pub fn types_parameter() {}
    #[test]
    pub fn types_post_declarations() {}
    #[test]
    pub fn reports_type_parameter_mismatch() {}
    #[test]
    pub fn reports_step_with_wrong_type() {}
    #[test]
    pub fn reports_array_with_wrong_type() {}
}
