/*
this file is a hand crafted minimum BNF parser
*/

pub enum AstNodeType {
    Bnf,
    Term,
    Stmt,
    RemainStmt,
    Expr,
    Expr0,
    RemainExpr,
    Name,
}

//<remain_stmt>::=E|"|"<stmt>
pub enum RemainStmt {
    Epsilon,
    OrStmt { stmt: Box<Ast> },
}

//<expr>::="E"|<expr0><remain_expr>
pub enum Expr {
    LetterE,
    Expr0Remain {
        expr0: Box<Ast>,
        remain_expr: Box<Ast>,
    },
}

//<expr0>::="<"<name>">"|"\""<name>"\""
pub enum Expr0 {
    NonTerminal { name: Box<Ast> },
    Terminal { name: Box<Ast> },
}

//<remain_expr>::=E|<expr>
pub enum RemainExpr {
    Epsilon,
    Expr { expr: Box<Ast> },
}

//<name>::="a"<name>|E
pub enum Name {
    Epsilon,
    HeadTail {
        head: String, // head is String of only one char
        tail: Box<Ast>,
    },
}

pub enum Ast {
    Bnf {
        term: Box<Ast>,
        stmt: Box<Ast>,
    },
    Term {
        name: Box<Ast>,
    },
    Stmt {
        expr: Box<Ast>,
        remain_stmt: Box<Ast>,
    },
    RemainStmt(RemainStmt),
    Expr(Expr),
    Expr0(Expr0),
    RemainExpr(RemainExpr),
    Name(Name),
    Epsilon,
}

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
            Ast::Bnf { stmt: s, term: t } => {
                let mut ret = vec![vec![(0, "bnf ::=".to_string())]];
                ret.append(&mut vec_add(t.mk_str_vec(), s.mk_str_vec()));
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
            Ast::Expr0(Expr0::NonTerminal { name: n }) => {
                let mut ret = vec![vec![(0, "Expr".to_string())]];
                ret.append(&mut n.mk_str_vec());
                ret[1].push((1, "<>".to_string()));
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

    pub fn show(&self) {
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

pub fn parse(b: &str) -> Result<Ast, String> {
    Ok(parse_bnf(b, AstNodeType::Bnf)?.r)
}

pub struct ParseResult<'a> {
    r: Ast,           // Ast node
    matched: &'a str, // matched str
    remain: &'a str,  // remain str
}

impl<'a> ParseResult<'a> {
    fn len(&self) -> usize {
        self.matched.len()
    }
}

pub fn parse_bnf<'a>(bnfstr: &'a str, state: AstNodeType) -> Result<ParseResult<'a>, String> {
    /*
    bnfstr
        .chars()
        .zip(bnfstr.chars().skip(1))
        .for_each(|(t, lookahead)| println!("{}{}", t, lookahead));
        */

    let match_chars = |caller: &'a str, s: &'a str, p: &'a str| {
        if s.len() >= p.len() && &s[..p.len()] == p {
            Ok(ParseResult {
                r: Ast::Epsilon,
                matched: &s[..p.len()],
                remain: &s[p.len()..],
            })
        } else {
            Err(format!("[{}] expect {}, found {}\n", caller, p, s))
        }
    };

    match state {
        AstNodeType::Bnf => {
            // <bnf>::=<term>"::="<stmt>
            let t = parse_bnf(bnfstr, AstNodeType::Term)?;
            let comma2_eq = match_chars("bnf", t.remain, "::=")?;
            let s = parse_bnf(comma2_eq.remain, AstNodeType::Stmt)?;
            Ok(ParseResult {
                matched: &bnfstr[..t.len() + comma2_eq.len() + s.len()],
                remain: &bnfstr[t.len() + comma2_eq.len() + s.len()..],
                r: Ast::Bnf {
                    term: Box::new(t.r),
                    stmt: Box::new(s.r),
                },
            })
        }
        AstNodeType::Term => {
            // <term>::="<"<name>">"
            let left_angle_bracket = match_chars("term", bnfstr, "<")?;
            let n = parse_bnf(left_angle_bracket.remain, AstNodeType::Name)?;
            let right_angle_bracket = match_chars("term", n.remain, ">")?;
            Ok(ParseResult {
                matched: &bnfstr[..right_angle_bracket.len() + n.len() + left_angle_bracket.len()],
                remain: &bnfstr[right_angle_bracket.len() + n.len() + left_angle_bracket.len()..],
                r: Ast::Term {
                    name: Box::new(n.r),
                },
            })
        }
        AstNodeType::Stmt => {
            // <stmt>::=<expr><remain_stmt>
            let e = parse_bnf(bnfstr, AstNodeType::Expr)?;
            let r = parse_bnf(e.remain, AstNodeType::RemainStmt)?;
            Ok(ParseResult {
                matched: &bnfstr[..e.len() + r.len()],
                remain: &bnfstr[e.len() + r.len()..],
                r: Ast::Stmt {
                    expr: Box::new(e.r),
                    remain_stmt: Box::new(r.r),
                },
            })
        }
        AstNodeType::RemainStmt => {
            // <remain_stmt>::=E|"|"<stmt>
            // try "|"<stmt>
            match bnfstr.len() {
                1.. => {
                    let vertical_line = match_chars("remain stmt", bnfstr, "|")?;
                    let s = parse_bnf(vertical_line.remain, AstNodeType::Stmt)?;
                    Ok(ParseResult {
                        matched: &bnfstr[..vertical_line.len() + s.len()],
                        remain: &bnfstr[vertical_line.len() + s.len()..],
                        r: Ast::RemainStmt(RemainStmt::OrStmt {
                            stmt: Box::new(s.r),
                        }),
                    })
                }
                0 =>
                // match E
                {
                    Ok(ParseResult {
                        r: Ast::RemainStmt(RemainStmt::Epsilon),
                        matched: "",
                        remain: bnfstr,
                    })
                }
                _ => Err(format!(
                    "[<remain_stmt>::=E|\"|\"<stmt>] expect | or end of line, found {}",
                    bnfstr
                )),
            }
        }
        AstNodeType::Expr => {
            // <expr>::="E"|<expr0><remain_expr>
            match bnfstr.len() {
                1.. => {
                    // try <expr0><remain_expr>, FIRST(<expr0>) = <"
                    if &bnfstr[..1] == "<" || &bnfstr[..1] == "\"" {
                        let e0 = parse_bnf(bnfstr, AstNodeType::Expr0)?;
                        let r = parse_bnf(e0.remain, AstNodeType::RemainExpr)?;
                        Ok(ParseResult {
                            matched: &bnfstr[..r.len() + e0.len()],
                            remain: &bnfstr[r.len() + e0.len()..],
                            r: Ast::Expr(Expr::Expr0Remain {
                                expr0: Box::new(e0.r),
                                remain_expr: Box::new(r.r),
                            }),
                        })
                    } else if &bnfstr[..1] == "E" {
                        // try "E"
                        let e = match_chars("expr", bnfstr, "E")?;
                        Ok(ParseResult {
                            r: Ast::Expr(Expr::LetterE),
                            matched: e.matched,
                            remain: e.remain,
                        })
                    } else {
                        Err(format!(
                            "[<expr>::=\"E\"|<expr0><remain_expr>] expect E or |, found {}",
                            bnfstr
                        ))
                    }
                }
                _ => Err(format!("expect <expr>, found nothing in {}", bnfstr)),
            }
        }
        AstNodeType::Expr0 => {
            // <expr0>::="<"<name>">"|"\""<name>"\""
            match bnfstr.len() {
                1.. => {
                    if &bnfstr[..1] == "<" {
                        // try "<"<name>">"
                        let left_angle_bracket = match_chars("expr0", bnfstr, "<")?;
                        let n = parse_bnf(left_angle_bracket.remain, AstNodeType::Name)?;
                        let right_angle_bracket = match_chars("expr0", n.remain, ">")?;
                        Ok(ParseResult {
                            matched: &bnfstr
                                [..left_angle_bracket.len() + n.len() + right_angle_bracket.len()],
                            remain: &bnfstr
                                [left_angle_bracket.len() + n.len() + right_angle_bracket.len()..],
                            r: Ast::Expr0(Expr0::NonTerminal {
                                name: Box::new(n.r),
                            }),
                        })
                    } else if &bnfstr[..1] == "\"" {
                        // try "\""<name>"\""
                        let left_quote = match_chars("expr0", bnfstr, "\"")?;
                        let n = parse_bnf(left_quote.remain, AstNodeType::Name)?;
                        let right_quote = match_chars("expr0", n.remain, "\"")?;
                        Ok(ParseResult {
                            matched: &bnfstr[..right_quote.len() + n.len() + left_quote.len()],
                            remain: &bnfstr[right_quote.len() + n.len() + left_quote.len()..],
                            r: Ast::Expr0(Expr0::Terminal {
                                name: Box::new(n.r),
                            }),
                        })
                    } else {
                        Err(format!(
                            r#"[<expr0>::="<"<name>">"|"""<name>"""] expect <,| found {}"#,
                            bnfstr
                        ))
                    }
                }
                _ => Err("[<expr0>] expect <,|, found nothing".to_string()),
            }
        }
        AstNodeType::RemainExpr => {
            // <remain_expr>::=E|<expr>
            match bnfstr.len() {
                1.. => {
                    if "E<\"".chars().any(|x| x.to_string() == bnfstr[..1]) {
                        // try <expr>, FIRST(expr) = E<"
                        let e = parse_bnf(bnfstr, AstNodeType::Expr)?;
                        Ok(ParseResult {
                            matched: e.matched,
                            remain: &bnfstr[e.len()..],
                            r: Ast::RemainExpr(RemainExpr::Expr {
                                expr: Box::new(e.r),
                            }),
                        })
                    } else if "|" == &bnfstr[..1] {
                        // try E, FOLLOW(remain expr) = |$, $ for endmark
                        Ok(ParseResult {
                            r: Ast::RemainExpr(RemainExpr::Epsilon),
                            matched: "",
                            remain: bnfstr,
                        })
                    } else {
                        Err(format!(
                            "[<remain_expr>::=E|<expr>] expect E<\"|, found {}\n",
                            bnfstr
                        ))
                    }
                }
                0 => Ok(ParseResult {
                    // remain expr is epsilon
                    r: Ast::RemainExpr(RemainExpr::Epsilon),
                    matched: "",
                    remain: bnfstr,
                }),
                _ => Err("no match rule".to_string()),
            }
        }
        AstNodeType::Name => {
            // <name>::="a"<name>|E
            let supported_chars = "abcdefghijklmnopqrstuvwxyz\
            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            0123456789 +-*/";
            match bnfstr.len() {
                1.. => {
                    if supported_chars
                        .chars()
                        .any(|x| x.to_string() == bnfstr[..1])
                    {
                        // try "a"<name>
                        let first = match_chars("name", bnfstr, &bnfstr[..1])?;
                        let rest = parse_bnf(first.remain, AstNodeType::Name)?;
                        Ok(ParseResult {
                            r: Ast::Name(Name::HeadTail {
                                head: bnfstr[..1].to_string(),
                                tail: Box::new(rest.r),
                            }),
                            matched: &bnfstr[..first.matched.len() + rest.matched.len()],
                            remain: &bnfstr[first.matched.len() + rest.matched.len()..],
                        })
                    } else if ">\"".chars().any(|x| x.to_string() == bnfstr[..1]) {
                        // elsilon
                        Ok(ParseResult {
                            r: Ast::Name(Name::Epsilon),
                            matched: "",
                            remain: bnfstr,
                        })
                    } else {
                        Err(format!(
                            "[<name>::=\"a\"<name>|E] expect \'{}\' or >\"|, found {}\n",
                            supported_chars, bnfstr
                        ))
                    }
                }
                0 => {
                    // epsilon
                    Ok(ParseResult {
                        r: Ast::Name(Name::Epsilon),
                        matched: "",
                        remain: bnfstr,
                    })
                }
                _ => Err(format!("no match rule for {}", bnfstr)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bnf() {
        assert_eq!(
            parse_bnf("<a>::=<aa>", AstNodeType::Bnf).unwrap().matched,
            "<a>::=<aa>"
        );
        assert_eq!(
            parse_bnf(r#"<a>::=<aa>|E|"aa"<a>"#, AstNodeType::Bnf)
                .unwrap()
                .matched,
            r#"<a>::=<aa>|E|"aa"<a>"#
        );
    }

    #[test]
    fn name() {
        assert_eq!(parse_bnf("aaa", AstNodeType::Name).unwrap().matched, "aaa");
        assert_eq!(parse_bnf("aaa>", AstNodeType::Name).unwrap().remain, ">");
        assert_eq!(parse_bnf("", AstNodeType::Name).unwrap().matched, "");
    }

    #[test]
    fn term() {
        assert_eq!(
            parse_bnf("<aaa>", AstNodeType::Term).unwrap().matched,
            "<aaa>"
        );
    }

    #[test]
    fn expr0() {
        assert_eq!(
            parse_bnf("<aaa>", AstNodeType::Expr0).unwrap().matched,
            "<aaa>"
        );
        assert_eq!(
            parse_bnf(r#""aaa""#, AstNodeType::Expr0).unwrap().matched,
            r#""aaa""#
        );
    }

    #[test]
    fn expr() {
        assert_eq!(
            parse_bnf("<aaa><aa>", AstNodeType::Expr).unwrap().matched,
            "<aaa><aa>"
        );
        assert_eq!(
            parse_bnf("<aaa>\"aa\"<aa>\"a\"", AstNodeType::Expr)
                .unwrap()
                .matched,
            "<aaa>\"aa\"<aa>\"a\""
        );
        assert_eq!(parse_bnf("E", AstNodeType::Expr).unwrap().matched, "E");
    }

    #[test]
    fn stmt() {
        assert_eq!(
            parse_bnf(r#"<aaa>"a"<aa>"aa"|"a"<aaaa>"a"|E"#, AstNodeType::Stmt)
                .unwrap()
                .matched,
            r#"<aaa>"a"<aa>"aa"|"a"<aaaa>"a"|E"#
        );
    }
}
