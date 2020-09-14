use std::iter::{Peekable, FromIterator};
use std::str::Chars;
use crate::iterator_helpers::TakeWhilePeekingImpl;

#[derive(Debug, PartialEq)]
pub enum Token
{
    Left(usize),
    Right(usize),
    Add(usize),
    Sub(usize),
    WriteAscii,
    WriteDec,
    Read,
    ScopeOpen,
    ScopeClose,
    CacheUp,
    CacheDown,
}

pub fn lex(string: &str) -> Vec<Token>
{
    let mut iter = string.chars().peekable();

    let mut tokens = vec![];

    while let Some(c) = iter.next()
    {
        let token = match c
        {
            '<' => Some(Token::Left     (get_command_multiplier(&mut iter))),
            '>' => Some(Token::Right    (get_command_multiplier(&mut iter))),
            '+' => Some(Token::Add      (get_command_multiplier(&mut iter))),
            '-' => Some(Token::Sub      (get_command_multiplier(&mut iter))),
            '!' => Some(Token::WriteAscii),
            '#' => Some(Token::WriteDec),
            '?' => Some(Token::Read),
            '[' => Some(Token::ScopeOpen),
            ']' => Some(Token::ScopeClose),
            '^' => Some(Token::CacheUp),
            'v' => Some(Token::CacheDown),
            '`' => {
                //` starts a comment
                //find another ` that closes the comment
                iter.take_while_peeking(|c| *c == '`');
                None
            }
            _ => None
        };

        if let Some(token) = token
        {
            tokens.push(token);
        }
    }

    tokens
}

fn get_command_multiplier(iter: &mut Peekable<Chars>) -> usize
{
    if let Some(next) = iter.peek()
    {
        if next.is_alphabetic()
        {
            return *next as usize;
        }
    }

    let number_seq: Vec<char> = iter
        .take_while_peeking(|x| x.is_numeric())
        .collect();

    if number_seq.len() < 1
    {
        return 1;
    }

    str::parse::<usize>(&String::from_iter(number_seq.iter())).unwrap()
}
