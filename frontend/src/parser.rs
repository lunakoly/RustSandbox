use parsing::stream::*;

use crate::lexer::{Token};
use crate::liner::{Liner};

use crate::ast::*;
use crate::ast::nodes::*;

use parsing::ruler::{Grammar, apply_rule};

use crate::grammar::{get_grammar};

use std::rc::Rc;
use std::cell::RefCell;

pub struct Parser<'a> {
    pub grammar: Grammar<'a, Box<dyn Node>, Token>,
    pub backend: Liner<'a>,
    pub last_ast: Rc<RefCell<Box<dyn Node>>>,
    pub end_token_met: bool,
}

impl <'a> Parser<'a> {
    fn parse(&mut self) -> Rc<RefCell<Box<dyn Node>>> {
        let tokens = self.backend.grab();

        match tokens.first() {
            Some(Token::End) => {
                self.end_token_met = true;

                return Rc::new(
                    RefCell::new(
                        Box::new(
                            Expressions {
                                values: vec![]
                            }
                        )
                    )
                );
            },
            _ => {},
        }

        let (ast, stop_index) = apply_rule(
            "expression",
            &tokens,
            0,
            &self.grammar,
        );

        if stop_index > 0 {
            if stop_index < tokens.len() - 1 {
                println!("Warning > Ignoring due to a syntax error at:");
                println!("    {:?}", tokens[stop_index]);
                println!("Which is right after:");
                println!("    {:?}", tokens[stop_index - 1]);
            }
        } else if tokens.len() >= 2 {
            println!("Warning > Ignoring due to a syntax error at:");
            println!("    {:?}", tokens[stop_index]);
        } else {
            return Rc::new(
                RefCell::new(
                    Box::new(
                        Expressions {
                            values: vec![]
                        }
                    )
                )
            );
        }

        if let Some(thing) = ast {
            return Rc::new(
                RefCell::new(
                    thing
                )
            );
        }

        return Rc::new(
            RefCell::new(
                Box::new(
                    Text {
                        value: "[error]".to_owned()
                    }
                )
            )
        );
    }

    pub fn new(
        backend: &'a mut (dyn Stream<Token> + 'a),
    ) -> Parser<'a> {
        return Parser::<'a> {
            grammar: get_grammar(),
            backend: Liner::<'a>::new(backend),
            last_ast: Rc::new(
                RefCell::new(
                    Box::new(
                        Expressions {
                            values: vec![]
                        }
                    )
                )
            ),
            end_token_met: false
        };
    }
}

impl <'a> Stream<Rc<RefCell<Box<dyn Node>>>> for Parser<'a> {
    fn has_next(&self) -> bool {
        return !self.end_token_met;
    }

    fn grab(&mut self) -> Rc<RefCell<Box<dyn Node>>> {
        return self.parse();
    }

    fn get_offset(&self) -> usize {
        return self.backend.get_offset();
    }
}
