use crate::queue::Queue;
use TokenKind::*;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenKind {
    // delimiters for functional and conditional expressions
    Lfn,
    Rfn,
    Lcond,
    Rcond,
    FieldDelim,
    Whitespace(String),
    // literals
    Integer(i128),
    Float(String), // code needs to be hashable
    Character(char),
    StringLiteral(String),
    // identifier
    Identifier(String),
    // symbol - sequence of non-alphanumeric, non-whitespade chars
    Symbol(String),
    // comment out fn_exprs by add '/' in front - handled in parser
    // single line comment '#'
    // multi line comment ###
    Comment(String),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub start: usize,
    pub length: usize,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, length: usize) -> Token {
        Token {
            start,
            length,
            kind,
        }
    }

    pub fn is_whitespace(&self) -> bool {
        match self.kind {
            Whitespace(_) => true,
            Comment(_) => true,
            _ => false,
        }
    }

    pub fn end(&self) -> usize {
        self.start + self.length
    }
}

impl<'a> Queue<'a, char> {
    pub fn s_pop_while(&mut self, mut f: impl FnMut(char) -> bool) -> String {
        let start = self.cursor;
        while self.cursor < self.data.len() && f(self.data[self.cursor]) {
            self.cursor += 1;
        }
        self.data[start..self.cursor].iter().collect()
    }

    pub fn range_string(&self, start: usize) -> String {
        self.data[start..self.cursor].iter().collect()
    }

    pub fn pop_char(&mut self, end: char) -> Result<Option<char>, String> {
        Ok(Some(match self.pop() {
            Some('\\') => match self.pop() {
                Some('n') => '\n',
                Some('r') => '\r',
                Some('t') => '\t',
                Some(&c) => c,
                None => return Err("Reached end of file while parsing string/char".to_string()),
            },
            Some(&c) => {
                if c == end {
                    return Ok(None);
                } else {
                    c
                }
            }
            None => return Err("Reached end of file while parsing string/char".to_string()),
        }))
    }
}

// helper function
fn is_symbol(c: char) -> bool {
    !c.is_whitespace()
        && !c.is_alphanumeric()
        && match c {
            '[' => false,
            ']' => false,
            '{' => false,
            '}' => false,
            '#' => false,
            ':' => false,
            '"' => false,
            '\'' => false,
            _ => true,
        }
}

pub fn tokenize_from_str(code: &str) -> Result<Vec<Token>, String> {
    let code: Vec<char> = code.chars().collect();
    tokenize(&code)
}

pub fn tokenize(code: &[char]) -> Result<Vec<Token>, String> {
    let mut queue = Queue::new(code);
    let mut tokens = Vec::new();
    let mut start = 0;
    while let Some(token) = pop_token(&mut queue)? {
        tokens.push(Token::new(token, start, queue.cursor - start));
        start = queue.cursor;
    }
    Ok(tokens)
}

pub fn pop_token(queue: &mut Queue<char>) -> Result<Option<TokenKind>, String> {
    if queue.empty() {
        return Ok(None);
    }

    if queue.head().is_alphabetic() {
        return Ok(Some(Identifier(
            queue.s_pop_while(|c| c.is_alphanumeric() || c == '_'),
        )));
    }

    if queue.head().is_whitespace() {
        return Ok(Some(Whitespace(queue.s_pop_while(|c| c.is_whitespace()))));
    }

    if queue.head().is_ascii_digit() {
        let mut num = queue.s_pop_while(|c| c.is_ascii_digit());
        return if queue.peak() == Some(&'.') {
            num.push(*queue.pop().unwrap());
            num.push_str(&queue.s_pop_while(|c| c.is_ascii_digit()));
            match num.parse::<f64>() {
                Ok(_) => Ok(Some(Float(num))),
                Err(_) => Err(format!(
                    "Scanner Error: Cannot parse \"{}\" as decimal",
                    num
                )),
            }
        } else {
            match num.parse() {
                Ok(number) => Ok(Some(Integer(number))),
                Err(_) => {
                    return Err(format!(
                        "Scanner Error: Cannot parse \"{}\" as integer",
                        num,
                    ))
                }
            }
        };
    }

    Ok(Some(match *queue.pop().unwrap() {
        '[' => Lfn,
        ']' => Rfn,
        '{' => Lcond,
        '}' => Rcond,
        ':' => FieldDelim,
        '#' => {
            let start = queue.cursor - 1;
            if queue.peak() == Some(&'#') {
                queue.pop_while(|c| *c == '#');
                queue.pop_until(&['#', '#']);
                queue.pop_while(|c| *c == '#');
            } else {
                queue.pop_until(&['\n']);
                queue.pop();
            }
            Comment(queue.range_string(start))
        }
        '"' => {
            let mut s = String::new();
            while let Some(c) = queue.pop_char('"')? {
                s.push(c);
            }
            StringLiteral(s)
        }
        '\'' => {
            if let Some(c) = queue.pop_char('\'')? {
                match queue.pop() {
                    Some('\'') => Character(c),
                    Some(_) => {
                        return Err("Character litteral can only contain one character".to_string())
                    }
                    None => return Err("Reached end of file while parsing char".to_string()),
                }
            } else {
                return Err("Character litteral must contain at least one character".to_string());
            }
        }
        _ => {
            let start = queue.cursor - 1;
            queue.pop_while(|&c| is_symbol(c));
            Symbol(queue.range_string(start))
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delimiters() {
        let code: Vec<char> = "[] \n {}:".chars().collect();
        assert_eq!(
            tokenize(&code).unwrap(),
            [
                Token::new(Lfn, 0, 1),
                Token::new(Rfn, 1, 1),
                Token::new(Whitespace(" \n ".to_string()), 2, 3),
                Token::new(Lcond, 5, 1),
                Token::new(Rcond, 6, 1),
                Token::new(FieldDelim, 7, 1),
            ]
        );
    }

    #[test]
    fn test_hello_world() {
        let code: Vec<char> = "[print \"Hello World\"]".chars().collect();
        assert_eq!(
            tokenize(&code).unwrap(),
            [
                Token::new(Lfn, 0, 1),
                Token::new(Identifier("print".to_string()), 1, 5),
                Token::new(Whitespace(" ".to_string()), 6, 1),
                Token::new(StringLiteral("Hello World".to_string()), 7, 13),
                Token::new(Rfn, 20, 1),
            ]
        );
    }

    #[test]
    fn test_identifiers() {
        let code: Vec<char> = "print u8 uint8_t".chars().collect();
        assert_eq!(
            tokenize(&code).unwrap(),
            [
                Token::new(Identifier("print".to_string()), 0, 5),
                Token::new(Whitespace(" ".to_string()), 5, 1),
                Token::new(Identifier("u8".to_string()), 6, 2),
                Token::new(Whitespace(" ".to_string()), 8, 1),
                Token::new(Identifier("uint8_t".to_string()), 9, 7),
            ]
        );
    }

    #[test]
    fn test_numeric_literals() {
        let code: Vec<char> = "7 42 3.1415".chars().collect();
        assert_eq!(
            tokenize(&code).unwrap(),
            [
                Token::new(Integer(7), 0, 1),
                Token::new(Whitespace(" ".to_string()), 1, 1),
                Token::new(Integer(42), 2, 2),
                Token::new(Whitespace(" ".to_string()), 4, 1),
                Token::new(Float("3.1415".to_string()), 5, 6),
            ]
        );
    }

    #[test]
    fn test_char_literals() {
        let code: Vec<char> = "'a''\\n''\\''".chars().collect();
        assert_eq!(
            tokenize(&code).unwrap(),
            [
                Token::new(Character('a'), 0, 3),
                Token::new(Character('\n'), 3, 4),
                Token::new(Character('\''), 7, 4),
            ]
        );
    }

    #[test]
    fn test_string_literals() {
        let code: Vec<char> = "\"asdf\"\"\\n\\r\\t\\\"\"".chars().collect();
        assert_eq!(
            tokenize(&code).unwrap(),
            [
                Token::new(StringLiteral("asdf".to_string()), 0, 6),
                Token::new(StringLiteral("\n\r\t\"".to_string()), 6, 10),
            ]
        );
    }

    #[test]
    fn test_comments() {
        let code: Vec<char> = "foo#bar\nfoo\nbar##foo\nbar##bazz#fizzbuzz"
            .chars()
            .collect();
        assert_eq!(
            tokenize(&code).unwrap(),
            [
                Token::new(Identifier("foo".to_string()), 0, 3),
                Token::new(Comment("#bar\n".to_string()), 3, 5),
                Token::new(Identifier("foo".to_string()), 8, 3),
                Token::new(Whitespace("\n".to_string()), 11, 1),
                Token::new(Identifier("bar".to_string()), 12, 3),
                Token::new(Comment("##foo\nbar##".to_string()), 15, 11),
                Token::new(Identifier("bazz".to_string()), 26, 4),
                Token::new(Comment("#fizzbuzz".to_string()), 30, 9),
            ]
        );
    }

    #[test]
    fn test_symbols() {
        let code: Vec<char> = "!%^ |&@foo".chars().collect();
        assert_eq!(
            tokenize(&code).unwrap(),
            [
                Token::new(Symbol("!%^".to_string()), 0, 3),
                Token::new(Whitespace(" ".to_string()), 3, 1),
                Token::new(Symbol("|&@".to_string()), 4, 3),
                Token::new(Identifier("foo".to_string()), 7, 3),
            ]
        );
    }
}
