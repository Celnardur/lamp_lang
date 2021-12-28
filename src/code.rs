use crate::map;
use crate::map::Map;
use crate::token::Token;
use crate::token::TokenKind as Tk;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Code {
    Integer(i128),
    Float(String),
    Character(char),
    StringLiteral(String),
    Identifier(String),
    List(Vec<Code>),
    Map(Map<Code, Code>),
}

fn skip_whitespace(code: &[Token]) -> &[Token] {
    let mut length = 0;
    while length < code.len() && code[length].is_whitespace() {
        length += 1;
    }
    &code[length..]
}

pub fn parse_code(code: &[Token]) -> Result<Option<(Code, &[Token])>, String> {
    if code.len() == 0 {
        return Ok(None);
    }

    let (code, tokens) = match &code[0].kind {
        Tk::Whitespace(_) => return parse_code(skip_whitespace(code)),
        Tk::Comment(_) => return parse_code(skip_whitespace(code)),
        Tk::Integer(num) => (Code::Integer(*num), &code[1..]),
        Tk::Float(float) => (Code::Float(float.clone()), &code[1..]),
        Tk::Character(c) => (Code::Character(*c), &code[1..]),
        Tk::StringLiteral(s) => (Code::StringLiteral(s.clone()), &code[1..]),
        Tk::Identifier(s) => (Code::Identifier(s.clone()), &code[1..]),
        Tk::Lfn => parse_fexpr(code)?,
        Tk::Rfn => return Err("Unexpected ']'".to_string()),
        Tk::Symbol(_) => return Err("Unexpected Symbol".to_string()),
        _ => return Err("Unimplmented".to_string()),
    };

    Ok(Some((code, skip_whitespace(tokens))))
}

pub fn parse_fexpr(code: &[Token]) -> Result<(Code, &[Token]), String> {
    let mut tokens = &code[1..];
    let mut parsed = Vec::new();
    while tokens.get(0).map(|t| &t.kind) != Some(&Tk::Rfn) {
        if let Some((p, t)) = parse_code(tokens)? {
            parsed.push(p);
            tokens = t;
        } else {
            return Err("Reached end of file while parsing List".to_string());
        }
    }
    Ok((Code::List(parsed), &code[1..]))
}

// pub fn parse_cexpr(code: &[Token]) -> Result<(Code, &[Token]), String> {
//     if code.len() == 0 || code[0].kind != Tk::Lcond {
//         return Err("Conditional Expression must start with '{'".to_string());
//     }

//     let mut tokens = &code[1..];
//     let func = parse_code(code)?;


//     Err("Unimplmented".to_string())
// }

// pub fn parse_map_pair(code: &[Token]) -> Result<(Code, Code, &[Token]), String> {
//     let mut tokens = code;
//     let key = if let Some((c, t)) = parse_code(tokens)? {
//         tokens = t;
//         c
//     } else {
//         return Err("Mismatched '{': Reached End of file while parsing".to_string());
//     };

//     if tokens.len() == 0 || tokens[0].kind != Tk::FieldDelim {
//         return Err("Expected ':'".to_string())
//     }
//     tokens = &tokens[1..];

//     let value = if let Some((c, t)) = parse_code(tokens)? {
//         tokens = t;
//         c
//     } else {
//         return Err("Mismatched '{': Reached End of file while parsing".to_string());
//     };

//     Ok((key, value, tokens))
// }

#[cfg(test)]
mod tests {
    use crate::token::*;
    use super::Code::*;
    use super::*;

    #[test]
    fn test_end_code() {
        let code = tokenize_from_str("'a'").unwrap();
        let empty: Vec<Token> = Vec::new();
        assert_eq!(
            parse_code(&code).unwrap().unwrap(),
            (Code::Character('a'), &empty[..])
        );
    }

    #[test]
    fn test_skip_whitespace() {
        let code = tokenize_from_str(" 'a' ").unwrap();
        let empty: Vec<Token> = Vec::new();
        assert_eq!(
            parse_code(&code).unwrap().unwrap(),
            (Code::Character('a'), &empty[..])
        );
    }

    #[test]
    fn test_parse_fexpr() {
        let code = tokenize_from_str("[asdf 10 'a' 42]").unwrap();
        assert_eq!(
            parse_code(&code).unwrap().unwrap().0,
            List(vec![
                Identifier("asdf".to_string()),
                Integer(10),
                Character('a'),
                Integer(42),
            ]),
        );
    }
}