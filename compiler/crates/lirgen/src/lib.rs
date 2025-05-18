mod gen;

use sb_compiler_parse_ast::Program;
use sb_compiler_lirgen_ir::LirTopElem;
use sb_compiler_utils::collections::LayeredTable;

const ZERO_REG: u32 = 0;
const RET_REG: u32 = 1;
const FARG_REG_BASE: u32 = 10;

#[derive(Debug, Clone)]
pub(crate) struct GenContext {
    reserved_regs: u32,
    reserved_labels: u32,
    sym_table: LayeredTable<String, u32>,
}

impl Default for GenContext {
    fn default() -> Self {
        Self {
            reserved_regs: 20,   // r0: ゼロレジスタ, r10 ~ r19: 引数レジスタ として確保済み
            reserved_labels: 0,
            sym_table: LayeredTable::default(),
        }
    }
}

impl GenContext {
    fn alloc_reg(&mut self) -> u32 {
        let allocated_reg = self.reserved_regs;
        self.reserved_regs += 1;
        allocated_reg
    }

    fn alloc_label(&mut self) -> u32 {
        let allocated_label = self.reserved_labels;
        self.reserved_labels += 1;
        allocated_label
    }
}

pub fn lirgen<'ast>(program: &'ast Program) -> Vec<LirTopElem> {
    let mut ctx = GenContext::default();
    gen::lirgen_program(&mut ctx, program)
}
