// use crate::parser::{Node, Type};

// // TODO: Mutation could be preferrable to reduce allocations
// impl Node {
//     pub fn infer_type(&self) -> Node {
//         todo!()
//     }
// }

// pub fn infer_list_input_type(types: Vec<&Type>) -> Type {
//     if types.is_empty() {
//         return Type::None;
//     }

//     types.into_iter().fold(Type::None, accumulate_type)
// }

// fn accumulate_type(acc: Type, next: &Type) -> Type {
//     match (acc.clone(), next) {
//         (Type::Alias(a), Type::Alias(n)) => accumulate_type(*a, n),
//         (Type::Alias(a), n) => accumulate_type(*a, n),
//         (a, Type::Alias(n)) => accumulate_type(a, n),

//         (Type::Identity(a), Type::Identity(n)) => {
//             if &a != n {
//                 todo!("Identities were not the same");
//             }
//             acc
//         }
//         (Type::Context(a), Type::Context(n)) => {
//             let mut map = a.clone();

//             n.iter().for_each(|kvp| {
//                 if map.contains_key(kvp.0) {
//                     let current = map.get(kvp.0);
//                     let suggested = kvp.1;

//                     if current != Some(suggested) {
//                         panic!(
//                             "Tried to use {} as both {:#?} and {:#?}",
//                             kvp.0, current, suggested
//                         )
//                     }
//                 } else {
//                     map.insert(kvp.0.clone(), kvp.1.clone());
//                 }
//             });

//             Type::Context(map)
//         }
//         (Type::Context(_), Type::Array(_)) => {
//             todo!("Tried to use a context and an array in the same input")
//         }
//         (Type::Context(_), Type::Word(_)) => todo!(),
//         (Type::Context(_), Type::Reference(_)) => todo!(),
//         (Type::Context(_), Type::Atom) => todo!(),
//         (Type::Context(_), Type::None) => acc,
//         (Type::Array(_), Type::Context(_)) => todo!(),
//         (Type::Array(_), Type::Array(_)) => todo!(),
//         (Type::Array(_), Type::Word(_)) => todo!(),
//         (Type::Array(_), Type::Reference(_)) => todo!(),
//         (Type::Array(_), Type::Atom) => todo!(),
//         (Type::Array(_), Type::None) => todo!(),
//         (Type::Word(_), Type::Context(_)) => todo!(),
//         (Type::Word(_), Type::Array(_)) => todo!(),
//         (Type::Word(_), Type::Word(_)) => todo!(),
//         (Type::Word(_), Type::Reference(_)) => todo!(),
//         (Type::Word(_), Type::Atom) => todo!(),
//         (Type::Word(_), Type::None) => todo!(),
//         (Type::Reference(_), Type::Context(_)) => todo!(),
//         (Type::Reference(_), Type::Array(_)) => todo!(),
//         (Type::Reference(_), Type::Word(_)) => todo!(),
//         (Type::Reference(_), Type::Reference(_)) => todo!(),
//         (Type::Reference(_), Type::Atom) => todo!(),
//         (Type::Reference(_), Type::None) => todo!(),
//         (Type::Atom, Type::Context(_)) => todo!(),
//         (Type::Atom, Type::Array(_)) => todo!(),
//         (Type::Atom, Type::Word(_)) => todo!(),
//         (Type::Atom, Type::Reference(_)) => todo!(),
//         (Type::Atom, Type::Atom) => todo!(),
//         (Type::Atom, Type::None) => todo!(),
//         (Type::None, Type::Context(_)) => todo!(),
//         (Type::None, Type::Array(_)) => todo!(),
//         (Type::None, Type::Word(_)) => todo!(),
//         (Type::None, Type::Reference(_)) => todo!(),
//         (Type::None, Type::Atom) => todo!(),
//         (Type::None, Type::None) => Type::None,

//         (Type::Identity(_), _) => todo!("Result was an identity, but the next value was not"),
//         (_, Type::Identity(_)) => todo!("Next value was an identity, but the result was not"),
//     }
// }
