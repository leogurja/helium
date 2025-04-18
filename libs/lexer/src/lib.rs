mod token;
use token::*;

use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.chars.next().unwrap_or('\0');

        Some(match next_char {
            '\0' => return None,
            '+' => Token::Operator(OperatorToken::Plus),
            '-' => Token::Operator(OperatorToken::Minus),
            '*' => Token::Operator(OperatorToken::Asterisk),
            '=' => Token::Operator(OperatorToken::Assign),
            '/' => Token::Operator(OperatorToken::Slash),
            '<' => Token::Operator(OperatorToken::LessThan),
            '>' => Token::Operator(OperatorToken::GreaterThan),
            ',' => Token::Delimiter(DelimiterToken::Comma),
            '(' => Token::Delimiter(DelimiterToken::LeftParen),
            ')' => Token::Delimiter(DelimiterToken::RightParen),
            '{' => Token::Delimiter(DelimiterToken::LeftBrace),
            '}' => Token::Delimiter(DelimiterToken::RightBrace),
            '[' => Token::Delimiter(DelimiterToken::LeftBracket),
            ']' => Token::Delimiter(DelimiterToken::RightBracket),
            _ => {
                if next_char.is_numeric() {
                    self.collect_number(next_char)
                } else {
                    self.collect_id(next_char)
                }
            }
        })
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Peekable<Self> {
        let lexer = Lexer {
            chars: input.chars().peekable(),
        };

        lexer.peekable()
    }

    fn collect_number(&mut self, current_char: char) -> Token {
        let mut collected = current_char.to_string();
        while let Some(next_char) = self.chars.next_if(|c| c.is_numeric() || c.eq(&'_')) {
            collected.push(next_char);
        }

        Token::Number(collected)
    }

    fn collect_id(&mut self, current_char: char) -> Token {
        let mut collected = current_char.to_string();
        while let Some(next_char) = self.chars.next_if(|c| c.is_alphanumeric() || c.eq(&'_')) {
            collected.push(next_char);
        }

        KeywordToken::lookup(&collected)
            .map(|k| Token::Keyword(k))
            .unwrap_or_else(|| Token::Id(collected))
    }
}
