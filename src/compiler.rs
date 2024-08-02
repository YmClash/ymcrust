


#[derive(Debug,PartialEq)]
pub enum Token {
    //operators
    Number(i64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    //keywords
    // LABEL,
    // GOTO,
    // INPUT,
    // PRINT,
    // LET,
    // IF,
    // THEN,
    // ENDIF,
    // WHILE,
    // REPEAT,
    // ENDWHILE,

}


pub fn lexxer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars:Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len(){
        match chars[i] {
            '0'..='9' => {
                let mut num = chars[i].to_digit(10).unwrap() as i64;
                while i+1 < chars.len() && chars[i+1].is_digit(10){
                    i += 1;
                    num = num * 10 + chars[i].to_digit(10).unwrap() as i64;
                }
                tokens.push(Token::Number(num));
            },
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ' ' | '\n' | '\t' => (),
            _ => panic!("Invalid character: {}",chars[i]),
        }
        i += 1;

    }
    tokens


}


//parser
// AST Anstract Syntax Tree

#[derive(Debug)]

pub enum ASTNode {
    Number(i64),
    Add(Box<ASTNode>, Box<ASTNode>),
    Subtract(Box<ASTNode>, Box<ASTNode>),
    Multiply(Box<ASTNode>, Box<ASTNode>),
    Divide(Box<ASTNode>, Box<ASTNode>),
}


fn parse(tokens: &[Token]) -> ASTNode {
    fn parse_expr(tokens: &[Token], pos: &mut usize) -> ASTNode {
        let mut left = parse_term(tokens, pos);
        while *pos < tokens.len() {
            match tokens[*pos] {
                Token::Plus => {
                    *pos += 1;
                    left = ASTNode::Add(Box::new(left), Box::new(parse_term(tokens, pos)));
                },
                Token::Minus => {
                    *pos += 1;
                    left = ASTNode::Subtract(Box::new(left), Box::new(parse_term(tokens, pos)));
                },
                _ => break,
            }
        }
        left
    }

    fn parse_term(tokens: &[Token], pos: &mut usize) -> ASTNode {
        let mut left = parse_factor(tokens, pos);
        while *pos < tokens.len() {
            match tokens[*pos] {
                Token::Multiply => {
                    *pos += 1;
                    left = ASTNode::Multiply(Box::new(left), Box::new(parse_factor(tokens, pos)));
                },
                Token::Divide => {
                    *pos += 1;
                    left = ASTNode::Divide(Box::new(left), Box::new(parse_factor(tokens, pos)));
                },
                _ => break,
            }
        }
        left
    }

    fn parse_factor(tokens: &[Token], pos: &mut usize) -> ASTNode {
        match tokens[*pos] {
            Token::Number(num) => {
                *pos += 1;
                ASTNode::Number(num)
            },
            Token::LParen => {
                *pos += 1;
                let expr = parse_expr(tokens, pos);
                if let Token::RParen = tokens[*pos] {
                    *pos += 1;
                    expr
                } else {
                    panic!("Expected closing parenthesis");
                }
            },
            _ => panic!("Unexpected token: {:?}", tokens[*pos]),
        }
    }

    let mut pos = 0;
    let ast = parse_expr(tokens, &mut pos);
    if pos != tokens.len() {
        panic!("Unexpected tokens at the end");
    }
    ast
}




//***/*/*/****//
//Evaluate the AST and generate the result

pub fn evaluate(ast: &ASTNode) -> i64 {
    match ast {
        ASTNode::Number(val) => *val,
        ASTNode::Add(left,right) => evaluate(left) + evaluate(right),
        ASTNode::Subtract(left,right) => evaluate(left) - evaluate(right),
        ASTNode::Multiply(left,right) => evaluate(left) * evaluate(right),
        ASTNode::Divide(left,right) => evaluate(left) / evaluate(right),
    }
}














//////////////////////////////////////////////////////////////////////////////////////

// module for my compiler
//
// #[derive(Debug, PartialEq)]
//
// pub enum Token {
//     TokenEndOfFile,
//     Def,
//     Extern,
//     Identifier(String),
//     Number(f64),
//
//
// }
//
// pub struct Lexer<'a> {
//     input: Chars<'a>,
//     current_char: Option<char>,
// }
//
//
// impl<'a>Lexer<'a> {
//     pub fn new(input:&'a str) -> Self{
//         let mut lexer = Lexer {
//             input: input.chars(),
//             current_char: None,
//         };
//         lexer.advance();
//         lexer
//
//     }
//     pub fn advance(&mut self) {
//         self.current_char = self.input.next();
//     }
//
//     pub fn skip_whitespace(&mut self) {
//         while let Some(c) = self.current_char {
//             if !c.is_whitespace(){
//                 break;
//             }
//             self.advance();
//         }
//     }
//     pub fn get_next_token(&mut self) -> Token {
//         self.skip_whitespace();
//
//         if let Some(c) = self.current_char {
//             match c {
//                 'd' => {
//                     self.advance();
//                     if let Some('e') = self.current_char {
//                         self.advance();
//                         if let Some('f') = self.current_char {
//                             self.advance();
//                             return Token::Def;
//                         }
//                     }
//                 }
//                 'e' => {
//                     self.advance();
//                     if let Some('x') = self.current_char {
//                         self.advance();
//                         if let Some('t') = self.current_char {
//                             self.advance();
//                             if let Some('e') = self.current_char {
//                                 self.advance();
//                                 if let Some('r') = self.current_char {
//                                     self.advance();
//                                     if let Some('n') = self.current_char {
//                                         self.advance();
//                                         return Token::Extern;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 c if c.is_alphabetic() => {
//                     let mut identifier = String::new();
//                     while let Some(c) = self.current_char {
//                         if c.is_alphanumeric() {
//                             identifier.push(c);
//                             self.advance();
//                         } else {
//                             break;
//                         }
//                     }
//                     return Token::Identifier(identifier);
//                 }
//                 c if c.is_digit(10) => {
//                     let mut number = String::new();
//                     while let Some(c) = self.current_char {
//                         if c.is_digit(10) || c == '.' {
//                             number.push(c);
//                             self.advance();
//                         } else {
//                             break;
//                         }
//                     }
//                     return Token::Number(number.parse().unwrap());
//                 }
//                 _ => {}
//             }
//         }
//
//         Token::TokenEndOfFile
//     }
//
//
//
//
//
//
// }
//








