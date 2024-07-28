use std::str::Chars;




// module for my compiler

#[derive(Debug, PartialEq)]

pub enum Token {
    TokenEndOfFile,
    Def,
    Extern,
    Identifier(String),
    Number(f64),


}

struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}


impl<'a>Lexer<'a> {
    pub fn new(input:&'a str) -> Self{
        let mut lexer = Lexer {
            input: input.chars(),
            current_char: None,
        };
        lexer.advance();
        lexer

    }
    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if !c.is_whitespace(){
                break;
            }
            self.advance();
        }
    }
    fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(c) = self.current_char {
            match c {
                'd' => {
                    self.advance();
                    if let Some('e') = self.current_char {
                        self.advance();
                        if let Some('f') = self.current_char {
                            self.advance();
                            return Token::Def;
                        }
                    }
                }
                'e' => {
                    self.advance();
                    if let Some('x') = self.current_char {
                        self.advance();
                        if let Some('t') = self.current_char {
                            self.advance();
                            if let Some('e') = self.current_char {
                                self.advance();
                                if let Some('r') = self.current_char {
                                    self.advance();
                                    if let Some('n') = self.current_char {
                                        self.advance();
                                        return Token::Extern;
                                    }
                                }
                            }
                        }
                    }
                }
                c if c.is_alphabetic() => {
                    let mut identifier = String::new();
                    while let Some(c) = self.current_char {
                        if c.is_alphanumeric() {
                            identifier.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    return Token::Identifier(identifier);
                }
                c if c.is_digit(10) => {
                    let mut number = String::new();
                    while let Some(c) = self.current_char {
                        if c.is_digit(10) || c == '.' {
                            number.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    return Token::Number(number.parse().unwrap());
                }
                _ => {}
            }
        }

        Token::TokenEndOfFile
    }






}









