/*
 * @Date: 2022-07-06
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-06
 * @FilePath: /algorithm/736_lisp_evaluate/lisp_evaluate.rs
 */

use std::{collections::HashMap, iter::FromIterator};

#[derive(Debug)]
enum Token {
    Start,
    End,
    Let,
    Mult,
    Add,
    Val(String),
}

struct TokenStream<I> {
    next: Option<Token>,
    tmp: Vec<char>,
    iter: I,
}

// impl<I: Iterator<Item = char>, 'a> Parser<I, 'a> {
//     pub fn from_str(s: &'a str) -> Self {
//         Self {
//             next: None,
//             tmp: vec![],
//             iter: s.chars(),
//         }
//     }
// }

impl<I: Iterator<Item = char>> Iterator for TokenStream<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.next.take() {
            return Some(token);
        }
        match self.iter.next() {
            Some(ch) => match ch {
                '(' => Some(Token::Start),
                ' ' => {
                    let t = String::from_iter(self.tmp.drain(..));
                    if t.is_empty() {
                        return self.next();
                    }
                    let tt = match t.as_str() {
                        "let" => Token::Let,
                        "mult" => Token::Mult,
                        "add" => Token::Add,
                        _ => Token::Val(t),
                    };
                    Some(tt)
                }
                ')' => {
                    let t = String::from_iter(self.tmp.drain(..));
                    if t.is_empty() {
                        Some(Token::End)
                    } else {
                        self.next = Some(Token::End);
                        Some(Token::Val(t))
                    }
                }
                _ => {
                    self.tmp.push(ch);
                    self.next()
                }
            },
            None => None,
        }
    }
}

#[derive(Debug, Default)]
struct Context(Vec<HashMap<String, i32>>);

impl Context {
    fn push(&mut self) {
        self.0.push(Default::default());
    }
    fn pop(&mut self) {
        self.0.pop();
    }
    fn set_value(&mut self, key: String, val: i32) {
        if let Some(v) = self.0.last_mut() {
            v.insert(key, val);
        }
    }
    fn get_value(&self, key: &String) -> Option<i32> {
        for v in self.0.iter().rev() {
            if let Some(value) = v.get(key) {
                return Some(*value);
            }
        }

        None
    }
}

fn eval<I: Iterator<Item = char>>(ctx: &mut Context, token_stream: &mut TokenStream<I>) -> i32 {
    ctx.push();
    let value = match token_stream.next() {
        Some(Token::Start) => eval(ctx, token_stream),
        Some(Token::Add) => {
            let mut v = 0;
            loop {
                let x = match token_stream.next() {
                    Some(Token::Val(v)) => match v.parse() {
                        Ok(a) => a,
                        Err(_) => ctx.get_value(&v).unwrap(),
                    },
                    Some(Token::Start) => eval(ctx, token_stream),
                    Some(Token::End) => break,
                    _ => unreachable!(),
                };
                v += x;
            }

            // println!("add {}", v);

            v
        }
        Some(Token::Mult) => {
            let mut v = 1;
            loop {
                let x = match token_stream.next() {
                    Some(Token::Val(v)) => match v.parse() {
                        Ok(a) => a,
                        Err(_) => ctx.get_value(&v).unwrap(),
                    },
                    Some(Token::Start) => eval(ctx, token_stream),
                    Some(Token::End) => break,
                    _ => unreachable!(),
                };
                v *= x;
            }

            // println!("mult {}", v);
            v
        }
        Some(Token::Let) => {
            let mut tmp = None;
            loop {
                match token_stream.next() {
                    Some(Token::Val(val)) => {
                        if let Some(key) = tmp.take() {
                            let a = match val.parse() {
                                Ok(v) => v,
                                Err(_) => {
                                    ctx.get_value(&val).expect(&format!("expect val {:?}", val))
                                }
                            };
                            ctx.set_value(key, a);
                        } else {
                            tmp = Some(val);
                        }
                    }
                    Some(Token::Start) => {
                        let val = eval(ctx, token_stream);
                        if let Some(key) = tmp.take() {
                            ctx.set_value(key, val);
                        } else {
                            tmp = Some(val.to_string());
                        }
                    }
                    Some(Token::End) => break,

                    _ => unreachable!(),
                }
            }

            // println!("let {:?}", tmp);
            let val = tmp.unwrap();
            match val.parse() {
                Ok(v) => v,
                Err(_) => ctx.get_value(&val).expect(&format!("expect val {:?}", val)),
            }
        }
        _ => 0,
    };

    ctx.pop();

    value
}

struct Solution;

impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        let mut token_stream = TokenStream {
            next: None,
            tmp: vec![],
            iter: expression.chars(),
        };
        // let tokens: Vec<_> = token_stream.collect();
        // println!("{:?}", tokens);
        // 0

        let mut ctx: Context = Default::default();

        eval(&mut ctx, &mut token_stream)
    }
}

fn main() {
    {
        let expression = String::from("(let x 2 (mult x (let x 3 y 4 (add x y))))");
        assert_eq!(Solution::evaluate(expression), 14);
    }

    {
        let expression = String::from("(let x 3 x 2 x)");
        assert_eq!(Solution::evaluate(expression), 2);
    }

    {
        let expression = String::from("(let x 1 y 2 x (add x y) (add x y))");
        assert_eq!(Solution::evaluate(expression), 5);
    }
}
