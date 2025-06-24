use sb_compiler_parse_cst::Spanned;
use sb_compiler_parse_syntax::SBRule;

use super::{VarDecl, Block, Return, If, While, For, InlineAsm, Expr, Visitor};

#[derive(Debug)]
pub enum Stmt<'src> {
    VarDecl {
        var_decl: VarDecl<'src>,
    },
    Block {
        block: Block<'src>,
    },
    Expr {
        expr: Expr<'src>,
    },
    Return {
        r#return: Return<'src>,
    },
    If {
        r#if: If<'src>,
    },
    While {
        r#while: While<'src>,
    },
    For {
        r#for: For<'src>,
    },
    InlineAsm {
        inline_asm: InlineAsm<'src>,
    },
}

impl<'src> From<Visitor<'src>> for Stmt<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
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

impl<'src> Spanned<'src> for Stmt<'src> {
    fn span(&self) -> sb_compiler_parse_cst::Span<'src> {
        match self {
            Stmt::VarDecl { var_decl } => var_decl.span(),
            Stmt::Block { block } => block.span(),
            Stmt::Expr { expr } => expr.span(),
            Stmt::Return { r#return } => r#return.span(),
            Stmt::If { r#if } => r#if.span(),
            Stmt::While { r#while } => r#while.span(),
            Stmt::For { r#for } => r#for.span(),
            Stmt::InlineAsm { inline_asm } => inline_asm.span(),
        }
    }
}
