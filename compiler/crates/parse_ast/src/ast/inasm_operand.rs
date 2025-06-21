use sb_compiler_parse_cst::Span;

use super::Visitor;

#[derive(Debug)]
pub enum InlineAsmOperandL<'src> {
    Reg {
        num: u8,
    },
    Var {
        name: Span<'src>,
    },
}

impl<'src> From<Visitor<'src>> for InlineAsmOperandL<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
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
pub enum InlineAsmOperandR<'src> {
    Reg {
        num: u8,
    },
    Var {
        name: Span<'src>,
    },
}

impl<'src> From<Visitor<'src>> for InlineAsmOperandR<'src> {
    fn from(mut visitor: Visitor<'src>) -> Self {
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
