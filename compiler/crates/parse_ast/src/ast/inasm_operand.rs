use sb_compiler_parse_cst::Span;

use super::Visitor;

#[derive(Debug)]
pub enum InlineAsmOperandL<'input> {
    Reg {
        num: u8,
    },
    Var {
        name: Span<'input>,
    },
}

impl<'input> From<Visitor<'input>> for InlineAsmOperandL<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        let operand = visitor.expect_leaf().1;
        let operand_s = operand.as_str();

        // 生レジスタ
        if operand_s.starts_with("R") {
            let num = operand_s[1..].parse().unwrap();
            return InlineAsmOperandL::Reg { num };
        }

        // 変数
        InlineAsmOperandL::Var { name: operand }
    }
}

#[derive(Debug)]
pub enum InlineAsmOperandR<'input> {
    Reg {
        num: u8,
    },
    Var {
        name: Span<'input>,
    },
}

impl<'input> From<Visitor<'input>> for InlineAsmOperandR<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        let operand = visitor.expect_leaf().1;
        let operand_s = operand.as_str();

        // 生レジスタ
        if operand_s.starts_with("R") {
            let num = operand_s[1..].parse().unwrap();
            return InlineAsmOperandR::Reg { num };
        }

        // 変数
        InlineAsmOperandR::Var { name: operand }
    }
}
