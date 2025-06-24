use sb_compiler_parse_cst::{Span, Spanned};

use super::Visitor;

#[derive(Debug)]
pub enum InlineAsmOperandL<'src> {
    Reg {
        span: Span<'src>,
        num: u8,
    },
    Var {
        name: Span<'src>,
    },
}

impl<'src> From<Visitor<'src>> for InlineAsmOperandL<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        let span = visitor.span();
        let operand = visitor.expect_leaf().1;
        let operand_s = operand.as_str();

        // 生レジスタ
        if operand_s.starts_with("R") {
            let num = operand_s[1..].parse().unwrap();
            return InlineAsmOperandL::Reg { span, num };
        }

        // 変数
        InlineAsmOperandL::Var { name: operand }
    }
}

impl<'src> Spanned<'src> for InlineAsmOperandL<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            InlineAsmOperandL::Reg { span, .. } => *span,
            InlineAsmOperandL::Var { name } => *name,
        }
    }
}

#[derive(Debug)]
pub enum InlineAsmOperandR<'src> {
    Reg {
        span: Span<'src>,
        num: u8,
    },
    Var {
        name: Span<'src>,
    },
}

impl<'src> From<Visitor<'src>> for InlineAsmOperandR<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
        let span = visitor.span();
        let operand = visitor.expect_leaf().1;
        let operand_s = operand.as_str();

        // 生レジスタ
        if operand_s.starts_with("R") {
            let num = operand_s[1..].parse().unwrap();
            return InlineAsmOperandR::Reg { span, num };
        }

        // 変数
        InlineAsmOperandR::Var { name: operand }
    }
}

impl<'src> Spanned<'src> for InlineAsmOperandR<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            InlineAsmOperandR::Reg { span, .. } => *span,
            InlineAsmOperandR::Var { name } => *name,
        }
    }
}
