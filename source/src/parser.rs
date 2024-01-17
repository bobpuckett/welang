use std::collections::HashMap;

use crate::lexer::{Parsable, Token, TokenContext};

#[derive(Debug, Clone)]
pub enum Node {
    List(Vec<Box<Node>>),
    Map(HashMap<String, Box<Node>>),
    Function(Vec<Box<Node>>),

    TypeParameter(Box<Node>, Box<Node>),
    TypeAlias(Box<Node>),
    TypeIdentity(Box<Node>),
    Discard,

    Integer(u32),
    IdentifierChain(Vec<String>),
    String(String),
}

pub fn parse_value(context: &mut TokenContext) -> Option<Box<Node>> {
    match context.current {
        Some(Token::ListStart) => Some(Box::new(parse_list(context))),
        Some(Token::ListEnd) => todo!("Found mismatched list end"),
        Some(Token::MapStart) => Some(Box::new(parse_map(context))),
        Some(Token::MapEnd) => todo!("Found mismatched map end"),
        Some(Token::FunctionStart) => Some(Box::new(parse_function(context))),
        Some(Token::FunctionEnd) => todo!("Found mismatched function end"),
        Some(Token::TypeParameterStart) => None,
        Some(Token::TypeParameterEnd) => None,
        Some(Token::TypeAlias) => None,
        Some(Token::TypeIdentity) => None,
        Some(Token::ClauseSeparator) => todo!(),
        Some(Token::ListSeparator) => todo!(),
        Some(Token::IdentifierSeparator) => todo!(),
        Some(Token::Define) => todo!(),
        Some(Token::MacroSymbol) => todo!(),
        Some(Token::DiscardSymbol) => None,
        Some(Token::Integer(value)) => Some(Box::new(Node::Integer(value))),
        Some(Token::Identifier(_)) => Some(Box::new(parse_identifier_chain(context))),
        Some(Token::String(ref value)) => Some(Box::new(Node::String(value.clone()))),
        Some(Token::Unknown) => todo!("We never have found the end of the chain"),
        None => todo!("No one knows where none will goes"),
    }
}

fn parse_list(context: &mut TokenContext) -> Node {
    match context.current {
        Some(Token::ListStart) => {}
        _ => panic!("Tried to parse a non-list as a list"),
    }

    let mut list: Vec<Box<Node>> = vec![];

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

    return Node::List(list);
}

fn parse_map(context: &mut TokenContext<'_>) -> Node {
    match context.current {
        Some(Token::MapStart) => {}
        _ => panic!("Tried to parse a non-map as a map"),
    }

    let mut map: HashMap<String, Box<Node>> = HashMap::new();

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

    Node::Map(map)
}

fn parse_function(context: &mut TokenContext<'_>) -> Node {
    match context.current {
        Some(Token::FunctionStart) => {}
        _ => panic!("Tried to parse a non-function as a function"),
    };

    let mut steps: Vec<Box<Node>> = vec![];
    let mut chain: Vec<Box<Node>> = vec![];
    loop {
        match context.get_next() {
            Some(Token::FunctionEnd) => break,
            Some(Token::ClauseSeparator) => {
                steps.append(&mut chain);
                chain.clear();
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
    chain.clear();

    Node::Function(steps)
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

    Node::IdentifierChain(chain)
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::TokenContext,
        parser::{parse_value, Node},
    };

    #[test]
    fn parses_list_correctly() {
        let mut context = TokenContext::new("[[], [], []]");
        let result = parse_value(&mut context);

        assert!(result.is_some(), "List was none");
        match result {
            Some(bx) => {
                let node = *bx;
                match node {
                    Node::List(list) => {
                        assert_eq!(list.len(), 3);
                        for ele in list {
                            match *ele {
                                Node::List(_) => {}
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
        let result = *parse_value(&mut context).unwrap();

        match result {
            Node::Function(steps) => {
                let mut current = 0;
                for step_box in steps {
                    let step = *step_box;
                    match step {
                        Node::Integer(value) => {
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
        let result = *parse_value(&mut context).unwrap();

        match result {
            Node::IdentifierChain(identifiers) => {
                assert_eq!("hello", identifiers.get(0).unwrap());
                assert_eq!("from", identifiers.get(1).unwrap());
                assert_eq!("the", identifiers.get(2).unwrap());
                assert_eq!("out", identifiers.get(3).unwrap());
                assert_eq!("side", identifiers.get(4).unwrap());
            }
            _ => assert!(false, "Node was not a function"),
        }
    }
}
