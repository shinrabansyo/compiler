use sb_compiler_parse_syntax::{SBToken, SBRule};

use super::{VarDecl, Block, If, While, For, InlineAsm, Expr, Visitor};

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
        expr: Expr,
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
        match visitor.peek() {
            (_, Some(SBRule::VarDecl)) => {
                Stmt::VarDecl {
                    var_decl: visitor.expect_node::<VarDecl>(),
                }
            }
            (_, Some(SBRule::Block)) => {
                Stmt::Block {
                    block: visitor.expect_node::<Block>(),
                }
            }
            (_, Some(SBRule::Expr)) => {
                Stmt::Expr {
                    expr: visitor.expect_node::<Expr>(),
                }
            }
            (Some(SBToken::Return), _) => {
                Stmt::Return {
                    expr: visitor.expect_node::<Expr>(),
                }
            }
            (_, Some(SBRule::If)) => {
                Stmt::If {
                    r#if: visitor.expect_node::<If>(),
                }
            }
            (_, Some(SBRule::While)) => {
                Stmt::While {
                    r#while: visitor.expect_node::<While>(),
                }
            }
            (_, Some(SBRule::For)) => {
                Stmt::For {
                    r#for: visitor.expect_node::<For>(),
                }
            }
            (_, Some(SBRule::InlineAsm)) => {
                Stmt::InlineAsm {
                    inline_asm: visitor.expect_node::<InlineAsm>(),
                }
            }
            _ => unreachable!(),
        }
    }
}
