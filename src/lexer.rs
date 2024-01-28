#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    ListStart,
    ListEnd,
    MapStart,
    MapEnd,
    FunctionStart,
    FunctionEnd,
    TypeParameterStart,
    TypeParameterEnd,
    TypeAlias,
    TypeIdentity,

    ClauseSeparator,
    ListSeparator,
    IdentifierSeparator,

    Define,
    MacroSymbol,
    DiscardSymbol,
    
    UseKeyword,

    Integer(u32),
    Identifier(String),
    String(String),

    Unknown(String),
}

pub struct TokenContext<'a> {
    position: usize,
    source: &'a str,
    pub current: Option<Token>,
    column: i32,
    row: i32,
}

impl TokenContext<'_> {
    pub fn new(source: &str) -> TokenContext {
        let mut context = TokenContext {
            position: 0,
            source,
            current: None,
            column: 0,
            row: 0,
        };

        // Prime with first value
        context.get_next();

        return context;
    }

    fn increment_position(&mut self) {
        self.position += 1;
        self.column += 1;
    }

    pub fn get_next(&mut self) -> Option<Token> {
        if self.position > self.source.len() {
            return None;
        }

        let mut next_char = self.source.chars().nth(self.position);

        while next_char.is_some_and(|c| c.is_whitespace()) {
            self.position += 1;

            // TODO: crlf
            if next_char.unwrap() == '\n' {
                self.column = 0;
                self.row += 1;
            } else {
                self.column += 1;
            }

            next_char = self.source.chars().nth(self.position);
        }

        if self.position >= self.source.len() {
            return None;
        }

        let token = match next_char {
            Some('[') => Token::ListStart,
            Some(']') => Token::ListEnd,
            Some('{') => Token::MapStart,
            Some('}') => Token::MapEnd,
            Some('(') => Token::FunctionStart,
            Some(')') => Token::FunctionEnd,
            Some('<') => Token::TypeParameterStart,
            Some('>') => Token::TypeParameterEnd,
            Some('\'') => Token::TypeAlias,
            Some('*') => Token::TypeIdentity,
            Some(';') => Token::ClauseSeparator,
            Some(',') => Token::ListSeparator,
            Some(':') => Token::Define,
            Some('@') => Token::MacroSymbol,
            Some('_') => Token::DiscardSymbol,

            // TODO: Wow this is ugly
            Some('u') if self.source.chars().nth(self.position + 1) == Some('s')
              && self.source.chars().nth(self.position + 2) == Some('e')
              && self.source.chars().nth(self.position + 3).is_some_and(|c| c.is_whitespace()) => {
                self.increment_position();
                self.increment_position();
                self.increment_position();

                Token::UseKeyword
            }

            // TODO: reduce duplication
            Some(id) if id.is_lowercase() => {
                let mut identifier: String = id.to_string();

                self.increment_position();
                next_char = self.source.chars().nth(self.position);

                while self.position < self.source.len() && next_char.unwrap().is_alphanumeric() {
                    identifier += &next_char.unwrap().to_string();
                    self.increment_position();
                    next_char = self.source.chars().nth(self.position);
                }
                self.position -= 1;

                Token::Identifier(identifier)
            }

            Some(n) if n.is_digit(10) => {
                let mut number: u32 = n.to_digit(10).unwrap();

                self.increment_position();
                next_char = self.source.chars().nth(self.position);

                while self.position < self.source.len() && next_char.unwrap().is_digit(10) {
                    number *= 10;
                    number += next_char.unwrap().to_digit(10).unwrap();

                    self.increment_position();
                    next_char = self.source.chars().nth(self.position);
                }
                self.position -= 1;

                Token::Integer(number)
            }

            Some('"') => {
                let mut string: String = "".to_string();

                let mut last_char: Option<char> = None;
                self.increment_position();
                next_char = self.source.chars().nth(self.position);

                while self.position < self.source.len()
                    && (next_char.unwrap() != '"' && last_char != Some('\\'))
                {
                    string += &next_char.unwrap().to_string();
                    self.increment_position();
                    last_char = next_char;
                    next_char = self.source.chars().nth(self.position);
                }

                Token::String(string)
            }
            Some('.') => Token::IdentifierSeparator,

            c => Token::Unknown(c.unwrap().to_string()),
        };

        self.increment_position();

        self.current = Some(token);
        self.current.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Token, TokenContext};

    #[test]
    fn handles_whitespace() {
        let mut context: TokenContext = TokenContext::new(" hi : ( bob.the.palindrome ) 123, ");

        assert_eq!(
            context.current.clone().unwrap(),
            Token::Identifier("hi".to_string())
        );
        assert_eq!(context.get_next().unwrap(), Token::Define);
        assert_eq!(context.get_next().unwrap(), Token::FunctionStart);
        assert_eq!(
            context.get_next().unwrap(),
            Token::Identifier("bob".to_string())
        );
        assert_eq!(context.get_next().unwrap(), Token::IdentifierSeparator);
        assert_eq!(
            context.get_next().unwrap(),
            Token::Identifier("the".to_string())
        );
        assert_eq!(context.get_next().unwrap(), Token::IdentifierSeparator);
        assert_eq!(
            context.get_next().unwrap(),
            Token::Identifier("palindrome".to_string())
        );
        assert_eq!(context.get_next().unwrap(), Token::FunctionEnd);
        assert_eq!(context.get_next().unwrap(), Token::Integer(123));
        assert_eq!(context.get_next().unwrap(), Token::ListSeparator);
    }

    #[test]
    fn can_end_with_identifier_chain() {
        let mut context: TokenContext = TokenContext::new("hello.from.the.out.side");

        assert_eq!(
            context.current.clone().unwrap(),
            Token::Identifier("hello".to_string())
        );
        assert_eq!(context.get_next().unwrap(), Token::IdentifierSeparator);
        assert_eq!(
            context.get_next().unwrap(),
            Token::Identifier("from".to_string())
        );
        assert_eq!(context.get_next().unwrap(), Token::IdentifierSeparator);
        assert_eq!(
            context.get_next().unwrap(),
            Token::Identifier("the".to_string())
        );
        assert_eq!(context.get_next().unwrap(), Token::IdentifierSeparator);
        assert_eq!(
            context.get_next().unwrap(),
            Token::Identifier("out".to_string())
        );
        assert_eq!(context.get_next().unwrap(), Token::IdentifierSeparator);
        assert_eq!(
            context.get_next().unwrap(),
            Token::Identifier("side".to_string())
        );
    }

    #[test]
    pub fn can_lex_string() {
        let mut context: TokenContext = TokenContext::new("anyString: \"anyString\"
        anyInt: 100
        ");

        assert_eq!(context.current, Some(Token::Identifier("anyString".to_owned())));
        assert_eq!(context.get_next().unwrap(), Token::Define);
        assert_eq!(context.get_next().unwrap(), Token::String("anyString".to_owned()));
        assert_eq!(context.get_next().unwrap(), Token::Identifier("anyInt".to_owned()));
        assert_eq!(context.get_next().unwrap(), Token::Define);
    }
}
