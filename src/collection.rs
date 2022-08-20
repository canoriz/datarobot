use rand::Rng;

use crate::parser::{self, parse, Ast, *};
use std::collections::HashMap;

pub struct Collection {
    h: HashMap<String, Ast>,
}

impl Collection {
    pub fn add(&mut self, bnf_expr: &str) -> Result<(), String> {
        let parse_result = parser::parse(bnf_expr);
        match parse_result {
            Ok(Ast::Bnf(b)) => {
                self.h.insert(b.term.bnf(), Ast::Bnf(b));
                Ok(())
            }
            Err(e) => Err(format!("parse {} failed, error: {}", bnf_expr, e)),
            _ => Err("Not valid bnf".to_string()),
        }
    }

    pub fn new() -> Self {
        Self { h: HashMap::new() }
    }

    pub fn gen(&self, bnf: &str) -> Result<String, String> {
        fn gen_from_ast(ast: &Ast, bnfs: &HashMap<String, Ast>) -> Result<String, String> {
            let mut stack = Vec::<&Ast>::new();
            let mut text = "".to_string();
            stack.push(ast);
            while !stack.is_empty() {
                let top_ast = stack.pop().ok_or_else(|| "Stack error".to_string())?;
                match top_ast {
                    Ast::Bnf(b) => {
                        stack.push(&b.stmt);
                    }
                    Ast::Expr(Expr::LetterE) => (),
                    Ast::Expr(Expr::Expr0Remain {
                        expr0: e0,
                        remain_expr: r,
                    }) => {
                        stack.push(r);
                        stack.push(e0);
                    }
                    Ast::Expr0(Expr0::Terminal { name: n }) => {
                        stack.push(n);
                    }
                    Ast::Expr0(Expr0::NonTerminal { term: t }) => match bnfs.get(&t.bnf()) {
                        Some(ast) => {
                            stack.push(ast);
                        }
                        None => {
                            return Err(format!("No production rule for {}", t.bnf()));
                        }
                    },
                    Ast::Name(Name::Epsilon) => (),
                    Ast::Name(Name::HeadTail { head: h, tail: t }) => {
                        stack.push(t);
                        text += h;
                    }
                    Ast::RemainExpr(RemainExpr::Epsilon) => (),
                    Ast::RemainExpr(RemainExpr::Expr { expr: e }) => {
                        stack.push(e);
                    }
                    Ast::RemainStmt(RemainStmt::Epsilon) => (),
                    Ast::RemainStmt(RemainStmt::OrStmt { stmt: s }) => {
                        stack.push(s);
                    }
                    Ast::Stmt {
                        expr: e,
                        remain_stmt: r,
                        parallels: par,
                    } => {
                        if let Ast::RemainStmt(RemainStmt::OrStmt { .. }) = &**r {
                            let mut rng = rand::thread_rng();

                            let rnd: f32 = rng.gen();
                            // the more length we have, the less we tend to jump
                            let p = 1.0 / ((1 + text.len()) as f32 / 50.0 + 1f32);
                            let uniform = (par - 1) as f32 / *par as f32;
                            //println!("par {} ast {}", par, r.bnf());
                            if rnd < p && rnd < uniform {
                                stack.push(r);
                            } else {
                                stack.push(e);
                            }
                        } else {
                            stack.push(e);
                        }
                    }
                    Ast::Term { name: n } => {
                        stack.push(n);
                    }
                    Ast::Epsilon => (),
                };
            }
            Ok(text)
        }

        let ast = self
            .h
            .get(bnf)
            .ok_or_else(|| format!("No production rule for {}", bnf))?;
        gen_from_ast(ast, &self.h)
    }
}
