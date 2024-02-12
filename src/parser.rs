use core::panic;
use std::collections::HashMap;

use crate::lexer::{Token, TokenContext};

pub type IdentifierChain = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Identity(Box<Type>),
    Alias(Box<Type>),

    Context(HashMap<String, Box<Type>>),
    Array(Box<Type>),
    Function(Box<Type>),

    Reference(IdentifierChain),
    
    Atom,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub in_type: Type,
    pub out_type: Type,
    pub value: Box<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Module{usings: Vec<IdentifierChain>, map:   HashMap<String, Node>},
    
    Array(Vec<Node>),
    Map(HashMap<String, Node>),
    Function(Vec<Node>),

    TypeAlias(Node),
    TypeIdentity(Node),
    Discard,

    Atom(u32),
    IdentifierChain(IdentifierChain),
    String(String),
}

pub fn parse_module(context: &mut TokenContext) -> Value {
    let mut usings: Vec<IdentifierChain> = vec![];
    let mut map: HashMap<String, Node> = HashMap::new();

    // Set usings
    while context.current == Some(Token::UseKeyword) {
        context.get_next();

        let chain = parse_value(context).unwrap_or_else(|| panic!("Missing identifier after 'use' keyword"));

        match *chain.value {
            Value::IdentifierChain(c) => usings.push(c),
            other => todo!(
                "Expected an identifier chain after using, but found: {:#?}",
                other
            ),
        };
    }

    // Set map
    while let Some(Token::Identifier(id)) = context.current.clone() {
        let key = id;

        if context.get_next() != Some(Token::Define) {
            // TODO: this can happen at the end of a file
            // which means something is not get_next-ing.
            // Really we need to rework the entire get_next
            // philosophy, so instead of root-causing, I'm opting
            // to just put this condition and move on.
            if context.get_next().is_none() {
                break;
            }

            todo!("Missing define symbol in declaration of {:#?}", key);
        }
        context.get_next();

        let value = parse_value(context);

        map.insert(
            key.clone(),
            value.unwrap_or_else(|| panic!("Missing value after declaration of {}", key)),
        );
    }

    Value::Module { usings, map }
}

fn parse_value(context: &mut TokenContext) -> Option<Node> {
    match context.current {
        Some(Token::ListStart) => Some(parse_list(context)),
        Some(Token::ListEnd) => todo!("Found mismatched list end"),

        Some(Token::MapStart) => Some(parse_map(context)),
        Some(Token::MapEnd) => todo!("Found mismatched map end"),

        Some(Token::FunctionStart) => Some(parse_function(context)),
        Some(Token::FunctionEnd) => todo!("Found mismatched function end"),

        Some(Token::TypeParameterStart) => Some(parse_type_parameter(context)),
        Some(Token::TypeParameterEnd) => todo!("Found mismatched type parameter end"),

        Some(Token::TypeAlias) => Some(parse_type_alias(context)),
        Some(Token::TypeIdentity) => Some(parse_type_identity(context)),

        Some(Token::ClauseSeparator) => todo!(),
        Some(Token::ListSeparator) => todo!(),
        Some(Token::IdentifierSeparator) => todo!(),
        Some(Token::Define) => todo!(),

        Some(Token::MacroSymbol) => todo!("Macros are not implemented yet"),

        Some(Token::Identifier(_)) => Some(parse_identifier_chain(context)),

        Some(Token::DiscardSymbol) => {
            context.get_next();
            Some(Node {
                in_type: Type::None,
                out_type: Type::None,
                value: Box::new(Value::Discard),
            })
        }
        Some(Token::Integer(value)) => {
            context.get_next();
            Some(Node {
                in_type: Type::None,
                out_type: Type::Atom,
                value: Box::new(Value::Atom(value.to_owned())),
            })
        }
        Some(Token::String(ref value)) => {
            let string = value.clone();

            context.get_next();

            Some(Node {
                in_type: Type::None,
                out_type: Type::Array(Box::new(Type::Atom)),
                value: Box::new(Value::String(string)),
            }) 
        },
        Some(Token::UseKeyword) => todo!("Found use keyword not at the top of the file. If you're trying to name a variable 'use', please choose something else so we can distinguish between using statements and variables."),
        Some(Token::Unknown(ref c)) => todo!("We never have found the end of the universe {}", &c),
        None => todo!("No one knows where none will goes"),
    }
}

fn parse_list(context: &mut TokenContext) -> Node {
    match context.current {
        Some(Token::ListStart) => {}
        _ => panic!("Tried to parse a non-list as a list"),
    }
    context.get_next();

    let mut list: Vec<Node> = vec![];

    let mut last_was_separator = true;
    let mut array_type: Option<Type> = None;
    loop {
        match context.current {    
            Some(Token::ListEnd) => {
                context.get_next();
                break;
            },
            Some(Token::ListSeparator) => {
                if last_was_separator {
                    todo!("Found a duplicate separator in a list");
                }
                context.get_next();
                last_was_separator = true;
            },
            Some(ref token) => {
                if !last_was_separator {
                    todo!("Found a missing separator. {:#?}", token)
                }
                last_was_separator = false;

                let next_value = parse_value(context);
                if next_value.is_none() {
                    panic!("Unknown token supplied to list");
                }

                let value = next_value.unwrap();

                if array_type.is_none() {
                    array_type = Some(value.clone().out_type);
                } else if value.out_type != array_type.clone().unwrap() {
                    let mut is_reference = false;
                    if let Type::Reference(_) = value.out_type {
                        is_reference = true;
                    } 
                    if let Type::Reference(_) = array_type.clone().unwrap() {
                        is_reference = true;
                    } 
                    if !is_reference {
                        todo!("Found missmatched type for array: found {:#?}, expected: {:#?}", value.out_type, array_type);
                    }
                }

                list.push(value);
            }
            None => break,
        }
    }

    return Node {
        in_type: infer_complex_type(list.iter().map(|i| &i.in_type).collect()),
        out_type: Type::Array(Box::new(array_type.unwrap_or(Type::None))),
        value: Box::new(Value::Array(list)),
    };
}

fn parse_map(context: &mut TokenContext<'_>) -> Node {
    match context.current {
        Some(Token::MapStart) => {}
        _ => panic!("Tried to parse a non-map as a map"),
    }

    let mut map: HashMap<String, Node> = HashMap::new();

    loop {
        let next = context.get_next();
        let identifier: String = match next {
            Some(Token::MapEnd) => {
                context.get_next();
                break;
            },
            Some(Token::Identifier(string)) => string,
            Some(_) => todo!("Was not identifier: {:#?}", next),
            None => todo!("Identifier was none"),
        };

        if map.contains_key(&identifier) {
            todo!("Identifier already used: {}", identifier);
        }

        match context.get_next() {
            Some(Token::Define) => {}
            Some(token) => todo!("{:#?} where Define should be", token),
            None => todo!("Missing define symbol"),
        }

        context.get_next(); // skip define symbol
        match parse_value(context) {
            Some(value) => map.insert(identifier, value),
            None => todo!("Missing value"),
        };
    }

    Node {
        in_type: infer_complex_type(map.clone().values().map(|e| &e.in_type).collect()),
        out_type: Type::Context(map.clone().into_iter().map(|e| (e.0, Box::new(e.1.out_type))).collect()),
        value: Box::new(Value::Map(map)),
    }
}

fn parse_function(context: &mut TokenContext<'_>) -> Node {
    match context.current {
        Some(Token::FunctionStart) => {}
        _ => panic!("Tried to parse a non-function as a function"),
    };
    context.get_next();

    let mut steps: Vec<Node> = vec![];
    let mut chain: Vec<Node> = vec![];
    loop {
        match context.current {
            Some(Token::FunctionEnd) => {
                context.get_next();
                break;
            }
            Some(Token::ClauseSeparator) => {
                steps.append(&mut chain);
                chain.clear();
                context.get_next();
            }
            Some(_) => {
                match parse_value(context) {
                    Some(value) => chain.insert(0, value),
                    None => todo!("Couldn't pase value in function"),
                };
            }
            None => todo!("Function not closed"),
        };
    }

    steps.append(&mut chain);

    let temp_steps = steps.clone();
    let in_type = temp_steps.first().unwrap_or(&Node { in_type: Type::None, out_type: Type::None, value: Box::new(Value::Discard) }).in_type.clone();
    let out_steps = steps.clone();
    let out_type = out_steps.last().unwrap_or(&Node { in_type: Type::None, out_type: Type::None, value: Box::new(Value::Discard) }).out_type.clone();

    Node {
        in_type,
        out_type: Type::Function(Box::new(out_type)),
        value: Box::new(Value::Function(steps)),
    }
}

fn parse_type_parameter(context: &mut TokenContext<'_>) -> Node {
    // Start
    match context.current {
        Some(Token::TypeParameterStart) => {}
        _ => todo!("Tried to parse a non-type-parameter as a type parameter"),
    };
    context.get_next();

    // In
    let in_type = parse_value(context)
        .expect("Could not find in type while parsing type parameter")
        .out_type;

    // Separator
    match &context.current {
        Some(Token::ListSeparator) => {}
        token => todo!("Missing separator in type parameter: {:#?}", token),
    };
    context.get_next();

    // Out
    let out_type = parse_value(context)
        .expect("Could not find out type while parsing type parameter")
        .out_type;

    // End
    match &context.current {
        Some(Token::TypeParameterEnd) => {}
        val => todo!(
            "Missing type parameter end symbol in type parameter: {:#?}",
            val
        ),
    };
    context.get_next();

    // Value
    let value = parse_value(context)
        .expect("Could not find a subsequent value after the type parameter")
        .value;

    // TODO: replace in a smart manner. Don't clobber existing info or trust human hands.
    Node {
        in_type,
        out_type,
        value,
    }
}

fn parse_type_alias(context: &mut TokenContext<'_>) -> Node {
    // Start
    match context.current {
        Some(Token::TypeAlias) => {}
        _ => todo!("Tried to parse a non-type-parameter as a type parameter"),
    };
    context.get_next();

    let alias = parse_value(context);

    match alias {
        Some(a) => Node {
            in_type: a.clone().in_type,
            out_type: Type::Alias(Box::new(a.clone().out_type.clone())),
            value: Box::new(Value::TypeAlias(a)),
        },
        None => todo!("No value parsed after alias"),
    }
}

fn parse_type_identity(context: &mut TokenContext<'_>) -> Node {
    // Start
    match context.current {
        Some(Token::TypeIdentity) => {}
        _ => todo!("Tried to parse a non-type-parameter as a type parameter"),
    };
    context.get_next();

    let id = parse_value(context);

    match id {
        Some(i) => Node {
            in_type: i.clone().in_type,
            out_type: Type::Identity(Box::new(i.clone().out_type)),
            value: Box::new(Value::TypeIdentity(i)),
        },
        None => todo!("No value parsed after identity"),
    }
}

fn parse_identifier_chain(context: &mut TokenContext<'_>) -> Node {
    match context.current {
        Some(Token::Identifier(_)) => {}
        _ => todo!("Tried to parse a non-identifier as an identifier chain"),
    };

    let mut chain: Vec<String> = vec![];
    let mut found_continue = true;
    loop {
        match context.current {
            Some(Token::Identifier(ref value)) => {
                if !found_continue {
                    break;
                }
                found_continue = false;

                chain.push(value.to_owned());

                context.get_next();
            }
            Some(Token::IdentifierSeparator) => {
                found_continue = true;
                context.get_next();
            }
            Some(_) => break,
            None => todo!(),
        };
    }

    Node {
        in_type: Type::Reference(chain.clone()),
        out_type: Type::Reference(chain.clone()),
        value: Box::new(Value::IdentifierChain(chain.clone())),
    }
}

fn infer_complex_type(types: Vec<&Type>) -> Type {
    if types.is_empty() {
        return Type::None;
    }
    
    types.into_iter().fold(Type::None, accumulate_type)
}

fn accumulate_type(acc: Type, next: &Type) -> Type {
    match (acc.clone(), next) {
        (Type::Alias(a), Type::Alias(n)) => accumulate_type(*a, n),
        (Type::Alias(a), n) => accumulate_type(*a, n),
        (a, Type::Alias(n)) => accumulate_type(a, n), 

        (Type::Identity(a), Type::Identity(n)) => {
            if &a != n {
                todo!("Identities were not the same");
            }
            acc
        },
        (Type::Context(a), Type::Context(n)) => {
            let mut map = a.clone();

            n.iter().for_each(|kvp| {
                if map.contains_key(kvp.0) {
                    let current = map.get(kvp.0);
                    let suggested = kvp.1;

                    if current != Some(suggested) {
                        panic!("Tried to use {} as both {:#?} and {:#?}", kvp.0, current, suggested)
                    }
                } else {
                    map.insert(kvp.0.clone(), kvp.1.clone());
                }
            });

            Type::Context(map)
        },
        (Type::Context(_), Type::Array(_)) => todo!("Tried to use a context and an array in the same input"),
        (Type::Context(_), Type::Function(_)) => todo!(),
        (Type::Context(_), Type::Reference(_)) => todo!(),
        (Type::Context(_), Type::Atom) => todo!(),
        (Type::Context(_), Type::None) => acc,
        (Type::Array(_), Type::Context(_)) => todo!(),
        (Type::Array(_), Type::Array(_)) => todo!(),
        (Type::Array(_), Type::Function(_)) => todo!(),
        (Type::Array(_), Type::Reference(_)) => todo!(),
        (Type::Array(_), Type::Atom) => todo!(),
        (Type::Array(_), Type::None) => todo!(),
        (Type::Function(_), Type::Context(_)) => todo!(),
        (Type::Function(_), Type::Array(_)) => todo!(),
        (Type::Function(_), Type::Function(_)) => todo!(),
        (Type::Function(_), Type::Reference(_)) => todo!(),
        (Type::Function(_), Type::Atom) => todo!(),
        (Type::Function(_), Type::None) => todo!(),
        (Type::Reference(_), Type::Context(_)) => todo!(),
        (Type::Reference(_), Type::Array(_)) => todo!(),
        (Type::Reference(_), Type::Function(_)) => todo!(),
        (Type::Reference(_), Type::Reference(_)) => todo!(),
        (Type::Reference(_), Type::Atom) => todo!(),
        (Type::Reference(_), Type::None) => todo!(),
        (Type::Atom, Type::Context(_)) => todo!(),
        (Type::Atom, Type::Array(_)) => todo!(),
        (Type::Atom, Type::Function(_)) => todo!(),
        (Type::Atom, Type::Reference(_)) => todo!(),
        (Type::Atom, Type::Atom) => todo!(),
        (Type::Atom, Type::None) => todo!(),
        (Type::None, Type::Context(_)) => todo!(),
        (Type::None, Type::Array(_)) => todo!(),
        (Type::None, Type::Function(_)) => todo!(),
        (Type::None, Type::Reference(_)) => todo!(),
        (Type::None, Type::Atom) => todo!(),
        (Type::None, Type::None) => Type::None,

        (Type::Identity(_), _) => todo!("Result was an identity, but the next value was not"),
        (_, Type::Identity(_)) => todo!("Next value was an identity, but the result was not"),
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        lexer::TokenContext,
        parser::{parse_value, Type, Value},
    };

    use super::parse_module;

    #[test]
    fn parses_list_correctly() {
        let mut context = TokenContext::new("[[], [], []]");
        let result = parse_value(&mut context);

        assert!(result.is_some(), "List was none");
        match result {
            Some(node) => {
                let value = *node.value;
                match value {
                    Value::Array(list) => {
                        assert_eq!(list.len(), 3);
                        for ele in list {
                            match *ele.value {
                                Value::Array(_) => {}
                                _ => panic!("List element was a different type"),
                            }
                        }
                    }
                    _ => panic!("List was a different type"),
                };
            }
            None => panic!("List was None"),
        }
    }

    #[test]
    fn finds_separator() {
        let mut context = TokenContext::new("[local, in]");
        // This has produced a missing separator error in the past
        parse_value(&mut context);
    }

    #[test]
    fn parses_function_steps_in_order() {
        let mut context = TokenContext::new("(2 1 0 ; 3 ; 4 ; 8 7 6 5)");
        let result = *parse_value(&mut context).unwrap().value;

        match result {
            Value::Function(steps) => {
                for (current, step_box) in steps.into_iter().enumerate() {
                    let step = *step_box.value;
                    match step {
                        Value::Atom(value) => {
                            assert_eq!(value as usize, current);
                        }
                        _ => panic!("Node was not an integer"),
                    }
                }
            }
            _ => panic!("Node was not a function"),
        }
    }

    #[test]
    fn parses_identifier_chain() {
        let mut context = TokenContext::new("hello.from.the.out.side");
        let result = *parse_value(&mut context).unwrap().value;

        match result {
            Value::IdentifierChain(identifiers) => {
                assert_eq!("hello", identifiers.first().unwrap());
                assert_eq!("from", identifiers.get(1).unwrap());
                assert_eq!("the", identifiers.get(2).unwrap());
                assert_eq!("out", identifiers.get(3).unwrap());
                assert_eq!("side", identifiers.get(4).unwrap());
            }
            _ => panic!("Node was not a function"),
        }
    }

    #[test]
    fn parses_type_parameter() {
        let mut context = TokenContext::new("<first.second, _>(log)");
        let result = parse_value(&mut context).unwrap();

        match result.in_type {
            Type::Reference(_) => {}
            tp => panic!("Expected in to be Reference, but was {:#?}", tp),
        }

        match result.out_type {
            Type::None => {}
            tp => panic!("Expected in to be None, but was {:#?}", tp),
        }
    }

    #[test]
    fn parses_module() {
        let mut context = TokenContext::new(
            r#"
        use first.thing
        use other
        
        fn: (1 2 3)
        "#,
        );
        if let Value::Module{usings, map} = parse_module(&mut context){
            assert_eq!(
                usings.first(),
                Some(&vec!["first".to_owned(), "thing".to_owned()])
            );
            assert_eq!(usings.get(1), Some(&vec!["other".to_owned()]));

            assert_eq!(map.len(), 1);
            assert!(
                map.get("fn").is_some(),
                "Expected function was not parsed."
            );
        }
    }
}
