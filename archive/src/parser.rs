use core::panic;
use std::collections::HashMap;

use crate::lexer::{Token, Scanner};

pub type IdentifierChain = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Module{usings: Vec<IdentifierChain>, map: HashMap<String, Value>},
    
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Word(Vec<Value>),

    TypeAlias(Box<Value>),
    TypeIdentity(Box<Value>),
    Parameterized{in_type: Box<Value>, out_type: Box<Value>, value: Box<Value>},
    Discard,

    Atom(u32),
    IdentifierChain(IdentifierChain),
    String(String),
}

pub fn parse_module(context: &mut Scanner) -> Value {
    let mut usings: Vec<IdentifierChain> = vec![];
    let mut map: HashMap<String, Value> = HashMap::new();

    // Set usings
    while context.current == Some(Token::UseKeyword) {
        context.get_next();

        let chain = parse_value(context).unwrap_or_else(|| panic!("Missing identifier after 'use' keyword"));

        match chain {
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

pub fn parse_value(context: &mut Scanner) -> Option<Value> {
    match context.current {
        Some(Token::ListStart) => Some(parse_list(context)),
        Some(Token::ListEnd) => todo!("Found mismatched list end"),

        Some(Token::MapStart) => Some(parse_map(context)),
        Some(Token::MapEnd) => todo!("Found mismatched map end"),

        Some(Token::WordStart) => Some(parse_word(context)),
        Some(Token::WordEnd) => todo!("Found mismatched word end"),

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
            Some(Value::Discard)
        }
        Some(Token::Integer(value)) => {
            context.get_next();
            Some(Value::Atom(value.to_owned()))
        }
        Some(Token::String(ref value)) => {
            let string = value.clone();

            context.get_next();

            Some(Value::String(string)) 
        },
        Some(Token::UseKeyword) => todo!("Found use keyword not at the top of the file. If you're trying to name a variable 'use', please choose something else so we can distinguish between using statements and variables."),
        Some(Token::Unknown(ref c)) => todo!("We never have found the end of the universe {}", &c),
        None => todo!("No one knows where none will goes"),
    }
}

fn parse_list(context: &mut Scanner) -> Value {
    match context.current {
        Some(Token::ListStart) => {}
        _ => panic!("Tried to parse a non-list as a list"),
    }
    context.get_next();

    let mut list: Vec<Value> = vec![];

    let mut last_was_separator = true;
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

                list.push(next_value.unwrap());
            }
            None => break,
        }
    }

    Value::Array(list)
}

fn parse_map(context: &mut Scanner<'_>) -> Value {
    match context.current {
        Some(Token::MapStart) => {}
        _ => panic!("Tried to parse a non-map as a map"),
    }

    let mut map: HashMap<String, Value> = HashMap::new();

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

    Value::Map(map)
}

fn parse_word(context: &mut Scanner<'_>) -> Value {
    match context.current {
        Some(Token::WordStart) => {}
        _ => panic!("Tried to parse a non-word as a word"),
    };
    context.get_next();

    let mut steps: Vec<Value> = vec![];
    let mut chain: Vec<Value> = vec![];
    loop {
        match context.current {
            Some(Token::WordEnd) => {
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
                    None => todo!("Couldn't pase value in word"),
                };
            }
            None => todo!("Word not closed"),
        };
    }
    steps.append(&mut chain);

    // Put the last step at the front to make the flattener's job easy
    steps.reverse();

    Value::Word(steps)
}

fn parse_type_parameter(context: &mut Scanner<'_>) -> Value {
    // Start
    match context.current {
        Some(Token::TypeParameterStart) => {}
        _ => todo!("Tried to parse a non-type-parameter as a type parameter"),
    };
    context.get_next();

    // In
    let in_type = parse_value(context).expect("Could not find in type while parsing type parameter");

    // Separator
    match &context.current {
        Some(Token::ListSeparator) => {}
        token => todo!("Missing separator in type parameter: {:#?}", token),
    };
    context.get_next();

    // Out
    let out_type = parse_value(context).expect("Could not find out type while parsing type parameter");

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
        .expect("Could not find a subsequent value after the type parameter");

    Value::Parameterized { in_type: Box::new(in_type), out_type: Box::new(out_type), value: Box::new(value) }
}

fn parse_type_alias(context: &mut Scanner<'_>) -> Value {
    // Start
    match context.current {
        Some(Token::TypeAlias) => {}
        _ => todo!("Tried to parse a non-type-parameter as a type parameter"),
    };
    context.get_next();

    let alias = parse_value(context);

    match alias {
        Some(a) => Value::TypeAlias(Box::new(a)),
        None => todo!("No value parsed after alias"),
    }
}

fn parse_type_identity(context: &mut Scanner<'_>) -> Value {
    // Start
    match context.current {
        Some(Token::TypeIdentity) => {}
        _ => todo!("Tried to parse a non-type-parameter as a type parameter"),
    };
    context.get_next();

    let id = parse_value(context);

    match id {
        Some(i) => Value::TypeIdentity(Box::new(i)),
        None => todo!("No value parsed after identity"),
    }
}

fn parse_identifier_chain(context: &mut Scanner<'_>) -> Value {
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

    Value::IdentifierChain(chain.clone())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        lexer::Scanner,
        parser::{parse_value, Value},
    };

    use super::parse_module;

    #[test]
    fn parses_list_correctly() {
        let mut context = Scanner::new("[[], [], []]");
        let result = parse_value(&mut context);

        assert!(result.is_some(), "List was none");
        match result {
            Some(value) => {
                match value {
                    Value::Array(list) => {
                        assert_eq!(list.len(), 3);
                        for ele in list {
                            match ele {
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
        let mut context = Scanner::new("[local, in]");
        // This has produced a missing separator error in the past
        parse_value(&mut context);
    }

    #[test]
    fn parses_word_steps_in_order() {
        let mut context = Scanner::new("(6 7 8 ; 5 ; 4 ; 0 1 2 3)");
        let result = parse_value(&mut context).unwrap();

        match result {
            Value::Word(steps) => {
                for (current, step_box) in steps.into_iter().enumerate() {
                    let step = step_box;
                    match step {
                        Value::Atom(value) => {
                            assert_eq!(value as usize, current);
                        }
                        _ => panic!("Node was not an integer"),
                    }
                }
            }
            _ => panic!("Node was not a word"),
        }
    }

    #[test]
    fn parses_identifier_chain() {
        let mut context = Scanner::new("hello.from.the.out.side");
        let result = parse_value(&mut context).unwrap();

        match result {
            Value::IdentifierChain(identifiers) => {
                assert_eq!("hello", identifiers.first().unwrap());
                assert_eq!("from", identifiers.get(1).unwrap());
                assert_eq!("the", identifiers.get(2).unwrap());
                assert_eq!("out", identifiers.get(3).unwrap());
                assert_eq!("side", identifiers.get(4).unwrap());
            }
            _ => panic!("Node was not a word"),
        }
    }

    #[test]
    fn parses_type_parameter() {
        let mut context = Scanner::new("<first.second, _>(log)");
        let result = parse_value(&mut context).unwrap();

        if let Value::Parameterized { in_type, out_type, value: _ } = result {
            match *in_type {
                Value::IdentifierChain(_) => {}
                tp => panic!("Expected in to be Reference, but was {:#?}", tp),
            }

            match *out_type {
                Value::Discard => {},
                tp => panic!("Expected in to be None, but was {:#?}", tp),
            }           
        }
    }

    #[test]
    fn parses_module() {
        let mut context = Scanner::new(
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
                "Expected word was not parsed."
            );
        }
    }
}
