use super::*;

impl Ast {
    fn mk_str_vec(&self) -> Vec<Vec<(usize, String)>> {
        let vec_add = |mut v: Vec<Vec<(usize, String)>>, vn: Vec<Vec<(usize, String)>>| {
            if vn.len() > v.len() {
                v.resize(vn.len(), vec![]);
            }
            let mut delay = 1;
            for (level, vi) in vn.into_iter().enumerate() {
                v[level].append(&mut vi.into_iter().map(|(nu, s)| (nu + delay, s)).collect());
                delay *= 2;
            }
            v
        };
        match self {
            Ast::Bnf(b) => {
                let mut ret = vec![vec![(0, "bnf ::=".to_string())]];
                ret.append(&mut vec_add(b.term.mk_str_vec(), b.stmt.mk_str_vec()));
                ret
            }
            Ast::Expr(Expr::LetterE) => {
                vec![vec![(0, "expr".to_string())], vec![(0, "E".to_string())]]
            }
            Ast::Expr(Expr::Expr0Remain {
                expr0: e0,
                remain_expr: r,
            }) => {
                let mut ret = vec![vec![(0, "Expr".to_string())]];
                ret.append(&mut vec_add(e0.mk_str_vec(), r.mk_str_vec()));
                ret
            }
            Ast::Expr0(Expr0::Terminal { name: n }) => {
                let mut ret = vec![vec![(0, "Expr".to_string())]];
                ret.append(&mut n.mk_str_vec());
                ret[1].push((1, "\"\"".to_string()));
                ret
            }
            Ast::Expr0(Expr0::NonTerminal { term: t }) => {
                let mut ret = vec![vec![(0, "Expr".to_string())]];
                ret.append(&mut t.mk_str_vec());
                ret
            }
            Ast::Name(Name::Epsilon) => {
                vec![vec![(0, "Name".to_string())], vec![(0, "e".to_string())]]
            }
            Ast::Name(Name::HeadTail { head: h, tail: t }) => {
                let mut ret = vec![vec![(0, "Name".to_string())]];
                ret.append(&mut vec_add(
                    vec![vec![(0, format!(r#""{}""#, h))]],
                    t.mk_str_vec(),
                ));
                ret
            }
            Ast::RemainExpr(RemainExpr::Epsilon) => {
                vec![
                    vec![(0, "RemainExpr".to_string())],
                    vec![(0, "e".to_string())],
                ]
            }
            Ast::RemainExpr(RemainExpr::Expr { expr: e }) => {
                let mut ret = vec![vec![(0, "RemainExpr".to_string())]];
                ret.append(&mut e.mk_str_vec());
                ret
            }
            Ast::RemainStmt(RemainStmt::Epsilon) => {
                vec![
                    vec![(0, "RemainStmt".to_string())],
                    vec![(0, "e".to_string())],
                ]
            }
            Ast::RemainStmt(RemainStmt::OrStmt { stmt: s }) => {
                let mut ret = vec![vec![(0, "RemainStmt".to_string())]];
                ret.append(&mut vec_add(
                    vec![vec![(0, "|".to_string())]],
                    s.mk_str_vec(),
                ));
                ret
            }
            Ast::Stmt {
                expr: e,
                remain_stmt: r,
                ..
            } => {
                let mut ret = vec![vec![(0, "Stmt".to_string())]];
                ret.append(&mut vec_add(e.mk_str_vec(), r.mk_str_vec()));
                ret
            }
            Ast::Term { name: n } => {
                let mut ret = vec![vec![(0, "Term".to_string())]];
                ret.append(&mut n.mk_str_vec());
                ret[1].push((1, "<>".to_string()));
                ret
            }
            _ => vec![vec![]],
        }
    }

    pub fn display(&self) {
        for (level, line) in self.mk_str_vec().iter().enumerate() {
            print!("#{}  ", level + 1);
            for (nu, word) in line {
                let n_tree = *nu as u32 + 2_u32.pow(level as u32) - 1;
                let pa_n_tree = if n_tree == 0 { 0 } else { (1 + n_tree) / 2 - 1 };
                let pa_nu = if pa_n_tree == 0 {
                    0
                } else {
                    pa_n_tree + 1 - 2_u32.pow(level as u32 - 1)
                };
                let l_or_r = if n_tree % 2 == 1 { "L" } else { "R" };
                print!("[{}, {} of {}]{}  ", nu, l_or_r, pa_nu, word);
            }
            println!();
        }
    }
}
