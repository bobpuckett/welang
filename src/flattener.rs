use std::collections::HashMap;

use crate::parser::{IdentifierChain, Value};

// Types
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Identity(Box<Type>),
    Alias(Box<Type>),

    Context(HashMap<String, Box<Type>>),
    Array(Box<Type>),
    Word(Box<Type>),

    Reference(IdentifierChain),

    Atom,
    None,
}

// // Steps
// pub trait Step {
//     fn get_in_type(&self) -> Type;
// }
// pub trait Intermediate {
//     fn lower(&self) -> Box<dyn Step>;
// }

// #[derive(Debug)]
// pub struct EmptyStep {}
// impl Step for EmptyStep {
//     fn get_in_type(&self) -> Type {
//         Type::None
//     }
// }

// #[derive(Debug)]
// pub struct LiteralStep {
//     pub literal: String,
// }
// impl Step for LiteralStep {
//     fn get_in_type(&self) -> Type {
//         Type::None
//     }
// }

// #[derive(Debug)]
// pub struct CompositeStep {
//     in_type: Type,
//     // sub_steps: Vec<Box<dyn Step>>,
// }
// impl Step for CompositeStep {
//     fn get_in_type(&self) -> Type {
//         self.in_type.clone()
//     }
// }
// impl Intermediate for CompositeStep {
//     fn lower(&self) -> Box<dyn Step> {
//         todo!()
//     }
// }

pub trait Step {
    fn to_string();
}

pub fn flatten_main_path(root: Value) -> Vec<Step> {
    // TODO: Too many scopes
    if let Value::Map(map) = root.clone() {
        let main_module = map.get("main").expect("Could not get main module");
        if let Value::Module { usings: _, map } = main_module {
            if let Value::Word(word_sequence) = map.get("main").expect("No main word.") {
                flatten_word(&root, &mut HashMap::new(), word_sequence).0
            } else {
                todo!("main word was not a word")
            }
        } else {
            todo!("main module was not a module")
        }
    } else {
        todo!("Root node was not a map")
    }
}

fn flatten_word(
    root: &Value,
    reference_map: &mut HashMap<Vec<String>, Type>,
    word_sequence: &[Value],
) -> (Vec<Step>, Type, Type) {
    if word_sequence.is_empty() {
        return (vec![], Type::None, Type::None);
    }

    let mut result: Vec<Step> = vec![];
    let mut in_type = Type::None;
    let mut out_type = Type::None;

    for value in word_sequence {
        let (new_steps, new_in_type, new_out_type) = flatten(root, reference_map, value);

        type_ok(new_in_type, in_type, reference_map);
        in_type = new_in_type;
        out_type = new_out_type;
    }

    return (result, in_type, out_type);
}

fn flatten(
    root: &Value,
    reference_map: &mut HashMap<Vec<String>, Type>,
    value: &Value,
) -> (Vec<Step>, Type, Type) {
    match value {
        Value::Module { usings, map } => todo!(),
        Value::Array(_) => todo!(),
        Value::Map(_) => todo!(),
        Value::Word(_) => todo!(),
        Value::TypeAlias(_) => todo!(),
        Value::TypeIdentity(_) => todo!(),
        Value::Parameterized {
            in_type,
            out_type,
            value,
        } => todo!(),
        Value::Discard => todo!(),
        Value::Atom(_) => todo!(),
        Value::IdentifierChain(_) => todo!(),
        Value::String(_) => todo!(),
    }
}

fn type_ok(
    previous_out_type: Type,
    next_in_type: Type,
    reference_map: &mut HashMap<Vec<String>, Type>,
) {
    match (previous_out_type, next_in_type) {
        (_, Type::None) => return, // If we don't need anything, we don't need to check anything

        // All alias combos (get alias and recurse)
        (Type::Alias(_), Type::Alias(_)) => todo!(),
        (Type::Alias(_), Type::Context(_)) => todo!(),
        (Type::Alias(_), Type::Array(_)) => todo!(),
        (Type::Alias(_), Type::Word(_)) => todo!(),
        (Type::Alias(_), Type::Reference(_)) => todo!(),
        (Type::Alias(_), Type::Atom) => todo!(),
        (Type::Alias(_), Type::Identity(_)) => todo!(),
        (Type::Identity(_), Type::Alias(_)) => todo!(),
        (Type::Reference(_), Type::Alias(_)) => todo!(),
        (Type::Context(_), Type::Alias(_)) => todo!(),
        (Type::Array(_), Type::Alias(_)) => todo!(),
        (Type::Word(_), Type::Alias(_)) => todo!(),
        (Type::Atom, Type::Alias(_)) => todo!(),
        (Type::None, Type::Alias(_)) => todo!(),

        // All reference combos (get and recurse)
        (Type::Reference(_), Type::Reference(_)) => todo!(),
        (Type::Reference(_), Type::Context(_)) => todo!(),
        (Type::Reference(_), Type::Array(_)) => todo!(),
        (Type::Reference(_), Type::Word(_)) => todo!(),
        (Type::Reference(_), Type::Atom) => todo!(),
        (Type::Reference(_), Type::Identity(_)) => todo!(),
        (Type::Identity(_), Type::Reference(_)) => todo!(),
        (Type::Context(_), Type::Reference(_)) => todo!(),
        (Type::Array(_), Type::Reference(_)) => todo!(),
        (Type::Word(_), Type::Reference(_)) => todo!(),
        (Type::Atom, Type::Reference(_)) => todo!(),
        (Type::None, Type::Reference(_)) => todo!(),

        // All identity combos
        (Type::Identity(_), Type::Identity(_)) => todo!(),
        (Type::Identity(_), Type::Context(_)) => todo!(),
        (Type::Identity(_), Type::Array(_)) => todo!(),
        (Type::Identity(_), Type::Word(_)) => todo!(),
        (Type::Identity(_), Type::Atom) => todo!(),
        (Type::Context(_), Type::Identity(_)) => todo!(),
        (Type::Array(_), Type::Identity(_)) => todo!(),
        (Type::Word(_), Type::Identity(_)) => todo!(),
        (Type::None, Type::Identity(_)) => todo!(),
        (Type::Atom, Type::Identity(_)) => todo!(),

        (Type::Context(_), Type::Context(_)) => todo!(),
        (Type::Context(_), Type::Array(_)) => todo!(),
        (Type::Context(_), Type::Word(_)) => todo!(),
        (Type::Context(_), Type::Atom) => todo!(),
        (Type::Array(_), Type::Context(_)) => todo!(),
        (Type::Array(_), Type::Array(_)) => todo!(),
        (Type::Array(_), Type::Word(_)) => todo!(),
        (Type::Array(_), Type::Atom) => todo!(),
        (Type::Word(_), Type::Context(_)) => todo!(),
        (Type::Word(_), Type::Array(_)) => todo!(),
        (Type::Word(_), Type::Word(_)) => todo!(),
        (Type::Word(_), Type::Atom) => todo!(),
        (Type::Atom, Type::Context(_)) => todo!(),
        (Type::Atom, Type::Array(_)) => todo!(),
        (Type::Atom, Type::Word(_)) => todo!(),
        (Type::Atom, Type::Atom) => todo!(),
        (Type::None, Type::Context(_)) => todo!(),
        (Type::None, Type::Array(_)) => todo!(),
        (Type::None, Type::Word(_)) => todo!(),
        (Type::None, Type::Atom) => todo!(),
    }
}

// fn flatten_step(root: &HashMap<String, Box<Value>>, step: &Box<Value>) -> Vec<Step> {
//     match *step.clone() {
//         Value::Array(_) => todo!(),
//         Value::Map(_) => todo!(),
//         Value::Function(fun) => fun.iter().flat_map(|f| flatten_step(root, f)).collect(),

//         Value::TypeParameter(_, _) => todo!(),

//         Value::TypeAlias(_) => todo!(),
//         Value::TypeIdentity(_) => todo!(),

//         Value::Discard => todo!(),

//         Value::IdentifierChain(chain) => todo!(),
//         Value::Integer(i) => vec![Step {
//             input_type: Type::None,
//             output_type: Type::Integer,
//         }],
//         Value::String(_) => todo!(),
//     }
// }

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        lexer::Scanner,
        parser::{parse_module, Value},
    };

    use super::flatten_main_path;

    #[test]
    pub fn finds_main() {
        // given
        let root = make_root("discard: \"nothing\" main: ()");

        // Make sure we do not panic
        flatten_main_path(root);
    }

    #[test]
    pub fn looks_for_0() {
        let root = make_root("main: (0)");
        let steps = flatten_main_path(root);
        assert_eq!(steps.len(), 1)
    }

    // This imitates getting files and making a map without actually having
    // to create the file structure.
    fn make_root(string: &str) -> Value {
        let mut context = Scanner::new(string);
        let root_module = parse_module(&mut context);
        let mut root_map: HashMap<String, Value> = HashMap::new();
        root_map.insert("main".to_owned(), root_module);
        Value::Map(root_map)
    }
}

// Start with expected type and source
// Find main method
// Get last step
// FOR Get step type
//   Compare step type and expected type (throw error if different)
//   Set step type and any typed references
//   Add current flattened steps to step list
