use logos::Logos;
use std::collections::VecDeque;

#[derive(Debug, Logos, PartialEq)]
pub enum Token {
    #[regex(r"fd|forward")]
    Forward,
    #[regex(r"bk|back|backward")]
    Backward,
    #[regex(r"lt|left")]
    Left,
    #[regex(r"rt|right")]
    Right,
    #[regex(r"[0-9]+(?:\.[0-9]+)?", |lex| lex.slice().parse::<f32>().ok())]
    Number(Option<f32>),
    #[token("*")]
    Mul,
    #[token("+")]
    Add,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("repeat")]
    Repeat,
    #[token("show")]
    Show,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
/* TODO
- function declaration
to name :arg1 :arg2
  body
end

- function call
builtin functions:
* clearscreen
* cleartext

- lists
[1 2 3]

*/

pub fn process(input: &str) -> VecDeque<Token> {
    let processed = Token::lexer(input)
        .map(|tok| match tok {
            Ok(t) => t,
            Err(()) => panic!("Syntax error"),
        })
        .collect::<VecDeque<Token>>();
    processed
}

pub fn _process_line(source: &str) {
    let tokens = process(source);
    tokens.iter().for_each(|token| match token {
        Token::Forward => println!("Forward"),
        Token::Number(Some(n)) => println!("Number {}", n),
        Token::Add => println!("Add"),
        Token::Mul => println!("Mul"),
        Token::Right => println!("Right"),
        _ => eprintln!("Unknown command / Unimplemented"),
    });
}
