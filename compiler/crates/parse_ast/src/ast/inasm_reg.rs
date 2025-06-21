use sb_compiler_parse_cst::Span;

use super::Visitor;

#[derive(Debug)]
pub enum InlineAsmReg<'input> {
    RawReg {
        num: u8,
    },
    TmpReg {
        num: u32,
    },
    Var {
        name: Span<'input>,
    },
}

impl<'input> From<Visitor<'input>> for InlineAsmReg<'input> {
    fn from(mut visitor: Visitor<'input>) -> Self {
        let reg = visitor.expect_leaf().1;
        let reg_s = reg.as_str();

        // 生レジスタ
        if reg_s.starts_with("R") {
            let num = reg_s[1..].parse().unwrap();
            return InlineAsmReg::RawReg { num };
        }

        // 一時レジスタ
        if reg_s.starts_with("T") {
            let num = reg_s[1..].parse().unwrap();
            return InlineAsmReg::TmpReg { num };
        }

        // 変数
        InlineAsmReg::Var {
            name: reg,
        }
    }
}
