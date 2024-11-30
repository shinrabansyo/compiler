use sb_compiler_parse_ast::Assign;
use sb_compiler_analyze::AnalyzeResult;
use sb_compiler_lirgen_ir::{lir,LIR, Add, Sub, ShiftL, ShiftR, ShiftRa, Push, Pop, Sw, Lw};

use super::{lirgen_logic_or, TMP_REG, TMP_REG_L, TMP_REG_R, VARBASE_REG, ZERO_REG};

pub fn lirgen_assign(lirs: &mut Vec<LIR>, assign: &Assign, analyze_result: &AnalyzeResult) {
    let get_reg_and_addr = |namespace, name| {
        let node_info = analyze_result.find(namespace, name);
        let addr = node_info.local_addr;
        let base_reg = if node_info.namespace == "global" {
            ZERO_REG
        } else {
            VARBASE_REG
        };
        (base_reg, addr)
    };

    match assign {
        Assign::Normal { namespace, ident, assign: child_assign, .. } => {
            lirgen_assign(lirs, child_assign, analyze_result);
            lirs.push(lir!(Pop TMP_REG));

            let (base_reg, addr) = get_reg_and_addr(namespace, ident);
            lirs.push(lir!(Sw base_reg, addr, TMP_REG));
            lirs.push(lir!(Push TMP_REG));
        }
        Assign::Plus { namespace, ident, assign: child_assign, .. } => {
            lirgen_assign(lirs, child_assign, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));

            let (base_reg, addr) = get_reg_and_addr(namespace, ident);
            lirs.push(lir!(Lw TMP_REG_L, base_reg, addr));
            lirs.push(lir!(Add TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Sw base_reg, addr, TMP_REG_L));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Assign::Minus { namespace, ident, assign: child_assign, .. } => {
            lirgen_assign(lirs, child_assign, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));

            let (base_reg, addr) = get_reg_and_addr(namespace, ident);
            lirs.push(lir!(Lw TMP_REG_L, base_reg, addr));
            lirs.push(lir!(Sub TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Sw base_reg, addr, TMP_REG_L));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Assign::ShiftL { namespace, ident, assign: child_assign, .. } => {
            lirgen_assign(lirs, child_assign, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));

            let (base_reg, addr) = get_reg_and_addr(namespace, ident);
            lirs.push(lir!(Lw TMP_REG_L, base_reg, addr));
            lirs.push(lir!(ShiftL TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Sw base_reg, addr, TMP_REG_L));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Assign::ShiftR { namespace, ident, assign: child_assign, .. } => {
            lirgen_assign(lirs, child_assign, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));

            let (base_reg, addr) = get_reg_and_addr(namespace, ident);
            lirs.push(lir!(Lw TMP_REG_L, base_reg, addr));
            lirs.push(lir!(ShiftR TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Sw base_reg, addr, TMP_REG_L));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Assign::ShiftRa { namespace, ident, assign: child_assign } => {
            lirgen_assign(lirs, child_assign, analyze_result);
            lirs.push(lir!(Pop TMP_REG_R));

            let (base_reg, addr) = get_reg_and_addr(namespace, ident);
            lirs.push(lir!(Lw TMP_REG_L, base_reg, addr));
            lirs.push(lir!(ShiftRa TMP_REG_L, TMP_REG_R));
            lirs.push(lir!(Sw base_reg, addr, TMP_REG_L));
            lirs.push(lir!(Push TMP_REG_L));
        }
        Assign::LogicOr { or, .. } => {
            lirgen_logic_or(lirs, or, analyze_result);
            return;
        }
    };
}
