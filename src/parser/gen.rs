use super::*;
use rand::Rng;

impl Ast {
    pub fn bnf(&self) -> String {
        match self {
            Ast::Bnf (b) => {
                format!("[BNF] {} ::= {}", b.term.bnf(), b.stmt.bnf())
            }
            Ast::Expr(Expr::LetterE) => "".to_string(),
            Ast::Expr(Expr::Expr0Remain {
                expr0: e0,
                remain_expr: r,
            }) => {
                format!("{} {}", e0.bnf(), r.bnf())
            }
            Ast::Expr0(Expr0::Terminal { name: n }) => n.bnf(),
            Ast::Expr0(Expr0::NonTerminal { term: t }) => t.bnf(),
            Ast::Name(Name::Epsilon) => "".to_string(),
            Ast::Name(Name::HeadTail { head: h, tail: t }) => {
                format!("{}{}", h, t.bnf())
            }
            Ast::RemainExpr(RemainExpr::Epsilon) => "".to_string(),
            Ast::RemainExpr(RemainExpr::Expr { expr: e }) => e.bnf(),
            Ast::RemainStmt(RemainStmt::Epsilon) => "".to_string(),
            Ast::RemainStmt(RemainStmt::OrStmt { stmt: s }) => {
                format!("| {}", s.bnf())
            }
            Ast::Stmt {
                expr: e,
                remain_stmt: r,
            } => {
                format!("{}{}", e.bnf(), r.bnf())
            }
            Ast::Term { name: n } => {
                format!("<{}>", n.bnf())
            }
            Ast::Epsilon => "".to_string(),
        }
    }

    pub fn gen(&self) -> String {
        match self {
            Ast::Bnf (b) => {
                format!("[GEN] {} ::= {}", b.term.bnf(), b.stmt.gen())
            }
            Ast::Expr(Expr::LetterE) => "".to_string(),
            Ast::Expr(Expr::Expr0Remain {
                expr0: e0,
                remain_expr: r,
            }) => {
                format!("{} {}", e0.gen(), r.gen())
            }
            Ast::Expr0(Expr0::Terminal { name: n }) => n.gen(),
            Ast::Expr0(Expr0::NonTerminal { term: t }) => t.gen(),
            Ast::Name(Name::Epsilon) => "".to_string(),
            Ast::Name(Name::HeadTail { head: h, tail: t }) => {
                format!("{}{}", h, t.gen())
            }
            Ast::RemainExpr(RemainExpr::Epsilon) => "".to_string(),
            Ast::RemainExpr(RemainExpr::Expr { expr: e }) => e.gen(),
            Ast::RemainStmt(RemainStmt::Epsilon) => "".to_string(),
            Ast::RemainStmt(RemainStmt::OrStmt { stmt: s }) => {
                format!("{}", s.gen())
            }
            Ast::Stmt {
                expr: e,
                remain_stmt: r,
            } => {
                if let Ast::RemainStmt(RemainStmt::OrStmt { .. }) = &**r {
                    let mut rng = rand::thread_rng();

                    let rnd: f32 = rng.gen();
                    let p = 0.80;
                    if rnd < p {
                        r.gen()
                    } else {
                        e.gen()
                    }
                } else {
                    e.gen()
                }
            }
            Ast::Term { name: n } => n.gen(),
            Ast::Epsilon => "".to_string(),
        }
    }
}
