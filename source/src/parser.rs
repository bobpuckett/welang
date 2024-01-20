use std::collections::HashMap;

use crate::lexer::{Parsable, Token, TokenContext};

pub type IdentifierChain = Vec<String>;

#[derive(Debug, Clone)]
pub enum Type {
    Identity(IdentifierChain),
    Alias,

    Context,
    Array(Box<Type>),

    Atom,
    Generic,
    Reference,
    None,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub in_type: Type,
    pub out_type: Type,
    pub value: Box<Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Array(Vec<Node>),
    Map(HashMap<String, Node>),
    Function(Vec<Node>),

    TypeAlias(Node),
    TypeIdentity(Node),
    Discard,

    Integer(u32),
    IdentifierChain(IdentifierChain),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Module {
    usings: Vec<IdentifierChain>,
    map: HashMap<String, Node>,
}

pub fn parse_module(context: &mut TokenContext) -> Module {
    let mut usings: Vec<IdentifierChain> = vec![];
    let mut map: HashMap<String, Node> = HashMap::new();

    // Set usings
    while context.current == Some(Token::UseKeyword) {
        context.get_next();

        let chain = parse_value(context).expect(&format!("Missing identifier after 'use' keyword"));

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
            todo!("Missing define symbol in declaration of {:#?}", key);
        }
        context.get_next();

        let value = parse_value(context);

        map.insert(
            key.clone(),
            value.expect(&format!("Missing value after declaration of {}", key)),
        );
    }

    Module { usings, map }
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

        Some(Token::TypeAlias) => None,
        Some(Token::TypeIdentity) => None,

        Some(Token::ClauseSeparator) => todo!(),
        Some(Token::ListSeparator) => todo!(),
        Some(Token::IdentifierSeparator) => todo!(),
        Some(Token::Define) => todo!(),

        Some(Token::MacroSymbol) => todo!(),

        Some(Token::DiscardSymbol) => {
            context.get_next();
            Some(Node {
                in_type: Type::None,
                out_type: Type::None,
                value: Box::new(Value::Discard),
            })
        }
        Some(Token::Identifier(_)) => Some(parse_identifier_chain(context)),
        Some(Token::Integer(value)) => {
            context.get_next();
            Some(Node {
                in_type: Type::None,
                out_type: Type::Atom,
                value: Box::new(Value::Integer(value.to_owned())),
            })
        }
        Some(Token::String(ref value)) => Some(Node {
            in_type: Type::None,
            out_type: Type::Array(Box::new(Type::Atom)),
            value: Box::new(Value::String(value.clone())),
        }),
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

    let mut list: Vec<Node> = vec![];

    let mut last_was_separator = true;

    loop {
        match context.get_next() {
            Some(Token::ListEnd) => break,
            Some(Token::ListSeparator) => last_was_separator = true,
            Some(ref token) => {
                if !last_was_separator {
                    todo!("Found a missing separator. {:#?}", token)
                }
                last_was_separator = false;

                let next_value = parse_value(context);
                if next_value.is_none() {
                    panic!("Unknown token supplied to list");
                }

                list.push(next_value.unwrap());
            }
            None => break,
        }
    }

    return Node {
        in_type: Type::Unknown,
        out_type: Type::Array(Box::new(Type::Unknown)),
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
            Some(Token::MapEnd) => break,
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
        in_type: Type::Unknown,
        out_type: Type::Context,
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

    Node {
        in_type: Type::Unknown,
        out_type: Type::Unknown,
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
        in_type: Type::Unknown,
        out_type: Type::Reference,
        value: Box::new(Value::IdentifierChain(chain)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::TokenContext,
        parser::{parse_value, Node, Type, Value},
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
                                _ => assert!(false, "List element was a different type"),
                            }
                        }
                    }
                    _ => assert!(false, "List was a different type"),
                };
            }
            None => assert!(false, "List was None"),
        }
    }

    #[test]
    fn parses_function_steps_in_order() {
        let mut context = TokenContext::new("(2 1 0 ; 3 ; 4 ; 8 7 6 5)");
        let result = *parse_value(&mut context).unwrap().value;

        match result {
            Value::Function(steps) => {
                let mut current = 0;
                for step_box in steps {
                    let step = *step_box.value;
                    match step {
                        Value::Integer(value) => {
                            assert_eq!(value, current);
                        }
                        _ => assert!(false, "Node was not an integer"),
                    }

                    current += 1;
                }
            }
            _ => assert!(false, "Node was not a function"),
        }
    }

    #[test]
    fn parses_identifier_chain() {
        let mut context = TokenContext::new("hello.from.the.out.side");
        let result = *parse_value(&mut context).unwrap().value;

        match result {
            Value::IdentifierChain(identifiers) => {
                assert_eq!("hello", identifiers.get(0).unwrap());
                assert_eq!("from", identifiers.get(1).unwrap());
                assert_eq!("the", identifiers.get(2).unwrap());
                assert_eq!("out", identifiers.get(3).unwrap());
                assert_eq!("side", identifiers.get(4).unwrap());
            }
            _ => assert!(false, "Node was not a function"),
        }
    }

    #[test]
    fn parses_type_parameter() {
        let mut context = TokenContext::new("<first.second, _>(log)");
        let result = parse_value(&mut context).unwrap();

        match result.in_type {
            Type::Reference => {}
            tp => assert!(1 == 0, "Expected in to be Reference, but was {:#?}", tp),
        }

        match result.out_type {
            Type::None => {}
            tp => assert!(1 == 0, "Expected in to be None, but was {:#?}", tp),
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
        let result = parse_module(&mut context);

        assert_eq!(
            result.usings.get(0),
            Some(&vec!["first".to_owned(), "thing".to_owned()])
        );
        assert_eq!(result.usings.get(1), Some(&vec!["other".to_owned()]));

        assert_eq!(result.map.len(), 1);
        assert!(result.map.get("fn").is_some(), "Expected function was not parsed.");
    }
}
