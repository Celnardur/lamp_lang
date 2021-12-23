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
    Float(i128, u64), // code needs to be hashable
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
    
    pub fn end(&self) -> usize {
        self.start + self.length
    }
}

macro_rules! rt {
    ( $kind:expr, $length:expr ) => {
        Ok(Token {
            start: 0,
            length: $length,
            kind: $kind,
        })
    };
}

// incr while code
macro_rules! iwc {
    ($incr:ident, $code:ident, $cond:expr) => {
        while $incr < $code.len() && ($cond) {
            $incr += 1;
        }
    };
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

fn get_char(code: &[char], index: usize, end: char) -> Result<Option<(char, usize)>, String> {
    match code.get(index) {
        Some('\\') => match code.get(index + 1) {
            Some('n') => Ok(Some(('\n', 2))),
            Some('r') => Ok(Some(('\r', 2))),
            Some('t') => Ok(Some(('\t', 2))),
            Some(&c) => Ok(Some((c, 2))),
            None => Err("Reached end of file while parsing string/char".to_string()),
        },
        Some(&c) => {
            if c == end {
                Ok(None)
            } else {
                Ok(Some((c, 1)))
            }
        }
        None => Err("Reached end of file while parsing string/char".to_string()),
    }
}

pub fn tokenize(code: &[char]) -> Result<Vec<Token>, String> {
    let mut cursor = 0;
    let mut tokens = Vec::new();
    while cursor < code.len() {
        let mut token = first(&code[cursor..])?;
        token.start = cursor;
        cursor += token.length;
        tokens.push(token);
    }
    Ok(tokens)
}

pub fn first(code: &[char]) -> Result<Token, String> {
    if code.len() < 1 {
        return Err("Cannot Scan Empty String".to_string());
    }
    // identifiers
    if code[0].is_alphabetic() {
        let mut length = 1;
        iwc!(
            length,
            code,
            code[length].is_alphanumeric() || code[length] == '_'
        );
        return rt!(Identifier(code[..length].iter().collect()), length);
    }

    // whitespace
    if code[0].is_whitespace() {
        let mut length = 1;
        iwc!(length, code, code[length].is_whitespace());
        return rt!(Whitespace(code[..length].iter().collect()), length);
    }

    // check for number literals
    if code[0].is_ascii_digit() {
        let mut length = 1;
        iwc!(length, code, code[length].is_ascii_digit());
        let left: String = code[..length].iter().collect();
        let left = match left.parse() {
            Ok(number) => number,
            Err(_) => return Err(format!(
                    "Scanner Error: Cannot parse \"{}\" as integer",
                    left
                )),
        };

        // double literal
        return if length < code.len() && code[length] == '.' {
            length += 1;
            let floating_start = length;
            iwc!(length, code, code[length].is_ascii_digit());

            let token: String = code[floating_start..length].iter().collect();
            match token.parse() {
                Ok(number) => rt!(Float(left, number), length),
                Err(_) => Err(format!(
                    "Scanner Error: Cannot parse \"{}\" as decimal",
                    token
                )),
            }
        } else {
            rt!(Integer(left), length)
        };
    }

    // symbols and comments
    match code[0] {
        '[' => rt!(Lfn, 1),
        ']' => rt!(Rfn, 1),
        '{' => rt!(Lcond, 1),
        '}' => rt!(Rcond, 1),
        ':' => rt!(FieldDelim, 1),
        '#' => {
            if code.get(1) == Some(&'#') {
                let mut length = 2;
                iwc!(length, code, code[length] == '#');
                iwc!(
                    length,
                    code,
                    !(code[length - 1] == '#' && code[length] == '#')
                );
                iwc!(length, code, code[length] == '#');
                rt!(Comment(code[..length].iter().collect()), length)
            } else {
                let mut length = 1;
                iwc!(length, code, code[length] != '\n');
                if length >= code.len() {
                    rt!(Comment(code[..length].iter().collect()), length)
                } else {
                    length += 1;
                    rt!(Comment(code[..length].iter().collect()), length)
                }
            }
        }
        '"' => {
            let mut length = 1;
            let mut s = String::new();
            while let Some((c, l)) = get_char(code, length, '"')? {
                length += l;
                s.push(c);
            }
            rt!(StringLiteral(s), length + 1)
        }
        '\'' => {
            if let Some((c, l)) = get_char(code, 1, '\'')? {
                match code.get(l + 1) {
                    Some('\'') => rt!(Character(c), l + 2),
                    Some(_) => {
                        Err("Character literal must be only one character long".to_string())
                    }
                    None => Err("Reached end of file while parsing char".to_string()),
                }
            } else {
                Err("Character literal must be only one character long".to_string())
            }
        }
        _ => {
            let mut length = 1;
            iwc!(length, code, is_symbol(code[length]));
            rt!(Symbol(code[..length].iter().collect()), length)
        }
    }
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
                Token::new(Float(3, 1415), 5, 6),
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
