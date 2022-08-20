use super::*;
use rand::Rng;

impl Ast {
    pub fn bnf(&self) -> String {
        match self {
            Ast::Bnf(b) => {
                format!("[BNF] {} ::= {}", b.term.bnf(), b.stmt.bnf())
            }
            Ast::Expr(Expr::LetterE) => "".to_string(),
            Ast::Expr(Expr::Expr0Remain {
                expr0: e0,
                remain_expr: r,
            }) => {
                format!("{} {}", e0.bnf(), r.bnf())
            }
            Ast::Expr0(Expr0::Terminal { name: n }) => format!("\"{}\"", n.bnf()),
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
                ..
            } => {
                format!("{}{}", e.bnf(), r.bnf())
            }
            Ast::Term { name: n } => {
                format!("<{}>", n.bnf())
            }
            Ast::Epsilon => "".to_string(),
        }
    }
}
