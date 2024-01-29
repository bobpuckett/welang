use crate::{
    modulizer::Tree,
    parser::{Type, Value},
};
pub fn type_out(tree: &mut Tree) {
    if tree.is_leaf() {
        let module = tree
            .module
            .as_mut()
            .expect("was leaf, but no module present.");
        
        for part in module.map.iter_mut() {
            let v = *part.1.value.clone();
            match v {
                Value::Function(steps) => {
                    if steps.len() == 0 {
                        // in
                        if part.1.in_type != Type::Unknown && part.1.in_type != Type::None {
                            todo!(
                                "{} is an empty function has no input type, but found {:#?}",
                                part.0,
                                part.1.in_type
                            );
                        }
                        // out
                        if part.1.out_type != Type::Unknown && part.1.out_type != Type::None {
                            todo!(
                                "{} is an empty function has no output type, but found {:#?}",
                                part.0,
                                part.1.out_type
                            );
                        }
                        part.1.in_type = Type::None;
                        part.1.out_type = Type::None;
                    }
                }
                Value::Array(value) => {
                    if value.len() == 0 {
                        // in
                        if part.1.in_type != Type::Unknown && part.1.in_type != Type::None {
                            todo!(
                                "{} is an empty array and has no input type, but found {:#?}",
                                part.0,
                                part.1.in_type
                            );
                        }
                        // out
                        if part.1.out_type != Type::Array(Box::new(Type::Unknown))
                            && part.1.out_type != Type::Array(Box::new(Type::None))
                        {
                            todo!(
                                "{} is an empty array and has no output type, but found {:#?}",
                                part.0,
                                part.1.out_type
                            );
                        }
                        part.1.in_type = Type::None;
                        part.1.out_type = Type::Array(Box::new(Type::None));
                    }
                }
                Value::Map(value) => {
                    if value.len() == 0 {
                        // in
                        if part.1.in_type != Type::Unknown && part.1.in_type != Type::None {
                            todo!(
                                "{} is an empty array and has no input type, but found {:#?}",
                                part.0,
                                part.1.in_type
                            );
                        }
                        // out
                        if part.1.out_type != Type::Context && part.1.out_type != Type::None {
                            todo!(
                                "{} is an empty array and has no output type, but found {:#?}",
                                part.0,
                                part.1.out_type
                            );
                        }
                        part.1.in_type = Type::None;
                        part.1.out_type = Type::Context;
                    }
                }
                Value::TypeAlias(_) => todo!(),
                Value::TypeIdentity(_) => todo!(),
                Value::Discard => {
                    // in
                    if part.1.in_type != Type::Unknown && part.1.in_type != Type::None {
                        todo!(
                            "{} is a discard and has no input type, but found {:#?}",
                            part.0,
                            part.1.in_type
                        );
                    }
                    // out
                    if part.1.out_type != Type::Unknown && part.1.out_type != Type::None {
                        todo!(
                            "{} is a discard and has no output type, but found {:#?}",
                            part.0,
                            part.1.out_type
                        );
                    }
                    part.1.in_type = Type::None;
                    part.1.out_type = Type::None;
                }
                Value::Integer(_) => {
                    if part.1.in_type != Type::None {
                        panic!(
                            "Integer must have a None input type but found {:#?}",
                            part.1.in_type
                        );
                    }
                    if part.1.out_type != Type::Atom {
                        panic!(
                            "Integer must have an output of an array of atoms but found {:#?}",
                            part.1.in_type
                        );
                    }
                    part.1.in_type = Type::None;
                    part.1.out_type = Type::Atom;
                }
                Value::IdentifierChain(_) => todo!(),
                Value::String(_) => {
                    if part.1.in_type != Type::None {
                        panic!(
                            "String must have a None input type but found {:#?}",
                            part.1.in_type
                        );
                    }
                    if part.1.out_type != Type::Array(Box::new(Type::Atom)) {
                        panic!(
                            "String must have an output of an array of atoms but found {:#?}",
                            part.1.in_type
                        );
                    }
                    part.1.in_type = Type::None;
                    part.1.out_type = Type::Array(Box::new(Type::Atom));
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::type_out;
    use crate::{
        modulizer::to_module_tree,
        parser::{Type, Value},
    };
    use std::collections::HashMap;
    #[test]
    pub fn types_simple_declarations() {
        let mut tree = to_module_tree("test/types_simple_declarations.we");
        type_out(&mut tree);

        println!("typed: {:#?}", tree);
        if !tree.is_leaf() {
            panic!("tree was not a leaf!");
        }

        let module = tree.module.clone().unwrap();

        // function
        let function = module
            .map
            .get("function")
            .expect("function was not in the map");
        assert_eq!(function.in_type, Type::None);
        assert_eq!(function.out_type, Type::None);
        assert_eq!(*function.value, Value::Function(vec![]));
        // array
        let array = module.map.get("array").expect("array was not in the map");
        assert_eq!(array.in_type, Type::None);
        assert_eq!(array.out_type, Type::Array(Box::new(Type::None)));
        assert_eq!(*array.value, Value::Array(vec![]));
        // map
        let map = module.map.get("map").expect("map was not in the map");
        assert_eq!(map.in_type, Type::None);
        assert_eq!(map.out_type, Type::Context);
        assert_eq!(*map.value, Value::Map(HashMap::new()));
        // string
        let string = module.map.get("string").expect("string was not in the map");
        assert_eq!(string.in_type, Type::None);
        assert_eq!(string.out_type, Type::Array(Box::new(Type::Atom)));
        assert_eq!(*string.value, Value::String("hi".to_string()));
        // int
        let int = module.map.get("int").expect("int was not in the map");
        assert_eq!(int.in_type, Type::None);
        assert_eq!(int.out_type, Type::Atom);
        assert_eq!(*int.value, Value::Integer(10));
        // discard
        let discard = module
            .map
            .get("discard")
            .expect("discard was not in the map");
        assert_eq!(discard.in_type, Type::None);
        assert_eq!(discard.out_type, Type::None);
        assert_eq!(*discard.value, Value::Discard);
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
