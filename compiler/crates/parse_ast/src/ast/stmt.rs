use sb_compiler_parse_syntax::{SBTokens, SBRules};

use super::{VarDecl, Block, If, While, For, InlineAsm, Expr, Visitor};

#[derive(Debug)]
pub enum Stmt {
    VarDecl {
        namespace: String,
        var_decl: VarDecl,
    },
    Block {
        namespace: String,
        block: Block,
    },
    Expr {
        namespace: String,
        expr: Expr,
    },
    Return {
        namespace: String,
        expr: Expr,
    },
    If {
        namespace: String,
        r#if: If,
    },
    While {
        namespace: String,
        r#while: While,
    },
    For {
        namespace: String,
        r#for: For,
    },
    InlineAsm {
        namespace: String,
        inline_asm: InlineAsm,
    },
}

impl From<(String, Visitor<'_>)> for Stmt {
    fn from((namespace, mut visitor): (String, Visitor<'_>)) -> Self {
        match visitor.peek() {
            (_, Some(SBRules::VarDecl)) => {
                Stmt::VarDecl {
                    namespace,
                    var_decl: visitor.expect_node::<VarDecl>(),
                }
            }
            (_, Some(SBRules::Block)) => {
                Stmt::Block {
                    namespace,
                    block: visitor.expect_node::<Block>(),
                }
            }
            (_, Some(SBRules::Expr)) => {
                Stmt::Expr {
                    namespace,
                    expr: visitor.expect_node::<Expr>(),
                }
            }
            (Some(SBTokens::Return), _) => {
                Stmt::Return {
                    namespace,
                    expr: visitor.expect_node::<Expr>(),
                }
            }
            (_, Some(SBRules::If)) => {
                Stmt::If {
                    namespace,
                    r#if: visitor.expect_node::<If>(),
                }
            }
            (_, Some(SBRules::While)) => {
                Stmt::While {
                    namespace,
                    r#while: visitor.expect_node::<While>(),
                }
            }
            (_, Some(SBRules::For)) => {
                Stmt::For {
                    namespace,
                    r#for: visitor.expect_node::<For>(),
                }
            }
            (_, Some(SBRules::InlineAsm)) => {
                Stmt::InlineAsm {
                    namespace,
                    inline_asm: visitor.expect_node::<InlineAsm>(),
                }
            }
            _ => unreachable!(),
        }
    }
}
