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
            match ast {
                Ast::Bnf(b) => gen_from_ast(&b.stmt, bnfs),
                Ast::Expr(Expr::LetterE) => Ok("".to_string()),
                Ast::Expr(Expr::Expr0Remain {
                    expr0: e0,
                    remain_expr: r,
                }) => Ok(format!(
                    "{}{}",
                    gen_from_ast(e0, bnfs)?,
                    gen_from_ast(r, bnfs)?
                )),
                Ast::Expr0(Expr0::Terminal { name: n }) => gen_from_ast(n, bnfs),
                Ast::Expr0(Expr0::NonTerminal { term: t }) => match bnfs.get(&t.bnf()) {
                    Some(ast) => gen_from_ast(ast, bnfs),
                    None => Err(format!("No production rule for {}", t.bnf())),
                },
                Ast::Name(Name::Epsilon) => Ok("".to_string()),
                Ast::Name(Name::HeadTail { head: h, tail: t }) => {
                    Ok(format!("{}{}", h, gen_from_ast(t, bnfs)?))
                }
                Ast::RemainExpr(RemainExpr::Epsilon) => Ok("".to_string()),
                Ast::RemainExpr(RemainExpr::Expr { expr: e }) => gen_from_ast(e, bnfs),
                Ast::RemainStmt(RemainStmt::Epsilon) => Ok("".to_string()),
                Ast::RemainStmt(RemainStmt::OrStmt { stmt: s }) => gen_from_ast(s, bnfs),
                Ast::Stmt {
                    expr: e,
                    remain_stmt: r,
                } => {
                    if let Ast::RemainStmt(RemainStmt::OrStmt { .. }) = &**r {
                        let mut rng = rand::thread_rng();

                        let rnd: f32 = rng.gen();
                        let p = 0.80;
                        if rnd < p {
                            gen_from_ast(r, bnfs)
                        } else {
                            gen_from_ast(e, bnfs)
                        }
                    } else {
                        gen_from_ast(e, bnfs)
                    }
                }
                Ast::Term { name: n } => gen_from_ast(n, bnfs),
                Ast::Epsilon => Ok("".to_string()),
            }
        }

        let ast = self
            .h
            .get(bnf)
            .ok_or_else(|| format!("No production rule for {}", bnf))?;
        gen_from_ast(ast, &self.h)
    }
}
