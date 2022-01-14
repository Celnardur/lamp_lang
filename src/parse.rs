use crate::map;
use crate::map::Map;
use crate::token::TokenKind as Tk;
use crate::{code::Code, queue::Queue, token::Token};

pub fn parse(tokens: &[Token]) -> Result<Vec<Code>, String> {
    let mut queue = Queue::new(tokens);
    let mut exprs = Vec::new();

    while let Some(code) = queue.pop_code()? {
        exprs.push(code);
    }

    Ok(exprs)
}

pub fn parse_to_pgm(tokens: &[Token]) -> Result<Code, String> {
    let mut pgm = vec![Code::Identifier("pgm".to_string())];
    pgm.append(&mut parse(tokens)?);
    Ok(Code::List(pgm))
}

impl<'a> Queue<'a, Token> {
    pub fn pop_whitespace(&mut self) {
        self.pop_while(|t| t.is_whitespace());
    }

    pub fn pop_code(&mut self) -> Result<Option<Code>, String> {
        if self.empty() {
            return Ok(None);
        }

        let code = match &self.pop().unwrap().kind {
            Tk::Whitespace(_) | Tk::Comment(_) => {
                self.pop_whitespace();
                return self.pop_code();
            }
            Tk::Integer(num) => Code::Integer(*num),
            Tk::Float(num) => Code::Float(num.clone()),
            Tk::Character(c) => Code::Character(*c),
            Tk::StringLiteral(s) => Code::StringLiteral(s.clone()),
            Tk::Identifier(s) => Code::Identifier(s.clone()),
            Tk::Lfn => self.pop_list()?,
            Tk::Lcond => self.pop_map()?,
            _ => {
                // we don't want to modify the queue on error
                self.cursor -= 1;
                return Err("Unexpected Token".to_string());
            }
        };

        self.pop_whitespace();
        Ok(Some(code))
    }

    pub fn pop_list(&mut self) -> Result<Code, String> {
        let mut parsed = Vec::new();
        while let Ok(Some(code)) = self.pop_code() {
            parsed.push(code);
        }

        match self.pop().map(|t| &t.kind) {
            Some(Tk::Rfn) => Ok(Code::List(parsed)),
            None => Err("Reached End of File while parsing List".to_string()),
            _ => Err("Unexpected Token while parsing List".to_string()),
        }
    }

    pub fn pop_map(&mut self) -> Result<Code, String> {
        let mut parsed: Map<Code, Code> = map![];
        let eof_str = String::from("Reached end of file while parsing map");

        let cop = self.pop_code()?.ok_or(eof_str.clone())?;
        if self.peak().ok_or(eof_str.clone())?.kind == Tk::FieldDelim {
            self.pop();
            let value = self.pop_code()?.ok_or(eof_str.clone())?;
            parsed.insert(cop, value);
        } else {
            parsed.insert(Code::Identifier("head_position_field".to_string()), cop);
        }

        while let Ok((field, value)) = self.pop_map_pair() {
            parsed.insert(field, value);
        }

        match self.pop().map(|t| &t.kind) {
            Some(Tk::Rcond) => Ok(Code::Map(parsed)),
            None => Err("Reached End of File while parsing Map".to_string()),
            _ => Err("Unexpected Token while parsing Map".to_string()),
        }
    }

    pub fn pop_map_pair(&mut self) -> Result<(Code, Code), String> {
        let eof_str = String::from("Reached end of file while parsing map");

        let field = self.pop_code()?.ok_or(eof_str.clone())?;
        if self.peak().ok_or(eof_str.clone())?.kind != Tk::FieldDelim {
            return Err("Unexpected Token While parsing Map Pair".to_string());
        } else {
            self.pop();
        }
        let value = self.pop_code()?.ok_or(eof_str.clone())?;

        Ok((field, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::code::Code::*;
    use crate::map;
    use crate::map::Map;

    #[test]
    fn test_basic() {
        assert_eq!(Code::from_str("'a'").unwrap()[0], Character('a'));
    }

    #[test]
    fn test_pop_whitespace() {
        assert_eq!(
            Code::from_str(" 'a' \n 'b'").unwrap(),
            vec![Character('a'), Character('b')]
        );
    }

    #[test]
    fn test_atomics() {
        assert_eq!(
            Code::from_str("42 3.14 'a' \"hello\" world").unwrap(),
            vec![
                Integer(42),
                Float("3.14".to_string()),
                Character('a'),
                StringLiteral("hello".to_string()),
                Identifier("world".to_string()),
            ]
        );
    }

    #[test]
    fn test_hello_world() {
        assert_eq!(
            Code::from_str("[print \"Hello World!\"]").unwrap(),
            vec![List(vec![
                Identifier("print".to_string()),
                StringLiteral("Hello World!".to_string()),
            ])]
        );
    }

    #[test]
    fn test_list() {
        assert_eq!(
            Code::from_str("[list 42 3.14 'a']").unwrap(),
            vec![List(vec![
                Identifier("list".to_string()),
                Integer(42),
                Float("3.14".to_string()),
                Character('a'),
            ])]
        );

        assert_eq!(
            Code::from_str("[list 42 [hello 34] [1 [2 3]]]").unwrap(),
            vec![List(vec![
                Identifier("list".to_string()),
                Integer(42),
                List(vec![Identifier("hello".to_string()), Integer(34)]),
                List(vec![Integer(1), List(vec![Integer(2), Integer(3)])]),
            ])]
        );
    }

    #[test]
    fn test_map() {
        assert_eq!(
            Code::from_str("{15: 30 2: 4}").unwrap()[0], 
            Code::Map(map![
                {Integer(15), Integer(30)},
                {Integer(2), Integer(4)},
            ])
        );
    }

    #[test]
    fn test_map_cond() {
        assert_eq!(
            Code::from_str("{if c: [equal msg \"hello\"] do: [print \"world\"]}").unwrap()[0],
            Map(map![
                {Identifier("head_position_field".to_string()), Identifier("if".to_string())},
                {Identifier("c".to_string()), List(vec![
                    Identifier("equal".to_string()),
                    Identifier("msg".to_string()),
                    StringLiteral("hello".to_string()),
                ])},
                {Identifier("do".to_string()), List(vec![
                    Identifier("print".to_string()),
                    StringLiteral("world".to_string()),
                ])},
            ])
        );
    }
}
