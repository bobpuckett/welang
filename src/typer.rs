// use std::collections::HashMap;

// use crate::{
//     modulizer::Tree,
//     parser::{Node, Type, Value},
// };

// pub fn type_out(original: Tree) -> Tree {
//     let mut new = original.clone();

//     match original {
//         Tree::Internal(_) => todo!(),
//         Tree::Leaf(name, ref module) => {
//             let typed_module: HashMap<_, _> = module
//                 .map
//                 .clone()
//                 .into_iter()
//                 .map(|part| {
//                     return match *part.1.value {
//                         Value::Function(steps) => {
//                             if steps.len() == 0 {
//                                 // in
//                                 if part.1.in_type != Type::Unknown && part.1.in_type != Type::None {
//                                     todo!(
//                                 "{} is an empty function has no input type, but found {:#?}",
//                                 part.0,
//                                 part.1.in_type
//                             );
//                                 }
//                                 // out
//                                 if part.1.out_type != Type::Unknown && part.1.out_type != Type::None
//                                 {
//                                     todo!(
//                                 "{} is an empty function has no output type, but found {:#?}",
//                                 part.0,
//                                 part.1.out_type
//                             );
//                                 }

//                                 return (
//                                     part.0,
//                                     Node {
//                                         in_type: Type::None,
//                                         out_type: Type::None,
//                                         value: part.1.value.clone(),
//                                     },
//                                 );
//                             }
//                         }
//                         Value::Array(value) => {
//                             if value.len() == 0 {
//                                 // in
//                                 if part.1.in_type != Type::Unknown && part.1.in_type != Type::None {
//                                     todo!(
//                                 "{} is an empty array and has no input type, but found {:#?}",
//                                 part.0,
//                                 part.1.in_type
//                             );
//                                 }
//                                 // out
//                                 if part.1.out_type != Type::Array(Box::new(Type::Unknown))
//                                     && part.1.out_type != Type::Array(Box::new(Type::None))
//                                 {
//                                     todo!(
//                                 "{} is an empty array and has no output type, but found {:#?}",
//                                 part.0,
//                                 part.1.out_type
//                             );
//                                 }
//                                 return (
//                                     part.0,
//                                     Node {
//                                         in_type: Type::None,
//                                         out_type: Type::Array(Box::new(Type::None)),
//                                         value: part.1.value.clone(),
//                                     },
//                                 );
//                             }
//                         }
//                         Value::Map(value) => {
//                             if value.len() == 0 {
//                                 // in
//                                 if part.1.in_type != Type::Unknown && part.1.in_type != Type::None {
//                                     todo!(
//                                 "{} is an empty array and has no input type, but found {:#?}",
//                                 part.0,
//                                 part.1.in_type
//                             );
//                                 }
//                                 // out
//                                 if part.1.out_type != Type::Context && part.1.out_type != Type::None
//                                 {
//                                     todo!(
//                                 "{} is an empty array and has no output type, but found {:#?}",
//                                 part.0,
//                                 part.1.out_type
//                             );
//                                 }
//                                 return (
//                                     part.0,
//                                     Node {
//                                         in_type: Type::None,
//                                         out_type: Type::Context,
//                                         value: part.1.value.clone(),
//                                     },
//                                 );
//                             }
//                         }
//                         Value::TypeAlias(_) => todo!(),
//                         Value::TypeIdentity(_) => todo!(),

//                         Value::Discard => {
//                             // in
//                             if part.1.in_type != Type::Unknown && part.1.in_type != Type::None {
//                                 todo!(
//                                     "{} is a discard and has no input type, but found {:#?}",
//                                     part.0,
//                                     part.1.in_type
//                                 );
//                             }
//                             // out
//                             if part.1.out_type != Type::Unknown && part.1.out_type != Type::None {
//                                 todo!(
//                                     "{} is a discard and has no output type, but found {:#?}",
//                                     part.0,
//                                     part.1.out_type
//                                 );
//                             }
//                             return (
//                                 part.0,
//                                 Node {
//                                     in_type: Type::None,
//                                     out_type: Type::None,
//                                     value: part.1.value.clone(),
//                                 },
//                             );
//                         }
//                         Value::Integer(value) => {
//                             if part.1.in_type != Type::None {
//                                 panic!(
//                                     "Integer must have a None input type but found {:#?}",
//                                     part.1.in_type
//                                 );
//                             }
//                             if part.1.out_type != Type::Atom {
//                                 panic!(
//                             "Integer must have an output of an array of atoms but found {:#?}",
//                             part.1.in_type
//                         );
//                             }
//                             return (
//                                 part.0,
//                                 Node {
//                                     in_type: Type::None,
//                                     out_type: Type::Atom,
//                                     value: part.1.value.clone(),
//                                 },
//                             );
//                         }
//                         Value::IdentifierChain(value) => todo!(),
//                         Value::String(_) => {
//                             if part.1.in_type != Type::None {
//                                 panic!(
//                                     "String must have a None input type but found {:#?}",
//                                     part.1.in_type
//                                 );
//                             }
//                             if part.1.out_type != Type::Array(Box::new(Type::Atom)) {
//                                 panic!(
//                             "String must have an output of an array of atoms but found {:#?}",
//                             part.1.in_type
//                         );
//                             }
//                             return (
//                                 part.0,
//                                 Node {
//                                     in_type: Type::None,
//                                     out_type: Type::Array(Box::new(Type::Atom)),
//                                     value: part.1.value.clone(),
//                                 },
//                             );
//                         }
//                     };
//                 })
//                 .collect();
//         }
//     }

//     new
// }

// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;

//     use crate::{
//         modulizer::{to_module_tree, Tree},
//         parser::{Type, Value},
//     };

//     use super::type_out;

//     #[test]
//     pub fn types_simple_declarations() {
//         let module = to_module_tree("test/types_simple_declarations.we");
//         let typed: crate::modulizer::Tree = type_out(module);
//         println!("typed: {:#?}", typed);

//         if let Tree::Leaf(id, inner) = typed {
//             assert_eq!(id, *"types_simple_declarations");

//             // function
//             let function = inner
//                 .map
//                 .get("function")
//                 .expect("function was not in the map");
//             assert_eq!(function.in_type, Type::None);
//             assert_eq!(function.out_type, Type::None);
//             assert_eq!(*function.value, Value::Function(vec![]));

//             // array
//             let array = inner.map.get("array").expect("array was not in the map");
//             assert_eq!(array.in_type, Type::None);
//             assert_eq!(array.out_type, Type::Array(Box::new(Type::None)));
//             assert_eq!(*array.value, Value::Array(vec![]));

//             // map
//             let map = inner.map.get("map").expect("map was not in the map");
//             assert_eq!(map.in_type, Type::None);
//             assert_eq!(map.out_type, Type::Context);
//             assert_eq!(*map.value, Value::Map(HashMap::new()));

//             // string
//             let string = inner.map.get("string").expect("string was not in the map");
//             assert_eq!(string.in_type, Type::None);
//             assert_eq!(string.out_type, Type::Array(Box::new(Type::Atom)));
//             assert_eq!(*string.value, Value::String("".to_string()));

//             // int
//             let int = inner.map.get("int").expect("int was not in the map");
//             assert_eq!(int.in_type, Type::None);
//             assert_eq!(int.out_type, Type::Atom);
//             assert_eq!(*int.value, Value::Integer(10));

//             // discard
//             let discard = inner
//                 .map
//                 .get("discard")
//                 .expect("discard was not in the map");
//             assert_eq!(discard.in_type, Type::None);
//             assert_eq!(discard.out_type, Type::None);
//             assert_eq!(*discard.value, Value::Discard);
//         } else {
//             panic!("Was not leaf!");
//         }
//     }

//     #[test]
//     pub fn types_function_with_reference() {}

//     #[test]
//     pub fn types_parameter() {}

//     #[test]
//     pub fn types_post_declarations() {}

//     #[test]
//     pub fn reports_type_parameter_mismatch() {}

//     #[test]
//     pub fn reports_step_with_wrong_type() {}

//     #[test]
//     pub fn reports_array_with_wrong_type() {}
// }
