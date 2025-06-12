use sb_compiler_parse_syntax::SBRule;

use super::{VarDecl, Block, Return, If, While, For, InlineAsm, Expr, Visitor};

#[derive(Debug)]
pub enum Stmt {
    VarDecl {
        var_decl: VarDecl,
    },
    Block {
        block: Block,
    },
    Expr {
        expr: Expr,
    },
    Return {
        r#return: Return,
    },
    If {
        r#if: If,
    },
    While {
        r#while: While,
    },
    For {
        r#for: For,
    },
    InlineAsm {
        inline_asm: InlineAsm,
    },
}

impl From<Visitor<'_>> for Stmt {
    fn from(mut visitor: Visitor<'_>) -> Self {
        match visitor.peek().1 {
            Some(SBRule::VarDecl) => {
                Stmt::VarDecl {
                    var_decl: visitor.expect_node::<VarDecl>(),
                }
            }
            Some(SBRule::Block) => {
                Stmt::Block {
                    block: visitor.expect_node::<Block>(),
                }
            }
            Some(SBRule::Expr) => {
                Stmt::Expr {
                    expr: visitor.expect_node::<Expr>(),
                }
            }
            Some(SBRule::Return) => {
                Stmt::Return {
                    r#return: visitor.expect_node::<Return>(),
                }
            }
            Some(SBRule::If) => {
                Stmt::If {
                    r#if: visitor.expect_node::<If>(),
                }
            }
            Some(SBRule::While) => {
                Stmt::While {
                    r#while: visitor.expect_node::<While>(),
                }
            }
            Some(SBRule::For) => {
                Stmt::For {
                    r#for: visitor.expect_node::<For>(),
                }
            }
            Some(SBRule::InlineAsm) => {
                Stmt::InlineAsm {
                    inline_asm: visitor.expect_node::<InlineAsm>(),
                }
            }
            _ => unreachable!(),
        }
    }
}
