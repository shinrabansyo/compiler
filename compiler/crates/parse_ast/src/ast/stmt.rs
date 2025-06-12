use sb_compiler_parse_syntax::SBRule;

use super::{VarDecl, Block, Return, If, While, For, InlineAsm, Expr, Visitor};

#[derive(Debug)]
pub enum Stmt<'input> {
    VarDecl {
        var_decl: VarDecl<'input>,
    },
    Block {
        block: Block<'input>,
    },
    Expr {
        expr: Expr<'input>,
    },
    Return {
        r#return: Return<'input>,
    },
    If {
        r#if: If<'input>,
    },
    While {
        r#while: While<'input>,
    },
    For {
        r#for: For<'input>,
    },
    InlineAsm {
        inline_asm: InlineAsm<'input>,
    },
}

impl<'input> From<Visitor<'input>> for Stmt<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
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
